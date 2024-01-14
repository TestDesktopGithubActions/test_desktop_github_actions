#[derive(Debug, serde::Serialize)]
pub struct LoginReq {
    email: String,
    password: String,
    proof: String,
}

impl LoginReq {
    pub fn new(email: &str, passwd: &str, proof: &str) -> Self {
        Self {
            email: email.to_string(),
            password: passwd.to_string(),
            proof: proof.to_string(),
        }
    }

    pub(crate) fn check(&self) -> Result<(), crate::Error> {
        if self.email.is_empty() {
            return Err(crate::Error::BadRequest(
                crate::LoginError::Param(crate::ParamError::EmailMissing).into(),
            ));
        }

        if self.password.is_empty() {
            return Err(crate::Error::BadRequest(
                crate::LoginError::Param(crate::ParamError::PasswdMissing).into(),
            ));
        }

        if !validator::validate_email(&self.email) {
            return Err(crate::Error::BadRequest(
                crate::LoginError::Param(crate::ParamError::EmailInvalid).into(),
            ));
        }
        Ok(())
    }

    async fn serde(&self) -> Result<String, crate::Error> {
        serde_json::to_string(self)
            .map_err(|_| crate::Error::Parse(crate::ParseError::JsonSerialize))
    }

    async fn get_key_and_signature(&self) -> Result<([u8; 32], String), crate::Error> {
        let body = self.serde().await?;
        crate::utils::signature::Signature::default_key_and_signature(&body)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Middleware(e).into()))
    }

    pub(crate) async fn login(
        &self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let (key, signature) = self.get_key_and_signature().await?;
        let url = http.uri("login?d=login".to_string()).await;
        tracing::info!("[login] url: {url}");
        let text = http
            .send_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::json!(self)),
                None,
                Some(vec![("signature", &signature)]),
            )
            .await?;

        let res: crate::utils::response::Response<serde_json::Value> = (key, text.as_str())
            .try_into()
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Middleware(e).into()))?;

        if let Some(ref result) = res.result {
            let user = LoginReq::update_db(self, result).await?;
            // update token when expired
            crate::service::api::event::api::update_token_once_expired(user.exp).await
        }
        tracing::info!("[login] res: {res:?}");
        res
    }

    pub(crate) async fn login_temporary(
        &self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let (key, signature) = self.get_key_and_signature().await?;
        let url = http
            .uri_web("login/temporary?d=temporary".to_string(), &self.proof)
            .await;
        tracing::info!("[login_temporary] url: {url}");
        let text = http
            .send_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::json!(self)),
                None,
                Some(vec![("signature", &signature)]),
            )
            .await?;

        let res: crate::utils::response::Response<serde_json::Value> = (key, text.as_str())
            .try_into()
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Middleware(e).into()))?;
        tracing::info!("[login_temporary] res: {res:?}");
        res
    }

    async fn get_connection(&self) -> Result<crate::database::db::DbConnection, crate::Error> {
        let storage = crate::database::db::USER_STORAGE
            .get()
            .ok_or(crate::BadRequest::Login(crate::LoginError::Database(
                crate::DatabaseError::GetUserStorageFailed,
            )))?;
        let user_storage = storage.join(&self.email);
        std::fs::create_dir_all(&user_storage).map_err(|_| {
            crate::Error::BadRequest(
                crate::LoginError::IO(crate::IOError::CreateDirAllFailed).into(),
            )
        })?;
        let user_storage = user_storage.to_str();
        let user_storage = user_storage.map(|path| format!("{path}/user.db"));

        // #[cfg(feature = "test")]
        // let user_storage = Some("./user.db".to_string());

        Ok(crate::database::db::DbConnection::default()
            .pri_migrator()
            .set_uri(user_storage)
            .init_database()
            .await
            .map_err(|e| {
                tracing::info!("[login] init_database error: {e:?}");
                crate::BadRequest::Login(crate::DatabaseError::DatabaseCreateFailed.into())
            })?)
    }

    async fn update_db(
        &self,
        result: &serde_json::Value,
    ) -> Result<crate::service::api::User, crate::Error> {
        let pri_conn = self.get_connection().await?;
        let pri_pool = pri_conn.get_pool().ok_or(crate::BadRequest::Login(
            crate::DatabaseError::GetUserSqlitePoolFailed.into(),
        ))?;
        let public_conn =
            crate::database::db::PUBLIC_SQLITE_POOL
                .get()
                .ok_or(crate::BadRequest::Login(crate::LoginError::Database(
                    crate::DatabaseError::GetPublicSqliteConnFailed,
                )))?;
        let public_pool = public_conn.get_pool().ok_or(crate::BadRequest::Login(
            crate::DatabaseError::GetPublicSqlitePoolFailed.into(),
        ))?;
        let user =
            serde_json::from_value::<crate::service::api::User>(result.clone()).map_err(|_| {
                crate::Error::BadRequest(
                    crate::LoginError::Parse(crate::ParseError::JsonDeserialize).into(),
                )
            })?;

        // update latest login with this user
        self.update_latest_login(public_pool, &user).await?;
        // insert user info
        self.insert_user_info(pri_pool, &user).await?;
        // update user sqlite pool
        let mut p = crate::database::db::USER_SQLITE_POOL.write().await;
        p.replace(pri_conn);
        Ok(user)
    }

    async fn update_latest_login(
        &self,
        conn: &sqlx::Pool<sqlx::Sqlite>,
        user: &crate::service::api::User,
    ) -> Result<(), crate::Error> {
        let l = crate::utils::http::HttpParams::get_lang().await;
        let latest_login = crate::database::latest_login::LatestLogin::new(
            &self.email,
            &user.token,
            user.exp,
            l.to_string(),
        );

        crate::database::latest_login::LatestLogin::del_one(conn)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Database(e).into()))?;

        if latest_login
            .insert_latest_login(conn)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Database(e).into()))?
            .rows_affected()
            == 0
        {
            return Err(crate::Error::BadRequest(
                crate::LoginError::Database(crate::DatabaseError::UpdateFailed).into(),
            ));
        };

        Ok(())
    }

    async fn insert_user_info(
        &self,
        conn: &sqlx::Pool<sqlx::Sqlite>,
        user: &crate::service::api::User,
    ) -> Result<(), crate::Error> {
        if crate::database::user::UserInfo::new(&self.email, &user.token)
            .insert_user(conn)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Database(e).into()))?
            .rows_affected()
            == 0
        {
            return Err(crate::Error::BadRequest(
                crate::LoginError::Database(crate::DatabaseError::InsertFailed).into(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct LogoutReq {
    token: String,
}

impl LogoutReq {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    async fn get_key_and_signature(
        &self,
        body: &str,
    ) -> Result<([u8; 32], [u8; 12], String), crate::Error> {
        crate::utils::signature::Signature::key_and_signature(&self.token, body)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::LogoutError::Middleware(e).into()))
    }

    pub(crate) async fn logout(
        self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let path_query = http.path_query("account/logout?d=logout").await;
        let (key, _, signature) = self.get_key_and_signature(&path_query).await?;
        let url = http.uri("account/logout?d=logout".to_string()).await;
        let text = http
            .send_request(
                reqwest::Method::GET,
                &url,
                None,
                None,
                Some(vec![
                    ("signature", &signature),
                    ("authorization", &self.token),
                ]),
            )
            .await?;

        let res: crate::utils::response::Response<serde_json::Value> = (key, text.as_str())
            .try_into()
            .map_err(|e| crate::Error::BadRequest(crate::LogoutError::Middleware(e).into()))?;
        if res.result.is_some() {
            // close ticker
            let ticker = crate::utils::time::TICKER.lock().await;
            if let Some(ticker) = ticker.as_ref() {
                let _ = ticker.send(crate::utils::time::Event::Close);
            }
        }
        res
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AccountUpdateTokenReq {
    token: String,
}

impl AccountUpdateTokenReq {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    async fn get_key_and_signature(
        &self,
        body: &str,
    ) -> Result<([u8; 32], [u8; 12], String), crate::Error> {
        crate::utils::signature::Signature::key_and_signature(&self.token, body)
            .await
            .map_err(|e| {
                crate::Error::BadRequest(crate::AccountUpdateTokenError::Middleware(e).into())
            })
    }

    pub(crate) async fn account_update_token(
        self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let path_query = http
            .path_query("account/update/token?d=/account/update/token")
            .await;
        let (key, _, signature) = self.get_key_and_signature(&path_query).await?;
        let url = http
            .uri("account/update/token?d=/account/update/token".to_string())
            .await;
        let text = http
            .send_request(
                reqwest::Method::GET,
                &url,
                None,
                None,
                Some(vec![
                    ("signature", &signature),
                    ("authorization", &self.token),
                ]),
            )
            .await?;
        let res: crate::utils::response::Response<serde_json::Value> =
            (key, text.as_str()).try_into().map_err(|e| {
                crate::Error::BadRequest(crate::AccountUpdateTokenError::Middleware(e).into())
            })?;

        if let Some(ref result) = res.result {
            self.update_db(result).await?;
        }
        res
    }

    async fn update_db(self, result: &serde_json::Value) -> Result<(), crate::Error> {
        let user: crate::service::api::User = result.try_into().map_err(|e| {
            crate::BadRequest::AccountUpdateToken(crate::AccountUpdateTokenError::Parse(e))
        })?;
        let public_conn = crate::database::db::DbConnection::get_pub_connection().map_err(|e| {
            crate::Error::BadRequest(crate::BadRequest::AccountUpdateToken(
                crate::AccountUpdateTokenError::Database(e),
            ))
        })?;
        let public_pool = public_conn.get_pool().ok_or(crate::Error::BadRequest(
            crate::AccountUpdateTokenError::Database(
                crate::DatabaseError::GetPublicSqlitePoolFailed,
            )
            .into(),
        ))?;
        let pri_conn = crate::database::db::USER_SQLITE_POOL.read().await;
        let pri_conn = pri_conn.as_ref().ok_or(crate::Error::BadRequest(
            crate::BadRequest::AccountUpdateToken(
                crate::DatabaseError::GetUserSqliteConnFailed.into(),
            ),
        ))?;
        let pri_pool = pri_conn.get_pool().ok_or(crate::Error::BadRequest(
            crate::BadRequest::AccountUpdateToken(
                crate::DatabaseError::GetUserSqlitePoolFailed.into(),
            ),
        ))?;

        // update latest login with this user
        self.update_latest_login(public_pool, &user).await?;
        // insert user info
        self.insert_user_info(pri_pool, &user).await?;

        Ok(())
    }

    async fn update_latest_login(
        &self,
        conn: &sqlx::Pool<sqlx::Sqlite>,
        user: &crate::service::api::User,
    ) -> Result<(), crate::Error> {
        let l = crate::utils::http::HttpParams::get_lang().await;
        crate::database::latest_login::LatestLogin::del_one(conn)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::LoginError::Database(e).into()))?;
        if crate::database::latest_login::LatestLogin::new(
            &user.email,
            &user.token,
            user.exp,
            l.to_string(),
        )
        .insert_latest_login(conn)
        .await
        .map_err(|_| {
            crate::Error::BadRequest(
                crate::AccountUpdateTokenError::Database(crate::DatabaseError::UpdateFailed).into(),
            )
        })?
        .rows_affected()
            == 0
        {
            return Err(crate::Error::BadRequest(
                crate::AccountUpdateTokenError::Database(crate::DatabaseError::UpdateFailed).into(),
            ));
        };

        Ok(())
    }

    async fn insert_user_info(
        &self,
        conn: &sqlx::Pool<sqlx::Sqlite>,
        user: &crate::service::api::User,
    ) -> Result<(), crate::Error> {
        if crate::database::user::UserInfo::new(&user.email, &user.token)
            .insert_user(conn)
            .await
            .map_err(|e| {
                crate::Error::BadRequest(crate::AccountUpdateTokenError::Database(e).into())
            })?
            .rows_affected()
            == 0
        {
            return Err(crate::Error::BadRequest(
                crate::AccountUpdateTokenError::Database(crate::DatabaseError::InsertFailed).into(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct NodeListReq {
    token: String,
}

impl NodeListReq {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    async fn get_key_and_signature(
        &self,
        body: &str,
    ) -> Result<([u8; 32], [u8; 12], String), crate::Error> {
        crate::utils::signature::Signature::key_and_signature(&self.token, body)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::NodeError::Middleware(e).into()))
    }

    pub(crate) async fn list(
        self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let path_query = http.path_query("node?d=list").await;
        let (key, _, signature) = self.get_key_and_signature(&path_query).await?;
        let url = http.uri("node?d=list".to_string()).await;

        let text = http
            .send_request(
                reqwest::Method::GET,
                &url,
                None,
                None,
                Some(vec![
                    ("signature", &signature),
                    ("authorization", &self.token),
                ]),
            )
            .await?;
        let res: crate::utils::response::Response<serde_json::Value> = (key, text.as_str())
            .try_into()
            .map_err(|e| crate::Error::BadRequest(crate::NodeError::Middleware(e).into()))?;
        // tracing::info!("[node_list] list: {res:#?}");
        res
    }
}

#[derive(Debug, serde::Serialize)]
pub struct NodeStartReq {
    guid: String,
    token: String,
}

impl NodeStartReq {
    pub fn new(guid: &str, token: &str) -> Self {
        Self {
            guid: guid.to_string(),
            token: token.to_string(),
        }
    }

    async fn get_key_and_signature(
        &self,
        body: &str,
    ) -> Result<([u8; 32], [u8; 12], String), crate::Error> {
        crate::utils::signature::Signature::key_and_signature(&self.token, body)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::NodeError::Middleware(e).into()))
    }

    pub(crate) async fn node_start(
        self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let path_query = http
            .path_query(&format!("node/start?d=/node/start&guid={}", self.guid))
            .await;
        let (key, _, signature) = self.get_key_and_signature(&path_query).await?;
        let url = http
            .uri(format!("node/start?d=/node/start&guid={}", self.guid))
            .await;
        let text = http
            .send_request(
                reqwest::Method::GET,
                &url,
                None,
                None,
                Some(vec![
                    ("signature", &signature),
                    ("authorization", &self.token),
                ]),
            )
            .await?;
        let mut res: crate::utils::response::Response<serde_json::Value> = (key, text.as_str())
            .try_into()
            .map_err(|e| crate::Error::BadRequest(crate::NodeError::Middleware(e).into()))?;
        if let Some(result) = res.result {
            res = NodeStartReq::connect(result).await?;
            if res.code == 200 {
                crate::service::tauri::action::update_system_tray_icon(true)?;
                // let mut flag = crate::service::node::command::LINK_FLAG.write().unwrap();
                // *flag = true;
            } else {
                NodeEndReq::disconnect().await;
            }
        }
        tracing::info!("[node_start] res: {res:?}");
        res
    }

    async fn connect(
        result: serde_json::Value,
    ) -> Result<crate::utils::response::Response<serde_json::Value>, crate::Error> {
        let req: crate::service::node::action::StartReq =
            serde_json::from_value(result).map_err(|_| {
                crate::BadRequest::Node(crate::NodeError::Parse(crate::ParseError::JsonDeserialize))
            })?;
        let mut node = crate::utils::http::NODEINFO.lock().await;
        node.pk = req.client_pubkey.clone();
        node.node_port = req.node_port;
        let connect_res = crate::service::node::action::connect(req).await;
        serde_json::from_str(&connect_res).map_err(|_| {
            crate::BadRequest::Node(crate::NodeError::Parse(crate::ParseError::JsonDeserialize))
                .into()
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct NodeEndReq {
    guid: String,
    token: String,
}

impl NodeEndReq {
    pub fn new(guid: &str, token: &str) -> Self {
        Self {
            guid: guid.to_string(),
            token: token.to_string(),
        }
    }
    async fn get_key_and_signature(
        &self,
        body: &str,
    ) -> Result<([u8; 32], [u8; 12], String), crate::Error> {
        crate::utils::signature::Signature::key_and_signature(&self.token, body)
            .await
            .map_err(|e| crate::Error::BadRequest(crate::NodeError::Middleware(e).into()))
    }

    async fn disconnect() -> crate::utils::response::Response<serde_json::Value> {
        let disconnect_res = crate::service::node::action::disconnect().await;
        tracing::info!("[node_end] disconnect res: {:?}", disconnect_res);
        serde_json::from_str::<crate::utils::response::Response<serde_json::Value>>(&disconnect_res)
            .map_err(|_| {
                crate::Error::BadRequest(
                    crate::NodeError::Parse(crate::ParseError::JsonDeserialize).into(),
                )
            })?
    }

    pub(crate) async fn node_end(
        self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let disconnect_res = NodeEndReq::disconnect().await;
        if disconnect_res.code == 200 {
            // std::thread::spawn(|| {
            //     let mut flag = crate::service::node::command::LINK_FLAG.write().unwrap();
            //     *flag = false;
            // });
            crate::service::tauri::action::update_system_tray_icon(false)?;
            let node = crate::utils::http::NODEINFO.lock().await;
            let json = serde_json::json!({
                "pkey" : node.pk,
                "port" : node.node_port
            });
            let body = serde_json::json!(json).to_string();
            let (key, nonce, signature) = self.get_key_and_signature(&body).await?;
            let encrypt_body =
                crate::utils::crypto::EncryptParams::new(&body, nonce, key)?.encrypt();
            let url = http
                .uri(format!("node/end?d=/node/end&guid={}", self.guid))
                .await;

            let text = http
                .send_request(
                    reqwest::Method::POST,
                    &url,
                    Some(serde_json::json!({
                        "body": encrypt_body
                    })),
                    None,
                    Some(vec![
                        ("signature", &signature),
                        ("authorization", &self.token),
                    ]),
                )
                .await?;

            let mut res: crate::utils::response::Response<serde_json::Value> = (key, text.as_str())
                .try_into()
                .map_err(|e| crate::Error::BadRequest(crate::NodeError::Middleware(e).into()))?;
            if res.code == 200 {
                res = disconnect_res
            }
            println!("<<<<<<<<<<<<<<<<<<<<<  断开结果: {res:?} >>>>>>>>>>>>>>>>>");
            res
        } else {
            disconnect_res
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UploadLogReq {
    token: String,
    email: String,
    name: String,
    file_path: std::path::PathBuf,
}

impl UploadLogReq {
    pub fn new(token: &str, email: &str, name: &str, file_path: std::path::PathBuf) -> Self {
        Self {
            token: token.to_string(),
            email: email.to_string(),
            name: name.to_string(),
            file_path,
        }
    }

    async fn gen_form(&self) -> Result<reqwest::multipart::Form, crate::Error> {
        use tokio_util::codec::{BytesCodec, FramedRead};
        let file = tokio::fs::File::open(&self.file_path)
            .await
            .map_err(|_| crate::BadRequest::Upload(crate::UploadError::FileOpenFailed))?;
        // tracing::info!("[upload log] file_path: {file_path:?}");
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = reqwest::Body::wrap_stream(stream);

        let file = reqwest::multipart::Part::stream(file_body)
            .file_name(self.name.clone())
            .mime_str("text/plain")
            .map_err(crate::Error::Reqwest)?;

        Ok(reqwest::multipart::Form::new()
            .text("email", self.email.to_owned())
            .text("name", self.name.to_owned())
            .part("log", file))
    }

    pub(crate) async fn upload(
        &self,
        http: &crate::utils::http::HttpParams,
    ) -> crate::utils::response::Response<serde_json::Value> {
        let url = http.uri("log/new?d=/log/new".to_string()).await;
        let text = http
            .send_request(
                reqwest::Method::POST,
                &url,
                None,
                Some(self.gen_form().await?),
                Some(vec![("authorization", &self.token)]),
            )
            .await?;

        serde_json::from_str(&text).map_err(|_| {
            crate::Error::BadRequest(
                crate::UploadError::Parse(crate::ParseError::JsonDeserialize).into(),
            )
        })?
    }
}
