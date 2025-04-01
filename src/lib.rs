use std::{error::Error, path::PathBuf};

use start_storage::StartStorage;

mod systypes;

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct StartDB {
    ss: StartStorage
}

pub fn in_memory() -> StartDB {
    StartDB {
        ss: StartStorage::in_memory()
    }
}

pub fn embedded(path: PathBuf) -> HandleResult<StartDB> {
    Ok(StartDB {
        ss: StartStorage::embedded(path)?
    })
}