use crate::utils::provider::discovery::ProviderWebhookInfo;

use super::GitProviderService;

impl GitProviderService {
    pub async fn webhook_status(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        callback_url: &str,
        organization_id: i64,
    ) -> Result<Option<ProviderWebhookInfo>, String> {
        self.validate_webhook_callback(id, callback_url, organization_id)
            .await?;
        self.discovery
            .webhook_status(
                &self.access(id, organization_id).await?,
                owner,
                repository,
                callback_url,
            )
            .await
    }

    pub async fn install_webhook(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        callback_url: &str,
        organization_id: i64,
    ) -> Result<ProviderWebhookInfo, String> {
        self.validate_webhook_callback(id, callback_url, organization_id)
            .await?;
        let secret = self.webhook_secret(id, organization_id).await?;
        self.discovery
            .install_webhook(
                &self.access(id, organization_id).await?,
                owner,
                repository,
                callback_url,
                &secret,
            )
            .await
    }

    pub async fn remove_webhook(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        callback_url: &str,
        organization_id: i64,
    ) -> Result<bool, String> {
        self.validate_webhook_callback(id, callback_url, organization_id)
            .await?;
        self.discovery
            .remove_webhook(
                &self.access(id, organization_id).await?,
                owner,
                repository,
                callback_url,
            )
            .await
    }

    pub async fn recreate_webhook(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        callback_url: &str,
        organization_id: i64,
    ) -> Result<ProviderWebhookInfo, String> {
        self.validate_webhook_callback(id, callback_url, organization_id)
            .await?;
        let access = self.access(id, organization_id).await?;
        let secret = self.webhook_secret(id, organization_id).await?;
        self.discovery
            .remove_webhook(&access, owner, repository, callback_url)
            .await?;
        self.discovery
            .install_webhook(&access, owner, repository, callback_url, &secret)
            .await
    }

    pub(super) async fn webhook_secret(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<String, String> {
        self.provider_row(id, organization_id).await?;
        self.queries
            .git_providers_get_webhook_secret(id)
            .await
            .map_err(|error| error.to_string())?
            .filter(|value| !value.is_empty())
            .ok_or("Webhook secret is not configured".into())
    }

    async fn validate_webhook_callback(
        &self,
        id: i64,
        callback_url: &str,
        organization_id: i64,
    ) -> Result<(), String> {
        use crate::utils::provider::GitProviderType;
        let provider = self
            .get(id, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let provider_kind: Option<GitProviderType> = provider.provider_type.parse().ok();
        let kind = match provider_kind {
            Some(GitProviderType::Github) => "github",
            Some(GitProviderType::Gitlab) => "gitlab",
            Some(GitProviderType::Gitea) => "gitea",
            Some(GitProviderType::Bitbucket) => "bitbucket",
            None => return Err("Unsupported Git provider".into()),
        };
        let expected = format!("/public/webhooks/{kind}/{id}");
        callback_url
            .trim_end_matches('/')
            .ends_with(&expected)
            .then_some(())
            .ok_or_else(|| format!("Webhook callback must end with {expected}"))
    }
}
