#![feature(let_chains)]
#![feature(try_trait_v2)]
pub mod database;
pub mod error;
pub mod i18n;
pub mod rpc;
pub mod service;
pub mod sys;
pub mod utils;
rust_i18n::i18n!("locales");

use error::{
    api::{
        account_update_token::AccountUpdateTokenError, init_database::InitDatabaseError,
        login::LoginError, logout::LogoutError, node::NodeError, ping::PingError,
        splashscreen::SplashscreenError, upload_log::UploadError,
    },
    bad_request::BadRequest,
    common::{
        database::DatabaseError,
        http::HttpError,
        io::IOError,
        jwt::JwtError,
        language::LanguageError,
        middleware::{DecryptError, MiddlewareError},
        param::ParamError,
        parse::ParseError,
        storage::StorageError,
        system_tray::SystemTrayError,
    },
    Error,
};

pub static RESOURCE_PATH: once_cell::sync::Lazy<once_cell::sync::OnceCell<String>> =
    once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);
pub static SYSTEM_TRAY_HANDLE: once_cell::sync::Lazy<
    once_cell::sync::OnceCell<tauri::SystemTrayHandle>,
> = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

pub fn version() -> std::collections::HashMap<String, String> {
    let mut version: std::collections::HashMap<_, _> = rf_node_client_desktop::version()
        .into_iter()
        .map(|(k, v)| (format!("rf-cli-desktop:{k}"), v))
        .collect();
    version.insert(
        "rf-cli-desktop".to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
    );
    version
}

#[cfg(test)]
mod test {
    #[test]
    fn version() {
        println!("{:#?}", crate::version());
    }
}
