use std::{collections::HashMap, rc::{Rc, Weak}};

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
    transaction: Option<Transaction>
}

impl Session {
    pub fn ctx(&self) -> Option<Rc<ServiceContext>> {
        self.ctx.upgrade()
    }
}

pub struct Transaction {
    txid: Uuid,
    rc_unit: RecoveryUnit
}