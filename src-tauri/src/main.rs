#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(try_trait_v2)]
#![feature(let_chains)]
use ram_flux::{
    __cmd__splashscreen,
    // pd,
    service::api::action,
    service::tauri::action::splashscreen,
};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting ...");
    // let _ = fix_path_env::fix();
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .system_tray(ram_flux::service::tauri::action::get_menu())
        .on_system_tray_event(|app, event| {
            ram_flux::service::tauri::action::event_handler(app, event)
        })
        .setup(|app| {
            // save system tray handle to update icon
            ram_flux::SYSTEM_TRAY_HANDLE.set(app.tray_handle()).unwrap();
            // don't show on the taskbar/springboard
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            ram_flux::service::tauri::init_resource(app);

            let public_storage = match ram_flux::database::init_storage(app) {
                Ok(storage) => storage,
                Err(e) => {
                    return Err(Box::new(e));
                }
            };

            if let Err(e) =
                ram_flux::service::node::init::init_log(public_storage.to_str().unwrap())
            {
                return Err(Box::new(e));
            };

            let window = app.get_window("main").unwrap();
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");

            ram_flux::service::node::notify::notify_tx_generator(window);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            action::login,
            action::login_temporary,
            action::logout,
            action::register,
            action::bind_device,
            action::activating,
            action::account_update_token,
            action::node_list,
            action::node_start,
            action::node_end,
            action::get_info,
            action::upload_log,
            action::ping,
            action::set_language,
            splashscreen
        ])
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, e| match e {
            tauri::RunEvent::Updater(event) => {
                tracing::info!("event: {event:?}");
                dbg!(event);
            }
            _ => (),
        });
    Ok(())
}
