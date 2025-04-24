use std::{error::Error, path::PathBuf, rc::Rc};

use db::{catalog::session::{Session, SessionCatalog}, service_context::{self, ServiceContext}};

pub mod db;
pub mod sql;
pub mod commands;

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct StartDB {
    pub ctx: Rc<ServiceContext>
}

impl StartDB {
    pub fn get_session(&self) -> Session {
        SessionCatalog::acquire(self.ctx.clone())
    }
}

pub fn db_in_memory() -> StartDB {
    StartDB { ctx: service_context::in_memory() }
}

pub fn db_embedded(path: PathBuf) -> HandleResult<StartDB> {
    Ok(StartDB { ctx: service_context::embedded(path)? })
}