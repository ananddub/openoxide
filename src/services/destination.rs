use std::sync::Arc;

use auto_di::singleton;
use os::string_enum;
use serde::Deserialize;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        destinations::{
            DestinationsCreateAndReturnParams, DestinationsGetAllRow,
            DestinationsUpdateAndReturnParams,
        },
        models::Destinations,
    },
    utils::rclone::{RcloneBuilder, RcloneCommand},
};

#[derive(Debug, Deserialize)]
pub struct CreateDestinationDto {
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub organization_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PatchDestinationDto {
    pub name: Option<String>,
    pub provider: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
}

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

// Replacement for the removed `db::models::types::DestinationsProviderEnum`.
// Values are the canonical database strings; serde is unused here because the
// public service API exchanges plain provider strings.
string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DestinationsProviderEnum {
        default = S3;

        S3 => "S3",
        R2 => "R2",
        Backblaze => "BACKBLAZE",
        Gcs => "GCS",
        DoSpaces => "DO_SPACES",
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
}

fn row_to_model(row: DestinationsGetAllRow) -> Destinations {
    Destinations {
        id: row.id,
        name: row.name,
        provider: row.provider,
        access_key: row.access_key,
        secret_access_key: row.secret_access_key,
        bucket: row.bucket,
        region: row.region,
        endpoint: row.endpoint,
        organization_id: row.organization_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub struct DestinationService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl DestinationService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: &str) -> sqlx::Result<Destinations> {
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;
        self.queries.destinations_get_by_id(id_i64).await
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Destinations>> {
        let rows = self.queries.destinations_get_all().await?;
        Ok(rows.into_iter().map(row_to_model).collect())
    }

    pub async fn create(&self, input: CreateDestinationDto) -> sqlx::Result<Destinations> {
        let provider = DestinationsProviderEnum::from_preset(&input.provider)
            .as_str()
            .to_string();
        self.queries
            .destinations_create_and_return(&DestinationsCreateAndReturnParams {
                name: input.name,
                provider,
                access_key: input.access_key,
                secret_access_key: input.secret_access_key,
                bucket: input.bucket,
                region: input.region,
                endpoint: input.endpoint,
                organization_id: input.organization_id,
                // The old repository insert persisted the service-supplied
                // zero-initialized timestamps.
                created_at: 0,
                updated_at: 0,
            })
            .await
    }

    pub async fn patch(&self, id: &str, input: PatchDestinationDto) -> sqlx::Result<Destinations> {
        let mut current = self.get_by_id(id).await?;
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;

        if let Some(v) = input.name {
            current.name = v;
        }
        if let Some(v) = input.provider {
            current.provider = DestinationsProviderEnum::from_preset(&v)
                .as_str()
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

        self.queries
            .destinations_update_and_return(&DestinationsUpdateAndReturnParams {
                name: current.name,
                provider: current.provider,
                access_key: current.access_key,
                secret_access_key: current.secret_access_key,
                bucket: current.bucket,
                region: current.region,
                endpoint: current.endpoint,
                id: id_i64,
            })
            .await
    }

    pub async fn delete(&self, id: &str) -> sqlx::Result<()> {
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;
        self.get_by_id(id).await?;
        self.queries.destinations_delete(id_i64).await
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

        let executor = crate::services::compose::remote::agent_executor_for(None).await?;
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
