use std::{cell::RefCell, rc::Rc};

use start_storage::StartStorage;

use super::catalog::collection::DOCUMENT_CONTENT_OFFSET;

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: [u8; 32],
    pub next_document: u64
}

pub const SYS_MASTER_OFFSET: usize = 100;
pub const SYS_MASTER: Collection = Collection {
    name: *b"sys-master\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    // master next document is next table
    next_document: SYS_TRASH_OFFSET
};

pub const SYS_TRASH_OFFSET: u64 = 156;
pub const SYS_TRASH: Collection = Collection {
    name: *b"sys-trash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    next_document: 0
};

impl Collection {
    pub fn insert_document() {}
    pub fn delete_document() {}
    pub fn find_doc() {}
    pub fn get_indexes() {}
    pub fn truncate() {}
    pub fn compact() {}
    pub fn rename() {}
    pub fn validate() {}

    pub fn write_next_document(
        ss: Rc<RefCell<StartStorage>>,
        offset: usize,
        next_offset: usize
    ) {
        let mut ss = ss.borrow_mut();
        ss[offset+DOCUMENT_CONTENT_OFFSET+32
        ..offset+DOCUMENT_CONTENT_OFFSET+40]
        .copy_from_slice(&next_offset.to_le_bytes());
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

    pub fn len() -> u64 {40}
}