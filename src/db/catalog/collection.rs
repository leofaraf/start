use std::{cell::RefMut, collections::HashMap, str};

use bson::Bson;
use log::trace;

use crate::db::{collection::{Collection, _SYSTEM_MASTER}, operation_context::OperationContext, ops::insert::insert, recovery_unit::RecoveryUnit};

pub struct CollectionCatalog {
    pub collection_metadata: HashMap<String, Collection>   
}

#[derive(Debug)]
pub struct RawDocument {
    pub flag_deleted: bool,
    pub next_document: u64,
    pub content_length: u64,
    pub content: Vec<u8>,
}

impl CollectionCatalog {
    pub fn new() -> Self {
        Self {
            collection_metadata: HashMap::new(),
        }
    }

    pub fn lookup_collection(&self, op_ctx: &OperationContext, colname: &str) -> Collection {
        let mut next_document = _SYSTEM_MASTER.next_document;

        trace!("Colname: {}", colname);

        let rc_unit = op_ctx.rc_unit();

        while next_document != 0 {
            let name = Collection::parse_name(&rc_unit.borrow(), 
                next_document + DOCUMENT_CONTENT_OFFSET);

            if let Ok(text) = std::str::from_utf8(&name) {
                let text = text.trim_matches('\0');
                trace!("text: '{}', colname: '{}'", text, colname);
                if text.eq(colname) {
                    trace!("equals");
                    let next_d = rc_unit.borrow().effective_view(next_document + DOCUMENT_CONTENT_OFFSET, 40);
                    trace!("NextD: {:?} ({})", next_d, next_document);

                    let col_next_document = Collection::parse_next_document(&rc_unit.borrow(), 
                        next_document + DOCUMENT_CONTENT_OFFSET);

                    let collection = Collection {
                        name,
                        next_document: col_next_document,
                        offset: next_document,
                    };

                    trace!("Col: {:?}", collection);

                    return collection;
                }
            }

            next_document = RawDocument::parse_next_document(&rc_unit.borrow(), next_document) as usize
        }

        let col = Collection::new(colname, 0);

        trace!("Col: {:?}", col);

        col
    }

    pub fn acquire_collection_or_create(&mut self, colname: &str, op_ctx: &mut OperationContext) -> Collection {
        let mut next_document = _SYSTEM_MASTER.next_document;

        trace!("Colname: {}", colname);
        let rc_unit = op_ctx.rc_unit();

        while next_document != 0 {
            let name = Collection::parse_name(&rc_unit.borrow(), 
                next_document + DOCUMENT_CONTENT_OFFSET);

            if let Ok(text) = std::str::from_utf8(&name) {
                let text = text.trim_matches('\0');
                trace!("text: '{}', colname: '{}'", text, colname);
                if text.eq(colname) {
                    trace!("equals");
                    let col_next_document = Collection::parse_next_document(&rc_unit.borrow(), 
                        next_document + DOCUMENT_CONTENT_OFFSET);

                    let collection = Collection {
                        name,
                        next_document: col_next_document,
                        offset: next_document,
                    };
                    trace!("Col aq: {:?}", collection);

                    return collection;
                }
            }

            next_document = RawDocument::parse_next_document(&rc_unit.borrow(), next_document) as usize
        }

        let mut collection = Collection::new(colname, 0);

        let mut master = _SYSTEM_MASTER;
        let col_offset = insert(op_ctx, &mut master, 
            &collection.to_bytes());

        collection.offset = col_offset;
        trace!("Col aq: {:?}", collection);

        collection
    }
}

pub const DOCUMENT_FLAG_DELETED: usize = 0;
pub const DOCUMENT_RESERVED: usize = 1;
const DOCUMENT_NEXT_DOCUMENT_OFFSET: usize = 4;
pub const DOCUMENT_CONTENT_LENGHT_OFFSET: usize = 12;
pub const DOCUMENT_CONTENT_OFFSET: usize = 20;

impl RawDocument {
    pub fn len(&self) -> usize {
        self.content.len() + DOCUMENT_CONTENT_OFFSET
    }

    pub fn parse(ss: &RecoveryUnit, offset: usize) -> RawDocument {
        let content_length =  Self::parse_content_length(ss, offset);
        RawDocument {
            flag_deleted: Self::parse_flag_deleted(ss, offset),
            next_document: Self::parse_next_document(ss, offset),
            content_length,
            content: Self::parse_content(ss, offset, content_length as usize),
        }
    }

    pub fn parse_flag_deleted(ss: &RecoveryUnit, offset: usize) -> bool {
        ss
            .effective_view(
                offset+DOCUMENT_FLAG_DELETED, 1
            )[0]
            != 0
    }

    pub fn parse_next_document(ss: &RecoveryUnit, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &ss.effective_view(offset+DOCUMENT_NEXT_DOCUMENT_OFFSET, 8)
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content_length(ss: &RecoveryUnit, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &ss.effective_view(offset+DOCUMENT_CONTENT_LENGHT_OFFSET, 8)

        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content(ss: &RecoveryUnit, offset: usize, content_length: usize) -> Vec<u8> {
        ss.effective_view(offset+DOCUMENT_CONTENT_OFFSET, content_length)
            .to_vec()
    }

    pub fn write_flag_deleted(mut rc_unit: RefMut<'_, RecoveryUnit>, offset: usize, deleted: bool) {
        rc_unit.write(offset+DOCUMENT_FLAG_DELETED, &[deleted as u8]);
    }

    pub fn write_next_document(mut rc_unit: RefMut<'_, RecoveryUnit>, offset: usize, next_offset: usize) {
        rc_unit.write(offset+DOCUMENT_NEXT_DOCUMENT_OFFSET, &next_offset.to_le_bytes());
    }

    pub fn write_content_length(mut rc_unit: RefMut<'_, RecoveryUnit>, offset: usize, content_length: usize) {
        rc_unit.write(offset+DOCUMENT_CONTENT_LENGHT_OFFSET, &content_length.to_le_bytes());
    }

    pub fn write_content(mut rc_unit: RefMut<'_, RecoveryUnit>, offset: usize, content: &[u8]) {
        rc_unit.write(offset+DOCUMENT_CONTENT_OFFSET, content);
    }
}

impl From<&Collection> for RawDocument {
    fn from(value: &Collection) -> Self {
        Self {
            flag_deleted: false,
            next_document: 0,
            content_length: Collection::len(),
            content: value.to_bytes(),
        }
    }
}

impl From<Bson> for RawDocument {
    fn from(value: Bson) -> Self {
        todo!()
    }
}