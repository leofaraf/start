use bson::Bson;

use crate::db::{operation_context::OperationContext, query::query_planner::QueryPlan};

pub fn execute_plan(oc: OperationContext, plan: QueryPlan) -> Vec<Bson> {
    vec![]
}