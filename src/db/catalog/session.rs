use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::db::{recovery_unit::RecoveryUnit, service_context::ServiceContext};

pub struct SessionCatalog;
impl SessionCatalog {
    pub fn acquire(ctx: Rc<ServiceContext>) -> Session {
        Session {
            sid: Uuid::new_v4(),
            ctx: Rc::downgrade(&ctx),
            transaction: None,
        }
    }
}

pub struct Session {
    sid: Uuid,
    ctx: Weak<ServiceContext>,
    transaction: Option<Rc<Transaction>>
}

impl Session {
    pub fn ctx(&self) -> Option<Rc<ServiceContext>> {
        self.ctx.upgrade()
    }

    pub fn transaction(&self) -> Option<Rc<Transaction>> {
        self.transaction.clone()
    }
}

pub struct Transaction {
    txid: Uuid,
    rc_unit: Rc<RefCell<RecoveryUnit>>
}

impl Transaction {
    pub fn rc_unit(&self) -> Rc<RefCell<RecoveryUnit>> {
        self.rc_unit.clone()
    }
}