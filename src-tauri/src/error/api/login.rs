#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Get mac address failed")]
    GetMacAddressFailed,
    #[error("Http error: {0}")]
    Http(#[from] crate::error::common::http::HttpError),
    #[error("Database error: {0}")]
    Database(#[from] crate::error::common::database::DatabaseError),
    #[error("{0}")]
    Middleware(#[from] crate::error::common::middleware::MiddlewareError),
    #[error("Param error: {0}")]
    Param(#[from] crate::error::common::param::ParamError),
    #[error("IO error: {0}")]
    IO(#[from] crate::error::common::io::IOError),
    #[error("Parse error")]
    Parse(#[from] crate::error::common::parse::ParseError),
}

impl LoginError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            LoginError::GetMacAddressFailed => 4002,
            LoginError::IO(msg) => msg.get_status_code(),
            LoginError::Http(msg) => msg.get_status_code(),
            LoginError::Database(msg) => msg.get_status_code(),
            LoginError::Middleware(msg) => msg.get_status_code(),
            LoginError::Param(msg) => msg.get_status_code(),
            LoginError::Parse(msg) => msg.get_status_code(),
        }
    }
}
