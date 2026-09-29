use serde::{Deserialize, Serialize};

/// A shortened link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub code: String,
    pub url: String,
    pub hits: i64,
}

/// Payload for creating a link.
#[derive(Debug, Deserialize)]
pub struct CreateLink {
    pub url: String,
    pub code: Option<String>,
}

/// Random 7-char code. UUID hex truncated: uniform enough for codes,
// collision-checked against the database on insert.
pub fn random_code() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..7].to_string()
}
