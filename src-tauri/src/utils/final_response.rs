#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum FinalResponse<T> {
    EncryptResponse(super::crypto::ResponseBody),
    UnencryptResponse(super::response::Response<T>),
}

impl TryFrom<([u8; 32], &str)> for super::response::Response<serde_json::Value> {
    type Error = crate::MiddlewareError;

    fn try_from((key, value): ([u8; 32], &str)) -> Result<Self, Self::Error> {
        let msg = value.try_into()?;
        match msg {
            crate::utils::final_response::FinalResponse::EncryptResponse(res_body) => {
                let text = crate::utils::crypto::Crypto::new(res_body.nonce, res_body.body, key)
                    .decrypto()?;
                serde_json::from_str::<crate::utils::response::Response<serde_json::Value>>(&text)
                    .map_err(|_| {
                        crate::MiddlewareError::Decrypt(crate::DecryptError::Parse(
                            crate::ParseError::JsonDeserialize,
                        ))
                    })
            }
            crate::utils::final_response::FinalResponse::UnencryptResponse(resp) => Ok(resp),
        }
    }
}

impl TryFrom<&str> for crate::utils::final_response::FinalResponse<serde_json::Value> {
    type Error = crate::MiddlewareError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
            .map_err(|_| crate::MiddlewareError::Parse(crate::ParseError::JsonDeserialize))
    }
}
