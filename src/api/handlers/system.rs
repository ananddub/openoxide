use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};
use sqlx::SqlitePool;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, SystemService, SystemServiceExt,
};
use crate::core::config::Config;
use crate::utils::docker::DockerCli;

pub struct SystemController {
    db: Arc<SqlitePool>,
    config: Arc<Config>,
}

#[singleton]
impl SystemController {
    pub fn new(db: Arc<SqlitePool>, config: Arc<Config>) -> Self {
        Self { db, config }
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl SystemService for SystemController {
    #[permission(Organization, CanRead)]
    async fn health<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
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

        let docker_healthy = match crate::services::compose::remote::agent_executor_for(None).await
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

        Response::ok(OperationResult {
            success: healthy,
            operation: "health".into(),
            message: if healthy { Some("all dependencies healthy".into()) } else { Some(messages.join(", ")) },
            ..Default::default()
        })
    }

    #[permission(Organization, CanWrite)]
    async fn test_config<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
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

        Response::ok(OperationResult {
            success: errors.is_empty(),
            operation: "test_config".into(),
            message: if details.is_empty() { Some("configuration is valid".into()) } else { Some(details.join(", ")) },
            ..Default::default()
        })
    }
}
