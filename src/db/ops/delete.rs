use crate::db::{catalog::collection::RawDocument, collection::Collection, operation_context::OperationContext, query::filtering::{self, Filter}};

pub fn delete(
    op_ctx: &mut OperationContext,
    col: Collection,
    filter: Option<Filter>,
) {
    let mut next_offset = col.next_document as usize;
    let rc_unit = op_ctx.rc_unit();
    
    while next_offset != 0 {
        let raw_doc = RawDocument::parse(&rc_unit.borrow(), next_offset);

        if raw_doc.flag_deleted {
            next_offset = raw_doc.next_document as usize;
            continue;
        }
        println!("RawDoc: {:?}", raw_doc);

        let doc = bson::from_slice(&raw_doc.content).unwrap();
        if let Some(filter) = &filter {
            if filtering::matches_filter(&doc, filter) {
                col.delete_document(op_ctx, next_offset);
            }
        };
        next_offset = raw_doc.next_document as usize;
    }
}