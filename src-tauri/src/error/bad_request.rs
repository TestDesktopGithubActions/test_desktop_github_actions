#[derive(Debug, thiserror::Error)]
pub enum BadRequest {
    #[error("Login error: {0}")]
    Login(#[from] super::api::login::LoginError),
    #[error("Logout error: {0}")]
    Logout(#[from] super::api::logout::LogoutError),
    #[error("Get info error: {0}")]
    GetInfo(#[from] super::api::info::GetInfoError),
    #[error("Account update token error: {0}")]
    AccountUpdateToken(#[from] super::api::account_update_token::AccountUpdateTokenError),
    #[error("Node error: {0}")]
    Node(#[from] super::api::node::NodeError),
    #[error("Upload error: {0}")]
    Upload(#[from] super::api::upload_log::UploadError),
    #[error("System tray error: {0}")]
    SystemTray(#[from] super::common::system_tray::SystemTrayError),
    #[error("Language error: {0}")]
    Language(#[from] super::common::language::LanguageError),
    #[error("Init database error: {0}")]
    InitDatabase(#[from] super::api::init_database::InitDatabaseError),
    #[error("Storage error: {0}")]
    Storage(#[from] super::common::storage::StorageError),
    #[error("Splash screen error: {0}")]
    Splashscreen(#[from] super::api::splashscreen::SplashscreenError),
    #[error("Ping error: {0}")]
    Ping(#[from] super::api::ping::PingError),
}

impl BadRequest {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            BadRequest::Login(msg) => msg.get_status_code(),
            BadRequest::Logout(msg) => msg.get_status_code(),
            BadRequest::GetInfo(msg) => msg.get_status_code(),
            BadRequest::AccountUpdateToken(msg) => msg.get_status_code(),
            BadRequest::Node(msg) => msg.get_status_code(),
            BadRequest::Upload(msg) => msg.get_status_code(),
            BadRequest::SystemTray(msg) => msg.get_status_code(),
            BadRequest::Language(msg) => msg.get_status_code(),
            BadRequest::InitDatabase(msg) => msg.get_status_code(),
            BadRequest::Storage(msg) => msg.get_status_code(),
            BadRequest::Splashscreen(msg) => msg.get_status_code(),
            BadRequest::Ping(msg) => msg.get_status_code(),
        }
    }
}
