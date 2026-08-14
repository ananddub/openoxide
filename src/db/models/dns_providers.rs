#![allow(unused_attributes)]

use serde::{Deserialize, Serialize};
use sqlx_gen::SqlxGen;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow, SqlxGen)]
#[sqlx_gen(kind = "table", schema = "main", table = "dns_providers")]
pub struct DnsProvider {
    #[sqlx_gen(primary_key, sql_type = "INTEGER")]
    pub id: Option<i64>,
    #[sqlx_gen(sql_type = "TEXT")]
    pub name: String,
    #[sqlx_gen(sql_type = "TEXT")]
    pub provider_type: String,
    #[sqlx_gen(sql_type = "TEXT")]
    pub credentials_json: String,
    #[sqlx_gen(sql_type = "INTEGER")]
    pub organization_id: i64,
    #[sqlx_gen(sql_type = "INTEGER", column_default = "unixepoch()")]
    pub created_at: i64,
    #[sqlx_gen(sql_type = "INTEGER", column_default = "unixepoch()")]
    pub updated_at: i64,
}
