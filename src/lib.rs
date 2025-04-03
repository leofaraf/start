use std::{error::Error, path::PathBuf};

use start_storage::StartStorage;
use systypes::{collection::{SYS_MASTER, SYS_MASTER_OFFSET, SYS_TRASH, SYS_TRASH_OFFSET}, document::RawDocument, header::Header};
use sysutils::{header::HeaderError, insert::insert_one::insert_document_by_offset};

mod systypes;
pub mod sysutils;
pub mod utils;

type HandleResult<T> = Result<T, Box<dyn Error>>;

pub struct StartDB {
    pub ss: StartStorage,
    header: Header
}

fn get_header(ss: &mut StartStorage) -> Header {
    if ss.len() == 0 {
        insert_document_by_offset(ss, SYS_MASTER_OFFSET as usize, RawDocument {
            next_document: SYS_TRASH_OFFSET,
            content_length: 40,
            content: SYS_MASTER.to_bytes(),
        });
        insert_document_by_offset(ss, SYS_TRASH_OFFSET as usize, RawDocument {
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