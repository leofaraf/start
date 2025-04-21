use std::{cell::RefCell, rc::Rc};

use start_storage::StartStorage;

use super::{catalog::collection::RawDocument, collection::_SYSTEM_MASTER, operation_context::ensure_capacity, ops::{self, insert::insert_one_by_offset}};

pub fn get_header(storage: Rc<RefCell<StartStorage>>) -> Header {
    let ss = storage.clone();
    
    if ss.borrow_mut().len() == 0 {
        insert_one_by_offset(storage.clone(), _SYSTEM_MASTER.offset, RawDocument {
            next_document: 0,
            content_length: 40,
            content: _SYSTEM_MASTER.to_bytes(),
        });

        match Header::create(storage) {
            Ok(header) => {
                header
            },
            Err(HeaderError::DatabaseError(err)) => panic!("DBerr: {}", err),
            Err(err) => panic!("Header parsing error: {:?}", err)
        }
    } else {
        match Header::parse(storage) {
            Ok(header) => {
                header
            },
            Err(HeaderError::DatabaseError(err)) => panic!("DBerr: {}", err),
            Err(err) => panic!("Header parsing error: {:?}", err)
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub magic_number: MagicNumber,
    pub version: Version,
}

#[derive(Debug)]
pub struct MagicNumber(pub u32);

#[derive(Debug)]
pub struct Version(pub String);

// To check that it's database file
const MAGIC_NUMBER: u32 = 0x14841488;
const MAGIC_NUMBER_OFFSET: usize = 0;

const VERSION_OFFSET: usize = 4;
const CURRENT_VERSION: &str = "0.0.1";

impl Header {
    pub fn parse(storage: Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let ss_clone = storage.clone();
        let ss = ss_clone.borrow_mut();
        let magic_number = if ss.len() > 4 {
            MagicNumber::get(storage.clone())
        } else {
            Err(HeaderError::MagicNumberParsingError(
                "File is too short".to_string()
            ))
        }?;
        Self::ensure_capacity(storage.clone())?;
        
        Ok(Header {
            magic_number,
            version: Version::get(storage)?,
        })
    }

    pub fn create(ss: Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        Self::ensure_capacity(ss.clone())?;

        Ok(Header {
            magic_number: MagicNumber::create(ss.clone())?,
            version: Version::create(ss)?,
        })
    }

    fn ensure_capacity(ss: Rc<RefCell<StartStorage>>) -> Result<(), HeaderError> {
        match ensure_capacity(ss, 100) {
            Ok(_) => Ok(()),
            Err(err) => Err(HeaderError::DatabaseError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        }
    }
}

impl MagicNumber {
    fn get(ss: Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let ss = ss.borrow();

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&ss[0..4]);
        let magic_number = u32::from_le_bytes(bytes);
        if magic_number!=MAGIC_NUMBER {
            return Err(HeaderError::MagicNumberParsingError(
                format!("This's not database file! ({})", magic_number).into()
            ));
        }
        Ok(MagicNumber(magic_number))
    }

    fn create(ss: Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let mut ss = ss.borrow_mut();

        ss[MAGIC_NUMBER_OFFSET..MAGIC_NUMBER_OFFSET+4]
            .copy_from_slice(&MAGIC_NUMBER.to_le_bytes());
        Ok(MagicNumber(MAGIC_NUMBER))
    }
}

impl Default for MagicNumber {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Version {
    fn get(ss: Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let ss = ss.borrow();

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ss[VERSION_OFFSET..VERSION_OFFSET+8]);
        match String::from_utf8(bytes.to_vec()) {
            Ok(value) => Ok(Version(
                value.trim_matches(char::from(0))
                .to_string()
            )),
            Err(_) => Err(HeaderError::VersionParsingError(
                "Cannot parse string version".into()
            )),
        }
    }

    fn create(ss: Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let mut ss = ss.borrow_mut();

        let mut bytes = [0u8; 8];
        bytes[..CURRENT_VERSION.len()].copy_from_slice(CURRENT_VERSION.as_bytes());
        ss[VERSION_OFFSET..VERSION_OFFSET+8].copy_from_slice(&bytes);
        Ok(Version(CURRENT_VERSION.to_string()))
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug)]
pub enum HeaderError {
    MagicNumberParsingError(String),
    VersionParsingError(String),
    CollectionsParsingError(String),
    DatabaseError(String)
}