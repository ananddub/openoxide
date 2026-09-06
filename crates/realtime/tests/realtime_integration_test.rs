use std::sync::Arc;
use std::time::{Duration, Instant};
use futures_util::StreamExt;
use sqlx::{sqlite::SqlitePoolOptions, FromRow};

use sqlite_realtime::{
    analyze_query_tables, LiveQueryRegistry, RealtimeHandle, RealtimePool, RealtimeTypedExt,
};

#[derive(Debug, Clone, PartialEq, FromRow)]
struct UserOrderJoin {
    user_name: String,
    order_id: i64,
    amount: f64,
}

#[tokio::test]
async fn test_explain_analyzer_joins_and_quotes() {
    // 1. Setup in-memory sqlite connection with raw handle
    let mut raw_db: *mut std::ffi::c_void = std::ptr::null_mut();
    let rc = unsafe {
        sqlite3_sys_test_open(
            b":memory:\0".as_ptr() as *const std::ffi::c_char,
            &mut raw_db as *mut *mut std::ffi::c_void as *mut *mut sqlite3_sys::Sqlite3,
        )
    };
    assert_eq!(rc, 0, "Failed to open sqlite3 in-memory db");

    // Execute schema
    let schema = "
        CREATE TABLE users(id INTEGER PRIMARY KEY, name TEXT);
        CREATE TABLE orders(id INTEGER PRIMARY KEY, user_id INT, amount REAL);
        CREATE TABLE items(id INTEGER PRIMARY KEY, order_id INT, title TEXT);
        CREATE TABLE \"audit log\"(id INTEGER PRIMARY KEY, msg TEXT);
    ";
    unsafe {
        let c_schema = std::ffi::CString::new(schema).unwrap();
        sqlite3_sys::sqlite3_exec(
            raw_db as *mut sqlite3_sys::Sqlite3,
            c_schema.as_ptr(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }

    // 2. Test 2-Table JOIN with aliases (u, o)
    let query1 = "SELECT u.name, o.amount FROM users u JOIN orders o ON u.id = o.user_id";
    let deps1 = analyze_query_tables(raw_db, query1);
    assert!(deps1.contains("users"), "Should depend on users: {:?}", deps1);
    assert!(deps1.contains("orders"), "Should depend on orders: {:?}", deps1);
    assert_eq!(deps1.len(), 2, "Should only have 2 tables: {:?}", deps1);

    // 3. Test 3-Table Complex JOIN
    let query2 = "SELECT u.name, o.amount, i.title FROM users u JOIN orders o ON u.id = o.user_id JOIN items i ON o.id = i.order_id WHERE u.name = 'Alice'";
    let deps2 = analyze_query_tables(raw_db, query2);
    assert!(deps2.contains("users"));
    assert!(deps2.contains("orders"));
    assert!(deps2.contains("items"));
    assert_eq!(deps2.len(), 3);

    // 4. Test Quoted Table Identifiers
    let query3 = "SELECT * FROM \"audit log\" WHERE id > 10";
    let deps3 = analyze_query_tables(raw_db, query3);
    assert!(deps3.contains("audit log"), "Quoted identifier should be extracted: {:?}", deps3);

    // Clean up
    unsafe {
        sqlite3_sys::sqlite3_close(raw_db as *mut sqlite3_sys::Sqlite3);
    }
}

#[tokio::test]
async fn test_registry_dependency_routing_and_memory_cleanup() {
    let registry = LiveQueryRegistry::new();

    // Mock dummy tables registration without SQLite pointer
    // Or with a mock address
    let mut raw_db: *mut std::ffi::c_void = std::ptr::null_mut();
    unsafe {
        sqlite3_sys_test_open(
            b":memory:\0".as_ptr() as *const std::ffi::c_char,
            &mut raw_db as *mut *mut std::ffi::c_void as *mut *mut sqlite3_sys::Sqlite3,
        );
        let c_schema = std::ffi::CString::new("
            CREATE TABLE users(id INT);
            CREATE TABLE orders(id INT);
            CREATE TABLE billing(id INT);
        ").unwrap();
        sqlite3_sys::sqlite3_exec(
            raw_db as *mut sqlite3_sys::Sqlite3,
            c_schema.as_ptr(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }

    let (sub1, deps1) = registry.register(raw_db as usize, "SELECT * FROM users JOIN orders").await;
    let (sub2, deps2) = registry.register(raw_db as usize, "SELECT * FROM billing").await;

    assert_eq!(registry.active_count().await, 2);
    assert!(deps1.contains("users") && deps1.contains("orders"));
    assert!(deps2.contains("billing"));

    // Check O(1) affected queries routing
    assert!(registry.is_affected("orders", sub1).await);
    assert!(registry.is_affected("users", sub1).await);
    assert!(!registry.is_affected("billing", sub1).await);

    assert!(registry.is_affected("billing", sub2).await);
    assert!(!registry.is_affected("users", sub2).await);

    // Unregister sub1 (Simulating Stream Drop) -> Check Memory Leak is Prevented
    registry.unregister(sub1).await;
    assert_eq!(registry.active_count().await, 1);
    assert!(!registry.is_affected("orders", sub1).await);
    assert!(!registry.is_affected("users", sub1).await);

    registry.unregister(sub2).await;
    assert_eq!(registry.active_count().await, 0);

    unsafe {
        sqlite3_sys::sqlite3_close(raw_db as *mut sqlite3_sys::Sqlite3);
    }
}

#[tokio::test]
async fn test_full_live_join_reactive_stream_with_sqlite_pool() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory pool");

    // Initialize Schema
    sqlx::query("
        CREATE TABLE users(id INTEGER PRIMARY KEY, name TEXT);
        CREATE TABLE orders(id INTEGER PRIMARY KEY, user_id INT, amount REAL);
        CREATE TABLE unrelated(id INTEGER PRIMARY KEY, val TEXT);
    ")
    .execute(&pool)
    .await
    .unwrap();

    // Attach realtime handle to connection
    let mut conn = pool.acquire().await.unwrap();
    let mut raw_handle = conn.lock_handle().await.unwrap();
    let raw_ptr = raw_handle.as_raw_handle().as_ptr() as *mut std::ffi::c_void;
    let handle = RealtimeHandle::attach_if_needed(raw_ptr).expect("Failed to attach RealtimeHandle");
    drop(raw_handle);
    drop(conn);

    let registry = Arc::new(LiveQueryRegistry::new());

    // Populate initial data
    sqlx::query("INSERT INTO users VALUES(1, 'Alice');")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO orders VALUES(101, 1, 99.50);")
        .execute(&pool)
        .await
        .unwrap();

    // Subscribe to Live JOIN Query
    let join_sql = "
        SELECT u.name AS user_name, o.id AS order_id, o.amount AS amount
        FROM users u
        JOIN orders o ON u.id = o.user_id
        ORDER BY o.id ASC
    ";

    let mut stream = pool
        .watch_query_typed::<UserOrderJoin>(&handle, registry.clone(), join_sql)
        .await
        .expect("Failed to watch live JOIN query");

    // 1. Initial Snapshot
    let initial = tokio::time::timeout(Duration::from_millis(500), stream.next())
        .await
        .expect("Timeout on initial snapshot")
        .expect("Stream closed");
    assert_eq!(initial.len(), 1);
    assert_eq!(initial[0].user_name, "Alice");
    assert_eq!(initial[0].amount, 99.50);

    // 2. Trigger mutation on TABLE: orders (INSERT)
    let start = Instant::now();
    sqlx::query("INSERT INTO orders VALUES(102, 1, 150.00);")
        .execute(&pool)
        .await
        .unwrap();

    let update1 = tokio::time::timeout(Duration::from_millis(1000), stream.next())
        .await
        .expect("Timeout waiting for orders change")
        .expect("Stream closed");
    let latency_us = start.elapsed().as_micros();
    println!("⚡ Order mutation JOIN re-evaluation latency: {} µs", latency_us);
    assert_eq!(update1.len(), 2);
    assert_eq!(update1[1].order_id, 102);
    assert_eq!(update1[1].amount, 150.00);

    // 3. Trigger mutation on TABLE: users (UPDATE user name)
    let start2 = Instant::now();
    sqlx::query("UPDATE users SET name = 'Alice In Wonderland' WHERE id = 1;")
        .execute(&pool)
        .await
        .unwrap();

    let update2 = tokio::time::timeout(Duration::from_millis(1000), stream.next())
        .await
        .expect("Timeout waiting for users change")
        .expect("Stream closed");
    let latency_us2 = start2.elapsed().as_micros();
    println!("⚡ User name mutation JOIN re-evaluation latency: {} µs", latency_us2);
    assert_eq!(update2.len(), 2);
    assert_eq!(update2[0].user_name, "Alice In Wonderland");
    assert_eq!(update2[1].user_name, "Alice In Wonderland");

    // 4. Test Unrelated Table Write (Does NOT trigger redundant updates)
    sqlx::query("INSERT INTO unrelated VALUES(1, 'nothing');")
        .execute(&pool)
        .await
        .unwrap();

    let non_event = tokio::time::timeout(Duration::from_millis(150), stream.next()).await;
    assert!(non_event.is_err(), "Unrelated table write should NOT trigger stream!");

    println!("✅ All Live JOIN Realtime tests PASSED with zero spurious triggers!");
}

#[tokio::test]
async fn test_burst_write_throughput_and_debounce_efficiency() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE metrics(id INTEGER PRIMARY KEY, cpu REAL);")
        .execute(&pool)
        .await
        .unwrap();

    let mut conn = pool.acquire().await.unwrap();
    let mut raw_handle = conn.lock_handle().await.unwrap();
    let raw_ptr = raw_handle.as_raw_handle().as_ptr() as *mut std::ffi::c_void;
    let handle = RealtimeHandle::attach_if_needed(raw_ptr).unwrap();
    drop(raw_handle);
    drop(conn);

    let registry = Arc::new(LiveQueryRegistry::new());

    #[derive(Debug, Clone, PartialEq, FromRow)]
    struct Metric {
        id: i64,
        cpu: f64,
    }

    let mut stream = pool
        .watch_query_typed::<Metric>(&handle, registry.clone(), "SELECT id, cpu FROM metrics")
        .await
        .unwrap();

    // Consume initial
    let _ = stream.next().await.unwrap();

    // Fire 50 rapid writes in a 10ms burst
    let start = Instant::now();
    for i in 1..=50 {
        sqlx::query("INSERT INTO metrics VALUES(?, ?)")
            .bind(i)
            .bind(i as f64 * 1.5)
            .execute(&pool)
            .await
            .unwrap();
    }
    let write_duration = start.elapsed();
    println!("⚡ 50 Rapid Writes completed in: {:?}", write_duration);

    // Collect stream updates
    // Because of leading (immediate) + trailing (50ms) debounce, 50 writes result in ONLY 1 or 2 evaluations!
    let mut eval_count = 0;
    let mut last_row_count = 0;

    let deadline = Instant::now() + Duration::from_millis(300);
    while Instant::now() < deadline {
        if let Ok(Some(rows)) = tokio::time::timeout(Duration::from_millis(80), stream.next()).await {
            eval_count += 1;
            last_row_count = rows.len();
        } else {
            break;
        }
    }

    println!("📊 Debounce Efficiency: 50 raw mutations -> only {} DB re-evaluations (saved {}% DB load)!", 
        eval_count, 
        (50 - eval_count) * 100 / 50
    );
    println!("📊 Final State Row Count: {} (expected 50)", last_row_count);

    assert_eq!(last_row_count, 50, "Subscriber must have all 50 items after burst");
    assert!(eval_count <= 3, "Debouncer should have coalesced 50 writes into <= 3 executions");
}

#[tokio::test]
async fn test_realtime_pool_clean_1_line_api() {
    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE tasks(id INTEGER PRIMARY KEY, title TEXT, done INT);")
        .execute(&sqlite_pool)
        .await
        .unwrap();

    // 🎯 1. Wrap once at startup into RealtimePool
    let realtime_pool = RealtimePool::new(sqlite_pool).await.unwrap();

    #[derive(Debug, Clone, PartialEq, FromRow)]
    struct Task {
        id: i64,
        title: String,
        done: i64,
    }

    // 🎯 2. Clean 1-Line watch_query without passing any handles or registries!
    let mut stream = realtime_pool
        .watch_query::<Task>("SELECT id, title, done FROM tasks WHERE done = 0")
        .await
        .unwrap();

    // Initial snapshot (empty)
    let initial = stream.next().await.unwrap();
    assert_eq!(initial.len(), 0);

    // Insert task
    sqlx::query("INSERT INTO tasks VALUES(1, 'Buy Milk', 0)")
        .execute(realtime_pool.inner())
        .await
        .unwrap();

    let update = stream.next().await.unwrap();
    assert_eq!(update.len(), 1);
    assert_eq!(update[0].title, "Buy Milk");

    println!("✅ RealtimePool 1-Line API works flawlessly without passing handles/registries!");
}

#[tokio::test]
async fn test_duplicate_data_suppression_via_8_byte_hash() {
    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE users(id INTEGER PRIMARY KEY, name TEXT, age INT);")
        .execute(&sqlite_pool)
        .await
        .unwrap();

    let realtime_pool = RealtimePool::new(sqlite_pool).await.unwrap();

    #[derive(Debug, Clone, PartialEq, FromRow)]
    struct User {
        id: i64,
        name: String,
    }

    // 🎯 Watch ONLY id and name (age is ignored in this query)
    let mut stream = realtime_pool
        .watch_query::<User>("SELECT id, name FROM users WHERE id = 1")
        .await
        .unwrap();

    // Initial snapshot (empty)
    let initial = stream.next().await.unwrap();
    assert_eq!(initial.len(), 0);

    // 1. First Mutation: Insert User 1 -> MUST emit!
    sqlx::query("INSERT INTO users VALUES(1, 'Alice', 25)")
        .execute(realtime_pool.inner())
        .await
        .unwrap();

    let update1 = stream.next().await.unwrap();
    assert_eq!(update1.len(), 1);
    assert_eq!(update1[0].name, "Alice");

    // 2. Second Mutation: UPDATE age = 26 (name and id DID NOT CHANGE!)
    // The query 'SELECT id, name FROM users' result is 100% IDENTICAL to previous!
    sqlx::query("UPDATE users SET age = 26 WHERE id = 1")
        .execute(realtime_pool.inner())
        .await
        .unwrap();

    // 🎯 Verify stream does NOT emit duplicate unchanged data!
    let non_event = tokio::time::timeout(Duration::from_millis(150), stream.next()).await;
    assert!(
        non_event.is_err(),
        "Stream must NOT emit duplicate data when query result hash is unchanged!"
    );

    println!("✅ Duplicate data emission suppressed via 8-byte hash (Zero bandwidth waste)!");
}

// Minimal FFI helper for unit test
mod sqlite3_sys {
    use std::ffi::{c_char, c_int, c_void};
    pub enum Sqlite3 {}
    unsafe extern "C" {
        pub fn sqlite3_open(filename: *const c_char, ppDb: *mut *mut Sqlite3) -> c_int;
        pub fn sqlite3_close(db: *mut Sqlite3) -> c_int;
        pub fn sqlite3_exec(
            db: *mut Sqlite3,
            sql: *const c_char,
            callback: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut *mut c_char, *mut *mut c_char) -> c_int>,
            arg: *mut c_void,
            errmsg: *mut *mut c_char,
        ) -> c_int;
    }
}

unsafe fn sqlite3_sys_test_open(
    filename: *const std::ffi::c_char,
    pp_db: *mut *mut sqlite3_sys::Sqlite3,
) -> i32 {
    unsafe { sqlite3_sys::sqlite3_open(filename, pp_db) }
}
