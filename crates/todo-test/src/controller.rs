use crate::models::{NewTodo, Todo};
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use sqlx::SqlitePool;

pub struct TodoController {
    pool: SqlitePool,
}

#[controller("/api/todos")]
impl TodoController {
    fn new() -> Self {
        Self {
            pool: crate::POOL.get().expect("database is initialized").clone(),
        }
    }

    #[get("")]
    #[live("todos")]
    async fn list(&self) -> Json<Vec<Todo>> {
        Json(
            sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default(),
        )
    }

    #[post("")]
    async fn create(&self, Json(todo): Json<NewTodo>) -> StatusCode {
        let title = todo.title.trim();
        if title.is_empty() {
            return StatusCode::BAD_REQUEST;
        }
        if sqlx::query("INSERT INTO todos(title) VALUES (?)")
            .bind(title)
            .execute(&self.pool)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        self.publish_todos().await;
        StatusCode::CREATED
    }

    #[post("/{id}/toggle")]
    async fn toggle(&self, Path(id): Path<i64>) -> StatusCode {
        if sqlx::query("UPDATE todos SET done = NOT done WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        self.publish_todos().await;
        StatusCode::NO_CONTENT
    }

    #[delete("/{id}")]
    async fn delete(&self, Path(id): Path<i64>) -> StatusCode {
        if sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        self.publish_todos().await;
        StatusCode::NO_CONTENT
    }

    async fn publish_todos(&self) {
        let todos = sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default();

        if let Ok(publisher) = TodoController::todos() {
            let _ = publisher.publish(todos).await;
        }
    }
}
