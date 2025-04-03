use start_storage::StartStorage;

use crate::{systypes::{collection::{Collection, SYS_MASTER_OFFSET}, document::{Document, RawDocument}}, sysutils::capacity::ensure_capacity};

use super::one::insert_one;

pub fn insert_collection(ss: &mut StartStorage, name: &str) -> usize {
    let mut bytes = [0u8; 32];
    bytes[0..name.len()].copy_from_slice(name.as_bytes());

    let collection = Collection {
        name: bytes,
        next_document: 0,
    };

    insert_one(ss, SYS_MASTER_OFFSET as usize, RawDocument {
        next_document: 0,
        content_length: Collection::len(),
        content: collection.to_bytes(),
    })
}

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