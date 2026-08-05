use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::services::webhook::{GitProviderKind, PushEvent};

#[derive(Debug)]
pub struct WebhookResource {
    pub id: i64,
    pub watch_paths: Option<String>,
}

pub struct WebhookRepository {
    db: Arc<SqlitePool>,
}

#[singleton]
impl WebhookRepository {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn credential_secret(
        &self,
        provider: GitProviderKind,
        provider_id: i64,
    ) -> sqlx::Result<Option<String>> {
        let secret = match provider {
            GitProviderKind::Github => sqlx::query_scalar!(
                "SELECT gp.webhook_secret FROM github_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?",
                provider_id
            )
            .fetch_optional(self.db.as_ref())
            .await?,
            GitProviderKind::Gitlab => sqlx::query_scalar!(
                "SELECT gp.webhook_secret FROM gitlab_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?",
                provider_id
            )
            .fetch_optional(self.db.as_ref())
            .await?,
            GitProviderKind::Gitea => sqlx::query_scalar!(
                "SELECT gp.webhook_secret FROM gitea_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?",
                provider_id
            )
            .fetch_optional(self.db.as_ref())
            .await?,
            GitProviderKind::Bitbucket => sqlx::query_scalar!(
                "SELECT gp.webhook_secret FROM bitbucket_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?",
                provider_id
            )
            .fetch_optional(self.db.as_ref())
            .await?,
        };
        Ok(secret.flatten())
    }

    pub async fn matching_applications(
        &self,
        event: &PushEvent,
    ) -> sqlx::Result<Vec<WebhookResource>> {
        let trigger = event.trigger.as_str();
        match event.provider {
            GitProviderKind::Github => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM applications WHERE source_type = 'GITHUB' AND auto_deploy = 1 AND trigger_type = ? AND lower(repository) = lower(?) AND lower(owner) = lower(?) AND branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
            GitProviderKind::Gitlab => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM applications WHERE source_type = 'GITLAB' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitlab_repository) = lower(?) AND lower(gitlab_owner) = lower(?) AND gitlab_branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
            GitProviderKind::Gitea => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM applications WHERE source_type = 'GITEA' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitea_repository) = lower(?) AND lower(gitea_owner) = lower(?) AND gitea_branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
            GitProviderKind::Bitbucket => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM applications WHERE source_type = 'BITBUCKET' AND auto_deploy = 1 AND trigger_type = ? AND lower(bitbucket_repository) = lower(?) AND lower(bitbucket_owner) = lower(?) AND bitbucket_branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
        }
    }

    pub async fn matching_compose_projects(
        &self,
        event: &PushEvent,
    ) -> sqlx::Result<Vec<WebhookResource>> {
        let trigger = event.trigger.as_str();
        match event.provider {
            GitProviderKind::Github => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM compose_projects WHERE source_type = 'GITHUB' AND auto_deploy = 1 AND trigger_type = ? AND lower(repository) = lower(?) AND lower(owner) = lower(?) AND branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
            GitProviderKind::Gitlab => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM compose_projects WHERE source_type = 'GITLAB' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitlab_repository) = lower(?) AND lower(gitlab_owner) = lower(?) AND gitlab_branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
            GitProviderKind::Gitea => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM compose_projects WHERE source_type = 'GITEA' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitea_repository) = lower(?) AND lower(gitea_owner) = lower(?) AND gitea_branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
            GitProviderKind::Bitbucket => sqlx::query_as!(WebhookResource, "SELECT id AS \"id!: i64\", watch_paths FROM compose_projects WHERE source_type = 'BITBUCKET' AND auto_deploy = 1 AND trigger_type = ? AND lower(bitbucket_repository) = lower(?) AND lower(bitbucket_owner) = lower(?) AND bitbucket_branch = ?", trigger, event.repository, event.owner, event.branch).fetch_all(self.db.as_ref()).await,
        }
    }
}
