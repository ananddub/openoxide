use super::{VaultService, VaultServiceError};
use crate::api::dto::vault::{VaultSecretListDto, VaultTestResultDto};

impl VaultService {
    pub(super) async fn scaleway_secrets(
        &self,
        config: Option<&str>,
    ) -> Result<Vec<String>, VaultServiceError> {
        let c: serde_json::Value = config
            .and_then(|r| serde_json::from_str(r).ok())
            .ok_or_else(|| {
                VaultServiceError::ProviderError("Scaleway config is required".into())
            })?;
        let base = format!(
            "{}/secret-manager/v1beta1/regions/{}",
            c["apiUrl"]
                .as_str()
                .unwrap_or("https://api.scaleway.com")
                .trim_end_matches('/'),
            c["region"].as_str().unwrap_or("fr-par")
        );
        let r = self
            .client
            .get(format!("{base}/secrets"))
            .query(&[
                ("project_id", c["projectId"].as_str().unwrap_or("")),
                ("page_size", "100"),
            ])
            .header("X-Auth-Token", c["secretKey"].as_str().unwrap_or(""))
            .send()
            .await?;
        let s = r.status();
        let d: serde_json::Value = r.json().await?;
        if !s.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Scaleway Secret Manager failed (status {s})"
            )));
        }
        Ok(d["secrets"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|x| x["name"].as_str().map(str::to_string))
            .collect())
    }
    pub(super) async fn fetch_scaleway_secret(
        &self,
        config: Option<&str>,
        name: &str,
    ) -> Result<String, VaultServiceError> {
        let c: serde_json::Value = config
            .and_then(|r| serde_json::from_str(r).ok())
            .ok_or_else(|| {
                VaultServiceError::ProviderError("Scaleway config is required".into())
            })?;
        let base = format!(
            "{}/secret-manager/v1beta1/regions/{}",
            c["apiUrl"]
                .as_str()
                .unwrap_or("https://api.scaleway.com")
                .trim_end_matches('/'),
            c["region"].as_str().unwrap_or("fr-par")
        );
        let r = self
            .client
            .get(format!(
                "{base}/secrets-by-path/versions/latest_enabled/access"
            ))
            .query(&[
                ("project_id", c["projectId"].as_str().unwrap_or("")),
                ("secret_name", name),
                ("secret_path", "/"),
            ])
            .header("X-Auth-Token", c["secretKey"].as_str().unwrap_or(""))
            .send()
            .await?;
        let s = r.status();
        let d: serde_json::Value = r.json().await?;
        if !s.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "Scaleway secret '{name}' unavailable (status {s})"
            )));
        }
        let raw = d["data"].as_str().ok_or_else(|| {
            VaultServiceError::ProviderError("Scaleway secret has no value".into())
        })?;
        use base64::Engine;
        String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(raw)
                .map_err(|e| VaultServiceError::ProviderError(e.to_string()))?,
        )
        .map_err(|e| VaultServiceError::ProviderError(e.to_string()))
    }
    pub(super) async fn test_scaleway(
        &self,
        config: Option<&str>,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        self.scaleway_secrets(config).await?;
        Ok(VaultTestResultDto {
            success: true,
            message: "Scaleway Secret Manager verified successfully".into(),
        })
    }
    pub(super) async fn list_scaleway(
        &self,
        config: Option<&str>,
    ) -> Result<VaultSecretListDto, VaultServiceError> {
        Ok(VaultSecretListDto {
            secrets: self.scaleway_secrets(config).await?,
        })
    }
}
