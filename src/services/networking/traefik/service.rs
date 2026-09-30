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

    /// Writes (or removes) the panel's own Traefik router so the dashboard is
 /// reachable on its configured domain — the equivalent of the standard 
    /// `updateServerTraefik`. Called after host settings change. A blank host
    /// removes the config. HTTPS adds a redirect-to-https on `web` plus a
    /// `websecure` router (with the letsencrypt/custom resolver).
    pub async fn update_panel_domain(
        &self,
        host: Option<&str>,
        https: bool,
        certificate_type: &str,
        custom_cert_resolver: Option<&str>,
    ) -> Result<(), String> {
        let base = Self::get_traefik_base_path();
        let config_path = format!("{base}/dynamic/openoxide.yml");
        let executor = self.get_executor(None).await?;
        let os = crate::utils::os::OsCli::new(&executor);

        let Some(host) = host.map(str::trim).filter(|h| !h.is_empty()) else {
            let _ = os.file(&config_path).delete().run().await;
            return Ok(());
        };
        let host = crate::utils::builder::shared::traefik::to_punycode(host);

        let service_url = std::env::var("OPENOXIDE_SERVER_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| {
                let port = std::env::var("PORT").unwrap_or_else(|_| "4000".into());
                format!("http://openoxide:{port}")
            });

        let mut routers = format!(
            "    openoxide-router-app:\n      rule: \"Host(`{host}`)\"\n      service: openoxide-service-app\n      entryPoints:\n        - web\n"
        );
        if https {
            routers.push_str("      middlewares:\n        - redirect-to-https\n");
            let tls = match certificate_type.to_ascii_uppercase().as_str() {
                "LETSENCRYPT" => "      tls:\n        certResolver: letsencrypt\n".to_string(),
                "CUSTOM" => custom_cert_resolver
                    .map(|r| format!("      tls:\n        certResolver: {r}\n"))
                    .unwrap_or_else(|| "      tls: {}\n".to_string()),
                _ => "      tls: {}\n".to_string(),
            };
            routers.push_str(&format!(
                "    openoxide-router-app-secure:\n      rule: \"Host(`{host}`)\"\n      service: openoxide-service-app\n      entryPoints:\n        - websecure\n{tls}"
            ));
        }
        let yaml = format!(
            "http:\n  routers:\n{routers}  services:\n    openoxide-service-app:\n      loadBalancer:\n        servers:\n          - url: \"{service_url}\"\n        passHostHeader: true\n"
        );

        if let Some(parent) = std::path::Path::new(&config_path).parent() {
            let _ = os.dir(parent.to_string_lossy().as_ref()).create().parents(true).run().await;
        }
        os.file(&config_path)
            .write(&yaml)
            .execute()
            .await
            .map_err(|e| format!("write panel traefik config: {e}"))?;
        Ok(())
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
