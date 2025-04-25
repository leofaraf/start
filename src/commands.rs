use bson::{doc, Bson, Document};
use serde::{de::DeserializeOwned, Serialize};

use crate::{db::{self, catalog::session::Session, query::filtering::Filter}, HandleResult};

pub struct FindQuery<'a> {
    session: &'a Session,
    filter: Option<Filter>,
    skip: Option<u64>,
    limit: Option<u64>,
}

impl <'a>FindQuery<'a> {
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn skip(mut self, skip: u64) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from<T>(mut self, collection: &str)-> HandleResult<Vec<T>>
    where T: DeserializeOwned {
        let raw_results = db::commands::find::find(
            self.session,
            collection,
            self.filter.take(), 
            self.skip,
            self.limit)?;

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

pub struct DeleteQuery<'a> {
    session: &'a Session,
    filter: Option<Filter>,
}

impl <'a>DeleteQuery<'a> {
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn from(mut self, collection: &str) -> HandleResult<()> {
        db::commands::delete::delete(self.session, collection, self.filter.take())
    }
}

pub struct UpdateQuery<'a> {
    session: &'a Session,
    filter: Option<Filter>,
    update_document: Document,
}

impl <'a>UpdateQuery<'a> {
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn set(mut self, document: Document) -> Self {
        self.update_document.insert("$set", document);
        self
    }

    pub fn from(mut self, collection: &str) -> HandleResult<()> {
        db::commands::update::update(
            self.session,
            self.filter.take(),
            self.update_document,
            collection,
        )
    }
}

impl Session {
    pub fn insert<T>(&self, collection: &str, document: T) -> HandleResult<()>
    where T: Serialize {
        db::commands::insert::insert(self, collection, bson::to_bson(&document)?);
        Ok(())
    }

    pub fn delete(&self) -> DeleteQuery {
        DeleteQuery { session: self, filter: None }
    }

    pub fn find(&self) -> FindQuery {
        FindQuery { session: self, filter: None, skip: None, limit: None }
    }

    pub fn update(&self) -> UpdateQuery {
        UpdateQuery { session: self, filter: None, update_document: Document::new() }
    }
}