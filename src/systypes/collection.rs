#[derive(Debug)]
pub struct Collection {
    pub name: [u8; 32],
    pub next_document: u64
}

pub const SYS_MASTER_OFFSET: u64 = 100;
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