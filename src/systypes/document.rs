#[derive(Debug)]
pub struct RawDocument {
    pub next_document: u64,
    pub content_length: u64,
    pub content: Vec<u8>,
}

pub struct Document {
    pub content: Vec<u8>,
}