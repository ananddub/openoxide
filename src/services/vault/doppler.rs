use super::{VaultService, VaultServiceError, config_field};
use crate::api::dto::vault::{VaultSecretListDto, VaultTestResultDto};

impl VaultService {
    pub(super) async fn fetch_doppler_secret(
        &self,
        auth_token: &str,
        config_json: Option<&str>,
        secret_name: &str,
    ) -> Result<String, VaultServiceError> {
        let secrets = self
            .download_doppler_secrets(auth_token, config_json)
            .await?;
        secrets
            .get(secret_name)
            .and_then(|value| value.as_str())
            .map(str::to_string)
            .ok_or_else(|| {
                VaultServiceError::ProviderError(format!(
                    "Doppler: secret '{secret_name}' not found in this config"
                ))
            })
    }

    pub(super) async fn test_doppler_credentials(
        &self,
        auth_token: &str,
        config_json: Option<&str>,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        let clean_token = auth_token.trim().trim_matches('"').trim_matches('\'');

        if clean_token.is_empty() {
            return Ok(VaultTestResultDto {
                success: false,
                message: "Doppler Service Token is required".into(),
            });
        }

        self.download_doppler_secrets(clean_token, config_json)
            .await?;
        Ok(VaultTestResultDto {
            success: true,
            message: "Doppler: secrets configuration verified successfully!".into(),
        })
    }

    pub(super) async fn list_doppler_secrets(
        &self,
        auth_token: &str,
        config_json: Option<&str>,
    ) -> Result<VaultSecretListDto, VaultServiceError> {
        let secrets = self
            .download_doppler_secrets(auth_token, config_json)
            .await?;
        Ok(VaultSecretListDto {
            secrets: secrets.keys().cloned().collect(),
        })
    }

    pub(super) async fn download_doppler_secrets(
        &self,
        auth_token: &str,
        config_json: Option<&str>,
    ) -> Result<serde_json::Map<String, serde_json::Value>, VaultServiceError> {
        let clean_token = auth_token.trim().trim_matches('"').trim_matches('\'');
        if clean_token.is_empty() {
            return Err(VaultServiceError::ProviderError(
                "Doppler Service Token is required".into(),
            ));
        }
        let mut request = self
            .client
            .get("https://api.doppler.com/v3/configs/config/secrets/download")
            .query(&[("format", "json")])
            .header("Authorization", format!("Bearer {clean_token}"));
        if let Some(project) = config_field(config_json, "project") {
            request = request.query(&[("project", project)]);
        }
        if let Some(config) = config_field(config_json, "config") {
            request = request.query(&[("config", config)]);
        }
        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Doppler: failed to fetch secrets (status {status})"
            )));
        }
        response
            .json::<serde_json::Map<String, serde_json::Value>>()
            .await
            .map_err(Into::into)
    }
}
