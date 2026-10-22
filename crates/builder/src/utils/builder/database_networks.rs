use crate::db::queries::FlashQueries;
use crate::utils::builder::swarm::OPENOXIDE_NETWORK;
use crate::utils::exec::{ExecError, ExecResult};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serialize)]
pub struct ExternalNetwork {
    pub external: bool,
    pub name: String,
}

pub async fn resolve_database_networks(
    network_ids: Option<&str>,
    detach_rustploy_network: i64,
) -> ExecResult<(Vec<String>, BTreeMap<String, ExternalNetwork>)> {
    let mut names = BTreeSet::new();
    if detach_rustploy_network == 0 {
        names.insert(OPENOXIDE_NETWORK.to_string());
    }

    if let Some(raw) = network_ids
        && let Ok(parsed) = serde_json::from_str::<Vec<String>>(raw)
    {
        let q = auto_di::resolve::<FlashQueries>()
            .await
            .map_err(|e| ExecError::CommandFailed {
                code: None,
                stderr: format!("Failed to resolve FlashQueries: {}", e),
            })?;
        for value in parsed {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                continue;
            }
            let resolved = match trimmed.parse::<i64>() {
                Ok(id) => q
                    .database_networks_get_by_id(id)
                    .await
                    .map(|network| network.docker_network_name)
                    .unwrap_or_else(|_| trimmed.to_string()),
                Err(_) => trimmed.to_string(),
            };
            let resolved = resolved.trim();
            if !resolved.is_empty() {
                names.insert(resolved.to_string());
            }
        }
    }

    let service_networks = names.iter().cloned().collect();
    let networks = names
        .into_iter()
        .map(|name| {
            (
                name.clone(),
                ExternalNetwork {
                    external: true,
                    name,
                },
            )
        })
        .collect();

    Ok((service_networks, networks))
}
