use start_storage::StartStorage;

use crate::systypes::document::RawDocument;

pub fn find(ss: &mut StartStorage, offset: usize, contains: &[u8]) -> Option<RawDocument> {
    let mut next_offset = offset;

    while next_offset != 0 {
        let raw_doc = RawDocument::parse(ss, next_offset);
        println!("RawDoc: {:?}", raw_doc);

        if raw_doc.content.starts_with(contains) {
            return Some(raw_doc);
        }

        next_offset = raw_doc.next_document as usize; 
    }

    None
}