use std::{path::Path, sync::Arc};

use auto_di::singleton;

use crate::{
    api::connect::generated::openoxide::api::v1::ComposeMountRequest,
    db::queries::FlashQueries,
    flash_gen::compose_extra::{ComposeMountCreateParams, ComposeMountUpdateParams},
    flash_gen::models::Mounts,
};

struct NormalizedMount {
    mount_type: crate::utils::builder::spec::MountType,
    host_path: Option<String>,
    volume_name: Option<String>,
    file_path: Option<String>,
    content: Option<String>,
    mount_path: String,
}

pub struct ComposeMountService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ComposeMountService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    async fn ensure_compose(&self, id: i64) -> sqlx::Result<()> {
        let service = self.queries.get_service_by_i_d(id).await?;
        if service.service_type != "COMPOSE" {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn list(&self, compose_id: i64) -> sqlx::Result<Vec<Mounts>> {
        self.ensure_compose(compose_id).await?;
        self.queries.mount_fetch_for_compose(compose_id).await
    }

    pub async fn create(
        &self,
        compose_id: i64,
        input: ComposeMountRequest,
    ) -> sqlx::Result<Mounts> {
        self.ensure_compose(compose_id).await?;
        let input = normalize(
            &input.mount_type,
            input.host_path,
            input.volume_name,
            input.file_path,
            input.content,
            input.mount_path,
        )?;
        // Flash bind params for nullable columns are non-Option; "" is the
        // NULL encoding and is filtered back to None on read.
        self.queries
            .compose_mount_create(&ComposeMountCreateParams {
                mount_type: input.mount_type.as_str().to_string(),
                host_path: input.host_path.unwrap_or_default(),
                volume_name: input.volume_name.unwrap_or_default(),
                file_path: input.file_path.unwrap_or_default(),
                content: input.content.unwrap_or_default(),
                mount_path: input.mount_path,
                service_id: compose_id,
            })
            .await
    }

    pub async fn update(
        &self,
        compose_id: i64,
        id: i64,
        input: ComposeMountRequest,
    ) -> sqlx::Result<Mounts> {
        self.ensure_compose(compose_id).await?;
        let input = normalize(
            &input.mount_type,
            input.host_path,
            input.volume_name,
            input.file_path,
            input.content,
            input.mount_path,
        )?;
        // Old update_for_compose returned Option; :one RETURNING surfaces a
        // miss as RowNotFound, matching the old `.ok_or(RowNotFound)`.
        self.queries
            .compose_mount_update(&ComposeMountUpdateParams {
                mount_type: input.mount_type.as_str().to_string(),
                host_path: input.host_path.unwrap_or_default(),
                volume_name: input.volume_name.unwrap_or_default(),
                file_path: input.file_path.unwrap_or_default(),
                content: input.content.unwrap_or_default(),
                mount_path: input.mount_path,
                id,
                service_id: compose_id,
            })
            .await
    }

    pub async fn delete(&self, compose_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_compose(compose_id).await?;
        // Old delete_for_compose returned rows_affected == 1.
        Ok(self
            .queries
            .compose_mount_delete(id, compose_id)
            .await
            .is_ok())
    }
}

fn normalize(
    mount_type: &str,
    host_path: Option<String>,
    volume_name: Option<String>,
    file_path: Option<String>,
    content: Option<String>,
    mount_path: String,
) -> sqlx::Result<NormalizedMount> {
    let mount_type = crate::utils::builder::spec::MountType::from_str(mount_type)
        .ok_or_else(|| sqlx::Error::Protocol(format!("invalid mount type: {mount_type}")))?;
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
        return Err(sqlx::Error::Protocol("mount_path must be absolute".into()));
    }
    match input.mount_type {
        crate::utils::builder::spec::MountType::Bind => {
            let value = input
                .host_path
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| sqlx::Error::Protocol("host_path is required for BIND".into()))?;
            if !Path::new(value).is_absolute() {
                return Err(sqlx::Error::Protocol("host_path must be absolute".into()));
            }
            input.host_path = Some(value.into());
            input.volume_name = None;
            input.file_path = None;
            input.content = None;
        }
        crate::utils::builder::spec::MountType::Volume => {
            let value = input
                .volume_name
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| {
                    sqlx::Error::Protocol("volume_name is required for VOLUME".into())
                })?;
            input.volume_name = Some(value.into());
            input.host_path = None;
            input.file_path = None;
            input.content = None;
        }
        crate::utils::builder::spec::MountType::File => {
            if input.content.is_none() {
                return Err(sqlx::Error::Protocol("content is required for FILE".into()));
            }
            input.host_path = None;
            input.volume_name = None;
        }
    }
    Ok(input)
}
