use std::{cell::{Ref, RefCell, RefMut}, collections::HashMap, rc::Rc, str};

use bson::Bson;
use start_storage::StartStorage;

use crate::db::{collection::{Collection, _SYSTEM_MASTER}, operation_context::OperationContext, ops::insert::insert, recovery_unit::RecoveryUnit};

pub struct CollectionCatalog {
    pub collection_metadata: HashMap<String, Collection>   
}

#[derive(Debug)]
pub struct RawDocument {
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

    pub fn lookup_collection(&self, name: &str) -> Collection {
        let col = match self.collection_metadata.get(name) {
            Some(col) => col.clone(),
            None => Collection::new(name, 0)
        };

        col
    }

    pub fn acquire_collection_or_create(&mut self, name: &str, op_ctx: &mut OperationContext) -> Collection {
        let col: Collection = match self.collection_metadata.get_mut(name) {
            Some(col) => col.clone(),
            None => {
                let mut collection = Collection::new(name, 0);

                let col_offset = insert(op_ctx, _SYSTEM_MASTER, 
                    &collection.to_bytes());

                collection.offset = col_offset;

                self.collection_metadata.insert(name.to_string(), collection.clone());
                collection
            }
        };

        col
    }
}

const DOCUMENT_NEXT_DOCUMENT_OFFSET: usize = 0;
pub const DOCUMENT_CONTENT_LENGHT_OFFSET: usize = 8;
pub const DOCUMENT_CONTENT_OFFSET: usize = 16;

impl RawDocument {
    pub fn len(&self) -> usize {
        self.content.len() + 8 + 8
    }

    pub fn parse(ss: &RecoveryUnit, offset: usize) -> RawDocument {
        let content_length =  Self::parse_content_length(ss, offset);
        RawDocument {
            next_document: Self::parse_next_document(ss, offset),
            content_length,
            content: Self::parse_content(ss, offset, content_length as usize),
        }
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

    pub fn write_next_document(ss: &mut RefMut<'_, StartStorage>, offset: usize, next_offset: usize) {
        ss[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
        ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        .copy_from_slice(&next_offset.to_le_bytes());
    }

    pub fn write_content_length(ss: &mut RefMut<'_, StartStorage>, offset: usize, content_length: usize) {
        ss[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET]
        .copy_from_slice(&content_length.to_le_bytes());
    }

    pub fn write_content(ss: &mut RefMut<'_, StartStorage>, offset: usize, content: &[u8]) {
        ss[offset+DOCUMENT_CONTENT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET+content.len()]
        .copy_from_slice(content);
    }
}

impl From<&Collection> for RawDocument {
    fn from(value: &Collection) -> Self {
        Self {
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