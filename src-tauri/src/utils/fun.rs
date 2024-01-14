pub fn md5<S: Into<String>>(input: S) -> String {
    use sha2::Digest;
    // create a Md5 hasher instance
    let mut hasher = md5::Md5::default();
    // process input message
    hasher.update(input.into());
    hex::encode(Digest::finalize(hasher))
}

// /**
//  * sha2 and 256
//  */
// pub fn sha2<T: Into<String>>(key: T) -> String {
//     use crypto::digest::Digest;
//     use crypto::sha2::Sha256;
//     let mut hasher = Sha256::new();
//     hasher.input_str(&key.into());
//     hasher.result_str()
// }
