// models/sentence.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Sentence {
    pub id: i32,
    pub uuid: String,
    pub content: String,
    pub category: Option<String>,
    pub from_source: Option<String>,
    pub from_author: Option<String>,
    pub created_at: NaiveDateTime,
    pub length: i32,
}
