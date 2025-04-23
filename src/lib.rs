use std::{error::Error, rc::Rc};

use db::{catalog::session::{Session, SessionCatalog}, service_context::ServiceContext};

// pub mod query_builder;
pub mod db;

type HandleResult<T> = Result<T, Box<dyn Error>>;

#[deprecated]
pub struct StartDB {
    pub ctx: Rc<ServiceContext>
}

impl StartDB {
    pub fn get_session(&self) -> Session {
        SessionCatalog::acquire(self.ctx.clone())
    }
}