#[derive(Debug)]
pub struct RawDocument {
    pub _id: [u8; 16],
    pub next_document: u64,
    pub content_lenght: u64,
    pub content_offset: Vec<u8>,
}

pub struct Document {
    pub primary_key: [u8; 16],
    pub content: Vec<u8>,
}