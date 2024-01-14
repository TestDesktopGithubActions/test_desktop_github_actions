use totp_rs::{Algorithm, Secret, TOTP};

pub fn get_code_token6() -> i32 {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        60,
        Secret::Raw("RAMFLUX.COM/SuperSecret".as_bytes().to_vec())
            .to_bytes()
            .unwrap(),
    )
    .unwrap();
    let token = totp.generate_current().unwrap();
    let code = token.parse().unwrap();
    code
}
