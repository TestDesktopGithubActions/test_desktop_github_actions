#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("Http error: {0}")]
    Http(#[from] crate::error::common::http::HttpError),
    #[error("{0}")]
    Middleware(#[from] crate::error::common::middleware::MiddlewareError),
    #[error("Param error: {0}")]
    Param(#[from] crate::error::common::param::ParamError),
    #[error("Json error")]
    Parse(#[from] crate::error::common::parse::ParseError),
}

impl NodeError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            NodeError::Http(msg) => msg.get_status_code(),
            NodeError::Middleware(msg) => msg.get_status_code(),
            NodeError::Param(msg) => msg.get_status_code(),
            NodeError::Parse(msg) => msg.get_status_code(),
        }
    }
}
