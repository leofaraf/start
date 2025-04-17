use std::error::Error;

use serde::Serialize;

use crate::{
    utils::insert::insert_one, StartDB
};

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct InsertQuery<'a> {
    db: &'a mut StartDB,
    document: Option<Vec<u8>>,
}

impl<'a> InsertQuery<'a> {
    pub fn new(db: &'a mut StartDB) -> Self {
        Self {
            db,
            document: None,
        }
    }

    pub fn insert<T: Serialize>(mut self, document: T) -> Self {
        match bson::to_vec(&document) {
            Ok(bytes) => self.document = Some(bytes),
            Err(err) => {
                eprintln!("Serialization error: {}", err);
                self.document = None;
            }
        };
        self
    }

    pub fn into(self, collection_name: &str) -> HandleResult<()> {
        let doc = self.document.ok_or("Document is missing")?;

        insert_one(&mut self.db.ss, collection_name, doc);
        Ok(())
    }
}
