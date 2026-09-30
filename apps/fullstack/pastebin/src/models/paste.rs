use serde::{Deserialize, Serialize};

/// A paste with its sanitized preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paste {
    pub id: String,
    pub title: String,
    pub body: String,
    pub preview: String,
}

/// Payload for creating a paste.
#[derive(Debug, Deserialize)]
pub struct CreatePaste {
    pub title: Option<String>,
    pub body: String,
}
