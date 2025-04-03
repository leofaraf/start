#[derive(Debug)]
pub struct Header {
    pub magic_number: MagicNumber,
    pub version: Version,
}

#[derive(Debug)]
pub struct MagicNumber(pub u32);

#[derive(Debug)]
pub struct Version(pub String);