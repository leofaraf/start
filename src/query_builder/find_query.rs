use std::{collections::HashMap, error::Error};

use serde::de::DeserializeOwned;

use crate::{
    utils::find::find_many,
    StartDB,
};

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct FindQuery<'a> {
    db: &'a mut StartDB,
    filters: HashMap<String, String>,
}

impl<'a> FindQuery<'a> {
    pub fn new(db: &'a mut StartDB) -> Self {
        Self {
            db,
            filters: HashMap::new(),
        }
    }

    pub fn filter(mut self, key: &str, value: &str) -> Self {
        self.filters.insert(key.to_string(), value.to_string());
        self
    }

    pub fn from<T: DeserializeOwned + serde::Serialize>(self, collection_name: &str) -> HandleResult<Vec<T>> {
        let docs = find_many(&mut self.db.ss, collection_name);

        let mut results = Vec::new();
        for doc in docs {
            match serde_json::from_slice::<T>(&doc.content) {
                Ok(value) => {
                    // Apply filters here
                    if self.filters.is_empty() || self.matches_filter(&value) {
                        results.push(value);
                    }
                }
                Err(err) => eprintln!("Deserialization error: {}", err),
            }
        }

        Ok(results)
    }

    fn matches_filter<T: serde::Serialize>(&self, item: &T) -> bool {
        let json_value = serde_json::to_value(item).unwrap_or_default();

        for (k, v) in &self.filters {
            if json_value.get(k).map(|jv| jv == v).unwrap_or(false) == false {
                return false;
            }
        }
        true
    }
}