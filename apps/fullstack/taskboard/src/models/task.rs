use serde::{Deserialize, Serialize};

/// A user task. `done` maps to 0/1 in sqlite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl Task {
    pub fn new(user_id: String, title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            title,
            done: false,
            created_at: None,
        }
    }
}

/// Payload for creating a task.
#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
}

/// Payload for updating a task. All fields optional.
#[derive(Debug, Deserialize, Default)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}
