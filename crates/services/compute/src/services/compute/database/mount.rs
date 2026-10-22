use std::{path::Path, sync::Arc};

use auto_di::singleton;

use crate::{
    api::connect::generated::openoxide::api::v1::{CreateMountRequest, UpdateMountRequest},
    db::queries::FlashQueries,
    flash_gen::models::Mounts,
    flash_gen::mount::{MountCreateForDatabaseParams, MountUpdateForDatabaseParams},
    services::compute::database::{DatabaseKind, DatabaseService},
};

struct NormalizedMount {
    mount_type: crate::utils::builder::spec::MountType,
    host_path: Option<String>,
    volume_name: Option<String>,
    file_path: Option<String>,
    content: Option<String>,
    mount_path: String,
}

pub struct DatabaseMountService {
    databases: Arc<DatabaseService>,
    queries: Arc<FlashQueries>,
}

#[cfg(test)]
impl DatabaseMountService {
    pub(crate) fn for_tests(databases: Arc<DatabaseService>, queries: Arc<FlashQueries>) -> Self {
        Self { databases, queries }
    }
}

#[singleton]
impl DatabaseMountService {
    fn new(databases: Arc<DatabaseService>, queries: Arc<FlashQueries>) -> Self {
        Self { databases, queries }
    }

    async fn ensure_database(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.databases.get_by_id(kind, id).await.map(|_| ())
    }

    pub async fn list(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<Vec<Mounts>> {
        self.ensure_database(kind, id).await?;
        self.queries.mount_fetch_for_database(id).await
    }

    pub async fn create(
        &self,
        kind: DatabaseKind,
        id: i64,
        input: CreateMountRequest,
    ) -> sqlx::Result<Mounts> {
        self.ensure_database(kind, id).await?;
        let input = normalize_mount(
            &input.mount_type,
            input.host_path,
            input.volume_name,
            input.file_path,
            input.content,
            input.mount_path,
        )
        .map_err(sqlx::Error::Protocol)?;
        let params = MountCreateForDatabaseParams {
            mount_type: input.mount_type.as_str().into(),
            host_path: input.host_path.unwrap_or_default(),
            volume_name: input.volume_name.unwrap_or_default(),
            file_path: input.file_path.unwrap_or_default(),
            content: input.content.unwrap_or_default(),
            mount_path: input.mount_path,
            service_id: id,
        };
        self.queries.mount_create_for_database(&params).await
    }

    pub async fn update(
        &self,
        kind: DatabaseKind,
        database_id: i64,
        id: i64,
        input: UpdateMountRequest,
    ) -> sqlx::Result<Mounts> {
        self.ensure_database(kind, database_id).await?;
        let input = normalize_mount(
            &input.mount_type,
            input.host_path,
            input.volume_name,
            input.file_path,
            input.content,
            input.mount_path,
        )
        .map_err(sqlx::Error::Protocol)?;
        let params = MountUpdateForDatabaseParams {
            mount_type: input.mount_type.as_str().into(),
            host_path: input.host_path.unwrap_or_default(),
            volume_name: input.volume_name.unwrap_or_default(),
            file_path: input.file_path.unwrap_or_default(),
            content: input.content.unwrap_or_default(),
            mount_path: input.mount_path,
            id,
            service_id: database_id,
        };
        self.queries.mount_update_for_database(&params).await
    }

    pub async fn delete(
        &self,
        kind: DatabaseKind,
        database_id: i64,
        id: i64,
    ) -> sqlx::Result<bool> {
        self.ensure_database(kind, database_id).await?;
        match self.queries.mount_get_for_database(id, database_id).await {
            Ok(_) => {
                self.queries
                    .mount_delete_for_database(id, database_id)
                    .await?;
                Ok(true)
            }
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
    }
}

fn normalize_mount(
    mount_type: &str,
    host_path: Option<String>,
    volume_name: Option<String>,
    file_path: Option<String>,
    content: Option<String>,
    mount_path: String,
) -> Result<NormalizedMount, String> {
    let mount_type = crate::utils::builder::spec::MountType::from_str_opt(mount_type)
        .ok_or_else(|| format!("invalid mount type: {mount_type}"))?;
    let mut input = NormalizedMount {
        mount_type,
        host_path,
        volume_name,
        file_path,
        content,
        mount_path,
    };
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
