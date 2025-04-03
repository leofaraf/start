use start_storage::StartStorage;

use crate::{systypes::{collection::{Collection, SYS_MASTER_OFFSET}, document::RawDocument}, sysutils::capacity::ensure_capacity};

pub fn find_collection(ss: &mut StartStorage, name: &str) -> Option<Collection> {
    ensure_capacity(ss, 200).unwrap();

    let mut next_offset = SYS_MASTER_OFFSET as usize;

    while next_offset != 0 {
        let raw_doc = RawDocument::parse(ss, next_offset);
        println!("RawDoc: {:?}", raw_doc);
        let collection = Collection::parse(&raw_doc.content);
        println!("Col: {:?}", collection);

        if let Ok(col_name) = std::str::from_utf8(&collection.name) {
            if col_name.trim_matches('\0') == name {
                return Some(collection);
            }
        }

        next_offset = collection.next_document as usize; // Move to the next document
    }

    None
}