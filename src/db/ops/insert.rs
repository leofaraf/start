use std::cell::RefMut;

use crate::db::{catalog::collection::RawDocument, collection::Collection, operation_context::{ensure_capacity, OperationContext}, recovery_unit::RecoveryUnit, storage::start_storage::StartStorage};

pub fn insert(
    op_ctx: &mut OperationContext,
    col: Collection,
    data: &[u8]
) -> usize {
    col.insert_document(op_ctx, data)
}

pub fn insert_one_by_offset(
    op_ctx: &mut OperationContext,
    offset: usize,
    raw_document: RawDocument
) {
    ensure_capacity(&mut op_ctx.storage().borrow_mut(), offset + raw_document.len()).unwrap();
    RawDocument::write_next_document(&mut op_ctx.rc_unit, offset, 0);
    RawDocument::write_content_length(&mut op_ctx.rc_unit, offset, raw_document.content_length as usize);
    RawDocument::write_content(&mut op_ctx.rc_unit, offset, &raw_document.content);
}