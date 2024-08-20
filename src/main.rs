use std::env;
use log::info;
use env_logger;
use warp::Filter;
use crate::database::connection::establish_connection;
use router::{cartences::hitokoto_route, create::create_route};

mod database;
mod models;
mod router;
mod auth;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Cartences has been activated successfully.");

    let pool = establish_connection().await.expect("Failed to establish connection.");

    let routes_public = hitokoto_route(pool.clone())
        .or(create_route(pool.clone()))
        .with(warp::log("cartences"));

    let routes_manager = create_route(pool.clone());

    let routes_auth = router::jwt::register(pool.clone())
        .or(router::jwt::login(pool.clone()))
        .or(router::jwt::jwt_validate());

    let routes = routes_public
        .or(routes_auth)
        .or(routes_manager);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
