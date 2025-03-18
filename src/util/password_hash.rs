use std::env;
use std::sync::LazyLock;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

static SALT: LazyLock<SaltString> = LazyLock::new(|| {
    env::var("SALT")
        .map(|salt| SaltString::from_b64(&salt).expect("Invalid salt"))
        .unwrap_or_else(|_| SaltString::generate(&mut OsRng))
});

pub fn hash_password(password: &str) -> String {
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &*SALT)
        .expect("Failed to hash password");

    password_hash.to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(hash).expect("Invalid password hash");

    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}
