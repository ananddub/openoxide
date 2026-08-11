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
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>"Rustploy Todo"</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                </head>
                <body class="min-h-screen bg-slate-950 text-slate-100">
                    <main class="mx-auto max-w-2xl px-4 py-12">
                        <div class="mb-8">
                            <p class="mb-2 text-sm font-medium uppercase tracking-widest text-cyan-400">"Rustploy demo"</p>
                            <h1 class="text-4xl font-bold tracking-tight">"Reactive Todo"</h1>
                            <p class="mt-3 text-slate-400">"SQLite changes automatically patch this list over SSE."</p>
                        </div>
                        <form method="post" action={Self::__PATH_create} class="mb-8 flex gap-3 rounded-2xl border border-slate-800 bg-slate-900 p-3 shadow-xl">
                            <input name="title" required placeholder="What needs to be done?" class="min-w-0 flex-1 rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 outline-none ring-cyan-500 placeholder:text-slate-600 focus:ring-2" />
                            <button class="rounded-xl bg-cyan-400 px-5 font-semibold text-slate-950 transition hover:bg-cyan-300">"Add todo"</button>
                        </form>
                        <div class="space-y-3">
                            @for todo in load_todos(pool.clone()).await["todos"] {
                                <div class="flex items-center gap-3 rounded-2xl border border-slate-800 bg-slate-900 px-4 py-4 shadow-lg">
                                    @if todo.done {
                                        <span class="flex-1 text-slate-500 line-through">{&todo.title}</span>
                                        <span class="rounded-full bg-emerald-400/10 px-3 py-1 text-xs font-medium text-emerald-400">"done"</span>
                                    } @else {
                                        <span class="flex-1 text-slate-100">{&todo.title}</span>
                                        <span class="rounded-full bg-amber-400/10 px-3 py-1 text-xs font-medium text-amber-400">"open"</span>
                                    }
                                    <form method="post" action={Self::__PATH_toggle.replace("{id}", &todo.id.to_string())}><button class="rounded-lg border border-slate-700 px-3 py-2 text-sm text-slate-300 hover:bg-slate-800">"Toggle"</button></form>
                                    <form method="post" action={Self::__PATH_delete.replace("{id}", &todo.id.to_string())}><button class="rounded-lg px-3 py-2 text-sm text-rose-400 hover:bg-rose-400/10">"Delete"</button></form>
                                </div>
                            }
                        </div>
                    </main>
                </body>
            </html>
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
