// router/jwt.rs

use warp::Filter;
use crate::auth::handlers::{handle_login, handle_register};
use crate::auth::jwt::decode_jwt;
use warp::reject::Reject;

#[derive(Debug)]
struct JWTValidationError;

impl Reject for JWTValidationError {}

pub fn login(pool: sqlx::MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_login)
}

pub fn register(pool: sqlx::MySqlPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_register)
}

pub fn jwt_validate() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("validate")
        .and(warp::header::<String>("authorization"))
        .and_then(|token: String| async move {
            if let Some(claims) = decode_jwt(&token.replace("Bearer ", "")).ok() {
                Ok(warp::reply::json(&claims))
            } else {
                Err(warp::reject::custom(JWTValidationError))
            }
        })
}

fn with_db(pool: sqlx::MySqlPool) -> impl Filter<Extract = (sqlx::MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
