pub mod controller;
pub mod db;
pub mod models;

use sqlx::SqlitePool;
use std::sync::OnceLock;

pub static POOL: OnceLock<SqlitePool> = OnceLock::new();
