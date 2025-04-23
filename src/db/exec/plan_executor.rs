use bson::Bson;

use crate::db::{catalog::collection::RawDocument, operation_context::OperationContext, query::query_planner::QueryPlan};

pub fn execute_plan(op_ctx: OperationContext, plan: QueryPlan) -> Vec<Bson> {
    println!("QueryPlan: {:?}", plan);

    let mut result = vec![];
    
    let mut next_offset = plan.collection.next_document as usize;
    let rc_unit = op_ctx.rc_unit();
    
    while next_offset != 0 {
        let raw_doc = RawDocument::parse(&rc_unit.borrow(), next_offset);
        println!("RawDoc: {:?}", raw_doc);

        if let Ok(text) = std::str::from_utf8(&raw_doc.content) {
            println!("{}. '{}'", next_offset, text);
        }

        result.push(bson::from_slice(&raw_doc.content).unwrap());

        // if next_offset == plan.collection.next_document as usize {
            // next_offset = plan.collection.next_document as usize; // Move to the next document
        // } else {
            // next_offset = raw_doc.next_document as usize; // Move to the next document
        // }

        next_offset = raw_doc.next_document as usize;
    }


    result
}