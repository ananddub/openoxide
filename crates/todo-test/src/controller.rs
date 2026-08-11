use crate::models::NewTodo;
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
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
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>"Rustploy Todo"</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                    <script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@main/bundles/datastar.js"></script>
                </head>
                <body class="bg-white text-gray-900" signals={ title: "" }>
                    <main class="mx-auto max-w-xl px-4 py-10">
                        <h1 class="text-2xl font-semibold">"Todo"</h1>
                        <p class="mb-6 mt-1 text-sm text-gray-500">"Reactive SQLite + SSE test"</p>
                        <div class="mb-6 flex gap-2">
                            <input bind:title="" placeholder="New todo" class="min-w-0 flex-1 rounded border px-3 py-2 outline-none focus:border-blue-500" />
                            <button type="button" on:click={Self::create} class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700">"+ Add"</button>
                        </div>
                        <div class="divide-y rounded border">
                            @for todo in load_todos(pool.clone()).await["todos"] {
                                <div class="flex items-center gap-2 p-3">
                                    @if todo.done {
                                        <span class="flex-1 text-gray-400 line-through">{&todo.title}</span>
                                    } @else {
                                        <span class="flex-1">{&todo.title}</span>
                                    }
                                    <button type="button" on:click={Self::toggle(todo.id)} class="rounded border px-2 py-1 text-sm" title="Mark complete">"✓"</button>
                                    <button type="button" on:click={Self::delete(todo.id)} class="px-2 py-1 text-sm text-red-600">"Delete"</button>
                                </div>
                            }
                        </div>
                    </main>
                </body>
            </html>
        }
    }

    #[post]
    async fn create(&self, Json(todo): Json<NewTodo>) -> StatusCode {
        if !todo.title.trim().is_empty() {
            let _ = sqlx::query("INSERT INTO todos(title) VALUES (?)")
                .bind(todo.title.trim())
                .execute(&self.pool)
                .await;
        }
        StatusCode::NO_CONTENT
    }

    #[post("/{id}/toggle")]
    async fn toggle(&self, Path(id): Path<i64>) -> StatusCode {
        let _ = sqlx::query("UPDATE todos SET done = NOT done WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
        StatusCode::NO_CONTENT
    }

    #[post("/{id}/delete")]
    async fn delete(&self, Path(id): Path<i64>) -> StatusCode {
        let _ = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
        StatusCode::NO_CONTENT
    }
}

async fn load_todos(pool: SqlitePool) -> Vec<crate::models::Todo> {
    sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
}
