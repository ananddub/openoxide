use std::sync::Arc;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

use sqlite_realtime::{RealtimeHandle, RealtimeOp, RealtimeRawEvent};

async fn attached_handle() -> (sqlx::SqlitePool, Arc<RealtimeHandle>) {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .expect("invalid url")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("failed to open in-memory sqlite");

    let mut conn = pool.acquire().await.expect("acquire");
    let mut raw = conn.lock_handle().await.expect("lock handle");
    let ptr = raw.as_raw_handle().as_ptr() as *mut std::ffi::c_void;
    let handle = RealtimeHandle::attach(ptr).expect("attach");
    drop(raw);
    drop(conn);
    (pool, handle)
}

async fn next_event(
    rx: &mut tokio::sync::broadcast::Receiver<RealtimeRawEvent>,
) -> RealtimeRawEvent {
    tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("timed out waiting for CDC event")
        .expect("broadcast closed")
}

#[tokio::test]
async fn cdc_events_flow_for_insert_update_delete() {
    let (pool, handle) = attached_handle().await;
    sqlx::query("CREATE TABLE users(id INTEGER PRIMARY KEY, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    let mut rx = handle.subscribe();

    sqlx::query("INSERT INTO users(id, name) VALUES (1, 'alice')")
        .execute(&pool)
        .await
        .unwrap();
    let ev = next_event(&mut rx).await;
    assert_eq!(ev.op, RealtimeOp::Insert);
    assert_eq!(ev.table, "users");
    assert_eq!(ev.rowid, 1);

    sqlx::query("UPDATE users SET name = 'bob' WHERE id = 1")
        .execute(&pool)
        .await
        .unwrap();
    let ev = next_event(&mut rx).await;
    assert_eq!(ev.op, RealtimeOp::Update);
    assert_eq!(ev.rowid, 1);

    sqlx::query("DELETE FROM users WHERE id = 1")
        .execute(&pool)
        .await
        .unwrap();
    let ev = next_event(&mut rx).await;
    assert_eq!(ev.op, RealtimeOp::Delete);
    assert_eq!(ev.rowid, 1);
}

#[tokio::test]
async fn rolled_back_transaction_produces_no_events() {
    let (pool, handle) = attached_handle().await;
    sqlx::query("CREATE TABLE tasks(id INTEGER PRIMARY KEY, title TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    let mut rx = handle.subscribe();

    let mut conn = pool.acquire().await.unwrap();
    sqlx::query("BEGIN").execute(&mut *conn).await.unwrap();
    sqlx::query("INSERT INTO tasks(id, title) VALUES (1, 'ghost')")
        .execute(&mut *conn)
        .await
        .unwrap();
    sqlx::query("ROLLBACK").execute(&mut *conn).await.unwrap();

    assert!(
        tokio::time::timeout(Duration::from_millis(300), rx.recv())
            .await
            .is_err(),
        "rolled-back write must not emit a CDC event"
    );
}

#[tokio::test]
async fn attach_if_needed_reuses_live_handle() {
    let (pool, first) = attached_handle().await;
    let mut conn = pool.acquire().await.unwrap();
    let mut raw = conn.lock_handle().await.unwrap();
    let ptr = raw.as_raw_handle().as_ptr() as *mut std::ffi::c_void;
    let second =
        RealtimeHandle::attach_if_needed(ptr).expect("idempotent re-attach must reuse the handle");
    assert_eq!(first.db_ptr(), second.db_ptr());
}
