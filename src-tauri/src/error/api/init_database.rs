#[derive(Debug, thiserror::Error)]
pub enum InitDatabaseError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::error::common::database::DatabaseError),
}

impl InitDatabaseError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            InitDatabaseError::DatabaseError(msg) => msg.get_status_code(),
        }
    }
}
