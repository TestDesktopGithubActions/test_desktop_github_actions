#[cfg(target_os = "linux")]
const OS_TYPE: &str = "linux";
#[cfg(target_os = "windows")]
const OS_TYPE: &str = "windows";
#[cfg(target_os = "macos")]
const OS_TYPE: &str = "macos";
#[cfg(target_os = "android")]
const OS_TYPE: &str = "android";
#[cfg(target_os = "ios")]
const OS_TYPE: &str = "ios";

pub fn get_os_type() -> String {
    OS_TYPE.to_string()
}
