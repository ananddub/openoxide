use crate::models::{ActivityEvent, MetricSample, NewTodo, Todo};
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
    #[live("todos", table = "todos")]
    async fn list(&self) -> Json<Vec<Todo>> {
        Json(
            sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default(),
        )
    }

    #[get("/metrics")]
    #[live("metrics", strategy = latest)]
    async fn current_metrics(&self) -> Json<MetricSample> {
        Json(MetricSample {
            sequence: 0,
            value: 0,
        })
    }

    #[get("/activity")]
    #[live("activity", strategy = stream, capacity = 32, replay = 10)]
    async fn activity_feed(&self) -> Json<ActivityEvent> {
        Json(ActivityEvent {
            sequence: 0,
            message: "ready".into(),
        })
    }

    #[post("/demo/latest")]
    async fn latest_demo(&self) -> StatusCode {
        for sequence in 1..=100 {
            let sample = MetricSample {
                sequence,
                value: sequence * 10,
            };
            let Ok(publisher) = TodoController::metrics() else {
                return StatusCode::SERVICE_UNAVAILABLE;
            };
            if publisher.publish(sample).await.is_err() {
                return StatusCode::SERVICE_UNAVAILABLE;
            }
        }
        StatusCode::NO_CONTENT
    }

    #[post("/demo/stream")]
    async fn stream_demo(&self) -> StatusCode {
        for sequence in 1..=20 {
            let event = ActivityEvent {
                sequence,
                message: format!("ordered event {sequence}"),
            };
            let Ok(publisher) = TodoController::activity() else {
                return StatusCode::SERVICE_UNAVAILABLE;
            };
            if publisher.publish(event).await.is_err() {
                return StatusCode::SERVICE_UNAVAILABLE;
            }
            tokio::time::sleep(std::time::Duration::from_millis(60)).await;
        }
        StatusCode::NO_CONTENT
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
        StatusCode::NO_CONTENT
    }
}
