use auto_di::singleton;

use super::middleware::render_structured_middleware;
use super::types::{
    StructuredMiddlewareDto, StructuredMiddlewareResponseDto, TraefikVersionDto,
    UpdateTraefikVersionDto,
};

pub struct TraefikService;

#[singleton]
impl TraefikService {
    fn new() -> Self {
        Self
    }

    /// Boot-time repair of the panel host's edge: rewrite a corrupt/missing
    /// Traefik static config and make sure the `openoxide-traefik` container
    /// exists and runs. Without this, domains attached to applications and
    /// compose stacks route to nothing.
    pub async fn ensure_local_edge(&self) -> Result<(), String> {
        let executor = self.get_executor(None).await?;
        let config = crate::utils::setup::SetupConfig::default();
        let setup = crate::utils::setup::ServerSetup::new(executor, config);
        setup.write_traefik_config().await.map_err(|e| e.to_string())?;
        setup.ensure_traefik().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn structured_middleware(
        input: StructuredMiddlewareDto,
    ) -> Result<StructuredMiddlewareResponseDto, String> {
        render_structured_middleware(input)
    }

    pub async fn version(&self, server_id: Option<i64>) -> Result<TraefikVersionDto, String> {
        let executor = self.get_executor(server_id).await?;
        let inspect = crate::utils::docker::DockerCli::from_executor(executor)
            .containers()
            .inspect("openoxide-traefik")
            .await
            .map_err(|error| error.to_string())?;
        Ok(TraefikVersionDto {
            server_id,
            current_image: inspect.config.image,
            desired_version: "3.6.7".into(),
        })
    }

    pub async fn update_version(
        &self,
        input: UpdateTraefikVersionDto,
    ) -> Result<TraefikVersionDto, String> {
        let version = input.version.trim().trim_start_matches('v');
        if version.is_empty()
            || !version
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-'))
        {
            return Err("invalid Traefik version".into());
        }
        let executor = self.get_executor(input.server_id).await?;
        let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
        docker
            .images()
            .pull(format!("traefik:v{version}"))
            .pull()
            .await
            .map_err(|error| error.to_string())?;
        let _ = docker
            .containers()
            .rm("openoxide-traefik")
            .force()
            .run()
            .await;
        let mut config = crate::utils::setup::SetupConfig::default();
        config.traefik_version = version.to_owned();
        crate::utils::setup::ServerSetup::new(executor, config)
            .ensure_traefik()
            .await
            .map_err(|error| error.to_string())?;
        self.version(input.server_id).await
    }
}
