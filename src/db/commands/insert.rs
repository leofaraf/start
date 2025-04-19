use bson::Bson;

use crate::db::{operation_context::OperationContext, ops, service_context::ServiceContext};

// By the idea it should accept BSON doc with fields
pub fn insert(
    ctx: &ServiceContext,
    collection: &str,
    document: Bson
) {
    let op_ctx = OperationContext::new(ctx);

    let autocol = 
        op_ctx.catalog().borrow()
        .collection();

    let collection = autocol.autocol(collection);

    ops::insert::insert(op_ctx, collection, document);
}