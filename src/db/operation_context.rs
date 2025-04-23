use std::{cell::{RefCell, RefMut}, rc::Rc};

use super::{catalog::{session::Session, Catalog}, recovery_unit::RecoveryUnit, service_context::ServiceContext, storage::start_storage::StartStorage};

pub struct OperationContext {
    storage: Rc<RefCell<StartStorage>>,
    catalog: Rc<RefCell<Catalog>>,
    pub rc_unit: RecoveryUnit,
    txn_id: Option<u64>, // if supporting transactions
}

impl OperationContext {
    pub fn new(session: &Session) -> Self {
        let ctx = session.ctx().unwrap();

        Self {
            storage: ctx.storage(),
            catalog: ctx.catalog(),
            rc_unit: RecoveryUnit::new(ctx.storage()),
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

pub fn ensure_capacity(
    ss: &mut RefMut<'_, StartStorage>, required_size: usize
) -> Result<(), DocumentsError> {
    let current_size = ss.len();
    if required_size > current_size {
        return match ss.resize(required_size) {
            Ok(_) => Ok(()),
            Err(err) => Err(DocumentsError::DatabaseError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum DocumentsError {
    PrimaryKeyError(String),
    DatabaseError(String)
}