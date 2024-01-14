#[derive(Debug, thiserror::Error)]
pub enum AccountUpdateTokenError {
    #[error("Database error: {0}")]
    Database(#[from] crate::error::common::database::DatabaseError),
    #[error("Http error: {0}")]
    Http(#[from] crate::error::common::http::HttpError),
    #[error("{0}")]
    Middleware(#[from] crate::error::common::middleware::MiddlewareError),
    #[error("Param error: {0}")]
    Param(#[from] crate::error::common::param::ParamError),
    #[error("Json error")]
    Parse(#[from] crate::error::common::parse::ParseError),
}

impl AccountUpdateTokenError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            AccountUpdateTokenError::Database(msg) => msg.get_status_code(),
            AccountUpdateTokenError::Http(msg) => msg.get_status_code(),
            AccountUpdateTokenError::Middleware(msg) => msg.get_status_code(),
            AccountUpdateTokenError::Param(msg) => msg.get_status_code(),
            AccountUpdateTokenError::Parse(msg) => msg.get_status_code(),
        }
    }
}
