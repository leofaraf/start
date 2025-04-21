use bson::Bson;

use crate::db::{catalog::collection::RawDocument, operation_context::OperationContext, ops, service_context::ServiceContext};

// By the idea it should accept BSON doc with fields
pub fn insert(
    ctx: &ServiceContext,
    collection: &str,
    document: Bson
) {
    let op_ctx = OperationContext::new(ctx);

    let catalog = 
        op_ctx.catalog().borrow_mut()
        .collection();

    let content = bson::to_vec(&document).unwrap();
    
    let raw_document = RawDocument {
        next_document: 0,
        content_length: content.len() as u64,
        content: content,
    };
    let meta = catalog.borrow_mut().acquire_collection_or_create(collection, &op_ctx);

    let new_doc_id = ops::insert::insert(&op_ctx, meta, raw_document);

    let mut binding = catalog.borrow_mut();
    let colmeta = binding.collection_metadata.get_mut(collection).unwrap();
    if colmeta.next_document == 0 {
        colmeta.next_document = new_doc_id;
    };
}