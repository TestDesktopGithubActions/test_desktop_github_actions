use serde::{de::DeserializeOwned, Deserialize, Serialize};

// use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiRoot<T> {
    pub code: i32,
    pub msg: String,
    pub result: Option<T>,
}
impl<T: Serialize + DeserializeOwned + Clone> std::default::Default for ApiRoot<T> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: "error".to_string(),
            result: None,
        }
    }
}

impl<T> ApiRoot<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn to_data(jsons: &str) -> Self {
        println!("to_data:{:?}", jsons);

        let data = serde_json::from_str(jsons);
        match data {
            Ok(data) => data,
            Err(err) => {
                println!("{:?}", err);
                ApiRoot::default()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QResult {
    pub vendor: String,
    pub category: String,
    pub os: String,
    #[serde(rename = "os_version")]
    pub os_version: String,
    #[serde(rename = "browser_type")]
    pub browser_type: String,
    pub version: String,
    pub ip: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub code: i64,
    #[serde(rename = "coin_balance")]
    pub coin_balance: String,
    pub balance: String,
    #[serde(rename = "is_card")]
    pub is_card: i64,
    #[serde(rename = "card_id")]
    pub card_id: Option<String>,
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    pub access: String,
    pub token: String,
    pub exp: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct NodeList {
    pub delay: String,
    pub guid: String,
    pub ip: String,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeStart {
    pub port: String,
    pub passwd: String,
    pub method: String,
    pub guid: String,
}
