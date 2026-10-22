use crate::db::queries::FlashQueries;
use crate::flash_gen::certificate_renewals::CertificateRenewalsFinishParams;
use crate::flash_gen::certificates::{CertificatesCreateParams, CertificatesUpdateParams};
use crate::flash_gen::models::{CertificateRenewals, Certificates};
use crate::flash_gen::resource_dependencies::CertificateDependencyCountsRow;
use auto_di::singleton;
use serde::Deserialize;
use sqlx::Error;
use std::sync::Arc;

use super::files;

#[derive(Debug, Deserialize)]
pub struct CreateCertificateDto {
    pub name: String,
    pub certificate_data: String,
    pub private_key: String,
    pub certificate_path: String,
    pub auto_renew: i64,
    pub server_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PatchCertificateDto {
    pub name: Option<String>,
    pub certificate_data: Option<String>,
    pub private_key: Option<String>,
    pub certificate_path: Option<String>,
    pub auto_renew: Option<i64>,
    pub server_id: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RenewCertificateDto {
    pub certificate_data: String,
    pub private_key: String,
}

pub struct CertificateService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl CertificateService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: i64, organization_id: i64) -> Result<Certificates, Error> {
        self.queries
            .certificates_get_by_id_for_organization(id, organization_id)
            .await
    }

    pub async fn list(&self, organization_id: i64) -> Result<Vec<Certificates>, Error> {
        self.queries.certificates_get_all(organization_id).await
    }

    pub async fn create(
        &self,
        input: CreateCertificateDto,
        organization_id: i64,
    ) -> Result<Certificates, Error> {
        files::validate_certificate_pair(&input.certificate_data, &input.private_key)
            .await
            .map_err(Error::Protocol)?;
        let server_id = if let Some(sid) = input.server_id {
            sid.parse::<i64>().ok()
        } else {
            None
        };

        let now = chrono::Utc::now().timestamp();
        let new_id = self
            .queries
            .certificates_create(&CertificatesCreateParams {
                name: input.name,
                certificate_data: input.certificate_data,
                private_key: input.private_key,
                certificate_path: input.certificate_path,
                auto_renew: input.auto_renew,
                server_id: server_id.unwrap_or(0),
                organization_id,
                created_at: now,
                updated_at: now,
            })
            .await?;
        let certificate = self.queries.certificates_get_by_id(new_id).await?;
        if let Err(error) = files::write(&certificate).await {
            let _ = self.queries.certificates_delete(new_id).await;
            return Err(Error::Protocol(error));
        }
        Ok(certificate)
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchCertificateDto,
        organization_id: i64,
    ) -> Result<Certificates, Error> {
        let mut current = self.get_by_id(id, organization_id).await?;
        let now = chrono::Utc::now().timestamp();

        if let Some(v) = input.name {
            current.name = v;
        }
        if let Some(v) = input
            .certificate_data
            .filter(|value| !value.trim().is_empty())
        {
            current.certificate_data = v;
        }
        if let Some(v) = input.private_key.filter(|value| !value.trim().is_empty()) {
            current.private_key = v;
        }
        if let Some(v) = input.certificate_path {
            current.certificate_path = v;
        }
        if let Some(v) = input.auto_renew {
            current.auto_renew = v;
        }
        if let Some(v) = input.server_id {
            current.server_id = if let Some(sid) = v {
                sid.parse::<i64>().ok()
            } else {
                None
            };
        }
        current.updated_at = now;

        files::validate_certificate_pair(&current.certificate_data, &current.private_key)
            .await
            .map_err(Error::Protocol)?;

        files::write(&current).await.map_err(Error::Protocol)?;
        self.certificates_update(id, &current).await?;
        Ok(current)
    }

    pub async fn delete(&self, id: i64, organization_id: i64) -> Result<(), Error> {
        let current = self.get_by_id(id, organization_id).await?;
        let dependencies = self.queries.certificate_dependency_counts(id).await?;
        if dependencies.running_renewals > 0 {
            return Err(Error::Protocol("certificate has a running renewal".into()));
        }
        files::delete(&current).await.map_err(Error::Protocol)?;
        self.queries
            .certificates_delete_for_organization(id, organization_id)
            .await?;
        Ok(())
    }

    pub async fn dependencies(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<CertificateDependencyCountsRow, Error> {
        self.get_by_id(id, organization_id).await?;
        self.queries.certificate_dependency_counts(id).await
    }

    pub async fn renew(
        &self,
        id: i64,
        input: RenewCertificateDto,
        organization_id: i64,
    ) -> Result<Certificates, Error> {
        let mut current = self.get_by_id(id, organization_id).await?;
        let previous = files::certificate_expiry(&current.certificate_data)
            .await
            .ok();
        let execution = self
            .queries
            .certificate_renewals_begin(
                &crate::flash_gen::certificate_renewals::CertificateRenewalsBeginParams {
                    certificate_id: id,
                    organization_id: current.organization_id,
                    previous_expires_at: previous.unwrap_or(0),
                },
            )
            .await?;
        let new_expiry =
            match files::validate_certificate_pair(&input.certificate_data, &input.private_key)
                .await
            {
                Ok(expiry) => expiry,
                Err(error) => {
                    self.finish_renewal(execution, None, Some(&error)).await?;
                    return Err(Error::Protocol(error));
                }
            };
        current.certificate_data = input.certificate_data;
        current.private_key = input.private_key;
        current.updated_at = chrono::Utc::now().timestamp();
        if let Err(error) = files::write(&current).await {
            self.finish_renewal(execution, None, Some(&error)).await?;
            return Err(Error::Protocol(error));
        }
        if let Err(error) = self.certificates_update(id, &current).await {
            let message = error.to_string();
            self.finish_renewal(execution, None, Some(&message)).await?;
            return Err(error);
        }
        self.finish_renewal(execution, Some(new_expiry), None)
            .await?;
        self.get_by_id(id, organization_id).await
    }

    pub async fn renewal_history(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<CertificateRenewals>, Error> {
        self.get_by_id(id, organization_id).await?;
        self.queries.certificate_renewals_list(id).await
    }
}

impl CertificateService {
    async fn certificates_update(&self, id: i64, c: &Certificates) -> Result<(), Error> {
        self.queries
            .certificates_update(&CertificatesUpdateParams {
                name: c.name.clone(),
                certificate_data: c.certificate_data.clone(),
                private_key: c.private_key.clone(),
                certificate_path: c.certificate_path.clone(),
                auto_renew: c.auto_renew,
                server_id: c.server_id.unwrap_or(0),
                organization_id: c.organization_id,
                created_at: c.created_at,
                updated_at: c.updated_at,
                id,
            })
            .await
    }

    async fn finish_renewal(
        &self,
        execution: i64,
        new_expires_at: Option<i64>,
        error: Option<&str>,
    ) -> Result<(), Error> {
        self.queries
            .certificate_renewals_finish(&CertificateRenewalsFinishParams {
                status: if error.is_some() {
                    "FAILED".to_string()
                } else {
                    "SUCCEEDED".to_string()
                },
                new_expires_at: new_expires_at.unwrap_or(0),
                error: error.unwrap_or("").to_string(),
                id: execution,
            })
            .await
    }
}
