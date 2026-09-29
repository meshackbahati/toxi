use serde::{Deserialize, Serialize};

/// A blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub author: String,
    pub title: String,
    pub body: String,
}

/// Payload for writing a post.
#[derive(Debug, Deserialize)]
pub struct WritePost {
    pub title: String,
    pub body: String,
}
