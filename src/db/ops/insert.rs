use crate::{db::{catalog::collection::RawDocument, collection::Collection, operation_context::{ensure_capacity, OperationContext}, recovery_unit::RecoveryUnit, storage::start_storage::StartStorage}, HandleResult};

pub fn insert(
    op_ctx: &mut OperationContext,
    col: &mut Collection,
    data: &[u8]
) -> usize {
    col.insert_document(op_ctx, data)
}

pub fn insert_one_by_offset(
    op_ctx: &mut OperationContext,
    offset: usize,
    raw_document: RawDocument
) -> HandleResult<()> {
    let rc_unit = op_ctx.rc_unit();
    if ensure_capacity(&mut op_ctx.storage().borrow_mut(), offset + raw_document.len()).is_err() {
        return Err("Database capacity error".into());
    }
    RawDocument::write_next_document(rc_unit.borrow_mut(), offset, 0);
    RawDocument::write_content_length(rc_unit.borrow_mut(), offset, raw_document.content_length as usize);
    RawDocument::write_content(rc_unit.borrow_mut(), offset, &raw_document.content);
    Ok(())
}