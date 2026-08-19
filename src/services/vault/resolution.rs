use super::{VaultService, VaultServiceError, providers};
use crate::api::dto::vault::VaultProviderType;
use crate::db::models::vault_providers::VaultProvider;
use std::collections::HashMap;

impl VaultService {
    /// Resolves all Vault references like `${{vault.provider_name.ref_path}}` in raw_env text during build/deployment phase.
    pub async fn resolve_vault_references(
        &self,
        raw_env: &str,
        organization_id: i64,
    ) -> Result<String, VaultServiceError> {
        if !raw_env.contains("${{vault.") {
            return Ok(raw_env.to_string());
        }

        let providers = self
            .repository
            .list_by_organization(organization_id)
            .await?;
        let mut provider_map: HashMap<String, VaultProvider> = HashMap::new();
        for p in providers {
            provider_map.insert(p.name.clone(), p);
        }

        let mut resolved = raw_env.to_string();
        let prefix = "${{vault.";
        let suffix = "}}";

        while let Some(start_idx) = resolved.find(prefix) {
            let rest = &resolved[start_idx + prefix.len()..];
            if let Some(end_idx) = rest.find(suffix) {
                let full_ref =
                    &resolved[start_idx..start_idx + prefix.len() + end_idx + suffix.len()];
                let inner = &rest[..end_idx];

                if let Some(dot_idx) = inner.find('.') {
                    let provider_name = &inner[..dot_idx];
                    let ref_path = &inner[dot_idx + 1..];

                    if let Some(provider) = provider_map.get(provider_name) {
                        let p_type: VaultProviderType = provider
                            .provider_type
                            .parse()
                            .unwrap_or(VaultProviderType::Hashicorp);

                        let val = match p_type {
                            VaultProviderType::Doppler => {
                                self.fetch_doppler_secret(
                                    &provider.auth_token,
                                    provider.config_json.as_deref(),
                                    ref_path,
                                )
                                .await?
                            }
                            VaultProviderType::Hashicorp => {
                                self.fetch_hashicorp_secret(
                                    &provider.api_url,
                                    &provider.auth_token,
                                    provider.namespace.as_deref(),
                                    provider.config_json.as_deref(),
                                    ref_path,
                                )
                                .await?
                            }
                            VaultProviderType::Infisical => {
                                self.fetch_infisical_secret(
                                    provider.config_json.as_deref(),
                                    ref_path,
                                )
                                .await?
                            }
                            VaultProviderType::Azure => {
                                self.fetch_azure_secret(provider.config_json.as_deref(), ref_path)
                                    .await?
                            }
                            VaultProviderType::Scaleway => {
                                self.fetch_scaleway_secret(
                                    provider.config_json.as_deref(),
                                    ref_path,
                                )
                                .await?
                            }
                            VaultProviderType::Aws => {
                                providers::aws::fetch(provider.config_json.as_deref(), ref_path)
                                    .await
                                    .map_err(VaultServiceError::ProviderError)?
                            }
                        };
                        resolved = resolved.replace(full_ref, &val);
                    } else {
                        return Err(VaultServiceError::ProviderError(format!(
                            "Vault provider '{provider_name}' not found"
                        )));
                    }
                } else {
                    return Err(VaultServiceError::ProviderError(format!(
                        "Invalid vault reference: {full_ref}"
                    )));
                }
            } else {
                break;
            }
        }

        Ok(resolved)
    }
}
