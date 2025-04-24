use crate::{db::{catalog::{collection::RawDocument, session::Session}, operation_context::OperationContext, ops, query::filtering::Filter, service_context::ServiceContext}, HandleResult};

pub fn delete(
    session: &Session,
    collection: &str,
    filter: Option<Filter>,
) -> HandleResult<()> {
    println!("__________________Delete____________________");
    let mut op_ctx = OperationContext::new(session);

    let autocol = 
        op_ctx.catalog().borrow()
        .collection();

    let meta = autocol.borrow().lookup_collection(&op_ctx, collection);

    ops::delete::delete(&mut op_ctx, meta, filter);

    if session.transaction().borrow().is_none() {
        op_ctx.rc_unit().borrow_mut().commit();
    }
    println!("___________________________________________");

    Ok(())
}