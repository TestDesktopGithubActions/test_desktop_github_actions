#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Parse vector to string failed")]
    VecToStringFailed,
    #[error("Parse addr failed")]
    Addr,
    #[error("Parse value to vector failed")]
    ValueToVecFailed,
    #[error("Parse vector to array failed")]
    VecToArrayFailed,
    #[error("Serialize json error")]
    JsonSerialize,
    #[error("Deserialize json error")]
    JsonDeserialize,
}

impl ParseError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            ParseError::VecToStringFailed => 6200,
            ParseError::Addr => 6202,
            ParseError::ValueToVecFailed => 6203,
            ParseError::VecToArrayFailed => 6204,
            ParseError::JsonSerialize => 6205,
            ParseError::JsonDeserialize => 6206,
        }
    }
}
