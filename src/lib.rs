use std::path::PathBuf;

pub struct StartDB;

pub fn in_memory() -> StartDB {
    StartDB
}

pub fn embedded(pathname: PathBuf) -> StartDB {
    StartDB
}