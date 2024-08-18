// models/newsentences.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewSentence {
    pub content: String,
    pub category: Option<String>,
    pub from_source: Option<String>,
    pub from_author: Option<String>,
}
