use std::error::Error;

pub mod insert_query;

use crate::{
    systypes::document::RawDocument, sysutils::{find::find_collection::find_collection, insert::collection::insert_collection}, utils::insert::insert_one, StartDB
};

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct QueryBuilder<'a> {
    db: &'a mut StartDB,
    document: Option<Vec<u8>>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(db: &'a mut StartDB) -> Self {
        Self {
            db,
            document: None,
        }
    }

    pub fn insert(mut self, document: Vec<u8>) -> Self {
        self.document = Some(document);
        self
    }

    pub fn into(self, collection_name: &str) -> HandleResult<()> {
        let doc = self.document.ok_or("Document is missing")?;

        insert_one(&mut self.db.ss, collection_name, doc);
        Ok(())
    }
}
