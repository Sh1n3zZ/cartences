use jsonwebtoken::{decode, DecodingKey, Validation};
use warp::{http::StatusCode, Rejection, reply::json, reply::with_status};
use serde::Serialize;
use crate::auth::jwt::Claims;
use crate::auth::jwt::SECRET;
use sqlx::MySql;
use sqlx::Pool;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Debug)]
#[allow(dead_code)]
enum CustomError {
    AuthError(String),
    DatabaseError(String),
    Other(String),
}

impl warp::reject::Reject for CustomError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(custom_error) = err.find::<CustomError>() {
        let (code, message) = match custom_error {
            CustomError::AuthError(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            CustomError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            CustomError::Other(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };
        let json = json(&ErrorResponse {
            code: code.as_u16(),
            message,
        });
        Ok(with_status(json, code))
    } else {
        // 未知错误
        Ok(with_status(
            json(&ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal Server Error".into(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn decode_jwt_and_check_role(auth_header: &str, _pool: &Pool<MySql>) -> Result<Claims, warp::Rejection> {
    let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

    let claims = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data.claims,
        Err(_) => {
            return Err(warp::reject::custom(CustomError::AuthError(
                "Invalid token".into(),
            )));
        }
    };

    // 假设有其他逻辑检查角色
    if claims.role != "manager" {
        return Err(warp::reject::custom(CustomError::AuthError(
            format!("User role check failed: expected 'manager', got '{}'", claims.role),
        )));
    }

    Ok(claims)
}

