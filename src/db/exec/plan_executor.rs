use bson::Bson;

use crate::{db::{catalog::collection::RawDocument, operation_context::OperationContext, query::{filtering::{matches_filter, Filter}, query_planner::{PlanNode, QueryPlan}}}, HandleResult};

pub fn execute_plan(op_ctx: OperationContext, plan: QueryPlan) -> HandleResult<Vec<Bson>> {
    println!("Executing QueryPlan: {:?}", plan);

    // Unwrap the plan chain to extract settings
    let PlanParams {
        filter,
        skip,
        limit,
        scan_node,
    } = extract_plan_params(&plan.root);

    println!("F: {:?}", filter);

    // Begin executing the actual scan
    let mut result = Vec::new();
    let mut next_offset = plan.collection.next_document as usize;
    let rc_unit = op_ctx.rc_unit();

    let mut skipped = 0;
    let mut returned = 0;

    while next_offset != 0 {
        let raw_doc = RawDocument::parse(&rc_unit.borrow(), next_offset);
        next_offset = raw_doc.next_document as usize;

        if raw_doc.flag_deleted {
            continue;
        }

        let doc = bson::from_slice::<bson::Document>(&raw_doc.content)?;

        // Apply filter
        if let Some(ref cond) = filter {
            println!("FILTER");
            if !matches_filter(&doc, cond) {
                continue;
            }
        }

        // Apply skip
        if skipped < skip {
            skipped += 1;
            continue;
        }

        // Apply limit
        if let Some(l) = limit {
            if returned >= l {
                break;
            }
        }

        result.push(Bson::Document(doc));
        returned += 1;
    }

    Ok(result)
}

fn extract_plan_params<'a>(mut node: &'a PlanNode) -> PlanParams<'a> {
    let mut filter = None;
    let mut skip = 0;
    let mut limit = None;

    loop {
        match node {
            PlanNode::Filter { condition, child } => {
                filter = Some(condition);
                node = child;
            }
            PlanNode::Skip { skip: s, child } => {
                skip = *s;
                node = child;
            }
            PlanNode::Limit { limit: l, child } => {
                limit = Some(*l);
                node = child;
            }
            PlanNode::CollectionScan { .. } => break,
        }
    }

    PlanParams {
        filter,
        skip,
        limit,
        scan_node: node,
    }
}

struct PlanParams<'a> {
    filter: Option<&'a Filter>,
    skip: usize,
    limit: Option<usize>,
    scan_node: &'a PlanNode,
}
