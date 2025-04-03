use start_storage::StartStorage;

use crate::{systypes::document::RawDocument, sysutils::{self, find::find_collection::find_collection}};

pub fn insert_one(
    ss: &mut StartStorage,
    collection: &str,
    content: Vec<u8>
) {
    let opt_col = find_collection(ss, collection);
    let raw_document = RawDocument {
        next_document: 0,
        content_length: content.len() as u64,
        content,
    };

    if let Some(col) = opt_col {
        sysutils::insert::one::insert_one(ss, col, raw_document);
        return;
    }

    let new_col = sysutils::insert::collection::insert_collection(ss, collection);
    sysutils::insert::one::insert_one(ss, new_col, raw_document);
}