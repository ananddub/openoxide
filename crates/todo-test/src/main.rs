use std::{convert::Infallible, str::FromStr, time::Duration};

use axum::{
    Form, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response, Sse, sse::Event},
    routing::{get, post},
};
use html_macro::html;
use html_rt::Markup;
use serde::Deserialize;
use sqlx::{
    FromRow, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(FromRow)]
struct Todo {
    id: i64,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("TODO_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://crates/todo-test/todo-test.sqlite3".into());
    let options = SqliteConnectOptions::from_str(&database_url)
        .expect("invalid TODO_DATABASE_URL")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
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

    let app = Router::new()
        .route("/", get(index))
        .route("/todos", post(create_todo))
        .route("/todos/{id}/toggle", post(toggle_todo))
        .route("/todos/{id}/delete", post(delete_todo))
        .route("/_rustploy/html/events/{session}", get(html_events))
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100")
        .await
        .expect("failed to bind port 3100");
    println!("Todo test app: http://127.0.0.1:3100");
    axum::serve(listener, app).await.expect("server failed");
}

async fn index(State(state): State<AppState>) -> Markup {
    let pool = state.pool.clone();
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
                <p>"Do browser tabs kholo. Ek tab me change karo; todos slot dono tabs me patch hoga."</p>
                <form method="post" action="/todos">
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
                        <form method="post" action={format!("/todos/{}/toggle", todo.id)}>
                            <button type="submit">"Toggle"</button>
                        </form>
                        <form method="post" action={format!("/todos/{}/delete", todo.id)}>
                            <button type="submit">"Delete"</button>
                        </form>
                    </div>
                }
            </body>
        </html>
    }
}

async fn load_todos(pool: SqlitePool) -> Vec<Todo> {
    tokio::time::sleep(Duration::from_millis(350)).await;
    sqlx::query_as::<_, Todo>("SELECT id, title, done FROM todos ORDER BY id DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
}

async fn create_todo(
    State(state): State<AppState>,
    Form(todo): Form<NewTodo>,
) -> impl IntoResponse {
    if !todo.title.trim().is_empty() {
        let _ = sqlx::query("INSERT INTO todos(title) VALUES (?)")
            .bind(todo.title.trim())
            .execute(&state.pool)
            .await;
        html_rt::publish_table_changes(vec!["todos".into()]);
    }
    redirect_home()
}

async fn toggle_todo(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let _ = sqlx::query("UPDATE todos SET done = NOT done WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await;
    html_rt::publish_table_changes(vec!["todos".into()]);
    redirect_home()
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let _ = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await;
    html_rt::publish_table_changes(vec!["todos".into()]);
    redirect_home()
}

fn redirect_home() -> impl IntoResponse {
    (StatusCode::SEE_OTHER, [(axum::http::header::LOCATION, "/")])
}

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
