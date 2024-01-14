#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Do not access illegally")]
    IllegalAccess(#[from] jsonwebtoken::errors::Error),
    #[error("token time expires")]
    TokenExpires,
}

impl JwtError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            JwtError::IllegalAccess(_) => 2000,
            JwtError::TokenExpires => 2001,
        }
    }
}
