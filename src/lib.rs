pub struct StartDB;

pub struct DatabaseBuilder();

pub fn in_memory() -> StartDB {
    StartDB
}

pub fn embedded(filename: String) -> StartDB {
    StartDB
}