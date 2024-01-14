#[derive(Debug, thiserror::Error)]
pub enum LogoutError {
    #[error("Param error: {0}")]
    Param(#[from] crate::error::common::param::ParamError),
    #[error("Http error: {0}")]
    Http(#[from] crate::error::common::http::HttpError),
    #[error("{0}")]
    Middleware(#[from] crate::error::common::middleware::MiddlewareError),
}

impl LogoutError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            LogoutError::Http(msg) => msg.get_status_code(),
            LogoutError::Middleware(msg) => msg.get_status_code(),
            LogoutError::Param(msg) => msg.get_status_code(),
        }
    }
}
