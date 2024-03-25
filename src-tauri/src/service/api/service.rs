pub(crate) async fn login(
    email: &str,
    passwd: &str,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    let passwd = &crate::utils::fun::md5(passwd);
    let req = crate::rpc::param::LoginReq::new(email, passwd, &http.proof);
    req.check()?;
    req.login(http).await
}

pub(crate) async fn login_temporary(
    email: &str,
    passwd: &str,
    proof: &str,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    let passwd = &crate::utils::fun::md5(passwd);
    let req = crate::rpc::param::LoginReq::new(email, passwd, proof);
    req.check()?;
    req.login_temporary(http).await
}

pub(crate) async fn logout(
    token: String,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    if token.is_empty() {
        Err(crate::Error::BadRequest(
            crate::LogoutError::Param(crate::ParamError::TokenMissing).into(),
        ))
        .into()
    } else {
        let req = crate::rpc::param::LogoutReq::new(token);
        req.logout(http).await
    }
}

pub(crate) async fn account_update_token(
    token: String,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    if token.is_empty() {
        return Err(crate::Error::BadRequest(
            crate::AccountUpdateTokenError::Param(crate::ParamError::TokenMissing).into(),
        ))
        .into();
    }
    let req = crate::rpc::param::AccountUpdateTokenReq::new(token);
    req.account_update_token(http).await
}

pub(crate) async fn node_list(
    token: String,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    if token.is_empty() {
        Err(crate::Error::BadRequest(
            crate::NodeError::Param(crate::ParamError::TokenMissing).into(),
        ))
        .into()
    } else {
        let req = crate::rpc::param::NodeListReq::new(token);
        req.list(http).await
    }
}

pub(crate) async fn node_start(
    token: &str,
    guid: &str,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    // tracing::info!("[guid] guid res: {:?}", guid);

    if token.is_empty() {
        return Err(crate::Error::BadRequest(
            crate::NodeError::Param(crate::ParamError::TokenMissing).into(),
        ))
        .into();
    }
    // tracing::error!("[node_start] guid: {guid}");


    let private = x25519_dalek::StaticSecret::random_from_rng(rand_core::OsRng);
    let public_key = x25519_dalek::PublicKey::from(&private);
    let public_key = hex::encode(public_key.as_bytes());
    let private = hex::encode(private.to_bytes());

    let req = crate::rpc::param::NodeStartReq::new(guid, token,public_key.as_str());
    // tracing::error!("[node_start] req: {:#?}",req);

    req.node_start(http,private).await
}

pub(crate) async fn node_end(
    token: &str,
    guid: &str,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    if token.is_empty() {
        return Err(crate::Error::BadRequest(
            crate::NodeError::Param(crate::ParamError::TokenMissing).into(),
        ))
        .into();
    }
    let req = crate::rpc::param::NodeEndReq::new(guid, token);
    req.node_end(http).await
}

pub(crate) async fn upload_log(
    token: &str,
    email: &str,
    http: &crate::utils::http::HttpParams,
) -> crate::utils::response::Response<serde_json::Value> {
    // let token = get_token().await;
    if token.is_empty() {
        return Err(crate::Error::BadRequest(
            crate::UploadError::Param(crate::ParamError::TokenMissing).into(),
        ))
        .into();
    }

    let pub_storage = crate::database::db::PUBLIC_STORAGE
        .get()
        .ok_or(crate::Error::BadRequest(
            crate::UploadError::DatabaseError(crate::DatabaseError::GetPublicStorageFailed).into(),
        ))?;

    let log_name = format!("{}.log", crate::service::node::init::DEFAULT_LOG_LEVEL);
    let pub_storage = pub_storage.join(&log_name);
    // tracing::info!("[upload log] pub_storage: {pub_storage:?}");

    let req = crate::rpc::param::UploadLogReq::new(token, email, &log_name, pub_storage);
    req.upload(http).await
}

pub(crate) async fn ping(
    ips: Vec<String>,
) -> crate::utils::response::Response<Vec<serde_json::Value>> {
    if ips.is_empty() {
        return Err(crate::Error::BadRequest(
            crate::PingError::Param(crate::ParamError::IpsEmpty).into(),
        ))
        .into();
    }

    let mut res = Vec::new();
    for ip in ips {
        let addr: std::net::IpAddr = ip.parse().map_err(|_| {
            crate::Error::BadRequest(crate::PingError::Parse(crate::ParseError::Addr).into())
        })?;
        let duration = match surge_ping::ping(addr, &[1, 2, 3, 4, 5, 6, 7, 8]).await {
            Ok((_, duration)) => duration.as_secs_f64(),
            Err(e) => {
                tracing::warn!("Ping failed: {:?}", e);
                -1.0
            }
        };
        res.push(serde_json::json!({
            "ip":ip,
            "delay":duration
        }));
    }
    Ok(res).into()
}

pub(crate) async fn set_language(language: &str) -> crate::utils::response::Response<()> {
    let pub_conn =
        crate::database::db::PUBLIC_SQLITE_POOL
            .get()
            .ok_or(crate::Error::BadRequest(
                crate::LanguageError::DatabaseError(
                    crate::DatabaseError::GetPublicSqliteConnFailed,
                )
                .into(),
            ))?;
    let conn = pub_conn.get_pool().ok_or(crate::Error::BadRequest(
        crate::LanguageError::DatabaseError(crate::DatabaseError::GetPublicSqlitePoolFailed).into(),
    ))?;
    if crate::database::latest_login::LatestLogin::update_lang(conn, language)
        .await
        .map_err(|_| {
            crate::Error::BadRequest(
                crate::LanguageError::DatabaseError(crate::DatabaseError::UpdateFailed).into(),
            )
        })?
        .rows_affected()
        == 0
    {
        return Err(crate::Error::BadRequest(
            crate::LanguageError::DatabaseError(crate::DatabaseError::UpdateFailed).into(),
        ))
        .into();
    };
    let mut lang = crate::utils::http::LANGUAGE.write().await;
    let l: crate::i18n::Language = language.into();
    crate::service::tauri::service::set_item(&l)
        .await
        .map_err(|e| crate::Error::BadRequest(e.into()))?;

    *lang = l;
    drop(lang);

    Ok(()).into()
}
