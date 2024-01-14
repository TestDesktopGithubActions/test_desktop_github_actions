#[derive(Debug, thiserror::Error)]
pub enum SystemTrayError {
    #[error("Update system tray icon failed")]
    UpdateIconFailed,
    #[error("System tray handle get failed")]
    HandleGetFailed,
    #[error("Set item failed")]
    SetItemFailed,
}

impl SystemTrayError {
    pub(crate) fn get_status_code(&self) -> u32 {
        match self {
            SystemTrayError::UpdateIconFailed => 4050,
            SystemTrayError::HandleGetFailed => 4051,
            SystemTrayError::SetItemFailed => 4052,
        }
    }
}
