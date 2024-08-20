// auth/handlers.rs

use warp::{Reply, http::StatusCode, reply::json, reply::with_status};
use sqlx::MySqlPool;
use serde::{Deserialize, Serialize};
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::auth::jwt::create_jwt;
use log::error;
use std::convert::Infallible;

#[derive(Deserialize)]
pub struct UserLogin {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserRegister {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

fn json_error_response(code: StatusCode, message: &str) -> Box<dyn Reply> {
    let json = json(&ErrorResponse {
        code: code.as_u16(),
        message: message.to_string(),
    });
    Box::new(with_status(json, code))
}

pub async fn handle_login(
    login: UserLogin,
    pool: MySqlPool,
) -> Result<Box<dyn Reply>, Infallible> {
    let row = match sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, username, password, role FROM users WHERE username = ?"
    )
    .bind(&login.username)
    .fetch_optional(&pool)
    .await {
        Ok(row) => row,
        Err(err) => {
            error!("Database query error: {:?}", err);
            return Ok(json_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database query error",
            ));
        }
    };

    if let Some((user_id, username, stored_password_hash, role)) = row {
        if verify(&login.password, &stored_password_hash).unwrap_or(false) {
            match create_jwt(user_id, &username, &role) {
                Ok(token) => Ok(Box::new(with_status(json(&LoginResponse { token }), StatusCode::OK))),
                Err(err) => {
                    error!("JWT creation error: {:?}", err);
                    Ok(json_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "JWT creation error",
                    ))
                }
            }
        } else {
            error!("Password verification failed for user: {}", login.username);
            Ok(json_error_response(StatusCode::UNAUTHORIZED, "Invalid username or password"))
        }
    } else {
        error!("User not found: {}", login.username);
        Ok(json_error_response(StatusCode::NOT_FOUND, "User not found"))
    }
}


pub async fn handle_register(
    register: UserRegister,
    pool: MySqlPool,
) -> Result<Box<dyn Reply>, Infallible> {
    let user_exists = sqlx::query("SELECT 1 FROM users WHERE username = ? OR email = ?")
        .bind(&register.username)
        .bind(&register.email)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if user_exists.is_some() {
        return Ok(Box::new(with_status(
            json(&serde_json::json!({
                "code": 409,
                "message": "Username or email already exists"
            })),
            StatusCode::CONFLICT,
        )));
    }

    let hashed_password = match hash(&register.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            error!("Password hashing error: {:?}", err);
            return Ok(json_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password hashing error",
            ));
        }
    };

    match sqlx::query(
        "INSERT INTO users (username, password, email, role) VALUES (?, ?, ?, ?)"
    )
    .bind(&register.username)
    .bind(&hashed_password)
    .bind(&register.email)
    .bind("user")
    .execute(&pool)
    .await {
        Ok(_) => Ok(Box::new(with_status(
            json(&serde_json::json!({
                "status": "success",
                "message": "User registered successfully",
                "user": {
                    "username": register.username,
                    "email": register.email,
                    "role": "user"
                }
            })),
            StatusCode::CREATED,
        ))),
        Err(err) => {
            error!("Database insertion error: {:?}", err);

            Ok(json_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database insertion error",
            ))
        }
    }
}
