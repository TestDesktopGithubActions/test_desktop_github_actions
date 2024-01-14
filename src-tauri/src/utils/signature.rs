#[derive(Debug, serde::Deserialize)]
pub struct Signature {
    pub timestamp: String,
    pub nonce: String,
}

impl Signature {
    pub(crate) fn new(timestamp: String, nonce: String) -> Self {
        Self { timestamp, nonce }
    }
    pub(crate) fn gen_signature(&self, body: &str, key: Vec<u8>) -> String {
        let Signature { timestamp, nonce } = self;
        // tracing::warn!("[gen_signature] body: {:?}", body);

        use base64::engine::Engine as _;

        let body_digest = sha256::digest(body);
        // println!("摘要: {:?}", body_digest);
        let sig_c = format!("{timestamp}{nonce}{body_digest}");
        // println!("待签名串C: {}", sig_c);
        let sig_e = hmac_sha256::HMAC::mac(sig_c, key);
        // println!("签名字节数组E: {:?}", sig_e);
        let sig_content = base64::engine::general_purpose::STANDARD.encode(sig_e);

        // println!("签名内容: {}", sig_content);
        let open_body_sig = format!(
            "RF-BODY-SIG Timestamp=\"{timestamp}\", Nonce=\"{nonce}\", Signature=\"{sig_content}\""
        );

        // println!("open_body_sig: {}", open_body_sig);

        open_body_sig
    }

    pub async fn default_key_and_signature(
        body: &str,
    ) -> Result<([u8; 32], String), crate::MiddlewareError> {
        let key = crate::utils::crypto::gen_key(None)?;
        let nonce = crate::utils::crypto::gen_nonce();
        let timestamp = crate::utils::crypto::gen_timestamp();

        let nonce_hex = hex::encode(nonce);
        Ok((
            key,
            crate::utils::signature::Signature::new(timestamp, nonce_hex)
                .gen_signature(body, key.to_vec()),
        ))
    }

    pub async fn key_and_signature(
        token: &str,
        body: &str,
    ) -> Result<([u8; 32], [u8; 12], String), crate::MiddlewareError> {
        let key = crate::utils::crypto::gen_key(Some(token))?;
        let nonce = crate::utils::crypto::gen_nonce();
        let timestamp = crate::utils::crypto::gen_timestamp();
        let nonce_hex = hex::encode(nonce);
        Ok((
            key,
            nonce,
            crate::utils::signature::Signature::new(timestamp, nonce_hex)
                .gen_signature(body, key.to_vec()),
        ))
    }
}
