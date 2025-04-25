use bson::{Bson, Document};

use crate::{db::{catalog::{collection::RawDocument, session::Session}, operation_context::OperationContext, ops, query::filtering::Filter, service_context::ServiceContext}, HandleResult};

pub fn update(
    session: &Session,
    filter: Option<Filter>,
    update_document: Document,
    collection: &str
) -> HandleResult<()> {
    println!("__________________Update____________________");
    let mut op_ctx = OperationContext::new(session);

    let catalog = 
        op_ctx.catalog().borrow_mut()
        .collection();

    let mut meta = catalog.borrow_mut().acquire_collection_or_create(collection, &mut op_ctx);

    ops::update::update(&mut op_ctx, filter, update_document, &mut meta);

    if session.transaction().borrow().is_none() {
        op_ctx.rc_unit().borrow_mut().commit();
    }
    println!("___________________________________________");
    Ok(())
}