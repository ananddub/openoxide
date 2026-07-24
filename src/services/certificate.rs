use crate::{
    api::dto::certificate::{CreateCertificateDto, PatchCertificateDto},
    db::models::certificates::Certificate,
    db::repository::certificates::CertificateRepository,
};
use auto_di::singleton;
use std::sync::Arc;

pub struct CertificateService {
    repo_cert: Arc<CertificateRepository>,
}

#[singleton]
impl CertificateService {
    fn new(repo_cert: Arc<CertificateRepository>) -> Self {
        Self { repo_cert }
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
}
