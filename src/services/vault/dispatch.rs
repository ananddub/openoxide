use super::{VaultService, VaultServiceError, providers};
use crate::api::dto::vault::{VaultProviderType, VaultSecretListDto, VaultTestResultDto};

impl VaultService {
    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;

        let p_type: VaultProviderType = provider
            .provider_type
            .parse()
            .unwrap_or(VaultProviderType::Hashicorp);

        match p_type {
            VaultProviderType::Hashicorp => {
                self.test_hashicorp_credentials(
                    &provider.api_url,
                    &provider.auth_token,
                    provider.namespace.as_deref(),
                )
                .await
            }
            VaultProviderType::Doppler => {
                self.test_doppler_credentials(&provider.auth_token, provider.config_json.as_deref())
                    .await
            }
            VaultProviderType::Infisical => {
                self.test_infisical(provider.config_json.as_deref()).await
            }
            VaultProviderType::Azure => self.test_azure(provider.config_json.as_deref()).await,
            VaultProviderType::Scaleway => {
                self.test_scaleway(provider.config_json.as_deref()).await
            }
            VaultProviderType::Aws => providers::aws::test(provider.config_json.as_deref())
                .await
                .map_err(VaultServiceError::ProviderError),
        }
    }

    pub async fn test_credentials(
        &self,
        provider_type: VaultProviderType,
        api_url: &str,
        auth_token: &str,
        namespace: Option<String>,
        config_json: Option<&str>,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        let clean_url = api_url.trim_end_matches('/');
        let clean_token = auth_token.trim().trim_matches('"').trim_matches('\'');

        match provider_type {
            VaultProviderType::Hashicorp => {
                self.test_hashicorp_credentials(clean_url, clean_token, namespace.as_deref())
                    .await
            }
            VaultProviderType::Doppler => {
                self.test_doppler_credentials(clean_token, config_json)
                    .await
            }
            VaultProviderType::Infisical => self.test_infisical(config_json).await,
            VaultProviderType::Azure => self.test_azure(config_json).await,
            VaultProviderType::Scaleway => self.test_scaleway(config_json).await,
            VaultProviderType::Aws => providers::aws::test(config_json)
                .await
                .map_err(VaultServiceError::ProviderError),
        }
    }

    pub async fn list_secret_names(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<VaultSecretListDto, VaultServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;

        let p_type: VaultProviderType = provider
            .provider_type
            .parse()
            .unwrap_or(VaultProviderType::Hashicorp);

        match p_type {
            VaultProviderType::Hashicorp => {
                self.list_hashicorp_secrets(
                    &provider.api_url,
                    &provider.auth_token,
                    provider.namespace.as_deref(),
                    provider.config_json.as_deref(),
                )
                .await
            }
            VaultProviderType::Doppler => {
                self.list_doppler_secrets(&provider.auth_token, provider.config_json.as_deref())
                    .await
            }
            VaultProviderType::Infisical => {
                self.list_infisical(provider.config_json.as_deref()).await
            }
            VaultProviderType::Azure => self.list_azure(provider.config_json.as_deref()).await,
            VaultProviderType::Scaleway => {
                self.list_scaleway(provider.config_json.as_deref()).await
            }
            VaultProviderType::Aws => providers::aws::list(provider.config_json.as_deref())
                .await
                .map_err(VaultServiceError::ProviderError),
        }
    }
}
