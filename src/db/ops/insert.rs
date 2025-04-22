use std::{cell::{RefCell, RefMut}, rc::Rc};

use bson::Bson;
use start_storage::StartStorage;

use crate::db::{catalog::collection::{RawDocument}, collection::Collection, operation_context::{ensure_capacity, OperationContext}};

pub fn insert(
    op_ctx: &mut OperationContext,
    col: Collection,
    data: &[u8]
) -> usize {
    col.insert_document(op_ctx, data)
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