use std::cell::RefMut;

use start_storage::StartStorage;

use crate::db::{catalog::collection::DOCUMENT_CONTENT_OFFSET, operation_context::ensure_capacity};

pub fn find_available_space() {}
// Create new space, link new space to collection
/// Returns offset to allocated space
pub fn allocate_extent(
    mut ss: RefMut<'_, StartStorage>,
    space_required: usize
) -> usize {
    let offset = ss.len();
    ensure_capacity(&mut ss, offset + space_required + DOCUMENT_CONTENT_OFFSET).unwrap();

    offset
}