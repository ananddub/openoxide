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
        let (networks, _) = crate::utils::builder::database_networks::resolve_database_networks(
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
        let mut spec =
            ApplicationSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        // For the AI-Dockerfile strategy, resolve the org's enabled AI provider
        // now (we have DI/DB access here; the sync mapper did not) so the
        // builder can call the model without a DB round-trip.
        if let Some(crate::utils::builder::spec::BuildStrategy::AiDockerfile { provider }) =
            spec.build.as_mut()
            && provider.is_none()
        {
            *provider = resolve_ai_provider(&self.queries, application_id).await;
        }

        Ok(spec)
    }
}

/// Resolve the organization's first enabled AI provider for an application.
async fn resolve_ai_provider(
    queries: &FlashQueries,
    application_id: i64,
) -> Option<crate::utils::ai::AiProviderConfig> {
    let ctx = queries
        .services_get_deployment_context(application_id)
        .await
        .ok()?;
    let project = queries.get_project_by_i_d(ctx.project_id).await.ok()?;
    let org_id = project.organization_id;
    let settings = queries
        .ai_settings_list_by_organization(org_id)
        .await
        .ok()?;
    let s = settings.into_iter().find(|s| s.is_enabled != 0)?;
    Some(crate::utils::ai::AiProviderConfig {
        api_url: s.api_url,
        api_key: s.api_key,
        model: s.model,
    })
}
