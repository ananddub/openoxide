use crate::repository::{
    ApplicationMiddlewareRepository, ApplicationRepository, DomainRepository, MountRepository,
    PatchRepository, PortRepository, RedirectRepository, SecurityRepository,
};
use crate::utils::builder::spec::ApplicationSpec;
use auto_di::singleton;
use std::sync::Arc;

use super::mapper::AppRowWithRelations;

#[derive(Clone)]
pub struct ApplicationSpecAdapter {
    app_repo: Arc<ApplicationRepository>,
    domain_repo: Arc<DomainRepository>,
    mount_repo: Arc<MountRepository>,
    patch_repo: Arc<PatchRepository>,
    port_repo: Arc<PortRepository>,
    redirect_repo: Arc<RedirectRepository>,
    security_repo: Arc<SecurityRepository>,
    middleware_repo: Arc<ApplicationMiddlewareRepository>,
}

#[singleton]
impl ApplicationSpecAdapter {
    pub fn new(
        app_repo: Arc<ApplicationRepository>,
        domain_repo: Arc<DomainRepository>,
        mount_repo: Arc<MountRepository>,
        patch_repo: Arc<PatchRepository>,
        port_repo: Arc<PortRepository>,
        redirect_repo: Arc<RedirectRepository>,
        security_repo: Arc<SecurityRepository>,
        middleware_repo: Arc<ApplicationMiddlewareRepository>,
    ) -> Self {
        Self {
            app_repo,
            domain_repo,
            mount_repo,
            patch_repo,
            port_repo,
            redirect_repo,
            security_repo,
            middleware_repo,
        }
    }

    pub async fn load(&self, application_id: i64) -> sqlx::Result<ApplicationSpec> {
        let app = self.app_repo.get_spec_row(application_id).await?;
        let (networks, _) = crate::utils::builder::database::builder::resolve_database_networks(
            Some(&app.network_ids),
            app.detach_rustploy_network,
        )
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let domains = self
            .domain_repo
            .list_by_application_raw(application_id)
            .await?;
        let mounts = self
            .mount_repo
            .fetch_for_application(application_id)
            .await?;
        let patches = self.patch_repo.list_by_application(application_id).await?;
        let ports = self.port_repo.list_by_application(application_id).await?;
        let redirects = self
            .redirect_repo
            .list_by_application(application_id)
            .await?;
        let security = self
            .security_repo
            .list_by_application(application_id)
            .await?;
        let middlewares = self
            .middleware_repo
            .list_by_application(application_id)
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
