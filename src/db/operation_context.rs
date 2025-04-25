use std::{cell::{RefCell, RefMut}, rc::Rc};

use crate::HandleResult;

use super::{catalog::{session::Session, Catalog}, recovery_unit::RecoveryUnit, service_context::ServiceContext, storage::start_storage::StartStorage};

pub struct OperationContext {
    storage: Rc<RefCell<StartStorage>>,
    catalog: Rc<RefCell<Catalog>>,
    rc_unit: Rc<RefCell<RecoveryUnit>>,
}

impl OperationContext {
    pub fn new(session: &Session) -> HandleResult<Self> {
        let ctx = match session.ctx() {
            Some(ctx) => ctx,
            None => return Err("Database closed connection".into()),
        };

        let transaction = session.transaction();

        Ok(Self {
            storage: ctx.storage(),
            catalog: ctx.catalog(),
            rc_unit: match transaction.borrow().as_ref() {
                Some(tx) => tx.rc_unit(),
                None => Rc::new(RefCell::new(RecoveryUnit::new(ctx.storage()))),
            },
        })
    }

    pub fn storage(&self) -> Rc<RefCell<StartStorage>> {
        Rc::clone(&self.storage)
    }

    pub fn catalog(&self) -> Rc<RefCell<Catalog>> {
        Rc::clone(&self.catalog)
    }

    pub fn rc_unit(&self) -> Rc<RefCell<RecoveryUnit>> {
        Rc::clone(&self.rc_unit)
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