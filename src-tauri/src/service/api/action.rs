// use std::sync::Mutex;
// use std::sync::Arc;

use crate::service::api::event::api;

// use crate::pd::db::{
//     DbConn
// };

// use tauri::State;

/**
 * 登录
 */
#[tauri::command]
pub async fn login(email: String, passwd: String) -> String {
    api::login(&email, &passwd).await.to_json()
}

/**
 * 临时登录
 */
#[tauri::command]
pub async fn login_temporary(email: String, passwd: String, proof: String) -> String {
    api::login_temporary(&email, &passwd, &proof)
        .await
        .to_json()
}

/**
 * 登出
 */
#[tauri::command]
pub async fn logout(token: String) -> String {
    api::logout(token).await.to_json()
}

/**
 * 注册
 */
#[tauri::command]
pub async fn register(_email: String, _passwd: String, _repeat_password: String) -> String {
    "".to_string()
}

/**
 * 绑定设备
 */
#[tauri::command]
pub async fn bind_device() -> String {
    "".to_string()
}

/**
 * 激活
 */
#[tauri::command]
pub async fn activating(_account_code: String, _code: String) -> String {
    "".to_string()
}

/**
 * 更新token
 */
#[tauri::command]
pub async fn account_update_token(token: String) -> String {
    api::account_update_token(token).await.to_json()
}

/**
 * 获取节点列表
 */
#[tauri::command]
pub async fn node_list(token: String) -> String {
    api::node_list(token).await.to_json()
}

/**
 * 启动节点
 */
#[tauri::command]
pub async fn node_start(token: String, guid: String) -> String {
    api::node_start(&token, &guid).await.to_json()
}

/**
 * 停止节点
 */
#[tauri::command]
pub async fn node_end(token: String, guid: String) -> String {
    api::node_end(&token, &guid).await.to_json()
}

/**
 * 上传日志
 */
#[tauri::command]
pub async fn upload_log(token: String, email: String) -> String {
    api::upload_log(&token, &email).await.to_json()
}

/**
 * ping
 */
#[tauri::command]
pub async fn ping(ips: Vec<String>) -> String {
    api::ping(ips).await.to_json()
}

/**
 * 获取用户信息
 */
#[tauri::command]
pub async fn get_info() -> String {
    // api::get_info().await.to_json()
    "".to_string()
}

/**
 * set language
 */
#[tauri::command]
pub async fn set_language(language: String) -> String {
    api::set_language(&language).await.to_json()
}

/**
 * get pk
 */
#[tauri::command]
pub async fn get_pk() -> String {
    api::get_pk()
}

#[cfg(test)]
mod tests {
    use crate::database::{db::DbConnection, latest_login::LatestLogin};

