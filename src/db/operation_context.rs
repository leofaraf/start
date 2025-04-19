use std::{cell::RefCell, rc::Rc, sync::Arc};

use start_storage::StartStorage;

use super::{catalog::Catalog, service_context::ServiceContext};

pub struct OperationContext {
    storage: Rc<RefCell<StartStorage>>,
    catalog: Rc<RefCell<Catalog>>,
    txn_id: Option<u64>, // if supporting transactions
}

impl OperationContext {
    pub fn new(sc: &ServiceContext) -> Self {
        Self {
            storage: sc.storage(),
            catalog: sc.catalog(),
            txn_id: None,
        }
    }

    pub fn storage(&self) -> Rc<RefCell<StartStorage>> {
        Rc::clone(&self.storage)
    }

    pub fn catalog(&self) -> Rc<RefCell<Catalog>> {
        Rc::clone(&self.catalog)
    }
}