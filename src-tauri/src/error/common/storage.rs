#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] super::database::DatabaseError),
    #[error("IO error: {0}")]
    IO(#[from] crate::error::common::io::IOError),
}

impl StorageError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            StorageError::DatabaseError(msg) => msg.get_status_code(),
            StorageError::IO(_) => 4070,
        }
    }
}