    #[tokio::test]
    async fn login_test() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
        // let email = "liikingsu@gmail.com";
        let email = "gin@techecho.io";
        // let password = "f0e8b8d978427c05b98bfd9254e43885";
        let password = "1808f422fedc097f2485839e9db4f8c1";
        let _proof = "80EE3D8E-AFC0-5C90-8D09-0C3F9BA6676E";
        let res = super::api::login(email, password).await;
        println!("[login] res: {:?}", res);
    }

    #[tokio::test]
    async fn logout_test() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
        // let email = "liikingsu@gmail.com";
        let _email = "gin@techecho.io";
        // let password = "f0e8b8d978427c05b98bfd9254e43885";
        let _password = "1808f422fedc097f2485839e9db4f8c1";
        let _proof = "80EE3D8E-AFC0-5C90-8D09-0C3F9BA6676E";
        // let res = super::api::_login(email, password).await;

        // let info = super::api::get_info().await.unwrap();
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiMzYxYmRiNmFmMWEwYzJlYmE4OTgxM2U1M2UwMGU0NyIsImtleSI6WzIwMSwxMDMsMTgyLDE0MSwxMzEsNzQsNjgsMjQsMzYsNzUsMTIsMTA0LDE4NiwxOTQsMTU3LDQ0LDk0LDE2NCw1NCwxOTEsMTY5LDksMjU1LDE2MSwzNSw1OCw0MSwxMzAsMTAyLDEwNSwxOTgsMTddLCJpYXQiOjE2OTU0NzYyNDIsImV4cCI6MTY5NTQ5NDI0Mn0.CBEB0JrBhvPoaT1ilqaGI94Mydr51EZzd0O7cyoVxqI";

        let res = crate::service::api::action::logout(token.to_string()).await;
        println!("[logout] res: {:?}", res);
    }

    #[tokio::test]
    async fn account_update_token_test() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
        let email = "gin@techecho.io";
        let password = "1808f422fedc097f2485839e9db4f8c1";
        let _ = super::api::login(email, password).await;

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = crate::service::api::action::account_update_token(data.token).await;
        println!("[account_update_token] res: {:?}", res);
    }

    #[tokio::test]
    async fn node_start_test() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;

        let email = "gin@techecho.io";
        // let password = "f0e8b8d978427c05b98bfd9254e43885";
        let password = "1808f422fedc097f2485839e9db4f8c1";
        let _ = super::api::login(email, password).await;
        let proof = "299B9466-E358-4604-8BAF-4B06D2166C7E";

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = crate::service::api::action::node_start(data.token, proof.to_string()).await;
        println!("[node_start] res: {:?}", res);
    }

    #[tokio::test]
    async fn node_end_test() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
        let email = "gin@techecho.io";
        // let password = "f0e8b8d978427c05b98bfd9254e43885";
        let password = "1808f422fedc097f2485839e9db4f8c1";
        let proof = "299B9466-E358-4604-8BAF-4B06D2166C7E";
        let _ = super::api::login(email, password).await;

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res =
            crate::service::api::action::node_start(data.token.clone(), proof.to_string()).await;
        println!("[node_start] res: {:?}", res);

        // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let res = crate::service::api::action::node_end(data.token, proof.to_string()).await;
        println!("[node_end] res: {:?}", res);
    }

    #[tokio::test]
    async fn node_list_test() {
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
        let email = "liikingsu@gmail.com";
        let password = "f0e8b8d978427c05b98bfd9254e43885";
        let _proof = "80EE3D8E-AFC0-5C90-8D09-0C3F9BA6676E";
        let _ = super::api::login(email, password).await;

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = crate::service::api::action::node_list(data.token).await;
        println!("[node_list] res: {:?}", res);
    }

    #[tokio::test]
    async fn upload_log_test() {
        // tracing_subscriber::fmt()
        //     .pretty()
        //     .with_max_level(tracing::Level::INFO)
        //     .with_writer(std::io::stdout)
        //     .init();
        let _ = crate::database::db::DbConnection::default()
            .pub_migrator()
            .set_uri(None)
            .init_database()
            .await;
        let email = "gin@techecho.io";
        let password = "1808f422fedc097f2485839e9db4f8c1";
        let _proof = "80EE3D8E-AFC0-5C90-8D09-0C3F9BA6676E";

        let user = "/Users/gin/Library/Application Support/com.tauri.falconflow/storage/users";
        let path = std::path::PathBuf::from(user);
        crate::database::db::USER_STORAGE.set(path.clone()).unwrap();
        let _ = super::api::login(email, password).await;

        let conn = DbConnection::get_pub_connection().unwrap();
        let conn = conn.get_pool().unwrap();
        let data = LatestLogin::get_one(conn).await.unwrap();
        let res = crate::service::api::action::upload_log(data.token, email.to_string()).await;
        println!("[node_list] res: {:?}", res);
    }

    #[tokio::test]
    async fn ping_test() {
        let ips = vec!["52.68.77.185".to_string(), "13.125.238.30".to_string()];
        let res = crate::service::api::action::ping(ips).await;

        println!("[ping] res: {res}");
    }

    #[tokio::test]
    async fn set_language_test() {
        let lang = "de".to_string();
        let res = crate::service::api::action::set_language(lang).await;

        println!("[set_language_test] res: {res}");
    }
}
