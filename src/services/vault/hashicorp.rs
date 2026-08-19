use super::{VaultService, VaultServiceError, config_field, encode_path};
use crate::api::dto::vault::{VaultSecretListDto, VaultTestResultDto};

impl VaultService {
    pub(super) async fn fetch_hashicorp_secret(
        &self,
        api_url: &str,
        auth_token: &str,
        namespace: Option<&str>,
        config_json: Option<&str>,
        path_and_key: &str,
    ) -> Result<String, VaultServiceError> {
        let clean_url = api_url.trim_end_matches('/');
        let clean_token = auth_token.trim().trim_matches('"').trim_matches('\'');

        let (path, field) = path_and_key
            .rsplit_once(':')
            .filter(|(path, field)| !path.is_empty() && !field.is_empty())
            .ok_or_else(|| {
                VaultServiceError::ProviderError(format!(
                    "Invalid HashiCorp Vault reference '{path_and_key}': expected <path>:<field>"
                ))
            })?;

        let mount = config_field(config_json, "mount").unwrap_or_else(|| "secret".into());
        let url = format!(
            "{}/v1/{}/data/{}",
            clean_url,
            urlencoding::encode(&mount),
            encode_path(path)
        );
        let mut req = self.client.get(&url).header("X-Vault-Token", clean_token);
        if let Some(ns) = namespace {
            if !ns.trim().is_empty() {
                req = req.header("X-Vault-Namespace", ns.trim());
            }
        }

        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "HashiCorp Vault: failed to read secret at '{path}' (status {status})"
            )));
        }
        let json = res.json::<serde_json::Value>().await?;
        let value = &json["data"]["data"][field];
        if value.is_null() {
            return Err(VaultServiceError::ProviderError(format!(
                "HashiCorp Vault: field '{field}' not found in secret '{path}'"
            )));
        }
        Ok(value
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| value.to_string()))
    }

    pub(super) async fn test_hashicorp_credentials(
        &self,
        api_url: &str,
        auth_token: &str,
        namespace: Option<&str>,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        let clean_url = api_url.trim_end_matches('/');
        let clean_token = auth_token.trim().trim_matches('"').trim_matches('\'');

        if clean_token.is_empty() {
            return Ok(VaultTestResultDto {
                success: false,
                message: "HashiCorp Vault token is required".into(),
            });
        }

        let url = format!("{}/v1/auth/token/lookup-self", clean_url);
        let mut req = self.client.get(&url).header("X-Vault-Token", clean_token);
        if let Some(ns) = namespace {
            if !ns.trim().is_empty() {
                req = req.header("X-Vault-Namespace", ns.trim());
            }
        }
        let res = req.send().await?;
        if res.status().is_success() {
            Ok(VaultTestResultDto {
                success: true,
                message: "HashiCorp Vault: token validated successfully!".into(),
            })
        } else {
            Ok(VaultTestResultDto {
                success: false,
                message: format!(
                    "HashiCorp Vault: token validation failed (status {})",
                    res.status()
                ),
            })
        }
    }

    pub(super) async fn list_hashicorp_secrets(
        &self,
        api_url: &str,
        auth_token: &str,
        namespace: Option<&str>,
        config_json: Option<&str>,
    ) -> Result<VaultSecretListDto, VaultServiceError> {
        let clean_url = api_url.trim_end_matches('/');
        let clean_token = auth_token.trim().trim_matches('"').trim_matches('\'');

        let mount = config_field(config_json, "mount").unwrap_or_else(|| "secret".into());
        let url = format!(
            "{}/v1/{}/metadata?list=true",
            clean_url,
            urlencoding::encode(&mount)
        );
        let mut req = self.client.get(&url).header("X-Vault-Token", clean_token);
        if let Some(ns) = namespace {
            if !ns.trim().is_empty() {
                req = req.header("X-Vault-Namespace", ns.trim());
            }
        }
        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(VaultServiceError::ProviderError(format!(
                "HashiCorp Vault: cannot list secrets (status {status})"
            )));
        }
        let json = res.json::<serde_json::Value>().await?;
        let secrets = json["data"]["keys"]
            .as_array()
            .ok_or_else(|| {
                VaultServiceError::ProviderError(
                    "HashiCorp Vault returned an invalid secret list".into(),
                )
            })?
            .iter()
            .filter_map(|key| key.as_str().map(str::to_string))
            .collect();
        Ok(VaultSecretListDto { secrets })
    }
}
