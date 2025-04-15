use start_storage::StartStorage;

use crate::{systypes::document::RawDocument, sysutils::{self, find::find_collection::find_collection}};

pub fn find_many(
    ss: &mut StartStorage,
    collection: &str,
) -> Vec<RawDocument> {
    let opt_col = find_collection(ss, collection);

    if let Some(col) = opt_col {
        return sysutils::find::many::find_many(ss, col);
    }

    vec![]
}