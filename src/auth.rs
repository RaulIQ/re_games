// auth.rs
use argon2::{self, Config};
use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = b"somesalt";
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password.as_bytes())
}

pub fn create_jwt(username: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims { sub: username.to_owned(), exp: expiration };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret_key".as_ref())).unwrap()
}
