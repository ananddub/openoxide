use crate::utils::exec::{CommandExecutor, LocalExecutor};
use crate::utils::rclone::{RcloneBuilder, RcloneCommand};
use crate::{
    api::dto::destination::{CreateDestinationDto, PatchDestinationDto},
    db::models::destinations::Destination,
    repository::destinations::DestinationRepository,
};
use auto_di::singleton;
use os::string_enum;
use std::sync::Arc;

use crate::db::models::types::DestinationsProviderEnum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DestinationProviderPreset {
        default = S3;

        S3 => "S3",
        R2 => "R2",
        CloudflareR2 => "CLOUDFLARE_R2",
        Backblaze => "BACKBLAZE",
        B2 => "B2",
        Gcs => "GCS",
        Gcp => "GCP",
        Google => "GOOGLE",
        DoSpaces => "DO_SPACES",
        DigitalOcean => "DIGITALOCEAN",
        Spaces => "SPACES",
    }
}

impl DestinationsProviderEnum {
    pub fn from_preset(s: &str) -> Self {
        match DestinationProviderPreset::from(s) {
            DestinationProviderPreset::R2 | DestinationProviderPreset::CloudflareR2 => Self::R2,
            DestinationProviderPreset::Backblaze | DestinationProviderPreset::B2 => Self::Backblaze,
            DestinationProviderPreset::Gcs
            | DestinationProviderPreset::Gcp
            | DestinationProviderPreset::Google => Self::Gcs,
            DestinationProviderPreset::DoSpaces
            | DestinationProviderPreset::DigitalOcean
            | DestinationProviderPreset::Spaces => Self::DoSpaces,
            DestinationProviderPreset::S3 => Self::S3,
        }
    }

    pub fn as_db_str(&self) -> &'static str {
        match self {
            Self::S3 => "S3",
            Self::R2 => "R2",
            Self::Backblaze => "BACKBLAZE",
            Self::Gcs => "GCS",
            Self::DoSpaces => "DO_SPACES",
        }
    }
}

pub struct DestinationService {
    repo_dest: Arc<DestinationRepository>,
}

#[singleton]
impl DestinationService {
    fn new(repo_dest: Arc<DestinationRepository>) -> Self {
        Self { repo_dest }
    }

    pub async fn get_by_id(&self, id: &str) -> sqlx::Result<Destination> {
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;
        self.repo_dest
            .get_by_id(id_i64)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Destination>> {
        self.repo_dest.get_all().await
    }

    pub async fn create(&self, input: CreateDestinationDto) -> sqlx::Result<Destination> {
        let item = Destination {
            id: None,
            name: input.name,
            provider: DestinationsProviderEnum::from_preset(&input.provider)
                .as_db_str()
                .to_string(),
            access_key: input.access_key,
            secret_access_key: input.secret_access_key,
            bucket: input.bucket,
            region: input.region,
            endpoint: input.endpoint,
            organization_id: input.organization_id,
            created_at: 0,
            updated_at: 0,
        };
        let new_id = self.repo_dest.create(&item).await?;
        self.repo_dest
            .get_by_id(new_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch(&self, id: &str, input: PatchDestinationDto) -> sqlx::Result<Destination> {
        let mut current = self.get_by_id(id).await?;
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;

        if let Some(v) = input.name {
            current.name = v;
        }
        if let Some(v) = input.provider {
            current.provider = DestinationsProviderEnum::from_preset(&v)
                .as_db_str()
                .to_string();
        }
        if let Some(v) = input.access_key {
            current.access_key = v;
        }
        if let Some(v) = input.secret_access_key {
            current.secret_access_key = v;
        }
        if let Some(v) = input.bucket {
            current.bucket = v;
        }
        if let Some(v) = input.region {
            current.region = v;
        }
        if let Some(v) = input.endpoint {
            current.endpoint = v;
        }

        self.repo_dest.update(id_i64, &current).await?;
        self.repo_dest
            .get_by_id(id_i64)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, id: &str) -> sqlx::Result<()> {
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;
        self.repo_dest
            .get_by_id(id_i64)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        self.repo_dest.delete(id_i64).await
    }

    pub async fn test_connection(&self, id: &str) -> Result<(), String> {
        let dest = self.get_by_id(id).await.map_err(|e| e.to_string())?;
        self.test_connection_raw(
            &dest.provider,
            &dest.access_key,
            &dest.secret_access_key,
            &dest.bucket,
            &dest.region,
            &dest.endpoint,
        )
        .await
    }

    pub async fn test_connection_raw(
        &self,
        provider: &str,
        access_key: &str,
        secret_access_key: &str,
        bucket: &str,
        region: &str,
        endpoint: &str,
    ) -> Result<(), String> {
        let target = crate::utils::backup::database::S3Destination {
            provider: Some(provider.to_string()),
            access_key: access_key.to_string(),
            secret_key: secret_access_key.to_string(),
            bucket: bucket.to_string(),
            region: region.to_string(),
            endpoint: endpoint.to_string(),
        }
        .to_rclone_target("");

        let builder = RcloneBuilder::new(RcloneCommand::Lsf)
            .source(target)
            .timeout("10s")
            .connect_timeout("5s")
            .retries(1);

        let executor = CommandExecutor::Local(LocalExecutor::new());
        let out = builder
            .execute(&executor)
            .await
            .map_err(|e| e.to_string())?;

        if !out.success() {
            return Err(format!("Connection test failed: {}", out.stderr));
        }

        Ok(())
    }
}
