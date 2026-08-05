use crate::{
    api::dto::certificate::{CreateCertificateDto, PatchCertificateDto, RenewCertificateDto},
    db::models::certificates::Certificate,
    db::repository::{CertificateRenewalRepository, certificates::CertificateRepository},
};
use auto_di::singleton;
use std::sync::Arc;

use crate::utils::{
    exec::{CommandExecutor, LocalExecutor},
    os::OsCli,
};

pub struct CertificateService {
    repo_cert: Arc<CertificateRepository>,
    renewals: Arc<CertificateRenewalRepository>,
}

#[singleton]
impl CertificateService {
    fn new(
        repo_cert: Arc<CertificateRepository>,
        renewals: Arc<CertificateRenewalRepository>,
    ) -> Self {
        Self {
            repo_cert,
            renewals,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Certificate> {
        self.repo_cert
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Certificate>> {
        self.repo_cert.get_all().await
    }

    pub async fn create(&self, input: CreateCertificateDto) -> sqlx::Result<Certificate> {
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
            organization_id: input.organization_id,
            created_at: now,
            updated_at: now,
        };
        let new_id = self.repo_cert.create(&item).await?;
        self.repo_cert
            .get_by_id(new_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch(&self, id: i64, input: PatchCertificateDto) -> sqlx::Result<Certificate> {
        let mut current = self.get_by_id(id).await?;
        let now = chrono::Utc::now().timestamp();

        if let Some(v) = input.name {
            current.name = v;
        }
        if let Some(v) = input.certificate_data {
            current.certificate_data = v;
        }
        if let Some(v) = input.private_key {
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

        self.repo_cert.update(id, &current).await?;
        self.repo_cert
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        // Check existence
        self.get_by_id(id).await?;
        self.repo_cert.delete(id).await
    }

    pub async fn renew(&self, id: i64, input: RenewCertificateDto) -> sqlx::Result<Certificate> {
        let mut current = self.get_by_id(id).await?;
        let previous = certificate_expiry(&current.certificate_data).await.ok();
        let execution = self
            .renewals
            .begin(id, current.organization_id, previous)
            .await?;
        let new_expiry =
            match validate_certificate_pair(&input.certificate_data, &input.private_key).await {
                Ok(expiry) => expiry,
                Err(error) => {
                    self.renewals.finish(execution, None, Some(&error)).await?;
                    return Err(sqlx::Error::Protocol(error));
                }
            };
        current.certificate_data = input.certificate_data;
        current.private_key = input.private_key;
        current.updated_at = chrono::Utc::now().timestamp();
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
        self.get_by_id(id).await
    }

    pub async fn renewal_history(
        &self,
        id: i64,
    ) -> sqlx::Result<Vec<crate::db::repository::certificate_renewals::CertificateRenewal>> {
        self.get_by_id(id).await?;
        self.renewals.list(id).await
    }
}

async fn certificate_expiry(certificate: &str) -> Result<i64, String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    OsCli::new(&executor)
        .crypto()
        .certificate(certificate)
        .expiry()
        .run()
        .await
        .map_err(|error| error.to_string())
}

async fn validate_certificate_pair(certificate: &str, private_key: &str) -> Result<i64, String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    OsCli::new(&executor)
        .crypto()
        .certificate(certificate)
        .validate_with_key(private_key)
        .run()
        .await
        .map(|result| result.expires_at)
        .map_err(|error| error.to_string())
}
