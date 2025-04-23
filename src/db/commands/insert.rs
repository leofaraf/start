use bson::Bson;

use crate::db::{catalog::{collection::RawDocument, session::Session}, operation_context::OperationContext, ops, service_context::ServiceContext};

// By the idea it should accept BSON doc with fields
pub fn insert(
    session: &Session,
    collection: &str,
    document: Bson
) {
    println!("__________________Insert____________________");
    let mut op_ctx = OperationContext::new(session);

    let catalog = 
        op_ctx.catalog().borrow_mut()
        .collection();

    let content = bson::to_vec(&document).unwrap();
    
    let meta = catalog.borrow_mut().acquire_collection_or_create(collection, &mut op_ctx);

    let new_doc_id = ops::insert::insert(&mut op_ctx, meta, &content);

    // let mut binding = catalog.borrow_mut();
    // let col = binding.collection_metadata.get_mut(collection).unwrap();
    // if col.next_document == 0 {
    //     col.next_document = new_doc_id;
    // };

    op_ctx.rc_unit().borrow_mut().commit();
    println!("___________________________________________");

}