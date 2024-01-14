#[derive(Debug, thiserror::Error)]
pub enum ParamError {
    #[error("Token is empty")]
    TokenMissing,
    #[error("Email is empty")]
    EmailMissing,
    #[error("Passwd is empty")]
    PasswdMissing,
    #[error("Sorry, please enter the correct email")]
    EmailInvalid,
    #[error("No valid IPs to ping")]
    IpsEmpty,
}

impl ParamError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            ParamError::TokenMissing => 6300,
            ParamError::EmailMissing => 6301,
            ParamError::PasswdMissing => 6302,
            ParamError::EmailInvalid => 6303,
            ParamError::IpsEmpty => 6304,
        }
    }
}
