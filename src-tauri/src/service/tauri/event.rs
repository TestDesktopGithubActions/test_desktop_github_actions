pub async fn splashscreen(window: tauri::Window) -> crate::utils::response::Response<()> {
    let res = crate::service::tauri::service::splashscreen(window).await;
    let l = crate::utils::http::HttpParams::get_lang().await;

    res.i18n(&l)
}
