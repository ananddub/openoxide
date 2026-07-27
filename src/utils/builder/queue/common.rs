use auto_di::resolve;

use crate::{
    services::{
        application::ApplicationOperation, compose::ComposeOperation, compose::ComposeStatus, database::DatabaseOperation,
    },
    utils::builder::{
        custom_type::{DeployState, IdType},
        queue::queue::BuilderQueue,
        spec::BuilderEvent,
    },
};

impl BuilderQueue {
    pub(super) async fn process(
        &self,
        deployment_id: i64,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        database_kind: Option<String>,
        operation: String,
    ) {
        tracing::info!(
            deployment_id,
            ?application_id,
            ?compose_id,
            ?database_id,
            ?database_kind,
            %operation,
            "builder queue: starting job"
        );

        let result = match (
            application_id,
            compose_id,
            database_id,
            database_kind.as_deref(),
        ) {
            (Some(app_id), None, None, None) => {
                let op = parse_application_operation(&operation);
                self.execute_operation_app(app_id, deployment_id, op).await
            }
            (None, Some(cmp_id), None, None) => {
                let op = parse_compose_operation(&operation);
                self.execute_operation_compose(cmp_id, deployment_id, op)
                    .await
            }
            (None, None, Some(db_id), Some(db_kind)) => {
                let op = parse_database_operation(&operation);
                self.execute_operation_db(db_id, db_kind.to_string(), deployment_id, op)
                    .await
            }
            _ => Err(crate::utils::builder::errors::BuilderError::Execution(
                format!(
                    "deployment {deployment_id} must have exactly one of application_id, compose_id, or database_id/kind"
                ),
            )),
        };

        let final_status = match &result {
            Ok(()) => "DONE",
            Err(e) if is_cancelled_error(&e.to_string()) => "CANCELLED",
            Err(_) => "ERROR",
        };
        let error_message = result.err().map(|e| e.to_string());

        let repo = match resolve::<crate::repository::DeploymentRepository>().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve DeploymentRepository");
                return;
            }
        };

        if let Err(e) = repo
            .update_final_status(deployment_id, final_status, error_message.as_deref())
            .await
        {
            tracing::error!(deployment_id, error = %e, "builder queue: could not persist final deployment status");
        }

        let is_stop = operation.eq_ignore_ascii_case("stop");
        let target_status = if final_status == "DONE" {
            if is_stop {
                ComposeStatus::Stopped.as_str()
            } else {
                ComposeStatus::Running.as_str()
            }
        } else {
            ComposeStatus::Error.as_str()
        };

        if let Some(app_id) = application_id {
            if let Err(e) = repo.set_application_status(app_id, target_status).await {
                tracing::error!(deployment_id, app_id, error = %e, "builder queue: could not persist application status");
            }
            self.application_state.remove_state(IdType::AppId(app_id));
        }

        if let Some(cmp_id) = compose_id {
            if let Err(e) = repo.set_compose_status(cmp_id, target_status).await {
                tracing::error!(deployment_id, cmp_id, error = %e, "builder queue: could not persist compose status");
            }
            self.application_state
                .remove_state(IdType::ComposeId(cmp_id));
        }

        if let (Some(db_id), Some(db_kind)) = (database_id, database_kind.as_deref()) {
            if let Err(e) = repo
                .set_database_status(db_id, db_kind, target_status)
                .await
            {
                tracing::error!(deployment_id, db_id, db_kind, error = %e, "builder queue: could not persist database status");
            }
            self.application_state
                .remove_state(IdType::DatabaseId(db_id));
        }

        tracing::info!(
            deployment_id,
            status = final_status,
            "builder queue: job finished"
        );
    }
}

pub fn builder_event_state(event: &BuilderEvent) -> &'static str {
    match event {
        BuilderEvent::Preparing => "PREPARING",
        BuilderEvent::SourceReady => "SOURCE_READY",
        BuilderEvent::Building => "BUILDING",
        BuilderEvent::ImageReady => "IMAGE_READY",
        BuilderEvent::Deploying => "DEPLOYING",
        BuilderEvent::Routing => "ROUTING",
        BuilderEvent::HealthCheck => "HEALTH_CHECK",
        BuilderEvent::Deployed => "DEPLOYED",
        BuilderEvent::Cancelled => "CANCELLED",
        BuilderEvent::Message(_) => "MESSAGE",
        BuilderEvent::Failed(_) => "FAILED",
        BuilderEvent::RecoverAfterRestart => "RECOVER_AFTER_RESTART",
    }
}

pub fn builder_event_state_opt(event: &BuilderEvent) -> Option<DeployState> {
    match event {
        BuilderEvent::Preparing => Some(DeployState::Preparing),
        BuilderEvent::SourceReady => Some(DeployState::GitSuccess),
        BuilderEvent::Building => Some(DeployState::Building),
        BuilderEvent::ImageReady => Some(DeployState::BuildSuccess),
        BuilderEvent::Deploying | BuilderEvent::Routing => Some(DeployState::Deploying),
        BuilderEvent::HealthCheck => Some(DeployState::HealthCheck),
        BuilderEvent::Deployed => Some(DeployState::Deployed),
        BuilderEvent::Cancelled => Some(DeployState::StoppedByUser),
        BuilderEvent::Failed(error) => Some(DeployState::Failed(error.clone())),
        BuilderEvent::RecoverAfterRestart => Some(DeployState::RecoverAfterRestart),
        BuilderEvent::Message(_) => None,
    }
}

pub fn is_cancelled_error(error: &str) -> bool {
    error.to_ascii_lowercase().contains("cancel")
}

fn parse_application_operation(value: &str) -> ApplicationOperation {
    match value {
        "redeploy" => ApplicationOperation::Redeploy,
        "rebuild" => ApplicationOperation::Rebuild,
        "reload" => ApplicationOperation::Reload,
        "start" => ApplicationOperation::Start,
        _ => ApplicationOperation::Deploy,
    }
}

fn parse_compose_operation(value: &str) -> ComposeOperation {
    match value {
        "redeploy" => ComposeOperation::Redeploy,
        "reload" => ComposeOperation::Reload,
        "start" => ComposeOperation::Start,
        "stop" => ComposeOperation::Stop,
        _ => ComposeOperation::Deploy,
    }
}

fn parse_database_operation(value: &str) -> DatabaseOperation {
    match value {
        "redeploy" => DatabaseOperation::Redeploy,
        "reload" => DatabaseOperation::Reload,
        "start" => DatabaseOperation::Start,
        "stop" => DatabaseOperation::Stop,
        _ => DatabaseOperation::Deploy,
    }
}
