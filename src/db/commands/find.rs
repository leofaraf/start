use bson::Bson;

use crate::db::{exec::plan_executor, operation_context::OperationContext, query::{filtering::Filter, query_planner::QueryPlanner}, service_context::ServiceContext};

pub fn find(
    ctx: &ServiceContext,
    collection: &str,
    filter: Option<Filter>,
    skip: Option<u64>,
    limit: Option<u64>
) -> Vec<Bson> {
    let op_ctx = OperationContext::new(ctx);

    let autocol = 
        op_ctx.catalog().borrow()
        .collection();

    let collection = autocol.autocol(collection);

    let plan = QueryPlanner::build_plan(collection, filter, skip, limit);
    plan_executor::execute_plan(op_ctx, plan)
}