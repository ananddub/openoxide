use super::{VaultService, VaultServiceError};
use crate::api::dto::vault::{VaultSecretListDto, VaultTestResultDto};

impl VaultService {
    pub(super) async fn azure_token(
        &self,
        config: Option<&str>,
    ) -> Result<(serde_json::Value, String), VaultServiceError> {
        let c: serde_json::Value = config
            .and_then(|r| serde_json::from_str(r).ok())
            .ok_or_else(|| VaultServiceError::ProviderError("Azure config is required".into()))?;
        let body = format!(
            "grant_type=client_credentials&client_id={}&client_secret={}&scope=https%3A%2F%2Fvault.azure.net%2F.default",
            urlencoding::encode(c["clientId"].as_str().unwrap_or("")),
            urlencoding::encode(c["clientSecret"].as_str().unwrap_or(""))
        );
        let response = self
            .client
            .post(format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                urlencoding::encode(c["tenantId"].as_str().unwrap_or(""))
            ))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?;
        let status = response.status();
        let data: serde_json::Value = response.json().await?;
        if !status.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Azure authentication failed (status {status})"
            )));
        }
        Ok((c, data["access_token"].as_str().unwrap_or("").to_string()))
    }
    pub(super) async fn azure_secrets(
        &self,
        config: Option<&str>,
    ) -> Result<Vec<String>, VaultServiceError> {
        let (c, t) = self.azure_token(config).await?;
        let url = format!(
            "{}/secrets?api-version=7.4&maxresults=100",
            c["vaultUri"].as_str().unwrap_or("").trim_end_matches('/')
        );
        let r = self.client.get(url).bearer_auth(t).send().await?;
        let s = r.status();
        let d: serde_json::Value = r.json().await?;
        if !s.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Azure Key Vault list failed (status {s})"
            )));
        }
        Ok(d["value"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|x| {
                x["id"]
                    .as_str()
                    .and_then(|v| v.rsplit('/').next())
                    .map(str::to_string)
            })
            .collect())
    }
    pub(super) async fn fetch_azure_secret(
        &self,
        config: Option<&str>,
        name: &str,
    ) -> Result<String, VaultServiceError> {
        let (c, t) = self.azure_token(config).await?;
        let u = format!(
            "{}/secrets/{}?api-version=7.4",
            c["vaultUri"].as_str().unwrap_or("").trim_end_matches('/'),
            urlencoding::encode(name)
        );
        let r = self.client.get(u).bearer_auth(t).send().await?;
        let s = r.status();
        let d: serde_json::Value = r.json().await?;
        if !s.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Azure secret '{name}' unavailable (status {s})"
            )));
        }
        Ok(d["value"].as_str().unwrap_or("").to_string())
    }
    pub(super) async fn test_azure(
        &self,
        config: Option<&str>,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        self.azure_secrets(config).await?;
        Ok(VaultTestResultDto {
            success: true,
            message: "Azure Key Vault verified successfully".into(),
        })
    }
    pub(super) async fn list_azure(
        &self,
        config: Option<&str>,
    ) -> Result<VaultSecretListDto, VaultServiceError> {
        Ok(VaultSecretListDto {
            secrets: self.azure_secrets(config).await?,
        })
    }
}
