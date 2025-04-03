use start_storage::StartStorage;

use crate::{systypes::{collection::Collection, document::RawDocument}, sysutils::capacity::ensure_capacity};

pub fn insert_one(
    ss: &mut StartStorage,
    collection_offset: usize,
    raw_document: RawDocument
) -> usize {
    println!("Inserting...");
    // Parsing collection
    ensure_capacity(ss, collection_offset + 56).unwrap();
    let raw_doc = RawDocument::parse(ss, collection_offset);
    let collection = Collection::parse(&raw_doc.content);
    let entry_point = collection.next_document;

    println!("Entry point: {}", entry_point);

    // Allocating space to new doc at the end
    let new_doc_offset = ss.len();
    insert_one_by_offset(ss, new_doc_offset, raw_document);
    println!("new doc offset: {}", new_doc_offset);

    // Adding ref to new doc to last doc
    if entry_point == 0 {
        Collection::write_next_document(ss, collection_offset, new_doc_offset);
    } else {
        let mut next_offset = entry_point as usize;

        'block: while next_offset != 0 {
            let raw_doc = RawDocument::parse(ss, next_offset);
            println!("RawDoc: {:?}", raw_doc);
    
            if raw_doc.next_document == 0 {
                println!("break");
                println!("{}", new_doc_offset);
                println!("current: {}", next_offset);
                println!("current's next_doc: {}", raw_doc.next_document);
                println!("ss len: {}", ss.len());
                RawDocument::write_next_document(ss, next_offset, new_doc_offset);
                break 'block;
            }
            
            next_offset = raw_doc.next_document as usize; 
        };
    }
    println!("Inserted");

    new_doc_offset
}

pub fn insert_one_by_offset(
    ss: &mut StartStorage,
    offset: usize,
    raw_document: RawDocument
) {
    ensure_capacity(ss, offset + raw_document.len()).unwrap();
    RawDocument::write_next_document(ss, offset, 0);
    RawDocument::write_content_length(ss, offset, raw_document.content_length as usize);
    RawDocument::write_content(ss, offset, &raw_document.content);
}