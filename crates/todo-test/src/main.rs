use std::{
    convert::Infallible,
    str::FromStr,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

use auto_route::{controller, get};
use axum::{
    Form,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response, Sse, sse::Event},
};
use html_macro::html;
use html_rt::Markup;
use serde::Deserialize;
use sqlx::{
    FromRow, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

static POOL: OnceLock<SqlitePool> = OnceLock::new();

#[derive(FromRow)]
struct Todo {
    id: i64,
    title: String,
    done: bool,
}

#[derive(Deserialize, poem_openapi::Object)]
struct NewTodo {
    title: String,
}

struct TodoController {
    pool: SqlitePool,
}

#[controller("/todo")]
impl TodoController {
    fn new() -> Self {
        Self {
            pool: POOL
                .get()
                .expect("todo database is not initialized")
                .clone(),
        }
    }

    #[get]
    async fn page(&self) -> Markup {
        let pool = self.pool.clone();
        html! {
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>"Rustploy reactive Todo test"</title>
                    <style>{r#"
                        body{font-family:system-ui;max-width:700px;margin:3rem auto;padding:0 1rem}
                        form,.todo{display:flex;gap:.6rem;margin:.7rem 0}.todo span{flex:1}
                        .done{text-decoration:line-through;opacity:.55}button,input{padding:.65rem}
                    "#}</style>
                </head>
                <body>
                    <h1>"Reactive Todo"</h1>
                    <p>"Do tabs kholo. Ek tab me change karne par dusra tab SSE patch se update hoga."</p>
                    <form method="post" action={Self::__PATH_create}>
                        <input name="title" required placeholder="New todo" />
                        <button type="submit">"Add"</button>
                    </form>

                    @for todo in load_todos(pool.clone()).await["todos"] {
                        <div class="todo">
                            @if todo.done {
                                <span class="done">{&todo.title}</span>
                            } @else {
                                <span>{&todo.title}</span>
                            }
                            <form method="post" action={format!("/todo/{}/toggle", todo.id)}>
                                <button type="submit">"Toggle"</button>
                            </form>
                            <form method="post" action={format!("/todo/{}/delete", todo.id)}>
                                <button type="submit">"Delete"</button>
                            </form>
                        </div>
                    }
                </body>
            </html>
        }
    }

    #[post]
    async fn create(&self, Form(todo): Form<NewTodo>) -> impl IntoResponse {
        if !todo.title.trim().is_empty() {
            let _ = sqlx::query("INSERT INTO todos(title) VALUES (?)")
                .bind(todo.title.trim())
                .execute(&self.pool)
                .await;
        }
        redirect_page()
    }

    #[post("/{id}/toggle")]
    async fn toggle(&self, Path(id): Path<i64>) -> impl IntoResponse {
        let _ = sqlx::query("UPDATE todos SET done = NOT done WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
        redirect_page()
    }

    #[post("/{id}/delete")]
    async fn delete(&self, Path(id): Path<i64>) -> impl IntoResponse {
        let _ = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await;
        redirect_page()
    }
}

#[tokio::main]
async fn main() {
    let pool = init_database().await;
    POOL.set(pool)
        .unwrap_or_else(|_| panic!("pool initialized twice"));

    let app = auto_route::routes()
        .await
        .expect("failed to build auto routes");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100")
        .await
        .expect("failed to bind port 3100");
    println!("Todo test app: http://127.0.0.1:3100/todo");
    axum::serve(listener, app).await.expect("server failed");
}

async fn init_database() -> SqlitePool {
    let database_url = std::env::var("TODO_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://crates/todo-test/todo-test.sqlite3".into());
    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("invalid TODO_DATABASE_URL")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|connection, _| {
            Box::pin(async move {
                let mut handle = connection.lock_handle().await?;
                install_reactive_hooks(&mut handle);
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .expect("failed to open todo database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (\
         id INTEGER PRIMARY KEY AUTOINCREMENT,\
         title TEXT NOT NULL,\
         done INTEGER NOT NULL DEFAULT 0)",
    )
    .execute(&pool)
    .await
    .expect("failed to create todos table");
    pool
}

fn install_reactive_hooks(handle: &mut sqlx::sqlite::LockedSqliteHandle<'_>) {
    let pending = Arc::new(Mutex::new(Vec::<String>::new()));
    let writes = Arc::clone(&pending);
    handle.set_preupdate_hook(move |change| {
        if let Ok(mut tables) = writes.lock() {
            tables.push(change.table.to_owned());
        }
    });
    let commits = Arc::clone(&pending);
    handle.set_commit_hook(move || {
        if let Ok(mut tables) = commits.lock() {
            tables.sort_unstable();
            tables.dedup();
            html_rt::publish_table_changes(std::mem::take(&mut *tables));
        }
        true
    });
    handle.set_rollback_hook(move || {
        if let Ok(mut tables) = pending.lock() {
            tables.clear();
        }
    });
}

async fn load_todos(pool: SqlitePool) -> Vec<Todo> {
    tokio::time::sleep(Duration::from_millis(350)).await;
    sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
}

fn redirect_page() -> impl IntoResponse {
    (
        StatusCode::SEE_OTHER,
        [(axum::http::header::LOCATION, "/todo")],
    )
}

#[get("/_rustploy/html/events/{session}")]
async fn html_events(Path(session): Path<String>) -> Response {
    let Some(receiver) = html_rt::take_session(&session) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let stream = futures::stream::unfold(receiver, |mut receiver| async move {
        receiver.recv().await.map(|patch| {
            let payload = serde_json::json!({"slot": patch.slot, "html": patch.html});
            (
                Ok::<Event, Infallible>(Event::default().data(payload.to_string())),
                receiver,
            )
        })
    });
    Sse::new(stream).into_response()
}
