mod controller;
mod db;
mod models;
mod sse;

use sqlx::SqlitePool;
use std::sync::OnceLock;

pub static POOL: OnceLock<SqlitePool> = OnceLock::new();

#[tokio::main]
async fn main() {
    POOL.set(db::connect().await)
        .expect("pool initialized twice");
    let app = auto_route::routes()
        .await
        .expect("failed to build auto routes");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100")
        .await
        .unwrap();
    println!("Todo test app: http://127.0.0.1:3100/todo");
    axum::serve(listener, app).await.unwrap();
}
