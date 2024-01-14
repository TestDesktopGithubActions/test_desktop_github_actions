pub(crate) struct EncryptParams {
    nonce: [u8; 12],
    body: String,
    key: [u8; 32],
}

impl EncryptParams {
    // TODO:
    pub(crate) fn new(body: &str, nonce: [u8; 12], key: [u8; 32]) -> Result<Self, crate::Error> {
        Ok(Self {
            nonce,
            body: body.to_owned(),
            key,
        })
    }

    pub(crate) fn encrypt(self) -> String {
        use aes_gcm::{aead::Aead, KeyInit};
        use hex::ToHex;

        let Self {
            nonce,
            body,
            ref key,
        } = self;
        let key_aes: &aes_gcm::Key<aes_gcm::Aes256Gcm> = key.into();
        let cipher = aes_gcm::Aes256Gcm::new(key_aes);

        let nonce_ga = nonce.into_iter().collect();
        cipher
            .encrypt(&nonce_ga, body.as_bytes())
            .unwrap()
            .encode_hex::<String>()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum FinalResponse<T> {
    EncryptResponse(ResponseBody),
    UnencryptResponse(super::response::Response<T>),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ResponseBody {
    pub nonce: [u8; 12],
    pub body: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Crypto {
    pub nonce: [u8; 12],
    pub body: String,
    pub key: [u8; 32],
}

impl Crypto {
    pub(crate) fn new(nonce: [u8; 12], body: String, key: [u8; 32]) -> Self {
        Self { nonce, body, key }
    }

    pub(crate) fn decrypto(self) -> Result<String, crate::MiddlewareError> {
        let Crypto {
            body,
            nonce,
            ref key,
        } = self;
        use aes_gcm::aead::Aead;
        use aes_gcm::KeyInit;
        use hex::FromHex;

        let body = <Vec<u8>>::from_hex(body).unwrap();
        let key: &aes_gcm::Key<aes_gcm::Aes256Gcm> = key.into();
        let cipher = aes_gcm::Aes256Gcm::new(key);

        let nonce = nonce.into_iter().collect();
        let plaintext = cipher.decrypt(&nonce, body.as_slice());
        let plaintext = plaintext.unwrap_or_default();
        String::from_utf8(plaintext).map_err(|_| {
            crate::MiddlewareError::Decrypt(crate::ParseError::VecToStringFailed.into())
        })
    }
}

pub fn gen_nonce() -> [u8; 12] {
    let mut rng = rand::thread_rng();
    let mut nonce: [u8; 12] = [0; 12];
    use rand::Rng;

    nonce
        .iter_mut()
        .enumerate()
        .for_each(|(_, item)| *item = rng.gen::<u8>());
    // println!("Generated nonce: {:?}", nonce);
    nonce
}

pub fn gen_timestamp() -> String {
    let now = chrono::Utc::now();
    let format = chrono::format::StrftimeItems::new("%Y%m%d%H%M%S");
    now.format_with_items(format).to_string()
}

pub(crate) fn gen_key(token: Option<&str>) -> Result<[u8; 32], crate::MiddlewareError> {
    // return Err(
    //     crate::MiddlewareError::Signature(crate::ParseError::ToArrayError.into()).into(),
    // );
    match token {
        Some(token) => {
            let parts: Vec<&str> = token.split('.').collect();

            if parts.len() != 3 {
                println!("Invalid token");
                return Err(crate::MiddlewareError::InvalidToken);
            }

            use base64::engine::Engine as _;
            let payload_str = parts[1];
            // println!("payload_str: {payload_str}");
            let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(payload_str)
                .map_err(|_| crate::MiddlewareError::Decode)?;
            // println!("payload_bytes: {payload_bytes:?}");
            let payload_json = String::from_utf8(payload_bytes).unwrap();
            // println!("payload_json: {payload_json}");
            let payload: std::collections::HashMap<String, serde_json::Value> =
                serde_json::from_str(&payload_json).unwrap();
            // println!("payload: {payload:?}");

            let key_value = payload
                .get("key")
                .ok_or(crate::MiddlewareError::WithoutKey)?;

            let key_list = key_value.as_array().ok_or(crate::MiddlewareError::Parse(
                crate::ParseError::ValueToVecFailed,
            ))?;

            let key: Vec<u8> = key_list
                .iter()
                .filter_map(|v| v.as_u64())
                .map(|u| u as u8)
                .collect();
            // println!("key: {:?}", key);
            to_u8_32(key)
        }
        None => Ok(get_default_key()),
    }
}

pub fn to_u8_32(key: Vec<u8>) -> Result<[u8; 32], crate::MiddlewareError> {
    if key.len() != 32 {
        return Err(crate::MiddlewareError::InvalidKey);
    }

    let array_u8: [u8; 32] = key
        .try_into()
        .map_err(|_| crate::MiddlewareError::Parse(crate::ParseError::VecToArrayFailed))?;

    Ok(array_u8)
}

pub(crate) fn get_default_key() -> [u8; 32] {
    // use default key
    let mut key = [0u8; 32];
    let key_bytes = "yek-fr".as_bytes();

    let sha = sha256_padding_u8_32(key_bytes);
    key.copy_from_slice(&sha[..32]);
    key
}

pub fn sha256_padding_u8_32(key: &[u8]) -> [u8; 32] {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(key);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..]);
    key
}

#[cfg(test)]
mod tests {
    #[test]
    fn encrypt_test() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiZjVhZjU5NjliOTA2ZDIyYzllYjdkZTBhZTA2YmYxZCIsImtleSI6WzEzNSwyMTQsNzIsMTY2LDM2LDExNCwxOCwxNjEsMTY3LDExOSwxMywyNTQsMTU3LDEyNCwyMjQsMTE5LDE1LDczLDE2NiwxMDIsMjU1LDM3LDEyNSw1NSwzOSwxNzcsNzksMTcsMjIsOTQsMjQ3LDIxN10sImlhdCI6MTY5NDQxMDU4NiwiZXhwIjoxNjk0NDI4NTg2fQ._QWePc2BItPPcjDTAS7T3OwxVLgMjqJlPaRpKHwiK0s";
        let nonce = crate::utils::crypto::gen_nonce();
        let key = crate::utils::crypto::gen_key(Some(token)).unwrap();
        let res = crate::utils::crypto::EncryptParams::new("body", nonce, key).unwrap();
        let res = res.encrypt();
        println!("res: {res:?}");
    }
    #[test]
    fn test_generate_custom_timestamp() {
        let custom_timestamp = crate::utils::crypto::gen_timestamp();
        assert_eq!(custom_timestamp.len(), 14); // 验证生成的时间戳长度是否为 14
    }

    #[test]
    fn deserialize_to_final_response() {
        let nonce = [11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11];
        let body = "asdasd".to_string();
        let json = serde_json::json!({
            "nonce": nonce,
            "body": body
        })
        .to_string();

        let res: crate::utils::crypto::FinalResponse<String> = serde_json::from_str(&json).unwrap();
        println!("[deserialize_to_final_response] res: {res:?}");
    }
}
