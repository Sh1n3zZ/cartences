// models/sentence.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Sentence {
    pub id: i32,
    pub uuid: String,
    pub content: String,
    pub category: Option<String>,
    pub from_source: Option<String>,
    pub from_author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub length: i32,
}
