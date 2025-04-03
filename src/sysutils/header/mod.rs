use start_storage::StartStorage;

use crate::systypes::header::{Header, MagicNumber, Version};

use super::capacity::ensure_capacity;

// To check that it's database file
const MAGIC_NUMBER: u32 = 0x14841488;
const MAGIC_NUMBER_OFFSET: usize = 0;

const VERSION_OFFSET: usize = 4;
const CURRENT_VERSION: &str = "0.0.1";

impl Header {
    pub fn parse(ss: &mut StartStorage) -> Result<Self, HeaderError> {
        let magic_number = if ss.len() > 4 {
            MagicNumber::get(ss)
        } else {
            Err(HeaderError::MagicNumberParsingError(
                "File is too short".to_string()
            ))
        }?;
        Self::ensure_capacity(ss)?;
        
        Ok(Header {
            magic_number,
            version: Version::get(&ss)?,
        })
    }

    pub fn create(ss: &mut StartStorage) -> Result<Self, HeaderError> {
        Self::ensure_capacity(ss)?;

        Ok(Header {
            magic_number: MagicNumber::create(ss)?,
            version: Version::create(ss)?,
        })
    }

    fn ensure_capacity(ss: &mut StartStorage) -> Result<(), HeaderError> {
        match ensure_capacity(ss, 100) {
            Ok(_) => Ok(()),
            Err(err) => Err(HeaderError::DatabaseError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        }
    }
}

impl MagicNumber {
    fn get(ss: &StartStorage) -> Result<Self, HeaderError> {
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

    fn create(ss: &mut StartStorage) -> Result<Self, HeaderError> {
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
    fn get(ss: &StartStorage) -> Result<Self, HeaderError> {
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

    fn create(ss: &mut StartStorage) -> Result<Self, HeaderError> {
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