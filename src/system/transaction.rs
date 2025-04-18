use crate::StartDB;

use super::query::Query;

pub struct Transaction<'a> {
    db: &'a mut StartDB,
    queries: Vec<Query>
}

