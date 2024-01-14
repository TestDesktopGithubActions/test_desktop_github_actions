pub mod api {

    use crate::utils::{http::HttpParams, response::Response};

    // const API: &str = "http://127.0.0.1:17776/";

    // async fn get_token() -> String {
    //     let data = pd::get_info().await;
    //     if data.is_none() {
    //         return "".to_string();
    //     } else {
    //         data.unwrap().token
    //     }
    // }

    pub(crate) fn get_pk() -> String {
        crate::sys::system_info::SystemInfo::get_sys().to_sha3()
    }

    pub(crate) async fn http<'a>() -> &'a Result<HttpParams, crate::Error> {
        crate::utils::http::http_params_generator()
    }

    pub async fn q() -> Response<String> {
        let http = http().await.as_ref()?;
        http.get_q().await.into()
    }

    #[cfg(target_os = "macos")]
    pub async fn get_mac_address() -> Result<String, crate::Error> {
        // return Err(crate::HttpError::GetPhysicalIfaceError.into());
        let iface = rf_node_client_desktop::get_physical_iface()
            .await
            .map_err(|_| crate::HttpError::GetPhysicalIfaceError)?;
        let iter = mac_address::MacAddressIterator::new().unwrap();

        for mac in iter {
            let name = mac_address::name_by_mac_address(&mac).ok().flatten();
            let name = name.as_ref();
            if name == Some(&iface) {
                return Ok(mac.to_string());
            }
            // println!("name: {name}, mac: {}", mac.to_string());
        }

        Err(crate::BadRequest::Login(crate::LoginError::GetMacAddressFailed).into())
    }

    #[cfg(target_os = "windows")]
    pub async fn get_mac_address() -> Result<String, crate::Error> {
        let name = mac_address::get_mac_address().ok().flatten();
        name.map(|name| name.to_string())
            .ok_or(crate::BadRequest::Login(crate::LoginError::GetMacAddressFailed).into())
    }

    // #[cfg(target_os = "windows")]
    // pub async fn get_mac_address() -> Result<String, crate::Error> {
    //     let iface = "WLAN".to_string();
    //     let iter = mac_address::MacAddressIterator::new().unwrap();
    //     for mac in iter {
    //         let name = mac_address::name_by_mac_address(&mac).ok().flatten();
    //         let name = name.as_ref();
    //         if name == Some(&iface) {
    //             return Ok(mac.to_string());
    //         }
    //     }

    //     Err(crate::Error::LoginError(
    //         crate::LoginError::GetMacAddressFailed,
    //     ))
    // }

    pub async fn login(email: &str, passwd: &str) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::login(email, passwd, http)
            .await
            .i18n(&l)
    }

    pub async fn login_temporary(
        email: &str,
        passwd: &str,
        proof: &str,
    ) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::login_temporary(email, passwd, proof, http)
            .await
            .i18n(&l)
    }

    pub async fn update_token_once_expired(exp: i64) {
        let now = crate::utils::time::now_time();
        tracing::info!("[update_token_once_expired] now: {now}, exp: {exp}");
        if now < exp {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

            let mut ticker = crate::utils::time::TICKER.lock().await;
            if let Some(ticker) = ticker.as_ref() {
                let _ = ticker.send(crate::utils::time::Event::Close);
            }
            ticker.replace(tx);
            tokio::spawn(async move {
                crate::utils::time::tick(rx, exp - now).await;
            });
        }
    }

    pub async fn logout(token: String) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::logout(token, http)
            .await
            .i18n(&l)
    }

    pub async fn account_update_token(token: String) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::account_update_token(token, http)
            .await
            .i18n(&l)
    }

    pub async fn node_list(token: String) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::node_list(token, http)
            .await
            .i18n(&l)
    }

    pub async fn node_start(token: &str, guid: &str) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::node_start(token, guid, http)
            .await
            .i18n(&l)
    }

    pub async fn node_end(token: &str, guid: &str) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::node_end(token, guid, http)
            .await
            .i18n(&l)
    }

    pub async fn upload_log(token: &str, email: &str) -> Response<serde_json::Value> {
        let http = http().await.as_ref()?;
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::upload_log(token, email, http)
            .await
            .i18n(&l)
    }

    pub async fn ping(ips: Vec<String>) -> Response<Vec<serde_json::Value>> {
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::ping(ips).await.i18n(&l)
    }

    pub async fn set_language(language: &str) -> Response<()> {
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::service::api::service::set_language(language)
            .await
            .i18n(&l)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{db::DbConnection, latest_login::LatestLogin};

    async fn init_db() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
    }
    #[tokio::test]
    async fn login_test() {
        init_db().await;
        // let email = "liikingsu@gmail.com";
        let email = "gin@techecho.io";
        // let password = "f0e8b8d978427c05b98bfd9254e43885";
        let password = "1808f422fedc097f2485839e9db4f8c1";
        let res = super::api::login(email, password).await;
        println!("[login] res: {:?}", res);
    }

    #[tokio::test]
    async fn account_update_token_test() {
        init_db().await;

        // let email = "liikingsu@gmail.com";
        // let password = "f0e8b8d978427c05b98bfd9254e43885";
        // let _ = super::api::_login(email, password).await;

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = super::api::account_update_token(data.token).await;
        println!("[account_update_token] res: {:?}", res);
    }

    #[tokio::test]
    async fn node_start_test() {
        init_db().await;

        let email = "liikingsu@gmail.com";
        let password = "f0e8b8d978427c05b98bfd9254e43885";
        let proof = "80EE3D8E-AFC0-5C90-8D09-0C3F9BA6676E";
        let _ = super::api::login(email, password).await;
        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = super::api::node_start(&data.token, proof).await;
        println!("[account_update_token] res: {:?}", res);
    }

    #[tokio::test]
    async fn node_end_test() {
        init_db().await;

        let email = "liikingsu@gmail.com";
        let password = "f0e8b8d978427c05b98bfd9254e43885";
        let proof = "80EE3D8E-AFC0-5C90-8D09-0C3F9BA6676E";
        let _ = super::api::login(email, password).await;

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = super::api::node_end(&data.token, proof).await;
        println!("[account_update_token] res: {:?}", res);
    }
    #[test]
    fn get_lang() {
        use std::env;
        if let Some(lang) = env::var("LANG").ok() {
            println!("当前设备的语言是: {}", lang);
        } else {
            println!("未能获取到语言信息");
        }
    }

    #[test]
    fn get_mac_address() {
        let iter = mac_address::MacAddressIterator::new().unwrap();
        for mac in iter {
            let name = mac_address::name_by_mac_address(&mac).unwrap().unwrap();
            println!("name: {name}, mac: {}", mac.to_string());
        }
    }
}
