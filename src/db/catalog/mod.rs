use std::{cell::RefCell, rc::Rc};

use collection::CollectionCatalog;
use session::SessionCatalog;

use super::storage::start_storage::StartStorage;

pub mod collection;
pub mod database;
pub mod session;

pub struct Catalog {
    collection_catalog: Rc<RefCell<CollectionCatalog>>,
    session_catalog: SessionCatalog
}

impl Catalog {
    pub fn new(_ss: Rc<RefCell<StartStorage>>) -> Self {
        Self {
            collection_catalog: Rc::new(RefCell::new(CollectionCatalog::new())),
            session_catalog: SessionCatalog
        }
    }

    pub fn collection(&self) -> Rc<RefCell<CollectionCatalog>> {
        self.collection_catalog.clone()
    }

    pub fn session(&self) -> SessionCatalog {
        SessionCatalog
    }
}