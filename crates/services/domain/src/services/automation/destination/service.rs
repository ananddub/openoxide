use std::sync::Arc;

use auto_di::singleton;
use os::string_enum;

use super::types::{CreateDestinationDto, PatchDestinationDto};

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        destinations::{DestinationsCreateAndReturnParams, DestinationsUpdateAndReturnParams},
        models::Destinations,
    },
    utils::rclone::{RcloneBuilder, RcloneCommand},
};

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DestinationProviderPreset {
        default = S3;

        S3 => "S3",
        Aws => "AWS",
        R2 => "R2",
        CloudflareR2 => "CLOUDFLARE_R2",
        Backblaze => "BACKBLAZE",
        B2 => "B2",
        Minio => "MINIO",
        Wasabi => "WASABI",
        Custom => "CUSTOM",
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
        Minio => "MINIO",
        Wasabi => "WASABI",
        Custom => "CUSTOM",
        Gcs => "GCS",
        DoSpaces => "DO_SPACES",
    }
}

impl DestinationsProviderEnum {
    pub fn from_preset(s: &str) -> Self {
        match DestinationProviderPreset::from(s) {
            DestinationProviderPreset::R2 | DestinationProviderPreset::CloudflareR2 => Self::R2,
            DestinationProviderPreset::Backblaze | DestinationProviderPreset::B2 => Self::Backblaze,
            DestinationProviderPreset::Minio => Self::Minio,
            DestinationProviderPreset::Wasabi => Self::Wasabi,
            DestinationProviderPreset::Custom => Self::Custom,
            DestinationProviderPreset::Gcs
            | DestinationProviderPreset::Gcp
            | DestinationProviderPreset::Google => Self::Gcs,
            DestinationProviderPreset::DoSpaces
            | DestinationProviderPreset::DigitalOcean
            | DestinationProviderPreset::Spaces => Self::DoSpaces,
            DestinationProviderPreset::S3 | DestinationProviderPreset::Aws => Self::S3,
        }
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

    pub async fn get_by_id(&self, id: &str, organization_id: i64) -> sqlx::Result<Destinations> {
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;
        let destination = self.queries.destinations_get_by_id(id_i64).await?;
        if destination.organization_id != organization_id {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(destination)
    }

    pub async fn list(&self, organization_id: i64) -> sqlx::Result<Vec<Destinations>> {
        self.queries
            .destinations_list_by_organization(organization_id)
            .await
    }

    pub async fn create(&self, input: CreateDestinationDto) -> sqlx::Result<Destinations> {
        let provider = DestinationsProviderEnum::from_preset(&input.provider)
            .as_str()
            .to_string();
        let now = chrono::Utc::now().timestamp();
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
                created_at: now,
                updated_at: now,
            })
            .await
    }

