// database/connection.rs

use serde::Deserialize;
use sqlx::{MySql, Pool};
use config::{Config, ConfigError};

#[derive(Deserialize)]
struct DatabaseConfig {
    database: DatabaseInnerConfig,
}

#[derive(Deserialize)]
struct DatabaseInnerConfig {
    url: String,
}

pub async fn establish_connection() -> Result<Pool<MySql>, ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    let database_config: DatabaseConfig = config.try_deserialize()?;

    println!("Database URL: {}", database_config.database.url);

    let pool = Pool::<MySql>::connect(&database_config.database.url)
        .await
        .expect("Failed to create pool.");

    Ok(pool)
}
