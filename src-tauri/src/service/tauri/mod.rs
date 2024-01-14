pub mod action;
pub mod event;
pub mod service;

pub fn init_resource(app: &tauri::App) {
    crate::RESOURCE_PATH.get_or_init(|| {
        let resource_path = app
            .path_resolver()
            .resolve_resource("wintun.dll")
            .expect("failed to resolve resource")
            .to_string_lossy()
            .to_string();
        tracing::error!("set resource path: {}", resource_path);
        resource_path
    });
}
