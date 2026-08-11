use crate::models::NewTodo;
use auto_route::controller;
use axum::{Form, extract::Path, http::StatusCode, response::IntoResponse};
use html_macro::html;
use html_rt::Markup;
use sqlx::SqlitePool;

pub struct TodoController {
    pool: SqlitePool,
}

#[controller("/todo")]
impl TodoController {
    fn new() -> Self {
        Self {
            pool: crate::POOL.get().unwrap().clone(),
        }
    }

    #[get]
    async fn page(&self) -> Markup {
        let pool = self.pool.clone();
        html! {
            <!DOCTYPE html><html><head><title>"Rustploy Todo"</title></head><body>
                <h1>"Reactive Todo"</h1>
                <form method="post" action="/todo"><input name="title" required /><button>"Add"</button></form>
                @for todo in load_todos(pool.clone()).await["todos"] {
                    <div>
                        @if todo.done { <span><s>{&todo.title}</s></span> }
                        @else { <span>{&todo.title}</span> }
                        <form method="post" action={format!("/todo/{}/toggle", todo.id)}><button>"Toggle"</button></form>
                        <form method="post" action={format!("/todo/{}/delete", todo.id)}><button>"Delete"</button></form>
                    </div>
                }
            </body></html>
        }
    }

    #[post]
    async fn create(&self, Form(todo): Form<NewTodo>) -> impl IntoResponse {
        let _ = sqlx::query("INSERT INTO todos(title) VALUES (?)")
            .bind(todo.title.trim())
            .execute(&self.pool)
            .await;
        redirect()
    }

    #[post("/{id}/toggle")]
    async fn toggle(&self, Path(id): Path<i64>) -> impl IntoResponse {
        let _ = sqlx::query("UPDATE todos SET done = NOT done WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
        redirect()
    }

    #[post("/{id}/delete")]
    async fn delete(&self, Path(id): Path<i64>) -> impl IntoResponse {
        let _ = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
        redirect()
    }
}

async fn load_todos(pool: SqlitePool) -> Vec<crate::models::Todo> {
    sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
}

fn redirect() -> impl IntoResponse {
    (
        StatusCode::SEE_OTHER,
        [(axum::http::header::LOCATION, "/todo")],
    )
}
