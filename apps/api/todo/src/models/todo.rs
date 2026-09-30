use serde::{Deserialize, Serialize};

/// A todo item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub done: bool,
}

/// Payload for creating a todo.
#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}
