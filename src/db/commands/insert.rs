use bson::Bson;
use log::trace;

use crate::{db::{catalog::{collection::RawDocument, session::Session}, operation_context::OperationContext, ops, service_context::ServiceContext}, HandleResult};

pub fn insert(
    session: &Session,
    collection: &str,
    document: Bson
) -> HandleResult<()> {
    trace!("__________________Insert____________________");
    let mut op_ctx = OperationContext::new(session)?;

    let catalog = 
        op_ctx.catalog().borrow_mut()
        .collection();

    let content = bson::to_vec(&document)?;
    
    let mut meta = catalog.borrow_mut().acquire_collection_or_create(collection, &mut op_ctx);

    ops::insert::insert(&mut op_ctx, &mut meta, &content);

    if session.transaction().borrow().is_none() {
        op_ctx.rc_unit().borrow_mut().commit();
    }
    trace!("___________________________________________");

    Ok(())
}