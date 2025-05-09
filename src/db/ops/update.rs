use bson::{Bson, Document};
use log::trace;

use crate::{db::{catalog::{collection::RawDocument, session::Session}, collection::Collection, operation_context::OperationContext, ops, query::filtering::{self, Filter}, service_context::ServiceContext}, HandleResult};

pub fn update(
    op_ctx: &mut OperationContext,
    filter: Option<Filter>,
    update_document: Document,
    collection: &mut Collection
) -> HandleResult<()> {
    let mut next_offset = collection.next_document as usize;
    let rc_unit = op_ctx.rc_unit();
    
    while next_offset != 0 {
        let raw_doc = RawDocument::parse(&rc_unit.borrow(), next_offset);

        if raw_doc.flag_deleted {
            next_offset = raw_doc.next_document as usize;
            continue;
        }
        trace!("RawDoc: {:?}", raw_doc);

        let doc = bson::from_slice(&raw_doc.content)?;

        if filter.as_ref().map_or(true, |f| filtering::matches_filter(&doc, f)) {
            // ✅ Handle $set update
            if let Some(Bson::Document(set_fields)) = update_document.get("$set") {
                let mut new_doc = doc.clone();
                for (k, v) in set_fields {
                    new_doc.insert(k.clone(), v.clone());
                }

                // 🚀 Serialize new document
                let updated_bytes = bson::to_vec(&new_doc)?;

                trace!("Update filters matched. New doc: {:?}", new_doc);

                // 💾 Insert new document and delete the old one
                collection.insert_document(op_ctx, &updated_bytes);
                collection.delete_document(op_ctx, next_offset as usize);
            } else {
                trace!("Warning: update_document does not contain $set, skipping.");
            }
        }

        next_offset = raw_doc.next_document as usize;
    }

    Ok(())
}