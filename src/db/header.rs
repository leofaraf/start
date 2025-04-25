use std::{cell::{RefCell, RefMut}, rc::Rc};

use crate::HandleResult;

use super::{
    catalog::collection::RawDocument,
    collection::{_SYSTEM_MASTER, _SYSTEM_TRASH},
    operation_context::{ensure_capacity, OperationContext},
    ops::insert::insert_one_by_offset,
    storage::start_storage::StartStorage,
};

pub fn get_header(op_ctx: &mut OperationContext) -> HandleResult<Header> {
    let storage = op_ctx.storage();

    if storage.borrow().len() == 0 {
        insert_one_by_offset(op_ctx, _SYSTEM_MASTER.offset, RawDocument {
            flag_deleted: false,
            next_document: 0,
            content_length: 40,
            content: _SYSTEM_MASTER.to_bytes(),
        })?;

        insert_one_by_offset(op_ctx, _SYSTEM_TRASH.offset, RawDocument {
            flag_deleted: false,
            next_document: 0,
            content_length: 40,
            content: _SYSTEM_TRASH.to_bytes(),
        })?;

        match Header::create(&storage) {
            Ok(header) => Ok(header),
            Err(HeaderError::DatabaseError(err)) => panic!("DBerr: {}", err),
            Err(err) => panic!("Header parsing error: {:?}", err),
        }
    } else {
        match Header::parse(&storage) {
            Ok(header) => Ok(header),
            Err(HeaderError::DatabaseError(err)) => panic!("DBerr: {}", err),
            Err(err) => panic!("Header parsing error: {:?}", err),
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

const MAGIC_NUMBER: u32 = 0x14841488;
const MAGIC_NUMBER_OFFSET: usize = 0;

const VERSION_OFFSET: usize = 4;
const CURRENT_VERSION: &str = "0.0.1";

impl Header {
    pub fn parse(storage: &Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        if storage.borrow().len() < VERSION_OFFSET + 8 {
            return Err(HeaderError::MagicNumberParsingError("File is too short".to_string()));
        }

        let magic_number = MagicNumber::get(storage)?;
        Self::ensure_capacity(&mut storage.borrow_mut())?;

        Ok(Header {
            magic_number,
            version: Version::get(storage)?,
        })
    }

    pub fn create(storage: &Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        Self::ensure_capacity(&mut storage.borrow_mut())?;

        Ok(Header {
            magic_number: MagicNumber::create(storage)?,
            version: Version::create(storage)?,
        })
    }

    fn ensure_capacity(ss: &mut RefMut<'_, StartStorage>) -> Result<(), HeaderError> {
        ensure_capacity(ss, 100).map_err(|err| {
            HeaderError::DatabaseError(format!("Ensure capacity error: {:?}", err))
        })
    }
}

impl MagicNumber {
    fn get(storage: &Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let ss = storage.borrow();
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&ss[MAGIC_NUMBER_OFFSET..MAGIC_NUMBER_OFFSET + 4]);
        let magic_number = u32::from_le_bytes(bytes);
        if magic_number != MAGIC_NUMBER {
            return Err(HeaderError::MagicNumberParsingError(
                format!("Invalid database file! ({})", magic_number),
            ));
        }
        Ok(MagicNumber(magic_number))
    }

    fn create(storage: &Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let mut ss = storage.borrow_mut();
        ss[MAGIC_NUMBER_OFFSET..MAGIC_NUMBER_OFFSET + 4]
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
    fn get(storage: &Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let ss = storage.borrow();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ss[VERSION_OFFSET..VERSION_OFFSET + 8]);
        String::from_utf8(bytes.to_vec())
            .map(|v| Version(v.trim_matches(char::from(0)).to_string()))
            .map_err(|_| HeaderError::VersionParsingError("Cannot parse string version".into()))
    }

    fn create(storage: &Rc<RefCell<StartStorage>>) -> Result<Self, HeaderError> {
        let mut ss = storage.borrow_mut();
        let mut bytes = [0u8; 8];
        bytes[..CURRENT_VERSION.len()].copy_from_slice(CURRENT_VERSION.as_bytes());
        ss[VERSION_OFFSET..VERSION_OFFSET + 8].copy_from_slice(&bytes);
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
    DatabaseError(String),
}
