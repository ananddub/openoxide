use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NewTodo {
    pub title: String,
}
