#[derive(Debug, thiserror::Error)]
pub enum IOError {
    #[error("Create directory failed")]
    CreateDirAllFailed,
}

impl IOError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            IOError::CreateDirAllFailed => 6400,
        }
    }
}
