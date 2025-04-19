use std::{collections::HashMap, str};

use crate::db::collection::Collection;

pub struct CollectionCatalog {
    collection_metadata: HashMap<String, Collection>   
}

impl CollectionCatalog {
    pub fn new() -> Self {
        Self {
            collection_metadata: HashMap::new(),
        }
    }

    pub fn autocol(&self, collection: &str) -> Collection {
        match self.collection_metadata.get(collection) {
            Some(col) => col.clone(),
            None => {
                let mut bytes = [0u8; 32];
                bytes[0..collection.len()].copy_from_slice(collection.as_bytes());
                Collection {
                    name: bytes,
                    next_document: 0,
                }
            }
        }
    }
}
