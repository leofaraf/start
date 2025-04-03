use start_storage::StartStorage;

use crate::{systypes::{collection::Collection, document::{Document, RawDocument}}, sysutils::capacity::ensure_capacity};

// pub fn insert_collection(ss: &mut StartStorage, name: &str) -> Collection {

// }

pub fn insert_collection_by_offset(
    ss: &mut StartStorage,
    offset: usize,
    col: Collection
) {
    // ensure_capacity(ss, offset + 8 + 8 + 40).unwrap();
    // let content = col.to_bytes();
    // let raw = RawDocument {
    //     next_document: 0,
    //     content_length: content.len(),
    //     content,
    // }
    // RawDocument::write_content(ss, offset, &content);
}