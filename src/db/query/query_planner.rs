use super::filtering::Filter;

pub struct QueryPlanner;

impl QueryPlanner {
    pub fn build_plan(
        collection: &str,
        filter: Option<Filter>,
        skip: Option<u64>,
        limit: Option<u64>,
    ) -> QueryPlan {
        let mut plan = QueryPlan::CollectionScan {
            collection_name: collection.to_string(),
        };

        if let Some(f) = filter {
            plan = QueryPlan::Filter {
                condition: f,
                child: Box::new(plan),
            };
        }

        if let Some(s) = skip {
            plan = QueryPlan::Skip {
                skip: s as usize,
                child: Box::new(plan),
            };
        }

        if let Some(l) = limit {
            plan = QueryPlan::Limit {
                limit: l as usize,
                child: Box::new(plan),
            };
        }

        plan
    }
}

#[derive(Debug, Clone)]
pub enum QueryPlan {
    CollectionScan {
        collection_name: String,
    },
    Filter {
        condition: Filter,
        child: Box<QueryPlan>,
    },
    Skip {
        skip: usize,
        child: Box<QueryPlan>,
    },
    Limit {
        limit: usize,
        child: Box<QueryPlan>,
    },
}