    pub async fn patch(
        &self,
        id: &str,
        input: PatchDestinationDto,
        organization_id: i64,
    ) -> sqlx::Result<Destinations> {
        let mut current = self.get_by_id(id, organization_id).await?;
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
                organization_id,
            })
            .await
    }

    pub async fn delete(&self, id: &str, organization_id: i64) -> sqlx::Result<()> {
        let id_i64 = id.parse::<i64>().map_err(|_| sqlx::Error::RowNotFound)?;
        self.get_by_id(id, organization_id).await?;
        self.queries
            .destinations_delete(id_i64, organization_id)
            .await
    }

    pub async fn test_connection(&self, id: &str, organization_id: i64) -> Result<(), String> {
        let dest = self
            .get_by_id(id, organization_id)
            .await
            .map_err(|e| e.to_string())?;
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

        let executor = crate::services::compute::compose::remote::agent_executor_for(None).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fixture;

    async fn service(tag: &str) -> (DestinationService, sqlx::SqlitePool) {
        let (queries, pool) = fixture(tag).await;
        (DestinationService { queries }, pool)
    }

    fn destination(name: &str, provider: &str, organization_id: i64) -> CreateDestinationDto {
        CreateDestinationDto {
            name: name.into(),
            provider: provider.into(),
            access_key: "access".into(),
            secret_access_key: "secret".into(),
            bucket: "backups".into(),
            region: "auto".into(),
            endpoint: "https://s3.example.com".into(),
            organization_id,
        }
    }

    #[test]
    fn provider_presets_collapse_to_canonical_values() {
        for (input, expected) in [
            ("AWS", "S3"),
            ("aws", "S3"),
            ("S3", "S3"),
            ("CLOUDFLARE_R2", "R2"),
            ("R2", "R2"),
            ("B2", "BACKBLAZE"),
            ("BACKBLAZE", "BACKBLAZE"),
            ("GOOGLE", "GCS"),
            ("GCP", "GCS"),
            ("SPACES", "DO_SPACES"),
            ("DIGITALOCEAN", "DO_SPACES"),
            ("MINIO", "MINIO"),
            ("WASABI", "WASABI"),
            ("CUSTOM", "CUSTOM"),
        ] {
            assert_eq!(
                DestinationsProviderEnum::from_preset(input).as_str(),
                expected,
                "{input}"
            );
        }
        assert_eq!(
            DestinationsProviderEnum::from_preset("not-a-provider").as_str(),
            "S3",
            "an unknown preset should fall back to the default provider"
        );
    }

    #[tokio::test]
    async fn create_canonicalizes_provider_and_scopes_to_the_organization() {
        let (service, _pool) = service("destination-create").await;

        let created = service
            .create(destination("nightly", "B2", 7))
            .await
            .unwrap();

        assert_eq!(created.name, "nightly");
        assert_eq!(
            created.provider, "BACKBLAZE",
            "the B2 alias must be stored as its canonical provider"
        );
        assert_eq!(created.organization_id, 7);
        assert_eq!(created.bucket, "backups");
        assert_eq!(created.secret_access_key, "secret");

        let read = service.get_by_id(&created.id.to_string(), 7).await.unwrap();
        assert_eq!(read.name, "nightly");

        assert!(
            matches!(
                service.get_by_id(&created.id.to_string(), 8).await,
                Err(sqlx::Error::RowNotFound)
            ),
            "another organization must not read this destination"
        );
        assert!(service.list(8).await.unwrap().is_empty());
        assert_eq!(service.list(7).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn get_by_id_rejects_malformed_and_missing_ids() {
        let (service, _pool) = service("destination-missing").await;

        assert!(matches!(
            service.get_by_id("not-a-number", 1).await,
            Err(sqlx::Error::RowNotFound)
        ));
        assert!(matches!(
            service.get_by_id("999", 1).await,
            Err(sqlx::Error::RowNotFound)
        ));
    }

    #[tokio::test]
    async fn create_stamps_real_timestamps() {
        let (service, _pool) = service("destination-create-time").await;

        let created = service
            .create(destination("nightly", "S3", 1))
            .await
            .unwrap();

        assert!(
            created.created_at > 0,
            "created_at must be a real timestamp, got {}",
            created.created_at
        );
        assert!(
            created.updated_at > 0,
            "updated_at must be a real timestamp, got {}",
            created.updated_at
        );
    }

    #[tokio::test]
    async fn patch_updates_only_the_fields_that_were_sent() {
        let (service, _pool) = service("destination-patch").await;
        let created = service
            .create(destination("nightly", "S3", 1))
            .await
            .unwrap();

        let patched = service
            .patch(
                &created.id.to_string(),
                PatchDestinationDto {
                    name: Some("weekly".into()),
                    provider: Some("MINIO".into()),
                    access_key: None,
                    secret_access_key: None,
                    bucket: None,
                    region: None,
                    endpoint: None,
                },
                1,
            )
            .await
            .unwrap();

        assert_eq!(patched.name, "weekly");
        assert_eq!(patched.provider, "MINIO");
        assert_eq!(
            patched.access_key, "access",
            "an omitted secret must survive"
        );
        assert_eq!(patched.bucket, "backups");
        assert_eq!(patched.endpoint, "https://s3.example.com");
    }

    #[tokio::test]
    async fn patch_bumps_updated_at() {
        let (service, _pool) = service("destination-patch-time").await;
        let created = service
            .create(destination("nightly", "S3", 1))
            .await
            .unwrap();
        // The column is second-granular; wait so a bump is observable even if
        // the create timestamp is real to begin with.
        tokio::time::sleep(std::time::Duration::from_millis(1_100)).await;

        let patched = service
            .patch(
                &created.id.to_string(),
                PatchDestinationDto {
                    name: Some("weekly".into()),
                    provider: None,
                    access_key: None,
                    secret_access_key: None,
                    bucket: None,
                    region: None,
                    endpoint: None,
                },
                1,
            )
            .await
            .unwrap();

        assert!(
            patched.updated_at > created.updated_at,
            "a patch must bump updated_at: before={} after={}",
            created.updated_at,
            patched.updated_at
        );
    }

    #[tokio::test]
    async fn patch_of_a_missing_destination_is_row_not_found() {
        let (service, _pool) = service("destination-patch-missing").await;

        let error = service
            .patch(
                "999",
                PatchDestinationDto {
                    name: Some("ghost".into()),
                    provider: None,
                    access_key: None,
                    secret_access_key: None,
                    bucket: None,
                    region: None,
                    endpoint: None,
                },
                1,
            )
            .await
            .unwrap_err();
        assert!(matches!(error, sqlx::Error::RowNotFound), "{error}");
    }

    #[tokio::test]
    async fn delete_removes_the_row_but_respects_organization_scope() {
        let (service, _pool) = service("destination-delete").await;
        let created = service
            .create(destination("nightly", "S3", 1))
            .await
            .unwrap();

        assert!(
            matches!(
                service.delete(&created.id.to_string(), 2).await,
                Err(sqlx::Error::RowNotFound)
            ),
            "another organization must not delete this destination"
        );
        assert!(service.get_by_id(&created.id.to_string(), 1).await.is_ok());

        service.delete(&created.id.to_string(), 1).await.unwrap();

        assert!(service.list(1).await.unwrap().is_empty());
    }
}
