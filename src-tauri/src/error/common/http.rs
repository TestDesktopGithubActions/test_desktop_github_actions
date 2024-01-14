#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Http builder generate failed")]
    HttpBuilderGenFailed,
    #[error("Received a non-success status: {0}")]
    NonSuccessStatus(reqwest::StatusCode),
    #[error("Get physical iface error")]
    GetPhysicalIfaceError,
    #[error("Operation timed out")]
    TimedOut,
    #[error("Request failed")]
    RequestFailed,
}

impl HttpError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            HttpError::HttpBuilderGenFailed => 6100,
            HttpError::NonSuccessStatus(_) => 6101,
            HttpError::GetPhysicalIfaceError => 6102,
            HttpError::TimedOut => 6103,
            HttpError::RequestFailed => 6104,
        }
    }
}
