use bson::Bson;

use crate::db::{collection::Collection, operation_context::OperationContext};

pub fn insert(
    op_ctx: OperationContext,
    collection: Collection,
    document: Bson
) {
    
}