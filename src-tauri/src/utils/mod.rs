pub mod crypto;
pub mod final_response;
pub mod fun;
pub mod http;
pub mod response;
pub mod signature;
pub mod time;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientMsg<T> {
    pub code: i32,
    pub msg: String,
    pub result: T,
}
