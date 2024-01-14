#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("File open failed")]
    FileOpenFailed,
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::error::common::database::DatabaseError),
    #[error("Http error: {0}")]
    Http(#[from] crate::error::common::http::HttpError),
    #[error("Param error: {0}")]
    Param(#[from] crate::error::common::param::ParamError),
    #[error("Json error")]
    Parse(#[from] crate::error::common::parse::ParseError),
}

impl UploadError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            UploadError::FileOpenFailed => 4040,
            UploadError::DatabaseError(msg) => msg.get_status_code(),
            UploadError::Http(msg) => msg.get_status_code(),
            UploadError::Param(msg) => msg.get_status_code(),
            UploadError::Parse(msg) => msg.get_status_code(),
        }
    }
}
