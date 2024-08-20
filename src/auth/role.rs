use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::auth::jwt::Claims;
use crate::auth::jwt::AuthError;
use crate::auth::jwt::SECRET;
use sqlx::MySql;
use sqlx::Pool;
use log::error;

pub async fn decode_jwt_and_check_role(auth_header: &str, pool: &Pool<MySql>) -> Result<Claims, warp::Rejection> {
    let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

    // Decode the token
    let claims = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data.claims,
        Err(_) => {
            error!("Failed to decode JWT");
            return Err(warp::reject::custom(AuthError));
        }
    };

    // Fetch user role from the database using `sub` (user id)
    let role_result: Result<(String,), _> = sqlx::query_as("SELECT role FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_one(pool)
        .await;

    let role = match role_result {
        Ok(row) => row.0,
        Err(e) => {
            error!("Database query failed: {:?}", e);
            return Err(warp::reject::custom(AuthError));
        }
    };

    // Check if the user role is 'manager'
    if role != "manager" {
        error!("User role check failed: expected 'manager', got '{}'", role);
        return Err(warp::reject::custom(AuthError));
    }

    Ok(claims)
}