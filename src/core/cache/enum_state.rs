use crate::services::{
    application::ApplicationRecord, compose::ComposeRecord, database::DatabaseRecord,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Application(i64),
    Compose(i64),
    Database(i64),
}

#[derive(Debug, Clone)]
pub enum CacheEnum {
    Application(ApplicationRecord),
    Compose(ComposeRecord),
    Database(DatabaseRecord),
}
