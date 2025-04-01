pub struct Table {
    name: [u8; 32],
    next_document: usize
}

const SYS_MASTER_OFFSET: usize = 100;
const SYS_MASTER: Table = Table {
    name: *b"sys-master\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    // master next document is next table
    next_document: SYS_TRASH_OFFSET
};

const SYS_TRASH_OFFSET: usize = 156;
const SYS_TRASH: Table = Table {
    name: *b"sys-trash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    next_document: 0
};