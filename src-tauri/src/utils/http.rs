use std::time::Duration;

use crate::utils::time;

pub static NODEINFO: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::Mutex<crate::database::node::NodeInfo>>,
> = once_cell::sync::Lazy::new(|| {
    std::sync::Arc::new(tokio::sync::Mutex::new(
        crate::database::node::NodeInfo::default(),
    ))
});

pub(crate) static LANGUAGE: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::RwLock<crate::i18n::Language>>,
> = once_cell::sync::Lazy::new(|| {
    std::sync::Arc::new(tokio::sync::RwLock::new(crate::i18n::Language::English))
});

// const API: &str = "https://api.ffdev.cc/";
const API: &str = "https://api.falconflow.io/";
// const API: &str = "http://localhost:17776/";

pub(crate) static HTTP_PARAMS: once_cell::sync::Lazy<
    once_cell::sync::OnceCell<Result<HttpParams, crate::Error>>,
> = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

pub(crate) fn http_params_generator<'a>() -> &'a Result<HttpParams, crate::Error> {
    HTTP_PARAMS.get_or_init(|| {
        std::thread::spawn(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let addr = API.to_string();
                    let proof = crate::service::api::event::api::get_mac_address()
                        .await
                        .unwrap();
                    // tracing::error!("[http] proof: {proof}");
                    let platform = crate::sys::action::get_os_type();
                    let sys = crate::sys::system_info::SystemInfo::get_sys();
                    let pk = sys.to_sha3();
                    let language = crate::i18n::Language::get_language().await.unwrap();
                    let mut lang = LANGUAGE.write().await;
                    *lang = language.clone();
                    drop(lang);
                    HttpParams::new(addr, proof, platform, sys, pk)
                })
        })
        .join()
        .map_err(|e| {
            tracing::error!("[http_params_generator] error: {e:?}");
            crate::HttpError::HttpBuilderGenFailed.into()
        })
    })
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HttpParams {
    //language
    // pub(crate) l: crate::i18n::Language,
    //language
    pub(crate) proof: String,
    //platform
    pub p: Option<String>,
    //system info
    pub sys: crate::sys::system_info::SystemInfo,
    //token is sha3
    pub pk: Option<String>,
    //is the request uri
    pub _uri: Option<String>,
    //is the request addr
    pub addr: String,
}

impl HttpParams {
    pub fn new(
        addr: String,
        proof: String,
        platform: String,
        sys: crate::sys::system_info::SystemInfo,
        pk: String,
    ) -> Self {
        Self {
            proof,
            p: Some(platform),
            _uri: None,
            sys,
            pk: Some(pk),
            addr,
        }
    }

    pub async fn get_lang() -> crate::i18n::Language {
        let lang = LANGUAGE.read().await;
        lang.to_owned()
    }

    pub async fn uri(&self, uri: String) -> String {
        let api = self.addr.clone();
        let l = HttpParams::get_lang().await.to_string();
        let p = self.p.clone().unwrap();
        let pk = self.pk.clone().unwrap();
        let v = self.sys.get_os_version();
        let v = if v.contains('(') {
            v.split_once('(')
                .map(|(os, _)| format!("&v={}", os.trim()))
                .unwrap_or_default()
        } else {
            format!("&v={}", v)
        };

        let t = time::now();
        format!(
            "{}{}&l={}&p={}&pk={}&t={}&proof={}{}",
            api, uri, l, p, pk, t, self.proof, v
        )
    }

    pub async fn uri_web(&self, uri: String, proof: &str) -> String {
        let api = self.addr.clone();
        let l = HttpParams::get_lang().await.to_string();
        let pk = self.pk.clone().unwrap();
        let v = self.sys.get_os_version();
        let v = if v.contains('(') {
            v.split_once('(')
                .map(|(os, _)| format!("&v={}", os.trim()))
                .unwrap_or_default()
        } else {
            format!("&v={}", v)
        };

        let t = time::now();
        format!(
            "{}{}&l={}&p=web&pk={}&t={}&proof={}{}",
            api, uri, l, pk, t, proof, v
        )
    }

    pub async fn path_query(&self, uri: &str) -> String {
        let l = HttpParams::get_lang().await.to_string();
        let p = self.p.clone().unwrap();
        let pk = self.pk.clone().unwrap();
        let v = self.sys.get_os_version();
        let v = if v.contains('(') {
            v.split_once('(')
                .map(|(os, _)| format!("&v={}", os.trim()))
                .unwrap_or_default()
        } else {
            format!("&v={}", v)
        };
        let t = time::now();

        format!(
            "/{}&l={}&p={}&pk={}&t={}&proof={}{}",
            uri, l, p, pk, t, self.proof, v
        )
    }

    fn user_agent(&self) -> &str {
        let user_agent: &str = concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
            "#123"
        );
        user_agent
    }

    async fn public_get(&self, api: String) -> Result<String, crate::Error> {
        let client = reqwest::Client::builder()
            .user_agent(self.user_agent())
            .build()
            .unwrap();
        let url = self.uri(api).await;
        // println!("url:{}", url);
        let res = client.get(url).send().await?;
        Ok(res.text().await?)
    }

    pub async fn get_q(&self) -> Result<String, crate::Error> {
        self.public_get("q?d=q".to_string()).await
    }

    pub async fn send_request(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<serde_json::Value>,
        multipart: Option<reqwest::multipart::Form>,
        headers: Option<Vec<(&str, &str)>>,
    ) -> Result<String, crate::Error> {
        let client = reqwest::Client::new();
        let mut request = client.request(method, url);

        if let Some(body_content) = body {
            // tracing::info!("[send_request] body_content: {body_content:?}");
            request = request.json(&body_content);
        }

        if let Some(multipart) = multipart {
            request = request.multipart(multipart);
        }

        if let Some(header_list) = headers {
            for (key, value) in header_list {
                request = request.header(key, value);
            }
        }

        let response = request
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    crate::HttpError::TimedOut
                } else {
                    crate::HttpError::RequestFailed
                }
            })?;

        // tracing::info!("[send_request] response status: {:?}", response.status());
        match response.status() {
            reqwest::StatusCode::UNAUTHORIZED => {
                return Err(crate::Error::Jwt(crate::JwtError::IllegalAccess(
                    jsonwebtoken::errors::ErrorKind::InvalidToken.into(),
                )))
            }
            reqwest::StatusCode::OK => {}
            code => return Err(crate::HttpError::NonSuccessStatus(code).into()),
        }

        let text = response.text().await.map_err(crate::Error::Reqwest)?;
        // tracing::info!("[send_request] text: {text}");
        Ok(text)
    }
}
