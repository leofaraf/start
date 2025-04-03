use start_storage::StartStorage;

pub fn ensure_capacity(ss: &mut StartStorage, required_size: usize) -> Result<(), DocumentsError> {
    let current_size = ss.len();
    if required_size > current_size {
        match ss.resize(required_size) {
            Ok(_) => Ok(()),
            Err(err) => Err(DocumentsError::DatabaseError(
                format!("Ensure capatiry error: {:?}", err).into()
            )),
        };
    }
    Ok(())
}

#[derive(Debug)]
pub enum DocumentsError {
    PrimaryKeyError(String),
    DatabaseError(String)
}