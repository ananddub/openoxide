use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PreviewDeploymentRow {
    pub id: i64,
    pub base_application_id: i64,
    pub preview_application_id: Option<i64>,
    pub provider_type: String,
    pub provider_id: i64,
    pub owner: String,
    pub repository: String,
    pub pull_request_number: String,
    pub source_branch: String,
    pub source_owner: Option<String>,
    pub source_repository: Option<String>,
    pub target_branch: String,
    pub commit_sha: Option<String>,
    pub author: Option<String>,
    pub status: String,
    pub domain: String,
    pub last_deployment_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct PreviewTarget {
    pub base_application_id: i64,
    pub provider_id: i64,
    pub preview_limit: i64,
    pub require_collaborator_permissions: bool,
}

pub struct NewPreviewDeployment<'a> {
    pub base_application_id: i64,
    pub preview_application_id: i64,
    pub provider_type: &'a str,
    pub provider_id: i64,
    pub owner: &'a str,
    pub repository: &'a str,
    pub pull_request_number: &'a str,
    pub source_branch: &'a str,
    pub source_owner: Option<&'a str>,
    pub source_repository: Option<&'a str>,
    pub target_branch: &'a str,
    pub commit_sha: Option<&'a str>,
    pub author: Option<&'a str>,
    pub domain: &'a str,
}

