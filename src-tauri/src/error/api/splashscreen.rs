#[derive(Debug, thiserror::Error)]
pub enum SplashscreenError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::error::common::database::DatabaseError),
    #[error("Systemtray error: {0}")]
    SystemTray(#[from] crate::error::common::system_tray::SystemTrayError),
    #[error("Language error: {0}")]
    Language(#[from] crate::error::common::language::LanguageError),
    #[error("Get main window failed")]
    GetMainWindowFailed,
    #[error("Show main window failed")]
    ShowMainWindowFailed,
    #[error("Close splashscreen failed")]
    CloseFailed,
}

impl SplashscreenError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            SplashscreenError::DatabaseError(msg) => msg.get_status_code(),
            SplashscreenError::SystemTray(msg) => msg.get_status_code(),
            SplashscreenError::Language(msg) => msg.get_status_code(),
            SplashscreenError::GetMainWindowFailed => 4080,
            SplashscreenError::ShowMainWindowFailed => 4081,
            SplashscreenError::CloseFailed => 4082,
        }
    }
}
