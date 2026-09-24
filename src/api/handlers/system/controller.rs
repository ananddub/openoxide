use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    SystemHealthRequest, SystemHealthResponse, SystemService, SystemServiceExt,
    SystemTestConfigRequest, SystemTestConfigResponse,
};
use crate::utils::docker::DockerCli;

use super::init::SystemController;

#[auto_grpc::service(connect = true, auth = true)]
impl SystemService for SystemController {
    #[permission(Organization, CanRead)]
    async fn health<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, SystemHealthRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SystemHealthResponse> + Send + use<'a>> {
        let database_healthy = match sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(self.db.as_ref())
            .await
        {
            Ok(_) => true,
            Err(e) => {
                tracing::error!(error = %e, "database health check failed");
                false
            }
        };

        let docker_healthy = match crate::services::compute::compose::remote::agent_executor_for(None).await
        {
            Ok(executor) => match DockerCli::from_executor(executor)
                .system()
                .info()
                .run()
                .await
            {
                Ok(_) => true,
                Err(error) => {
                    tracing::error!(error = %error, "docker health check failed");
                    false
                }
            },
            Err(error) => {
                tracing::error!(error = %error, "docker agent unavailable");
                false
            }
        };

        let healthy = database_healthy && docker_healthy;
        let mut messages: Vec<String> = Vec::new();
        if !database_healthy {
            messages.push("database: unhealthy".into());
        }
        if !docker_healthy {
            messages.push("docker: unreachable".into());
        }

        Response::ok(SystemHealthResponse {
            success: healthy,
            message: Some(if healthy {
                "all dependencies healthy".into()
            } else {
                messages.join(", ")
            }),
            ..Default::default()
        })
    }

    #[permission(Organization, CanWrite)]
    async fn test_config<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, SystemTestConfigRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SystemTestConfigResponse> + Send + use<'a>> {
        let mut errors: Vec<String> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();
        if self.config.secret_key == "your_secret_key_here" || self.config.secret_key.len() < 32 {
            errors.push("SECRET_KEY must be changed and contain at least 32 characters".into());
        }
        if self.config.metrics_token.is_empty() {
            warnings.push("METRICS_TOKEN is empty; monitoring-agent ingestion is disabled".into());
        }
        if self.config.host.trim().is_empty() {
            errors.push("HOST cannot be empty".into());
        }

        let mut details: Vec<String> = Vec::new();
        if !errors.is_empty() {
            details.push(format!("errors: {}", errors.join("; ")));
        }
        if !warnings.is_empty() {
            details.push(format!("warnings: {}", warnings.join("; ")));
        }

        Response::ok(SystemTestConfigResponse {
            success: errors.is_empty(),
            message: Some(if details.is_empty() {
                "configuration is valid".into()
            } else {
                details.join(", ")
            }),
            ..Default::default()
        })
    }
}
