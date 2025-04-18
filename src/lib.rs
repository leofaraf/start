use std::{error::Error, path::PathBuf};

use query_builder::{find_query::FindQuery, insert_query::InsertQuery};
use serde::Serialize;
use start_storage::StartStorage;
use systypes::{collection::{SYS_MASTER, SYS_MASTER_OFFSET, SYS_TRASH, SYS_TRASH_OFFSET}, document::RawDocument, header::Header};
use sysutils::{header::HeaderError, insert::one::{insert_one, insert_one_by_offset}};

pub mod systypes;
pub mod sysutils;
pub mod utils;
pub mod query_builder;
pub mod system;
pub mod db;

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct StartDB {
    pub ss: StartStorage,
    header: Header
}

impl StartDB {
    pub fn insert<T: Serialize>(&mut self, document: T) -> InsertQuery {
        InsertQuery::new(self).insert(document)
    }

    pub fn find(&mut self) -> FindQuery {
        FindQuery::new(self)
    }
}

fn get_header(ss: &mut StartStorage) -> Header {
    if ss.len() == 0 {
        insert_one_by_offset(ss, SYS_MASTER_OFFSET as usize, RawDocument {
            next_document: 0,
            content_length: 40,
            content: SYS_MASTER.to_bytes(),
        });
        insert_one(ss, SYS_MASTER_OFFSET as usize, RawDocument {
            next_document: 0,
            content_length: 40,
            content: SYS_TRASH.to_bytes(),
        });

        match Header::create(ss) {
            Ok(header) => {
                header
            },
            Err(HeaderError::DatabaseError(err)) => panic!("DBerr: {}", err),
            Err(err) => panic!("Header parsing error: {:?}", err)
        }
    } else {
        match Header::parse(ss) {
            Ok(header) => {
                header
            },
            Err(HeaderError::DatabaseError(err)) => panic!("DBerr: {}", err),
            Err(err) => panic!("Header parsing error: {:?}", err)
        }
    }
}

pub fn in_memory() -> StartDB {
    let mut ss = StartStorage::in_memory();
    StartDB {
        header: get_header(&mut ss),
        ss,
    }
}

pub fn embedded(path: PathBuf) -> HandleResult<StartDB> {
    let mut ss = StartStorage::embedded(path)?;
    Ok(StartDB {
        header: get_header(&mut ss),
        ss,
    })
}