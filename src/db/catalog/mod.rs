use std::{cell::RefCell, rc::Rc};

use collection::CollectionCatalog;
use start_storage::StartStorage;

pub mod collection;

pub struct Catalog {
    collection_catalog: Rc<CollectionCatalog>
}

impl Catalog {
    pub fn new(_ss: Rc<RefCell<StartStorage>>) -> Self {
        Self {
            collection_catalog: Rc::new(CollectionCatalog::new()),
        }
    }

    pub fn collection(&self) -> Rc<CollectionCatalog> {
        self.collection_catalog.clone()
    }
}