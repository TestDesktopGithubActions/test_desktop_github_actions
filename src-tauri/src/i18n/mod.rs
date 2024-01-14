use rust_i18n::t;

rust_i18n::i18n!("locales");
const SIMPLIFIED_CHINESE: &str = "zh_Hans";
const TRADITIONAL_CHINESE: &str = "zh_Hant";
const ENGLISH: &str = "en";
const GEORGIAN: &str = "ka";
const RUSSIAN: &str = "ru";
const PORTUGUESE: &str = "pt";
const SPANISH: &str = "es";
const FRENCH: &str = "fr";
const JAPANESE: &str = "ja";
const GERMAN: &str = "de";
const KOREAN: &str = "ko";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum Language {
    SimplifiedChinese,
    TraditionalChinese,
    English,
    Georgian,
    Russian,
    Portuguese,
    Spanish,
    French,
    Japanese,
    German,
    Korean,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::SimplifiedChinese => SIMPLIFIED_CHINESE.to_string(),
            Language::TraditionalChinese => TRADITIONAL_CHINESE.to_string(),
            Language::English => ENGLISH.to_string(),
            Language::Georgian => GEORGIAN.to_string(),
            Language::Russian => RUSSIAN.to_string(),
            Language::Portuguese => PORTUGUESE.to_string(),
            Language::Spanish => SPANISH.to_string(),
            Language::French => FRENCH.to_string(),
            Language::Japanese => JAPANESE.to_string(),
            Language::German => GERMAN.to_string(),
            Language::Korean => KOREAN.to_string(),
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            SIMPLIFIED_CHINESE | "zh_CN" => Self::SimplifiedChinese,
            TRADITIONAL_CHINESE | "zh_TW" | "zh_HK" => Self::TraditionalChinese,
            GEORGIAN => Self::Georgian,
            RUSSIAN => Self::Russian,
            PORTUGUESE => Self::Portuguese,
            SPANISH => Self::Spanish,
            FRENCH => Self::French,
            JAPANESE => Self::Japanese,
            GERMAN => Self::German,
            KOREAN => Self::Korean,
            _ => Self::English,
        }
    }
}

impl Language {
    pub(crate) async fn get_language() -> Result<Self, crate::LanguageError> {
        let pub_conn = crate::database::db::PUBLIC_SQLITE_POOL.get().ok_or(
            crate::LanguageError::DatabaseError(crate::DatabaseError::GetPublicSqliteConnFailed),
        )?;
        let conn = pub_conn
            .get_pool()
            .ok_or(crate::LanguageError::DatabaseError(
                crate::DatabaseError::GetPublicSqlitePoolFailed,
            ))?;
        let uid = crate::database::latest_login::LatestLogin::get_one(conn)
            .await
            .ok();

        let res = match (uid, std::env::var("LANG")) {
            (Some(info), _) => info.language.as_str().into(),
            (None, Ok(lang)) => {
                // tracing::warn!("language: {lang}");
                let v: Vec<&str> = lang.split('.').collect();
                if !v.is_empty() {
                    let lang = v[0];
                    lang.into()
                } else {
                    crate::i18n::Language::English
                }
            }
            (None, Err(_)) => crate::i18n::Language::English,
        };
        Ok(res)
    }

    pub fn i18n(&self, msg: &str) -> String {
        let l = self.to_string();
        rust_i18n::set_locale(l.as_str());
        t!(&msg, locales = l)
    }
}

impl<T> crate::utils::response::Response<T> {
    pub(crate) fn i18n(mut self, language: &Language) -> Self {
        let msg: Vec<&str> = self.message.split(": ").collect();
        let l = language.to_string();
        let l = l.as_str();
        rust_i18n::set_locale(l);
        let msg: Vec<String> = msg
            .into_iter()
            .map(|val| t!(&val, locales = l))
            .map(|val| val.replace(&format!("{l}."), "").replace('`', ""))
            .collect();

        let translate_msg = msg.join(": ");
        self.message = translate_msg;
        self
    }
}
