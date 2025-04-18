use bson::Bson;

use crate::db::{operation_context::OperationContext, query::{filtering::Filter, plan_executor, query_planner::{QueryPlan, QueryPlanner}}, service_context::ServiceContext};

// By the idea it should accept BSON doc with fields
pub fn find(
    sc: &ServiceContext,
    collection: &str,
    filter: Option<Filter>,
    skip: Option<u64>,
    limit: Option<u64>
) -> Vec<Bson> {
    let oc = OperationContext::new(sc);

    let plan = QueryPlanner::build_plan(collection, filter, skip, limit);
    plan_executor::execute_plan(oc, plan)
}