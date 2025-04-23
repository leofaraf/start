use bson::Bson;

use crate::db::{catalog::session::Session, exec::plan_executor, operation_context::OperationContext, query::{filtering::Filter, query_planner::QueryPlanner}, service_context::ServiceContext};

pub fn find(
    session: &Session,
    collection: &str,
    filter: Option<Filter>,
    skip: Option<u64>,
    limit: Option<u64>
) -> Vec<Bson> {
    let op_ctx = OperationContext::new(session);

    let autocol = 
        op_ctx.catalog().borrow()
        .collection();

    let meta = autocol.borrow().lookup_collection(&op_ctx, collection);

    let plan = QueryPlanner::build_plan(meta, filter, skip, limit);
    plan_executor::execute_plan(op_ctx, plan)
}