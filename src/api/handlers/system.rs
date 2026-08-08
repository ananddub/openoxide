use crate::{
    api::dto::system::{ConfigTestDto, DependencyStatusDto, SystemHealthDto},
    core::{
        config::Config,
        middleware::permission::{CanRead, CanWrite, Organization, RequirePermission},
    },
    utils::docker::DockerCli,
};
use auto_route::controller;
use axum::Json;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct SystemController {
    db: Arc<SqlitePool>,
    config: Arc<Config>,
}

#[controller("/system")]
impl SystemController {
    fn new(db: Arc<SqlitePool>, config: Arc<Config>) -> Self {
        Self { db, config }
    }

    #[get("/health")]
    async fn health(
        &self,
        RequirePermission(_claims, _): RequirePermission<Organization, CanRead>,
    ) -> Json<SystemHealthDto> {
        let database = match sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(self.db.as_ref())
            .await
        {
            Ok(_) => DependencyStatusDto {
                name: "database".into(),
                healthy: true,
                detail: "SQLite query succeeded".into(),
            },
            Err(e) => DependencyStatusDto {
                name: "database".into(),
                healthy: false,
                detail: e.to_string(),
            },
        };
        let docker = match DockerCli::new_local().system().info().run().await {
            Ok(_) => DependencyStatusDto {
                name: "docker".into(),
                healthy: true,
                detail: "Docker API reachable".into(),
            },
            Err(e) => DependencyStatusDto {
                name: "docker".into(),
                healthy: false,
                detail: e.to_string(),
            },
        };
        let dependencies = vec![database, docker];
        Json(SystemHealthDto {
            healthy: dependencies.iter().all(|d| d.healthy),
            dependencies,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }

    #[get("/config/test")]
    async fn config_test(
        &self,
        RequirePermission(_claims, _): RequirePermission<Organization, CanWrite>,
    ) -> Json<ConfigTestDto> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        if self.config.secret_key == "your_secret_key_here" || self.config.secret_key.len() < 32 {
            errors.push("SECRET_KEY must be changed and contain at least 32 characters".into());
        }
        if self.config.metrics_token.is_empty() {
            warnings.push("METRICS_TOKEN is empty; monitoring-agent ingestion is disabled".into());
        }
        if self.config.host.trim().is_empty() {
            errors.push("HOST cannot be empty".into());
        }
        Json(ConfigTestDto {
            valid: errors.is_empty(),
            errors,
            warnings,
        })
    }
}
