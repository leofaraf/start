use crate::db::collection::Collection;

use super::filtering::Filter;

pub struct QueryPlanner;
#[derive(Debug)]
pub struct QueryPlan {
    pub root: PlanNode,
    pub collection: Collection
}

impl QueryPlanner {
    pub fn build_plan(
        collection: Collection,
        filter: Option<Filter>,
        skip: Option<u64>,
        limit: Option<u64>,
    ) -> QueryPlan {
        let mut plan = PlanNode::CollectionScan {
            collection_name: String::from_utf8(collection.name.to_vec()).unwrap(),
        };

        if let Some(f) = filter {
            plan = PlanNode::Filter {
                condition: f,
                child: Box::new(plan),
            };
        }

        if let Some(s) = skip {
            plan = PlanNode::Skip {
                skip: s as usize,
                child: Box::new(plan),
            };
        }

        if let Some(l) = limit {
            plan = PlanNode::Limit {
                limit: l as usize,
                child: Box::new(plan),
            };
        }

        QueryPlan {
            root: plan, 
            collection
        }
    }
}



#[derive(Debug, Clone)]
pub enum PlanNode {
    CollectionScan {
        collection_name: String,
    },
    Filter {
        condition: Filter,
        child: Box<PlanNode>,
    },
    Skip {
        skip: usize,
        child: Box<PlanNode>,
    },
    Limit {
        limit: usize,
        child: Box<PlanNode>,
    },
}