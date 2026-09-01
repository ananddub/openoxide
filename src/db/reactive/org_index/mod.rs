use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use sqlx::sqlite::{PreupdateHookResult, SqliteOperation};
use sqlx::{Decode, Row};

/// Resolves the `organization_id` column index for each table so the
/// hook can route changes to an organization without issuing any schema
/// reads on the hot path.
pub struct OrgColumnIndex {
    columns: RwLock<HashMap<String, Option<i32>>>,
}

impl OrgColumnIndex {
    pub fn global() -> &'static Self {
        static INDEX: OnceLock<OrgColumnIndex> = OnceLock::new();
        INDEX.get_or_init(|| Self {
            columns: RwLock::new(HashMap::new()),
        })
    }

    pub async fn refresh(pool: &sqlx::SqlitePool) {
        let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table'")
            .fetch_all(pool)
            .await;
        let Ok(tables) = tables else {
            return;
        };
        let mut columns = HashMap::new();
        for table in tables {
            let Ok(name) = table.try_get::<String, _>(0) else {
                continue;
            };
            let info = sqlx::query(sqlx::AssertSqlSafe(format!(
                "PRAGMA table_info(\"{name}\")"
            )))
            .fetch_all(pool)
            .await;
            let Ok(info) = info else {
                continue;
            };
            let index = info
                .iter()
                .enumerate()
                .find_map(|(index, column)| {
                    column
                        .try_get::<String, _>(1)
                        .is_ok_and(|column| column == "organization_id")
                        .then_some(index as i32)
                });
            columns.insert(name, index);
        }
        if let Ok(mut self_columns) = Self::global().columns.write() {
            *self_columns = columns;
        }
    }

    pub fn organization_id(&self, result: &PreupdateHookResult<'_>) -> Option<i64> {
        let index = *self.columns.read().ok()?.get(result.table)?;
        let index = index?;
        let value = match result.operation {
            SqliteOperation::Delete => result.get_old_column_value(index),
            _ => result.get_new_column_value(index),
        };
        Option::<i64>::decode(value.ok()?).ok().flatten()
    }
}
