use std::{cell::RefCell, rc::Rc};

use bson::Bson;
use start_storage::StartStorage;

use crate::db::{catalog::collection::{CollectionMetadata, RawDocument}, collection::Collection, operation_context::{ensure_capacity, OperationContext}};

pub fn insert(
    op_ctx: &OperationContext,
    col_meta: CollectionMetadata,
    raw_document: RawDocument
) -> usize {
    println!("Inserting...");
    // Parsing collection

    ensure_capacity(op_ctx.storage(), col_meta.offset + 56).unwrap();
    #[deprecated]
    let raw_doc = RawDocument::parse(op_ctx.storage(), col_meta.offset);
    let collection = Collection::parse(&raw_doc.content);
    let entry_point = collection.next_document;

    println!("Entry point: {}", entry_point);

    // Allocating space to new doc at the end

    let new_doc_offset = op_ctx.storage().borrow().len();
    insert_one_by_offset(op_ctx.storage(), new_doc_offset, raw_document);
    println!("new doc offset: {}", new_doc_offset);

    // Adding ref to new doc to last doc
    if entry_point == 0 {
        println!("Inserting in start of collection: {:?}", col_meta.offset);
        Collection::write_next_document(op_ctx.storage(), col_meta.offset, new_doc_offset);
        
        // FIX ASAP AAA, research best way in catalog
        op_ctx.catalog().borrow_mut().collection().borrow_mut()
        .collection_metadata.get_mut("students").unwrap().collection.next_document = new_doc_offset as u64;
    } else {
        let mut next_offset = entry_point as usize;

        'block: while next_offset != 0 {
            let raw_doc = RawDocument::parse(op_ctx.storage(), next_offset);
            println!("RawDoc: {:?}", raw_doc);
    
            if raw_doc.next_document == 0 {
                println!("break");
                println!("{}", new_doc_offset);
                println!("current: {}", next_offset);
                println!("current's next_doc: {}", raw_doc.next_document);
                println!("ss len: {}", op_ctx.storage().borrow().len());
                RawDocument::write_next_document(op_ctx.storage(), next_offset, new_doc_offset);
                break 'block;
            }
            
            next_offset = raw_doc.next_document as usize; 
        };
    }
    println!("Inserted");

    new_doc_offset
}

pub fn insert_one_by_offset(
    ss: Rc<RefCell<StartStorage>>,
    offset: usize,
    raw_document: RawDocument
) {
    ensure_capacity(ss.clone(), offset + raw_document.len()).unwrap();
    RawDocument::write_next_document(ss.clone(), offset, 0);
    RawDocument::write_content_length(ss.clone(), offset, raw_document.content_length as usize);
    RawDocument::write_content(ss, offset, &raw_document.content);
}