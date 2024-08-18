use std::env;
use log::info;
use env_logger;
use warp::Filter;
use crate::database::connection::establish_connection;
use router::{cartences::hitokoto_route, create::create_route};

mod database;
mod models;
mod router;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Cartences has been activated successfully.");

    let pool = establish_connection().await.expect("Failed to establish connection.");

    let routes = hitokoto_route(pool.clone())
        .or(create_route(pool.clone()))
        .with(warp::log("cartences"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
