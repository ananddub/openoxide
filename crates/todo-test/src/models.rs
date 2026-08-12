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

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct MetricSample {
    pub sequence: u64,
    pub value: u64,
}

impl MetricSample {
    pub const TYPESCRIPT: &str = "{ sequence: number; value: number }";
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct ActivityEvent {
    pub sequence: u64,
    pub message: String,
}

impl ActivityEvent {
    pub const TYPESCRIPT: &str = "{ sequence: number; message: string }";
}
