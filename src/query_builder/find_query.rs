use std::error::Error;

use serde::de::DeserializeOwned;

use crate::{
    utils::find::find_many,
    StartDB,
};

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct FindQuery<'a> {
    db: &'a mut StartDB,
}

impl<'a> FindQuery<'a> {
    pub fn new(db: &'a mut StartDB) -> Self {
        Self { db }
    }

    pub fn from<T: DeserializeOwned>(self, collection_name: &str) -> HandleResult<Vec<T>> {
        let docs = find_many(&mut self.db.ss, collection_name);

        let mut results = Vec::new();
        for doc in docs {
            match serde_json::from_slice::<T>(&doc.content) {
                Ok(value) => results.push(value),
                Err(err) => eprintln!("Deserialization error: {}", err),
            }
        }

        Ok(results)
    }
}