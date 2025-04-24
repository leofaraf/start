use bson::Bson;
use serde::{de::DeserializeOwned, Serialize};

use crate::{db::{self, catalog::session::Session, query::filtering::Filter}, HandleResult};

pub struct FindQuery<'a> {
    session: &'a Session,
    filter: Option<Filter>,
    skip: Option<u64>,
    limit: Option<u64>,
}

impl <'a>FindQuery<'a> {
    pub fn from<T>(&mut self, collection: &str)-> HandleResult<Vec<T>>
    where T: DeserializeOwned {
        let raw_results = db::commands::find::find(
            self.session,
            collection,
            self.filter.take(), 
            self.skip,
            self.limit);

        // Convert Vec<Bson> or Vec<Document> to Vec<T>
        let mut results = Vec::new();
        for entry in raw_results {
            let doc = match entry {
                Bson::Document(d) => d,
                _ => return Err("Expected Bson::Document".into()),
            };

            let deserialized: T = bson::from_document(doc)?;
            results.push(deserialized);
        }

        Ok(results)
    }
}

impl Session {
    pub fn insert<T>(&self, collection: &str, document: T) -> HandleResult<()>
    where T: Serialize {
        db::commands::insert::insert(self, collection, bson::to_bson(&document)?);
        Ok(())
    }

    pub fn find(&self) -> FindQuery {
        FindQuery { session: self, filter: None, skip: None, limit: None }
    }
}