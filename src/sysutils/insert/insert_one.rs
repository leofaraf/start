use start_storage::StartStorage;

use crate::{systypes::document::RawDocument, sysutils::capacity::ensure_capacity};

pub fn insert_document_by_offset(
    ss: &mut StartStorage,
    offset: usize,
    raw_document: RawDocument
) {
    ensure_capacity(ss, offset + raw_document.len()).unwrap();
    RawDocument::write_next_document(ss, offset, 0);
    RawDocument::write_content_length(ss, offset, raw_document.content_length as usize);
    RawDocument::write_content(ss, offset, &raw_document.content);
}