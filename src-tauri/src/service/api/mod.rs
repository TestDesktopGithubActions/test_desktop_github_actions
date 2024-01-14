pub mod action;
pub mod event;
pub mod service;

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub code: i64,
    pub email: String,
    pub coin_balance: String,
    pub balance: String,
    pub comm_balance: String,
    pub is_card: i8,
    pub card_id: String,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub surplus_day: i32,
    pub access: String,
    pub token: String,
    pub is_usdt: i32,
    pub usdt_addr: String,
    pub exp: i64,
}

impl TryFrom<&serde_json::Value> for User {
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value.clone()).map_err(|_| crate::ParseError::JsonDeserialize)
    }
    type Error = crate::ParseError;
}

impl User {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        code: i64,
        email: &str,
        coin_balance: &str,
        balance: &str,
        comm_balance: &str,
        is_card: i8,
        card_id: &str,
        start_at: Option<String>,
        end_at: Option<String>,
        surplus_day: i32,
        access: &str,
        token: &str,
        is_usdt: i32,
        usdt_addr: &str,
        exp: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            code,
            email: email.to_string(),
            coin_balance: coin_balance.to_string(),
            balance: balance.to_string(),
            comm_balance: comm_balance.to_string(),
            is_card,
            card_id: card_id.to_string(),
            start_at,
            end_at,
            surplus_day,
            access: access.to_string(),
            token: token.to_string(),
            is_usdt,
            usdt_addr: usdt_addr.to_string(),
            exp,
        }
    }

    pub fn to_data(json: &str) -> Result<Self, serde_json::Error> {
        let response: Response<Self> = serde_json::from_str(json)?;
        Ok(response.result)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeStart {
    pub guid: String,
    pub method: String,
    pub port: String,
    pub passwd: String,
}

impl NodeStart {
    pub fn to_data(json: &str) -> Self {
        let response: Response<Self> = serde_json::from_str(json).unwrap();
        response.result
    }
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub code: u16,
    pub msg: String,
    pub result: T,
}
