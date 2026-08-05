use std::{collections::BTreeSet, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::application::network::UpdateApplicationNetworksDto,
    core::cache::{AppStateCache, CacheKey},
    db::models::database_networks::DatabaseNetwork,
    repository::{ApplicationRepository, DatabaseNetworkRepository},
};

pub struct ApplicationNetworkService {
    applications: Arc<ApplicationRepository>,
    networks: Arc<DatabaseNetworkRepository>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ApplicationNetworkService {
    fn new(
        applications: Arc<ApplicationRepository>,
        networks: Arc<DatabaseNetworkRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            applications,
            networks,
            cache,
        }
    }

    pub async fn get(&self, application_id: i64) -> sqlx::Result<(bool, Vec<DatabaseNetwork>)> {
        let app = self
            .applications
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let ids = parse_ids(&app.network_ids)?;
        let mut resolved = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(network) = self.networks.get_by_id(id).await? {
                resolved.push(network);
            }
        }
        Ok((app.detach_rustploy_network != 0, resolved))
    }

    pub async fn update(
        &self,
        application_id: i64,
        input: UpdateApplicationNetworksDto,
    ) -> sqlx::Result<(bool, Vec<DatabaseNetwork>)> {
        let app = self
            .applications
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let unique: BTreeSet<i64> = input.network_ids.into_iter().collect();
        let mut resolved = Vec::with_capacity(unique.len());
        for id in &unique {
            let network = self
                .networks
                .get_by_id(*id)
                .await?
                .ok_or_else(|| sqlx::Error::Protocol(format!("network {id} not found")))?;
            if network.server_id != app.server_id {
                return Err(sqlx::Error::Protocol(format!(
                    "network {id} belongs to a different server"
                )));
            }
            resolved.push(network);
        }
        if input.detach_rustploy_network && resolved.is_empty() {
            return Err(sqlx::Error::Protocol(
                "at least one network is required when the Rustploy network is detached".into(),
            ));
        }
        let serialized = serde_json::to_string(
            &unique
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>(),
        )
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.applications
            .update_networks(
                application_id,
                &serialized,
                i64::from(input.detach_rustploy_network),
            )
            .await?;
        self.cache
            .invalidate(&CacheKey::Application(application_id))
            .await;
        Ok((input.detach_rustploy_network, resolved))
    }
}

fn parse_ids(value: &str) -> sqlx::Result<Vec<i64>> {
    let values: Vec<String> = serde_json::from_str(value).map_err(|error| {
        sqlx::Error::Protocol(format!("invalid application network state: {error}"))
    })?;
    values
        .into_iter()
        .map(|value| {
            value.parse::<i64>().map_err(|_| {
                sqlx::Error::Protocol("application network state contains a non-numeric id".into())
            })
        })
        .collect()
}
