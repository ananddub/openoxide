use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, poem_openapi::Object)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub const TYPESCRIPT: &str = "{ id: number; title: string; done: boolean }";
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewTodo {
    pub title: String,
}
