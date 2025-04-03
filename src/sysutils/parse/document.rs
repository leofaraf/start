use start_storage::StartStorage;

use crate::systypes::document::RawDocument;

const DOCUMENT_NEXT_DOCUMENT_OFFSET: usize = 0;
const DOCUMENT_CONTENT_LENGHT_OFFSET: usize = 8;
pub const DOCUMENT_CONTENT_OFFSET: usize = 16;

impl RawDocument {
    pub fn len(&self) -> usize {
        self.content.len() + 8 + 8
    }

    pub fn parse(ss: &StartStorage, offset: usize) -> RawDocument {
        let content_length =  Self::parse_content_length(ss, offset);
        RawDocument {
            next_document: Self::parse_next_document(ss, offset),
            content_length,
            content: Self::parse_content(ss, offset, content_length as usize),
        }
    }

    pub fn parse_next_document(ss: &StartStorage, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &ss[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
            ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content_length(ss: &StartStorage, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &ss[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
            ..offset+DOCUMENT_CONTENT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content(ss: &StartStorage, offset: usize, content_length: usize) -> Vec<u8> {
        ss[offset + DOCUMENT_CONTENT_OFFSET
            ..offset + DOCUMENT_CONTENT_OFFSET + content_length]
            .to_vec()
    }

    pub fn write_next_document(ss: &mut StartStorage, offset: usize, next_offset: usize) {
        ss[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
        ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        .copy_from_slice(&next_offset.to_le_bytes());
    }

    pub fn write_content_length(ss: &mut StartStorage, offset: usize, content_length: usize) {
        ss[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET]
        .copy_from_slice(&content_length.to_le_bytes());
    }

    pub fn write_content(ss: &mut StartStorage, offset: usize, content: &[u8]) {
        ss[offset+DOCUMENT_CONTENT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET+content.len()]
        .copy_from_slice(content);
    }
}