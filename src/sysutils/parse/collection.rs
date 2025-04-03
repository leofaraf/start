use start_storage::StartStorage;

use crate::systypes::collection::Collection;

use super::document::DOCUMENT_CONTENT_OFFSET;

impl Collection {
    pub const fn len() -> u64 {
        40
    }

    pub fn parse(content: &[u8]) -> Collection {
        Collection {
            name: Self::parse_name(&content),
            next_document: Self::next_document(&content),
        }
    }

    pub fn parse_name(content: &[u8]) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&content[..32]);
        bytes
    }

    pub fn next_document(content: &[u8]) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&content[32..40]);
        u64::from_le_bytes(bytes)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(40);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.next_document.to_le_bytes());
        bytes
    }

    pub fn write_next_document(
        ss: &mut StartStorage,
        offset: usize,
        next_offset: usize
    ) {
        ss[offset+DOCUMENT_CONTENT_OFFSET+32
        ..offset+DOCUMENT_CONTENT_OFFSET+40]
        .copy_from_slice(&next_offset.to_le_bytes());
    }
}