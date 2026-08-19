use super::{VaultService, VaultServiceError};
use crate::api::dto::vault::{VaultSecretListDto, VaultTestResultDto};

impl VaultService {
    pub(super) async fn infisical_config(
        &self,
        config_json: Option<&str>,
    ) -> Result<serde_json::Value, VaultServiceError> {
        let value: serde_json::Value = config_json
            .and_then(|raw| serde_json::from_str(raw).ok())
            .ok_or_else(|| {
            VaultServiceError::ProviderError("Infisical config is required".into())
        })?;
        for key in [
            "siteUrl",
            "clientId",
            "clientSecret",
            "projectId",
            "environmentSlug",
        ] {
            if value[key].as_str().unwrap_or("").is_empty() && key != "siteUrl" {
                return Err(VaultServiceError::ProviderError(format!(
                    "Infisical config field '{key}' is required"
                )));
            }
        }
        Ok(value)
    }

    pub(super) async fn infisical_secrets(
        &self,
        config_json: Option<&str>,
    ) -> Result<serde_json::Map<String, serde_json::Value>, VaultServiceError> {
        let c = self.infisical_config(config_json).await?;
        let base = c["siteUrl"]
            .as_str()
            .unwrap_or("https://app.infisical.com")
            .trim_end_matches('/');
        let login = self
            .client
            .post(format!("{base}/api/v1/auth/universal-auth/login"))
            .json(
                &serde_json::json!({"clientId": c["clientId"], "clientSecret": c["clientSecret"]}),
            )
            .send()
            .await?;
        let status = login.status();
        let body: serde_json::Value = login.json().await?;
        if !status.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Infisical authentication failed (status {status})"
            )));
        }
        let token = body["accessToken"].as_str().ok_or_else(|| {
            VaultServiceError::ProviderError("Infisical: no access token returned".into())
        })?;
        let url = format!("{base}/api/v3/secrets/raw");
        let response = self
            .client
            .get(url)
            .query(&[
                ("workspaceId", c["projectId"].as_str().unwrap_or("")),
                ("environment", c["environmentSlug"].as_str().unwrap_or("")),
                ("secretPath", c["secretPath"].as_str().unwrap_or("/")),
            ])
            .bearer_auth(token)
            .send()
            .await?;
        let status = response.status();
        let payload: serde_json::Value = response.json().await?;
        if !status.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Infisical: failed to fetch secrets (status {status})"
            )));
        }
        let mut out = serde_json::Map::new();
        for item in payload["secrets"].as_array().into_iter().flatten() {
            if let (Some(k), Some(v)) = (item["secretKey"].as_str(), item["secretValue"].as_str()) {
                out.insert(k.into(), v.into());
            }
        }
        Ok(out)
    }

    pub(super) async fn fetch_infisical_secret(
        &self,
        config: Option<&str>,
        name: &str,
    ) -> Result<String, VaultServiceError> {
        self.infisical_secrets(config)
            .await?
            .remove(name)
            .and_then(|v| v.as_str().map(str::to_string))
            .ok_or_else(|| {
                VaultServiceError::ProviderError(format!("Infisical: secret '{name}' not found"))
            })
    }
    pub(super) async fn test_infisical(
        &self,
        config: Option<&str>,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        self.infisical_secrets(config).await?;
        Ok(VaultTestResultDto {
            success: true,
            message: "Infisical: credentials verified successfully".into(),
        })
    }
    pub(super) async fn list_infisical(
        &self,
        config: Option<&str>,
    ) -> Result<VaultSecretListDto, VaultServiceError> {
        Ok(VaultSecretListDto {
            secrets: self
                .infisical_secrets(config)
                .await?
                .keys()
                .cloned()
                .collect(),
        })
    }
}
