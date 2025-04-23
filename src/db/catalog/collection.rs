use std::{collections::HashMap, str};

use bson::Bson;

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

    pub fn lookup_collection(&self, op_ctx: &OperationContext, colname: &str) -> Collection {
        let mut next_document = _SYSTEM_MASTER.next_document;

        println!("Colname: {}", colname);

        while next_document != 0 {
            let name = Collection::parse_name(&op_ctx.rc_unit, 
                next_document + DOCUMENT_CONTENT_OFFSET);

            if let Ok(text) = std::str::from_utf8(&name) {
                let text = text.trim_matches('\0');
                println!("text: '{}', colname: '{}'", text, colname);
                if text.eq(colname) {
                    println!("equals");
                    let next_d = op_ctx.rc_unit.effective_view(next_document + DOCUMENT_CONTENT_OFFSET, 40);
                    println!("NextD: {:?} ({})", next_d, next_document);

                    let col_next_document = Collection::parse_next_document(&op_ctx.rc_unit, 
                        next_document + DOCUMENT_CONTENT_OFFSET);

                    let collection = Collection {
                        name,
                        next_document: col_next_document,
                        offset: next_document,
                    };

                    println!("Col: {:?}", collection);

                    return collection;
                }
            }

            next_document = RawDocument::parse_next_document(&op_ctx.rc_unit, next_document) as usize
        }

        let col = Collection::new(colname, 0);

        println!("Col: {:?}", col);

        col
    }

    pub fn acquire_collection_or_create(&mut self, colname: &str, op_ctx: &mut OperationContext) -> Collection {
        let mut next_document = _SYSTEM_MASTER.next_document;

        println!("Colname: {}", colname);

        while next_document != 0 {
            let name = Collection::parse_name(&op_ctx.rc_unit, 
                next_document + DOCUMENT_CONTENT_OFFSET);

            if let Ok(text) = std::str::from_utf8(&name) {
                let text = text.trim_matches('\0');
                println!("text: '{}', colname: '{}'", text, colname);
                if text.eq(colname) {
                    println!("equals");
                    let col_next_document = Collection::parse_next_document(&op_ctx.rc_unit, 
                        next_document + DOCUMENT_CONTENT_OFFSET);

                    let collection = Collection {
                        name,
                        next_document: col_next_document,
                        offset: next_document,
                    };
                    println!("Col aq: {:?}", collection);

                    return collection;
                }
            }

            next_document = RawDocument::parse_next_document(&op_ctx.rc_unit, next_document) as usize
        }

        let mut collection = Collection::new(colname, 0);

        let col_offset = insert(op_ctx, _SYSTEM_MASTER, 
            &collection.to_bytes());

        collection.offset = col_offset;
        println!("Col aq: {:?}", collection);

        collection
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

    pub fn write_next_document(ss: &mut RecoveryUnit, offset: usize, next_offset: usize) {
        ss.write(offset+DOCUMENT_NEXT_DOCUMENT_OFFSET, &next_offset.to_le_bytes());
    }

    pub fn write_content_length(ss: &mut RecoveryUnit, offset: usize, content_length: usize) {
        ss.write(offset+DOCUMENT_CONTENT_LENGHT_OFFSET, &content_length.to_le_bytes());
    }

    pub fn write_content(ss: &mut RecoveryUnit, offset: usize, content: &[u8]) {
        ss.write(offset+DOCUMENT_CONTENT_OFFSET, content);
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