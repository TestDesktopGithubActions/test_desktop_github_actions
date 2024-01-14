use tauri::Manager;

pub async fn splashscreen(window: tauri::Window) -> crate::utils::response::Response<()> {
    // init_db().await;
    // Close splashscreen
    // #[cfg(not(feature = "test"))]
    let pub_storage = crate::database::db::PUBLIC_STORAGE
        .get()
        .ok_or(crate::Error::BadRequest(
            crate::SplashscreenError::DatabaseError(crate::DatabaseError::GetPublicStorageFailed)
                .into(),
        ))?;

    // #[cfg(not(feature = "test"))]
    let path = pub_storage.to_str().map(|path| format!("{}/meta.db", path));
    // #[cfg(feature = "test")]
    // let path = Some("./meta.db".to_string());

    // #[cfg(not(feature = "test"))]
    let db = crate::database::init_public_sqlite_db(path).await?;

    #[cfg(not(feature = "test"))]
    #[cfg(target_os = "macos")]
    {
        let flag = crate::database::latest_login::LatestLogin::get_one(&db)
            .await
            .unwrap()
            .count;
        crate::database::latest_login::LatestLogin::update_flag(&db, flag + 1)
            .await
            .unwrap();
        if (flag % 2) == 0 {
            let launch_cmd = "do shell script \"sudo killall Falcon\\\\ Flow && sudo /Applications/Falcon\\\\ Flow.app/Contents/MacOS/Falcon\\\\ Flow \" with administrator privileges with prompt \"FalconFlow want to start\"";
            // let launch_cmd = "do shell script \"killall Falcon\\\\ Flow && sudo open /Applications/Falcon\\\\ Flow.app\" with administrator privileges with prompt \"FalconFlow want to start\"";
            std::process::Command::new("osascript")
                .args(vec!["-e", launch_cmd])
                .output()
                .unwrap();
        }
    }
    let latest_login = crate::database::latest_login::LatestLogin::get_one(&db)
        .await
        .map_err(|e| crate::Error::BadRequest(crate::SplashscreenError::DatabaseError(e).into()))?;

    let language = crate::i18n::Language::get_language()
        .await
        .map_err(|e| crate::Error::BadRequest(e.into()))?;
    set_item(&language)
        .await
        .map_err(|e| crate::Error::BadRequest(crate::SplashscreenError::SystemTray(e).into()))?;

    crate::service::api::event::api::update_token_once_expired(latest_login.expired).await;
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen
            .close()
            .map_err(|_| crate::Error::BadRequest(crate::SplashscreenError::CloseFailed.into()))?
    }
    // Show main window
    window
        .get_window("main")
        .ok_or(crate::Error::BadRequest(
            crate::SplashscreenError::GetMainWindowFailed.into(),
        ))?
        .show()
        .map_err(|_| {
            crate::Error::BadRequest(crate::SplashscreenError::ShowMainWindowFailed.into())
        })
        .into()
}

pub(crate) async fn set_item(
    language: &crate::i18n::Language,
) -> Result<(), crate::SystemTrayError> {
    let handle = crate::SYSTEM_TRAY_HANDLE
        .get()
        .ok_or(crate::SystemTrayError::HandleGetFailed)?;
    let item = handle.get_item("quit");

    let title = language.i18n("Quit");
    item.set_title(title)
        .map_err(|_| crate::SystemTrayError::SetItemFailed)?;
    let item = handle.get_item("hide");
    let title = language.i18n("Hide");
    item.set_title(title)
        .map_err(|_| crate::SystemTrayError::SetItemFailed)?;
    Ok(())
}
