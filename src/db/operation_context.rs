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

#[deprecated]
pub fn ensure_capacity(
    ss: Rc<RefCell<StartStorage>>, required_size: usize
) -> Result<(), DocumentsError> {
    let mut ss = ss.borrow_mut();
    let current_size = ss.len();
    if required_size > current_size {
        match ss.resize(required_size) {
            Ok(_) => Ok(()),
            Err(err) => Err(DocumentsError::DatabaseError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        };
    }
    Ok(())
}

#[derive(Debug)]
pub enum DocumentsError {
    PrimaryKeyError(String),
    DatabaseError(String)
}