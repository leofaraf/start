use std::path::PathBuf;

use service_context::ServiceContext;
use start_storage::StartStorage;

use crate::{systypes::{collection::{SYS_MASTER, SYS_MASTER_OFFSET, SYS_TRASH}, document::RawDocument, header::Header}, sysutils::{header::HeaderError, insert::one::{insert_one, insert_one_by_offset}}, HandleResult};

pub mod query;
pub mod exec;
pub mod commands;
pub mod ops;
pub mod collection;
pub mod catalog;
pub mod operation_context;
pub mod service_context;
pub mod header;