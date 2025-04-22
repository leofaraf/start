use std::{cell::{Ref, RefCell}, rc::Rc};

use start_storage::StartStorage;

use super::{catalog::collection::{RawDocument, DOCUMENT_CONTENT_LENGHT_OFFSET, DOCUMENT_CONTENT_OFFSET}, operation_context::OperationContext, recovery_unit::RecoveryUnit, storage::record_state};

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: [u8; 32],
    pub next_document: usize,
    pub offset: usize
}

/// We always assume that database created master table physically
pub const _SYSTEM_MASTER: Collection = Collection {
    name: *b"_system-master\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    next_document: 0,
    offset: 100
};

impl Collection {
    pub fn new(name: &str, offset: usize) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..name.len()].copy_from_slice(name.as_bytes());

        Self {
            name: bytes,
            next_document: 0,
            offset,
        }
    }

    pub fn insert_document(
        self,
        op_ctx: &mut OperationContext,
        data: &[u8]
    ) -> usize {
        let storage = op_ctx.storage();
        let allocated_space = record_state::allocate_extent(storage.borrow_mut(), data.len());
        println!("Allotaed: {}", allocated_space);
        let last = self.last_document(&op_ctx.rc_unit);
        println!("Last: {}", last);

        println!("Length");
        op_ctx.rc_unit.write(allocated_space + DOCUMENT_CONTENT_LENGHT_OFFSET, &data.len().to_le_bytes());
        println!("Content");
        op_ctx.rc_unit.write(allocated_space + DOCUMENT_CONTENT_OFFSET, data);
        println!("Linking");
        if last == 0 {
            op_ctx.rc_unit.write(self.offset + DOCUMENT_CONTENT_OFFSET + 32, &allocated_space.to_le_bytes());
        } else {
            op_ctx.rc_unit.write(last, &allocated_space.to_le_bytes());
        }
        allocated_space
    }

    pub fn delete_document() {}
    pub fn find_doc() {}
    pub fn get_indexes() {}
    pub fn truncate() {}
    pub fn compact() {}
    pub fn rename() {}
    pub fn validate() {}

    pub fn last_document(
        &self,
        rc_unit: &RecoveryUnit,
    ) -> usize {
        let mut next_offset  = self.next_document;

        while next_offset != 0 {
            let raw_doc_next = RawDocument::parse_next_document(rc_unit, next_offset);
    
            if raw_doc_next == 0 {
                return next_offset;
            }
            
            next_offset = raw_doc_next as usize; 
        }

        next_offset
    }

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

    pub fn parse(ss: &Ref<'_, StartStorage>, offset: usize) -> Collection {
        Collection {
            name: Self::parse_name(ss, offset),
            next_document: Self::next_document(ss, offset),
            offset
        }
    }

    pub fn parse_name(ss: &Ref<'_, StartStorage>, offset: usize) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&ss[offset..offset+32]);
        bytes
    }

    pub fn next_document(ss: &Ref<'_, StartStorage>, offset: usize) -> usize {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&ss[offset+32..offset+40]);
        u64::from_le_bytes(bytes) as usize
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(40);
        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.next_document.to_le_bytes());
        bytes
    }

    pub fn len() -> u64 {40}
}