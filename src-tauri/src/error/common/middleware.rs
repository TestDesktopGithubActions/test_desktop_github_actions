#[derive(Debug, thiserror::Error)]
pub enum MiddlewareError {
    #[error("Decode error")]
    Decode,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Invalid token, without key")]
    WithoutKey,
    #[error("Invalid key")]
    InvalidKey,

    #[error("Parse error")]
    Parse(#[from] super::parse::ParseError),
    #[error("Auth error: {0}")]
    Auth(#[from] AuthError),
    #[error("Encrypt error: {0}")]
    Encrypt(#[from] EncryptError),
    #[error("Decrypt error: {0}")]
    Decrypt(#[from] DecryptError),
    #[error("International error: {0}")]
    I18n(#[from] I18nError),
    #[error("Signature error: {0}")]
    Signature(#[from] SignatureError),
}

impl MiddlewareError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            MiddlewareError::Decode => 5000,
            MiddlewareError::InvalidToken => 5001,
            MiddlewareError::WithoutKey => 5002,
            MiddlewareError::InvalidKey => 5003,
            MiddlewareError::Parse(msg) => msg.get_status_code(),
            MiddlewareError::Auth(msg) => msg.get_status_code(),
            MiddlewareError::Encrypt(msg) => msg.get_status_code(),
            MiddlewareError::Decrypt(msg) => msg.get_status_code(),
            MiddlewareError::I18n(msg) => msg.get_status_code(),
            MiddlewareError::Signature(msg) => msg.get_status_code(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Parse error: {0}")]
    Parse(#[from] super::parse::ParseError),
}

impl AuthError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            AuthError::Parse(msg) => msg.get_status_code(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EncryptError {
    #[error("Parse error: {0}")]
    Parse(#[from] super::parse::ParseError),
}

impl EncryptError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            EncryptError::Parse(msg) => msg.get_status_code(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecryptError {
    #[error("Parse error: {0}")]
    Parse(#[from] super::parse::ParseError),
}

impl DecryptError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            DecryptError::Parse(msg) => msg.get_status_code(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum I18nError {
    #[error("Parse error: {0}")]
    Parse(#[from] super::parse::ParseError),
}

impl I18nError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            I18nError::Parse(msg) => msg.get_status_code(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    #[error("Missing signature")]
    Missing,
    #[error("Invalid signature format")]
    Invalid,
    #[error("Invalid nonce")]
    InvalidNonce,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Incorrect signature")]
    Incorrect,
}

impl SignatureError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            SignatureError::Missing => 1130,
            SignatureError::Invalid => 1131,
            SignatureError::InvalidNonce => 1132,
            SignatureError::InvalidTimestamp => 1133,
            SignatureError::Incorrect => 1136,
        }
    }
}
