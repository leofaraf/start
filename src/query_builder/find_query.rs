use std::{collections::HashMap, error::Error};

use serde::de::DeserializeOwned;

use crate::{
    utils::find::find_many,
    StartDB,
};

use super::filtering::{matches_filter, Filter};

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct FindQuery<'a> {
    db: &'a mut StartDB,
    filter: Option<Filter>,
}

impl<'a> FindQuery<'a> {
    pub fn new(db: &'a mut StartDB) -> Self {
        Self {
            db,
            filter: None,
        }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn from<T: DeserializeOwned + serde::Serialize>(self, collection_name: &str) -> HandleResult<Vec<T>> {
        let docs = find_many(&mut self.db.ss, collection_name);

        let mut results = Vec::new();
        for doc in docs {
            match serde_json::from_slice::<serde_json::Value>(&doc.content) {
                Ok(json_value) => {
                    if self.filter.as_ref().map_or(true, |f| matches_filter(&json_value, f)) {
                        match serde_json::from_value::<T>(json_value) {
                            Ok(value) => results.push(value),
                            Err(err) => eprintln!("Deserialization error: {}", err),
                        }
                    }
                }
                Err(err) => eprintln!("Failed to parse JSON for filtering: {}", err),
            }
        }

        Ok(results)
    }
}