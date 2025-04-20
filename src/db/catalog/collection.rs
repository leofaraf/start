use std::{cell::RefCell, collections::HashMap, rc::Rc, str};

use start_storage::StartStorage;

use crate::db::{collection::{Collection, SYS_MASTER}, operation_context::OperationContext, ops::insert::insert};

#[derive(Debug, Clone)]
pub struct CollectionMetadata {
    pub collection: Collection,
    pub offset: usize,
    pub name: String
}

pub struct CollectionCatalog {
    pub collection_metadata: HashMap<String, CollectionMetadata>   
}

#[derive(Debug)]
#[deprecated]
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

    pub fn autocol_readonly(&self, name: &str) -> CollectionMetadata {
        let col = match self.collection_metadata.get(name) {
            Some(col) => col.clone(),
            None => {
                let mut bytes = [0u8; 32];
                bytes[0..name.len()].copy_from_slice(name.as_bytes());
                let collection = Collection {
                    name: bytes,
                    next_document: 0,
                };
                CollectionMetadata {
                    collection,
                    offset: 0,
                    name: name.to_string()
                }
            }
        };

        println!("Readonly Collection catalog: {:?}", self.collection_metadata);

        col
    }

    pub fn autocol(&mut self, name: &str, op_ctx: &OperationContext) -> CollectionMetadata {
        let col: CollectionMetadata = match self.collection_metadata.get(name) {
            Some(col) => col.clone(),
            None => {
                let mut bytes = [0u8; 32];
                bytes[0..name.len()].copy_from_slice(name.as_bytes());
            
                let collection = Collection {
                    name: bytes,
                    next_document: 0,
                };

                let master_meta = CollectionMetadata {
                    collection: SYS_MASTER,
                    offset: 100,
                    name: name.to_string()
                };
            
                let col_offset = insert(op_ctx, master_meta, RawDocument {
                    next_document: 0,
                    content_length: Collection::len(),
                    content: collection.to_bytes(),
                }, false);

                let collection = Collection {
                    name: bytes,
                    next_document: 0,
                };
                let meta = CollectionMetadata {
                    collection,
                    offset: col_offset,
                    name: name.to_string()
                };

                self.collection_metadata.insert(name.to_string(), meta.clone());
                meta
            }
        };

        println!("AutoCol Res: {:?}", col);
        println!("Collection catalog: {:?}", self.collection_metadata);

        col
    }
}

const DOCUMENT_NEXT_DOCUMENT_OFFSET: usize = 0;
const DOCUMENT_CONTENT_LENGHT_OFFSET: usize = 8;
pub const DOCUMENT_CONTENT_OFFSET: usize = 16;

impl RawDocument {
    pub fn len(&self) -> usize {
        self.content.len() + 8 + 8
    }

    pub fn parse(ss: Rc<RefCell<StartStorage>>, offset: usize) -> RawDocument {
        let content_length =  Self::parse_content_length(ss.clone(), offset);
        RawDocument {
            next_document: Self::parse_next_document(ss.clone(), offset),
            content_length,
            content: Self::parse_content(ss, offset, content_length as usize),
        }
    }

    pub fn parse_next_document(ss: Rc<RefCell<StartStorage>>, offset: usize) -> u64 {
        let ss = ss.borrow();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &ss[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
            ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content_length(ss: Rc<RefCell<StartStorage>>, offset: usize) -> u64 {
        let ss = ss.borrow();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(
            &ss[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
            ..offset+DOCUMENT_CONTENT_OFFSET]
        );
        u64::from_le_bytes(bytes)
    }

    pub fn parse_content(ss: Rc<RefCell<StartStorage>>, offset: usize, content_length: usize) -> Vec<u8> {
        let ss = ss.borrow();
        ss[offset + DOCUMENT_CONTENT_OFFSET
            ..offset + DOCUMENT_CONTENT_OFFSET + content_length]
            .to_vec()
    }

    pub fn write_next_document(ss: Rc<RefCell<StartStorage>>, offset: usize, next_offset: usize) {
        let mut ss = ss.borrow_mut();
        ss[offset+DOCUMENT_NEXT_DOCUMENT_OFFSET
        ..offset+DOCUMENT_CONTENT_LENGHT_OFFSET]
        .copy_from_slice(&next_offset.to_le_bytes());
    }

    pub fn write_content_length(ss: Rc<RefCell<StartStorage>>, offset: usize, content_length: usize) {
        let mut ss = ss.borrow_mut();
        ss[offset+DOCUMENT_CONTENT_LENGHT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET]
        .copy_from_slice(&content_length.to_le_bytes());
    }

    pub fn write_content(ss: Rc<RefCell<StartStorage>>, offset: usize, content: &[u8]) {
        let mut ss = ss.borrow_mut();
        ss[offset+DOCUMENT_CONTENT_OFFSET
        ..offset+DOCUMENT_CONTENT_OFFSET+content.len()]
        .copy_from_slice(content);
    }
}