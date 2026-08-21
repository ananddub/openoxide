use crate::{
    api::dto::certificate::{CreateCertificateDto, PatchCertificateDto, RenewCertificateDto},
    db::models::certificates::Certificate,
    db::repository::{
        CertificateRenewalRepository, ResourceDependencyRepository,
        certificates::CertificateRepository,
    },
};
use auto_di::singleton;
use std::sync::Arc;

mod files;

pub struct CertificateService {
    repo_cert: Arc<CertificateRepository>,
    renewals: Arc<CertificateRenewalRepository>,
    dependencies: Arc<ResourceDependencyRepository>,
}

#[singleton]
impl CertificateService {
    fn new(
        repo_cert: Arc<CertificateRepository>,
        renewals: Arc<CertificateRenewalRepository>,
        dependencies: Arc<ResourceDependencyRepository>,
    ) -> Self {
        Self {
            repo_cert,
            renewals,
            dependencies,
        }
    }

    pub async fn get_by_id(&self, id: i64, organization_id: i64) -> sqlx::Result<Certificate> {
        self.repo_cert
            .get_by_id_for_organization(id, organization_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self, organization_id: i64) -> sqlx::Result<Vec<Certificate>> {
        self.repo_cert.get_all(organization_id).await
    }

    pub async fn create(
        &self,
        input: CreateCertificateDto,
        organization_id: i64,
    ) -> sqlx::Result<Certificate> {
        files::validate_certificate_pair(&input.certificate_data, &input.private_key)
            .await
            .map_err(sqlx::Error::Protocol)?;
        let server_id = if let Some(sid) = input.server_id {
            sid.parse::<i64>().ok()
        } else {
            None
        };

        let now = chrono::Utc::now().timestamp();
        let item = Certificate {
            id: None,
            name: input.name,
            certificate_data: input.certificate_data,
            private_key: input.private_key,
            certificate_path: input.certificate_path,
            auto_renew: input.auto_renew,
            server_id,
            organization_id,
            created_at: now,
            updated_at: now,
        };
        let new_id = self.repo_cert.create(&item).await?;
        let certificate = self
            .repo_cert
            .get_by_id(new_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if let Err(error) = files::write(&certificate).await {
            let _ = self.repo_cert.delete(new_id).await;
            return Err(sqlx::Error::Protocol(error));
        }
        Ok(certificate)
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchCertificateDto,
        organization_id: i64,
    ) -> sqlx::Result<Certificate> {
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
            .map_err(sqlx::Error::Protocol)?;

        files::write(&current)
            .await
            .map_err(sqlx::Error::Protocol)?;
        self.repo_cert.update(id, &current).await?;
        Ok(current)
    }

    pub async fn delete(&self, id: i64, organization_id: i64) -> sqlx::Result<()> {
        // Check existence
        let current = self.get_by_id(id, organization_id).await?;
        let dependencies = self.dependencies.certificate(id).await?;
        if dependencies.running_renewals > 0 {
            return Err(sqlx::Error::Protocol(
                "certificate has a running renewal".into(),
            ));
        }
        files::delete(&current)
            .await
            .map_err(sqlx::Error::Protocol)?;
        if !self
            .repo_cert
            .delete_for_organization(id, organization_id)
            .await?
        {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn dependencies(
        &self,
        id: i64,
        organization_id: i64,
    ) -> sqlx::Result<crate::repository::CertificateDependencyCounts> {
        self.get_by_id(id, organization_id).await?;
        self.dependencies.certificate(id).await
    }

    pub async fn renew(
        &self,
        id: i64,
        input: RenewCertificateDto,
        organization_id: i64,
    ) -> sqlx::Result<Certificate> {
        let mut current = self.get_by_id(id, organization_id).await?;
        let previous = files::certificate_expiry(&current.certificate_data)
            .await
            .ok();
        let execution = self
            .renewals
            .begin(id, current.organization_id, previous)
            .await?;
        let new_expiry =
            match files::validate_certificate_pair(&input.certificate_data, &input.private_key)
                .await
            {
                Ok(expiry) => expiry,
                Err(error) => {
                    self.renewals.finish(execution, None, Some(&error)).await?;
                    return Err(sqlx::Error::Protocol(error));
                }
            };
        current.certificate_data = input.certificate_data;
        current.private_key = input.private_key;
        current.updated_at = chrono::Utc::now().timestamp();
        if let Err(error) = files::write(&current).await {
            self.renewals.finish(execution, None, Some(&error)).await?;
            return Err(sqlx::Error::Protocol(error));
        }
        if let Err(error) = self.repo_cert.update(id, &current).await {
            let message = error.to_string();
            self.renewals
                .finish(execution, None, Some(&message))
                .await?;
            return Err(error);
        }
        self.renewals
            .finish(execution, Some(new_expiry), None)
            .await?;
        self.get_by_id(id, organization_id).await
    }

    pub async fn renewal_history(
        &self,
        id: i64,
        organization_id: i64,
    ) -> sqlx::Result<Vec<crate::db::repository::certificate_renewals::CertificateRenewal>> {
        self.get_by_id(id, organization_id).await?;
        self.renewals.list(id).await
    }
}
