use std::{collections::BTreeSet, sync::Arc};

use auto_di::singleton;
use serde::Deserialize;

use crate::{
    core::cache::{AppStateCache, CacheKey},
    db::queries::FlashQueries,
    flash_gen::models::{DatabaseNetworks, Services},
    flash_gen::services::UpdateServiceNetworksParams,
};

#[derive(Debug, Deserialize)]
pub struct UpdateApplicationNetworksDto {
    pub network_ids: Vec<i64>,
    pub detach_rustploy_network: bool,
}

pub struct ApplicationNetworkService {
    queries: Arc<FlashQueries>,
    cache: Arc<AppStateCache>,
}

#[cfg(test)]
impl ApplicationNetworkService {
    pub(crate) fn for_tests(queries: Arc<FlashQueries>, cache: Arc<AppStateCache>) -> Self {
        Self { queries, cache }
    }
}

#[singleton]
impl ApplicationNetworkService {
    fn new(queries: Arc<FlashQueries>, cache: Arc<AppStateCache>) -> Self {
        Self { queries, cache }
    }

    async fn get_application(&self, application_id: i64) -> sqlx::Result<Services> {
        self.queries
            .app_sub_get_application_by_i_d(application_id)
            .await
    }

    async fn network_by_id(&self, id: i64) -> sqlx::Result<Option<DatabaseNetworks>> {
        match self.queries.database_networks_get_by_id(id).await {
            Ok(row) => Ok(Some(row)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn get(&self, application_id: i64) -> sqlx::Result<(bool, Vec<DatabaseNetworks>)> {
        let app = self.get_application(application_id).await?;
        let ids = parse_ids(&app.network_ids)?;
        let mut resolved = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(network) = self.network_by_id(id).await? {
                resolved.push(network);
            }
        }
        Ok((app.detach_rustploy_network != 0, resolved))
    }

    pub async fn update(
        &self,
        application_id: i64,
        input: UpdateApplicationNetworksDto,
    ) -> sqlx::Result<(bool, Vec<DatabaseNetworks>)> {
        let app = self.get_application(application_id).await?;
        let unique: BTreeSet<i64> = input.network_ids.into_iter().collect();
        let mut resolved = Vec::with_capacity(unique.len());
        for id in &unique {
            let network = self
                .network_by_id(*id)
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
                "at least one network is required when the OpenOxide network is detached".into(),
            ));
        }
        let serialized = serde_json::to_string(
            &unique
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>(),
        )
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.queries
            .update_service_networks(&UpdateServiceNetworksParams {
                network_ids: serialized,
                detach_rustploy_network: i64::from(input.detach_rustploy_network),
                id: application_id,
            })
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
