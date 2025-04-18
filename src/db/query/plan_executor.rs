use bson::Bson;

use crate::db::operation_context::OperationContext;

use super::query_planner::QueryPlan;

pub fn execute_plan(oc: OperationContext, plan: QueryPlan) -> Vec<Bson> {
    vec![]
}