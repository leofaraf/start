use std::error::Error;

// pub mod query_builder;
pub mod db;

type HandleResult<T> = Result<T, Box<dyn Error>>;