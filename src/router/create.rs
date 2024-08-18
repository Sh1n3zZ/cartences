// router/create.rs

use warp::Filter;
use std::convert::Infallible;
use sqlx::MySql;
use sqlx::Pool;
use uuid::Uuid;
use crate::models::newsentences::NewSentence;

pub fn create_route(pool: Pool<MySql>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(handle_create_sentence)
}

fn with_db(pool: Pool<MySql>) -> impl Filter<Extract = (Pool<MySql>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn handle_create_sentence(new_sentence: NewSentence, pool: Pool<MySql>) -> Result<impl warp::Reply, Infallible> {
    let uuid = Uuid::new_v4().to_string();
    let length = new_sentence.content.chars().count();  

    let result = sqlx::query(
        r#"
        INSERT INTO sentences (uuid, content, category, from_source, from_author, length)
        VALUES (?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&uuid)
    .bind(&new_sentence.content)
    .bind(&new_sentence.category)
    .bind(&new_sentence.from_source)
    .bind(&new_sentence.from_author)
    .bind(length as i32)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!({"status": "success", "uuid": uuid}))),
        Err(e) => {
            let error_message = format!("SQL Error: {:?}", e);
            Ok(warp::reply::json(&serde_json::json!({"status": "failure", "error": error_message})))
        },
    }
}
