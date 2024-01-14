#[derive(Debug, thiserror::Error)]
pub enum LanguageError {
    #[error("Set language failed")]
    SetFailed,
    #[error("Database error: {0}")]
    DatabaseError(#[from] super::database::DatabaseError),
    #[error("System tray error: {0}")]
    SystemTray(#[from] super::system_tray::SystemTrayError),
}

impl LanguageError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            LanguageError::SetFailed => 4060,
            LanguageError::DatabaseError(msg) => msg.get_status_code(),
            LanguageError::SystemTray(msg) => msg.get_status_code(),
        }
    }
}
