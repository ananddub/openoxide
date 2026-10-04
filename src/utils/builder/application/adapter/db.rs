use crate::db::queries::FlashQueries;
use crate::utils::builder::spec::ApplicationSpec;
use auto_di::singleton;
use std::sync::Arc;

use super::mapper::AppRowWithRelations;

#[derive(Clone)]
pub struct ApplicationSpecAdapter {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ApplicationSpecAdapter {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn load(&self, application_id: i64) -> sqlx::Result<ApplicationSpec> {
        let app = self
            .queries
            .application_get_spec_row(application_id)
            .await?;
        let (networks, _) = crate::utils::builder::database::builder::resolve_database_networks(
            Some(&app.network_ids),
            app.detach_rustploy_network,
        )
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let domains = self
            .queries
            .domains_list_by_application_raw(application_id)
            .await?;
        let mounts = self
            .queries
            .mount_fetch_for_application(application_id)
            .await?;
        let patches = self
            .queries
            .patches_list_by_application(application_id)
            .await?;
        let ports = self
            .queries
            .ports_list_by_application(application_id)
            .await?;
        let redirects = self
            .queries
            .list_redirects_by_application(application_id)
            .await?;
        let security = self
            .queries
            .security_list_by_application(application_id)
            .await?;
        let middlewares = self
            .queries
            .application_middlewares_list_by_application(application_id)
            .await?;

        let data = AppRowWithRelations {
            app,
            domains,
            mounts,
            patches,
            ports,
            redirects,
            security,
            middlewares,
            networks,
        };
        ApplicationSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))
    }
}
