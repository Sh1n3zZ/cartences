use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::reject::Reject;
use crate::auth::jwtcfg::SECRET;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
    aud: String,
    iat: usize,
    nbf: usize,
    pub role: String,
    username: String,
}

#[derive(Debug)]
pub struct AuthError;

impl Reject for AuthError {}

pub fn create_jwt(user_id: i64, username: &str, role: &str) -> Result<String, warp::Rejection> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 7 * 24 * 60 * 60; // 7 days

    let issued_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        aud: "cartences".to_owned(),
        iat: issued_at as usize,
        nbf: issued_at as usize,
        role: role.to_owned(),
        username: username.to_owned(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .map_err(|_| warp::reject::custom(AuthError))
}

pub fn decode_jwt(token: &str) -> Result<Claims, warp::Rejection> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| warp::reject::custom(AuthError))
}
