use std::{cell::{Ref, RefCell}, rc::Rc};

use super::{catalog::collection::{RawDocument, DOCUMENT_CONTENT_LENGHT_OFFSET, DOCUMENT_CONTENT_OFFSET}, operation_context::OperationContext, recovery_unit::RecoveryUnit, storage::{record_state, start_storage::StartStorage}};

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: [u8; 32],
    pub next_document: usize,
    pub offset: usize
}

/// We always assume that database created master table physically
pub const _SYSTEM_MASTER: Collection = Collection {
    name: *b"_system-master\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    next_document: 156,
    offset: 100
};

pub const _SYSTEM_TRASH: Collection = Collection {
    name: *b"_system-trash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    next_document: 0,
    offset: 156
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
        let last = self.last_document(&op_ctx.rc_unit().borrow());
        println!("Last: {}", last);

        let rc_unit = op_ctx.rc_unit();
        
        RawDocument::write_flag_deleted(rc_unit.borrow_mut(), allocated_space, false);
        println!("Length");
        RawDocument::write_content_length(rc_unit.borrow_mut(), allocated_space, data.len());
        println!("Content");
        RawDocument::write_content(rc_unit.borrow_mut(), allocated_space, data);
        println!("Linking");
        if last == 0 {
            rc_unit.borrow_mut().write(self.offset + DOCUMENT_CONTENT_OFFSET + 32, &allocated_space.to_le_bytes());
            let next_d = op_ctx.rc_unit().borrow().effective_view(self.offset + DOCUMENT_CONTENT_OFFSET, 40);
            println!("NextD: {:?} ({})", next_d, self.offset);
        } else {
            RawDocument::write_next_document(rc_unit.borrow_mut(), last, allocated_space);
        }
        allocated_space
    }

    pub fn delete_document(
        &self,
        op_ctx: &mut OperationContext,
        offset: usize
    ) {
        RawDocument::write_flag_deleted(
            op_ctx.rc_unit().borrow_mut(),
            offset,
            true
        );
    }

    pub fn find_doc() {}
    pub fn get_indexes() {}
    pub fn truncate() {}
    pub fn compact() {}
    pub fn rename() {}
    pub fn validate() {}

    pub fn last_document(
        &self,
        rc_unit: &Ref<'_, RecoveryUnit>,
    ) -> usize {
        let mut next_offset = self.next_document;

        while next_offset != 0 {
            let raw_doc_next = RawDocument::parse_next_document(rc_unit, next_offset);
    
            if raw_doc_next == 0 {
                return next_offset;
            }
            
            next_offset = raw_doc_next as usize; 
        }

        next_offset
    }

    // #[deprecated]
    // pub fn write_next_document(
    //     ss: Rc<RefCell<StartStorage>>,
    //     offset: usize,
    //     next_offset: usize
    // ) {
    //     let mut ss = ss.borrow_mut();
    //     ss[offset+DOCUMENT_CONTENT_OFFSET+32
    //     ..offset+DOCUMENT_CONTENT_OFFSET+40]
    //     .copy_from_slice(&next_offset.to_le_bytes());
    // }

    pub fn parse(rc_unit: &Ref<'_, RecoveryUnit>, offset: usize) -> Collection {
        Collection {
            name: Self::parse_name(&rc_unit, offset),
            next_document: Self::parse_next_document(&rc_unit, offset),
            offset
        }
    }

    pub fn parse_name(rc_unit: &Ref<'_, RecoveryUnit>, offset: usize) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&rc_unit.effective_view(offset, 32));
        bytes
    }

    pub fn parse_next_document(rc_unit: &Ref<'_, RecoveryUnit>, offset: usize) -> usize {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&rc_unit.effective_view(offset+32, 8));
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