use std::{path::Path, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::database::UpsertDatabaseMountDto,
    db::models::mounts::Mount,
    repository::MountRepository,
    services::database::{DatabaseKind, DatabaseService},
};

pub struct DatabaseMountService {
    databases: Arc<DatabaseService>,
    mounts: Arc<MountRepository>,
}

#[singleton]
impl DatabaseMountService {
    fn new(databases: Arc<DatabaseService>, mounts: Arc<MountRepository>) -> Self {
        Self { databases, mounts }
    }

    async fn ensure_database(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.databases.get_by_id(kind, id).await.map(|_| ())
    }

    pub async fn list(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<Vec<Mount>> {
        self.ensure_database(kind, id).await?;
        self.mounts.fetch_for_database_kind(kind, id).await
    }

    pub async fn create(
        &self,
        kind: DatabaseKind,
        id: i64,
        input: UpsertDatabaseMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_database(kind, id).await?;
        let input = normalize_mount(input).map_err(sqlx::Error::Protocol)?;
        self.mounts
            .create_for_database(
                kind,
                id,
                input.mount_type.as_str(),
                input.host_path.as_deref(),
                input.volume_name.as_deref(),
                input.file_path.as_deref(),
                input.content.as_deref(),
                &input.mount_path,
            )
            .await
    }

    pub async fn update(
        &self,
        kind: DatabaseKind,
        database_id: i64,
        id: i64,
        input: UpsertDatabaseMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_database(kind, database_id).await?;
        let input = normalize_mount(input).map_err(sqlx::Error::Protocol)?;
        self.mounts
            .update_for_database(
                kind,
                database_id,
                id,
                input.mount_type.as_str(),
                input.host_path.as_deref(),
                input.volume_name.as_deref(),
                input.file_path.as_deref(),
                input.content.as_deref(),
                &input.mount_path,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(
        &self,
        kind: DatabaseKind,
        database_id: i64,
        id: i64,
    ) -> sqlx::Result<bool> {
        self.ensure_database(kind, database_id).await?;
        self.mounts.delete_for_database(kind, database_id, id).await
    }
}

fn normalize_mount(mut input: UpsertDatabaseMountDto) -> Result<UpsertDatabaseMountDto, String> {
    input.mount_path = input.mount_path.trim().to_string();
    if !Path::new(&input.mount_path).is_absolute() {
        return Err("mount_path must be an absolute container path".into());
    }

    match input.mount_type {
        crate::utils::builder::spec::MountType::Bind => {
            let value = required_trimmed(input.host_path.as_deref(), "host_path")?;
            if !Path::new(&value).is_absolute() {
                return Err("host_path must be absolute for BIND mounts".into());
            }
            input.host_path = Some(value);
            input.volume_name = None;
            input.file_path = None;
            input.content = None;
        }
        crate::utils::builder::spec::MountType::Volume => {
            input.volume_name = Some(required_trimmed(
                input.volume_name.as_deref(),
                "volume_name",
            )?);
            input.host_path = None;
            input.file_path = None;
            input.content = None;
        }
        crate::utils::builder::spec::MountType::File => {
            input.file_path = Some(required_trimmed(input.file_path.as_deref(), "file_path")?);
            if input.content.is_none() {
                return Err("content is required for FILE mounts".into());
            }
            input.host_path = None;
            input.volume_name = None;
        }
    }
    Ok(input)
}

fn required_trimmed(value: Option<&str>, field: &str) -> Result<String, String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("{field} is required"))
}
