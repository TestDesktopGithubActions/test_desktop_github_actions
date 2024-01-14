#[derive(Debug, thiserror::Error)]
pub enum GetInfoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::error::common::database::DatabaseError),
}

impl GetInfoError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            GetInfoError::DatabaseError(msg) => msg.get_status_code(),
        }
    }
}
