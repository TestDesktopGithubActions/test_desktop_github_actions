use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

/**
 * Create system tray menu
 */
pub fn get_menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    #[cfg(target_os = "windows")]
    let tray_builder = SystemTray::new().with_menu(tray_menu);

    #[cfg(target_os = "macos")]
    let tray_builder = SystemTray::new()
        .with_menu(tray_menu)
        .with_menu_on_left_click(false);
    tray_builder
}

/**
 * Handle system tray events
 */
pub fn event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
            let window = app.get_window("main").unwrap();
            if window.is_minimized().unwrap() {
                println!("is minimized");
                window.unminimize().unwrap();
            }
            if window.is_visible().unwrap() {
                println!("is invisible");
                window.show().unwrap();
            }
            window.set_focus().unwrap();
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

pub(crate) fn update_system_tray_icon(node_start: bool) -> Result<(), crate::Error> {
    let handle = crate::SYSTEM_TRAY_HANDLE
        .get()
        .ok_or(crate::Error::BadRequest(
            crate::SystemTrayError::HandleGetFailed.into(),
        ))?;

    let icon = if node_start {
        include_bytes!("../../../icons/icon_light.png").to_vec()
    } else {
        include_bytes!("../../../icons/icon_dark.png").to_vec()
    };
    handle
        .set_icon(tauri::Icon::Raw(icon))
        .map_err(|_| crate::Error::BadRequest(crate::SystemTrayError::UpdateIconFailed.into()))
}

/**
 * Show splashscreen
 */
#[tauri::command]
pub async fn splashscreen(window: tauri::Window) -> String {
    super::event::splashscreen(window).await.to_json()
    // tokio::spawn(async {
    //     tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    //     let command = crate::service::node::command::command_tx_generator();
    //     let _ = command.send(crate::service::node::command::Event::Disconnected(
    //         "test".to_string(),
    //     ));
    // });

    // res
}
