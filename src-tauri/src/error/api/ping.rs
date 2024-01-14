#[derive(Debug, thiserror::Error)]
pub enum PingError {
    #[error("Param error: {0}")]
    Param(#[from] crate::error::common::param::ParamError),
    #[error("Parse error")]
    Parse(#[from] crate::error::common::parse::ParseError),
}

impl PingError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            PingError::Param(msg) => msg.get_status_code(),
            PingError::Parse(msg) => msg.get_status_code(),
        }
    }
}
