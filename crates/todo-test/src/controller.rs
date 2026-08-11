use crate::models::NewTodo;
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use html_macro::html;
use html_rt::Markup;
use sqlx::SqlitePool;
use std::sync::Arc;

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
    async fn page(self: Arc<Self>) -> Markup {
        html! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>"Rustploy Todo"</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                    <script type="module" src="https://cdn.jsdelivr.net/gh/starfederation/datastar@main/bundles/datastar.js"></script>
                    <script>{html_rt::PreEscaped(NAV_SCRIPT)}</script>
                </head>
                <body class="bg-white text-gray-900" signals={ title: "" }>
                    <div id="app">
                        {self.navbar("home")}
                    <main class="mx-auto max-w-xl px-4 py-10">
                        <h1 class="text-2xl font-semibold">"Todo"</h1>
                        <p class="mb-6 mt-1 text-sm text-gray-500">"Reactive SQLite + SSE test"</p>
                        <div class="mb-6 flex gap-2">
                            <input bind:title="" placeholder="New todo" class="min-w-0 flex-1 rounded border px-3 py-2 outline-none focus:border-blue-500" />
                            <button type="button" on:click={self.create} class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700">"+ Add"</button>
                        </div>
                        <div class="divide-y rounded border">
                            @for todo in self.load_todos().await["todos"] {
                                <div class="flex items-center gap-2 p-3">
                                    @if todo.done {
                                        <span class="flex-1 text-gray-400 line-through">{&todo.title}</span>
                                    } @else {
                                        <span class="flex-1">{&todo.title}</span>
                                    }
                                    @if todo.done {
                                        <input type="checkbox" checked on:change={self.toggle(todo.id)} class="h-4 w-4 cursor-pointer" title="Mark undone" />
                                    } @else {
                                        <input type="checkbox" on:change={self.toggle(todo.id)} class="h-4 w-4 cursor-pointer" title="Mark done" />
                                    }
                                    <button type="button" on:click={self.delete(todo.id)} class="px-2 py-1 text-sm text-red-600">"Delete"</button>
                                </div>
                            }
                        </div>
                    </main>
                    </div>
                </body>
            </html>
        }
    }

    #[get("/about")]
    async fn about(&self) -> Markup {
        html! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>"About · Rustploy Todo"</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                    <script>{html_rt::PreEscaped(NAV_SCRIPT)}</script>
                </head>
                <body class="bg-white text-gray-900">
                    <div id="app">
                        {self.navbar("about")}
                        <main class="mx-auto max-w-xl px-4 py-10">
                            <h1 class="text-2xl font-semibold">"About"</h1>
                            <p class="mt-3 text-gray-600">"This page tests flicker-free navigation with real auto_route endpoints."</p>
                        </main>
                    </div>
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

    async fn load_todos(&self) -> Vec<crate::models::Todo> {
        sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
    }

    fn navbar(&self, active: &str) -> Markup {
        html! {
            <nav class="border-b">
                <div class="mx-auto flex max-w-xl gap-5 px-4 py-3 text-sm">
                    <a data-nav href={self.page} class={if active == "home" { "font-semibold text-blue-600" } else { "text-gray-600" }} >"Home"</a>
                    <a data-nav href={self.about} class={if active == "about" { "font-semibold text-blue-600" } else { "text-gray-600" }} >"About"</a>
                </div>
            </nav>
        }
    }
}

const NAV_SCRIPT: &str = r#"
document.addEventListener('click', async event => {
  const link = event.target.closest('a[data-nav]');
  if (!link || event.ctrlKey || event.metaKey || event.shiftKey) return;
  event.preventDefault();
  const response = await fetch(link.href, {headers: {'x-rustploy-navigation': 'true'}});
  const documentNext = new DOMParser().parseFromString(await response.text(), 'text/html');
  const appNext = documentNext.querySelector('#app');
  if (appNext) {
    document.querySelector('#app').replaceWith(appNext);
    history.pushState({}, '', link.href);
  }
});
addEventListener('popstate', () => location.reload());
"#;
