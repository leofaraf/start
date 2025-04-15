use start_storage::StartStorage;

use crate::systypes::{collection::Collection, document::RawDocument};

pub fn find_many(
    ss: &mut StartStorage,
    offset: usize,
) -> Vec<RawDocument> {
    let raw_doc = RawDocument::parse(ss, offset);
    println!("RawDoc: {:?}", raw_doc);
    let collection = Collection::parse(&raw_doc.content);
    println!("Col: {:?}", collection);

    let mut result = vec![];

    let mut next_offset = collection.next_document as usize;
    
    while next_offset != 0 {
        let raw_doc = RawDocument::parse(ss, next_offset);
        println!("RawDoc: {:?}", raw_doc);

        if next_offset == offset as usize {
            next_offset = collection.next_document as usize; // Move to the next document
        } else {
            next_offset = raw_doc.next_document as usize; // Move to the next document
        }
        result.push(raw_doc);
    }

    result
}