use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct CreateTagDto {
    pub name: String,
    pub color: String,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct UpdateTagDto {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object, ts_rs::TS)]
pub struct TagDto {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub organization_id: i64,
    pub created_at: i64,
}

impl From<crate::db::models::tags::Tag> for TagDto {
    fn from(t: crate::db::models::tags::Tag) -> Self {
        Self {
            id: t.id.unwrap_or(0),
            name: t.name,
            color: t.color,
            organization_id: t.organization_id,
            created_at: t.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct AttachProjectTagDto {
    pub tag_id: i64,
}
