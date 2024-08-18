// router/cartences.rs

use warp::Filter;
use std::convert::Infallible;
use sqlx::MySql;
use sqlx::Pool;
use crate::models::sentence::Sentence;

pub fn hitokoto_route(pool: Pool<MySql>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("cartences")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_hitokoto)
        .with(warp::log("cartences"))
}

fn with_db(pool: Pool<MySql>) -> impl Filter<Extract = (Pool<MySql>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn handle_hitokoto(pool: Pool<MySql>) -> Result<impl warp::Reply, Infallible> {
    let row: Option<Sentence> = sqlx::query_as::<_, Sentence>(
        "SELECT * FROM sentences ORDER BY RAND() LIMIT 1"
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to fetch hitokoto");

    if let Some(sentence) = row {
        Ok(warp::reply::json(&sentence))
    } else {
        Ok(warp::reply::json(&serde_json::json!({"error": "No hitokoto found"})))
    }
}
