pub fn hash_password(password: &str, salt: &str) -> String {
    // [TODO] Properly hash the password
    format!("{}:{}", salt, password)
}

pub fn verify_password(password: &str, hashed_password: &str, salt: &str) -> bool {
    // [TODO] Properly verify the password
    hashed_password == hash_password(password, salt)
}
