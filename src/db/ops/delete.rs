use crate::db::{collection::Collection, operation_context::OperationContext, query::filtering::Filter};

pub fn delete(
    op_ctx: &mut OperationContext,
    col: Collection,
    filter: Option<Filter>,
) {
    
}