pub struct PreviewDeploymentRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PreviewDeploymentRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list(&self, active_only: bool) -> sqlx::Result<Vec<PreviewDeploymentRow>> {
        if active_only {
            sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments WHERE status != 'CLOSED' ORDER BY created_at DESC"#).fetch_all(self.pool.as_ref()).await
        } else {
            sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments ORDER BY created_at DESC"#).fetch_all(self.pool.as_ref()).await
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Option<PreviewDeploymentRow>> {
        sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments WHERE id = ?"#, id).fetch_optional(self.pool.as_ref()).await
    }

    pub async fn list_open_for_base_application(
        &self,
        base_application_id: i64,
    ) -> sqlx::Result<Vec<PreviewDeploymentRow>> {
        sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments WHERE base_application_id = ? AND status != 'CLOSED' ORDER BY created_at"#, base_application_id)
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn find_by_preview_application(
        &self,
        preview_application_id: i64,
    ) -> sqlx::Result<Option<PreviewDeploymentRow>> {
        sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments WHERE preview_application_id = ?"#, preview_application_id)
            .fetch_optional(self.pool.as_ref())
            .await
    }

    pub async fn find_for_pull_request(
        &self,
        base_application_id: i64,
        provider_type: &str,
        pull_request_number: &str,
    ) -> sqlx::Result<Option<PreviewDeploymentRow>> {
        sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments WHERE base_application_id = ? AND provider_type = ? AND pull_request_number = ?"#, base_application_id, provider_type, pull_request_number).fetch_optional(self.pool.as_ref()).await
    }

    pub async fn find_open_for_event(
        &self,
        provider: &str,
        owner: &str,
        repository: &str,
        pull_request_number: &str,
    ) -> sqlx::Result<Vec<PreviewDeploymentRow>> {
        sqlx::query_as!(PreviewDeploymentRow, r#"SELECT id AS "id!: i64", base_application_id AS "base_application_id!: i64", preview_application_id, provider_type AS "provider_type!: String", provider_id AS "provider_id!: i64", owner AS "owner!: String", repository AS "repository!: String", pull_request_number AS "pull_request_number!: String", source_branch AS "source_branch!: String", source_owner, source_repository, target_branch AS "target_branch!: String", commit_sha, author, status AS "status!: String", domain AS "domain!: String", last_deployment_id, created_at AS "created_at!: i64", updated_at AS "updated_at!: i64" FROM preview_deployments WHERE provider_type = ? AND lower(owner) = lower(?) AND lower(repository) = lower(?) AND pull_request_number = ? AND status != 'CLOSED'"#, provider, owner, repository, pull_request_number).fetch_all(self.pool.as_ref()).await
    }

    pub async fn matching_targets(
        &self,
        provider: &str,
        owner: &str,
        repository: &str,
        target_branch: &str,
    ) -> sqlx::Result<Vec<PreviewTarget>> {
        match provider {
            "github" => sqlx::query_as!(PreviewTarget, r#"SELECT CAST(a.id AS INTEGER) AS "base_application_id!: i64", gp.id AS "provider_id!: i64", COALESCE(a.preview_limit, 3) AS "preview_limit!: i64", a.preview_require_collaborator_permissions != 0 AS "require_collaborator_permissions!: bool" FROM applications a JOIN github_providers p ON p.id = a.github_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'GITHUB' AND a.is_preview_deployments_active = 1 AND lower(a.repository) = lower(?) AND lower(a.owner) = lower(?) AND a.branch = ?"#, repository, owner, target_branch).fetch_all(self.pool.as_ref()).await,
            "gitlab" => sqlx::query_as!(PreviewTarget, r#"SELECT CAST(a.id AS INTEGER) AS "base_application_id!: i64", gp.id AS "provider_id!: i64", COALESCE(a.preview_limit, 3) AS "preview_limit!: i64", a.preview_require_collaborator_permissions != 0 AS "require_collaborator_permissions!: bool" FROM applications a JOIN gitlab_providers p ON p.id = a.gitlab_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'GITLAB' AND a.is_preview_deployments_active = 1 AND lower(a.gitlab_repository) = lower(?) AND lower(a.gitlab_owner) = lower(?) AND a.gitlab_branch = ?"#, repository, owner, target_branch).fetch_all(self.pool.as_ref()).await,
            "gitea" => sqlx::query_as!(PreviewTarget, r#"SELECT CAST(a.id AS INTEGER) AS "base_application_id!: i64", gp.id AS "provider_id!: i64", COALESCE(a.preview_limit, 3) AS "preview_limit!: i64", a.preview_require_collaborator_permissions != 0 AS "require_collaborator_permissions!: bool" FROM applications a JOIN gitea_providers p ON p.id = a.gitea_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'GITEA' AND a.is_preview_deployments_active = 1 AND lower(a.gitea_repository) = lower(?) AND lower(a.gitea_owner) = lower(?) AND a.gitea_branch = ?"#, repository, owner, target_branch).fetch_all(self.pool.as_ref()).await,
            "bitbucket" => sqlx::query_as!(PreviewTarget, r#"SELECT CAST(a.id AS INTEGER) AS "base_application_id!: i64", gp.id AS "provider_id!: i64", COALESCE(a.preview_limit, 3) AS "preview_limit!: i64", a.preview_require_collaborator_permissions != 0 AS "require_collaborator_permissions!: bool" FROM applications a JOIN bitbucket_providers p ON p.id = a.bitbucket_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'BITBUCKET' AND a.is_preview_deployments_active = 1 AND lower(a.bitbucket_repository) = lower(?) AND lower(a.bitbucket_owner) = lower(?) AND a.bitbucket_branch = ?"#, repository, owner, target_branch).fetch_all(self.pool.as_ref()).await,
            _ => Err(sqlx::Error::Protocol("unsupported preview provider".into())),
        }
    }

    pub async fn active_count(&self, base_application_id: i64) -> sqlx::Result<i64> {
        sqlx::query_scalar!(r#"SELECT COUNT(*) AS "count!: i64" FROM preview_deployments WHERE base_application_id = ? AND status != 'CLOSED'"#, base_application_id).fetch_one(self.pool.as_ref()).await
    }

    pub async fn create(&self, item: NewPreviewDeployment<'_>) -> sqlx::Result<i64> {
        let result = sqlx::query!(r#"INSERT INTO preview_deployments (base_application_id, preview_application_id, provider_type, provider_id, owner, repository, pull_request_number, source_branch, source_owner, source_repository, target_branch, commit_sha, author, status, domain) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'QUEUED', ?)"#, item.base_application_id, item.preview_application_id, item.provider_type, item.provider_id, item.owner, item.repository, item.pull_request_number, item.source_branch, item.source_owner, item.source_repository, item.target_branch, item.commit_sha, item.author, item.domain).execute(self.pool.as_ref()).await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn update_source(
        &self,
        id: i64,
        source_branch: &str,
        source_owner: Option<&str>,
        source_repository: Option<&str>,
        commit_sha: Option<&str>,
        author: Option<&str>,
    ) -> sqlx::Result<()> {
        sqlx::query!("UPDATE preview_deployments SET source_branch = ?, source_owner = ?, source_repository = ?, commit_sha = ?, author = ?, status = 'QUEUED', updated_at = strftime('%s', 'now') WHERE id = ?", source_branch, source_owner, source_repository, commit_sha, author, id).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn reopen(
        &self,
        id: i64,
        preview_application_id: i64,
        source_branch: &str,
        source_owner: Option<&str>,
        source_repository: Option<&str>,
        target_branch: &str,
        commit_sha: Option<&str>,
        author: Option<&str>,
        domain: &str,
    ) -> sqlx::Result<()> {
        sqlx::query!("UPDATE preview_deployments SET preview_application_id = ?, source_branch = ?, source_owner = ?, source_repository = ?, target_branch = ?, commit_sha = ?, author = ?, domain = ?, last_deployment_id = NULL, status = 'QUEUED', updated_at = strftime('%s', 'now') WHERE id = ?", preview_application_id, source_branch, source_owner, source_repository, target_branch, commit_sha, author, domain, id).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn link_deployment(&self, id: i64, deployment_id: i64) -> sqlx::Result<()> {
        let mut tx = self.pool.begin().await?;
        let deployment = sqlx::query!(
            "UPDATE deployments SET is_preview_deployment = 1 WHERE id = ?",
            deployment_id
        )
        .execute(&mut *tx)
        .await?;
        if deployment.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }
        let preview = sqlx::query!("UPDATE preview_deployments SET last_deployment_id = ?, status = 'QUEUED', updated_at = strftime('%s', 'now') WHERE id = ?", deployment_id, id)
            .execute(&mut *tx)
            .await?;
        if preview.rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn set_status(&self, id: i64, status: &str) -> sqlx::Result<()> {
        sqlx::query!("UPDATE preview_deployments SET status = ?, updated_at = strftime('%s', 'now') WHERE id = ?", status, id).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn close(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("UPDATE preview_deployments SET preview_application_id = NULL, status = 'CLOSED', updated_at = strftime('%s', 'now') WHERE id = ?", id).execute(self.pool.as_ref()).await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM preview_deployments WHERE id = ?", id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
