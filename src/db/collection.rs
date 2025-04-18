#[derive(Debug)]
pub struct Collection {
    pub name: [u8; 32],
    pub next_document: u64
}

pub const SYS_MASTER_OFFSET: u64 = 100;
pub const SYS_MASTER: Collection = Collection {
    name: *b"sys-master\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    // master next document is next table
    next_document: 0
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
}