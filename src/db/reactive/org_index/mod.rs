use sqlx::Row;
use std::collections::HashSet;
use std::sync::{OnceLock, RwLock};

/// Names of tables that carry an `organization_id` column, computed once at
/// startup. Lets the CDC bridge skip the per-event row lookup for tables
/// that can never be org-scoped.
static ORG_TABLES: OnceLock<RwLock<HashSet<String>>> = OnceLock::new();

pub(super) async fn refresh(pool: &sqlx::SqlitePool) {
    let tables = match sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table'")
        .fetch_all(pool)
        .await
    {
        Ok(tables) => tables,
        Err(_) => return,
    };
    let mut org_tables = HashSet::new();
    for table in tables {
        let Ok(name) = table.try_get::<String, _>(0) else {
            continue;
        };
        let info = sqlx::query(sqlx::AssertSqlSafe(format!(
            "PRAGMA table_info(\"{name}\")"
        )))
        .fetch_all(pool)
        .await;
        if info.is_ok_and(|info| {
            info.iter().any(|column| {
                column
                    .try_get::<String, _>(1)
                    .is_ok_and(|column| column == "organization_id")
            })
        }) {
            org_tables.insert(name);
        }
    }
    let slot = ORG_TABLES.get_or_init(|| RwLock::new(HashSet::new()));
    if let Ok(mut current) = slot.write() {
        *current = org_tables;
    }
}

pub(super) fn is_org_scoped(table: &str) -> bool {
    ORG_TABLES
        .get()
        .and_then(|tables| tables.read().ok())
        .is_some_and(|tables| tables.contains(table))
}
