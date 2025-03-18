pub fn hash_password(password: &str, salt: &str) -> String {
    // [TODO] Properly hash the password
    format!("{}:{}", salt, password)
}
