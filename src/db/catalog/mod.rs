use std::{cell::RefCell, rc::Rc};

use collection::CollectionCatalog;

use super::storage::start_storage::StartStorage;

pub mod collection;
pub mod database;
pub mod session;

pub struct Catalog {
    collection_catalog: Rc<RefCell<CollectionCatalog>>
}

impl Catalog {
    pub fn new(_ss: Rc<RefCell<StartStorage>>) -> Self {
        Self {
            collection_catalog: Rc::new(RefCell::new(CollectionCatalog::new())),
        }
    }

    pub fn collection(&self) -> Rc<RefCell<CollectionCatalog>> {
        self.collection_catalog.clone()
    }
}