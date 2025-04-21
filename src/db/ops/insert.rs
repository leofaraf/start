use std::{cell::{RefCell, RefMut}, rc::Rc};

use bson::Bson;
use start_storage::StartStorage;

use crate::db::{catalog::collection::{RawDocument}, collection::Collection, operation_context::{ensure_capacity, OperationContext}};

pub fn insert(
    op_ctx: &OperationContext,
    col_meta: Collection,
    raw_document: RawDocument,
    user: bool
) -> usize {
    println!("Inserting...");
    // Parsing collection

    let storage = op_ctx.storage();
    ensure_capacity(&mut storage.borrow_mut(), col_meta.offset + 56).unwrap();
    #[deprecated]
    let collection = Collection::parse(&op_ctx.storage().borrow(), col_meta.offset);
    let entry_point = collection.next_document;

    println!("Entry point: {}", entry_point);

    // Allocating space to new doc at the end

    let new_doc_offset = op_ctx.storage().borrow().len();
    insert_one_by_offset(&mut storage.borrow_mut(), new_doc_offset, raw_document);
    println!("new doc offset: {}", new_doc_offset);

    // Adding ref to new doc to last doc
    if entry_point == 0 {
        println!("Inserting in start of collection: {:?}", col_meta.offset);
        Collection::write_next_document(op_ctx.storage(), col_meta.offset, new_doc_offset);
    } else {
        let mut next_offset = entry_point as usize;

        'block: while next_offset != 0 {
            let raw_doc = RawDocument::parse(&op_ctx.storage().borrow(), next_offset);
            println!("RawDoc: {:?}", raw_doc);
    
            if raw_doc.next_document == 0 {
                println!("break");
                println!("{}", new_doc_offset);
                println!("current: {}", next_offset);
                println!("current's next_doc: {}", raw_doc.next_document);
                println!("ss len: {}", op_ctx.storage().borrow().len());
                RawDocument::write_next_document(&mut op_ctx.storage().borrow_mut(), next_offset, new_doc_offset);
                break 'block;
            }
            
            next_offset = raw_doc.next_document as usize; 
        };
    }
    println!("Inserted");

    new_doc_offset
}

pub fn insert_one_by_offset(
    ss: &mut RefMut<'_, StartStorage>,
    offset: usize,
    raw_document: RawDocument
) {
    ensure_capacity(ss, offset + raw_document.len()).unwrap();
    RawDocument::write_next_document(ss, offset, 0);
    RawDocument::write_content_length(ss, offset, raw_document.content_length as usize);
    RawDocument::write_content(ss, offset, &raw_document.content);
}