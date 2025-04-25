use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{db::{recovery_unit::RecoveryUnit, service_context::ServiceContext, storage::start_storage::StartStorage}, HandleResult};

pub struct SessionCatalog;
impl SessionCatalog {
    pub fn acquire(ctx: Rc<ServiceContext>) -> Session {
        Session {
            sid: Uuid::new_v4(),
            ctx: Rc::downgrade(&ctx),
            transaction: Rc::new(RefCell::new(None)),
        }
    }
}

pub struct Session {
    sid: Uuid,
    ctx: Weak<ServiceContext>,
    transaction: Rc<RefCell<Option<Transaction>>>
}

impl Session {
    pub(crate) fn ctx(&self) -> Option<Rc<ServiceContext>> {
        self.ctx.upgrade()
    }

    pub(crate) fn transaction(&self) -> Rc<RefCell<Option<Transaction>>> {
        self.transaction.clone()
    }

    pub fn start_transaction(&self) -> HandleResult<()> {
        let ctx = match self.ctx() {
            Some(ctx) => ctx,
            None => return Err("Database closed connection".into()),
        };
        self.transaction.borrow_mut().replace(Transaction::new(ctx.storage()));
        Ok(())
    }

    pub fn commit_transaction(&self) -> HandleResult<()> {
        let transaction = self.transaction.borrow_mut();
        if let Some(tx) = transaction.as_ref() {
            tx.rc_unit().borrow_mut().commit();
            drop(transaction);
            self.transaction.replace(None);
            Ok(())
        } else {
            Err("Transaction hasn't been opened".into())
        }
    }

    pub fn rollback_transaction(&self) -> HandleResult<()> {
        let transaction = self.transaction.borrow_mut();
        if let Some(tx) = transaction.as_ref() {
            tx.rc_unit().borrow_mut().rollback();
            drop(transaction);
            self.transaction.replace(None);
            Ok(())
        } else {
            Err("Transaction hasn't been opened".into())
        }
    }
}

pub struct Transaction {
    txid: Uuid,
    rc_unit: Rc<RefCell<RecoveryUnit>>
}

impl Transaction {
    pub fn new(storage: Rc<RefCell<StartStorage>>) -> Self {
        Transaction {
            txid: Uuid::new_v4(),
            rc_unit: Rc::new(RefCell::new(RecoveryUnit::new(storage)))
        }
    }

    pub fn rc_unit(&self) -> Rc<RefCell<RecoveryUnit>> {
        self.rc_unit.clone()
    }
}