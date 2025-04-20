use bson::Bson;

use crate::db::{catalog::collection::RawDocument, operation_context::OperationContext, ops, service_context::ServiceContext};

// By the idea it should accept BSON doc with fields
pub fn insert(
    ctx: &ServiceContext,
    collection: &str,
    document: Bson
) {
    let op_ctx = OperationContext::new(ctx);

    let mut catalog = 
        op_ctx.catalog().borrow_mut()
        .collection();

        let content = bson::to_vec(&document).unwrap();
    
    let raw_document = RawDocument {
        next_document: 0,
        content_length: content.len() as u64,
        content: content,
    };
    let meta = catalog.borrow_mut().autocol(collection, &op_ctx);

    ops::insert::insert(&op_ctx, meta, raw_document, true);
}