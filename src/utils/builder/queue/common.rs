use auto_di::resolve;

use crate::{
    services::{
        application::ApplicationOperation,
        compose::ComposeOperation,
        compose::ComposeStatus,
        database::DatabaseOperation,
        notification::{
            NotificationLevel, NotificationMessage, NotificationScope, NotificationService,
            NotificationTrigger,
        },
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

        if let Err(e) = repo.finalize_with_resource(deployment_id, final_status, error_message.as_deref(), application_id, compose_id, database_id, database_kind.as_deref(), target_status).await {
            tracing::error!(deployment_id, error = %e, "builder queue: could not atomically persist deployment outcome");
        }

        if let Some(app_id) = application_id {
            self.cache
                .invalidate(&crate::core::cache::CacheKey::Application(app_id))
                .await;
            self.application_state.remove_state(IdType::AppId(app_id));
        }

        if let Some(cmp_id) = compose_id {
            self.cache
                .invalidate(&crate::core::cache::CacheKey::Compose(cmp_id))
                .await;
            self.application_state
                .remove_state(IdType::ComposeId(cmp_id));
        }

        if let (Some(db_id), Some(_db_kind)) = (database_id, database_kind.as_deref()) {
            self.cache
                .invalidate(&crate::core::cache::CacheKey::Database(db_id))
                .await;
            self.application_state
                .remove_state(IdType::DatabaseId(db_id));
        }

        tracing::info!(
            deployment_id,
            status = final_status,
            "builder queue: job finished"
        );

        self.notify_deployment_finished(
            deployment_id,
            application_id,
            compose_id,
            database_id,
            database_kind.as_deref(),
            &operation,
            final_status,
            error_message.as_deref(),
        )
        .await;
    }

    /// Sends a deployment outcome to whichever notification channels asked for
    /// it.
    ///
    /// Called after the status has already been persisted, and every failure
    /// path only logs: a dead webhook must never turn a successful deployment
    /// into a reported failure.
    #[allow(clippy::too_many_arguments)]
    async fn notify_deployment_finished(
        &self,
        deployment_id: i64,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        database_kind: Option<&str>,
        operation: &str,
        final_status: &str,
        error_message: Option<&str>,
    ) {
        // A cancellation is a deliberate user action, not an event worth
        // paging anyone about.
        let Some(trigger) = deployment_notification_trigger(final_status) else {
            return;
        };
        let succeeded = final_status == "DONE";

        let service = match resolve::<NotificationService>().await {
            Ok(service) => service,
            Err(error) => {
                tracing::error!(error = %error, "could not resolve NotificationService");
                return;
            }
        };

        let (kind, name) = self
            .describe_deployment_target(application_id, compose_id, database_id, database_kind)
            .await;

        // Without an owning organization the alert would have to fan out to
        // every tenant, so stay silent instead.
        let Some(organization_id) = self
            .deployment_organization_id(application_id, compose_id, database_id, database_kind)
            .await
        else {
            tracing::warn!(
                deployment_id,
                "could not resolve owning organization, skipping notification"
            );
            return;
        };

        let title = if succeeded {
            format!("{kind} {operation} succeeded: {name}")
        } else {
            format!("{kind} {operation} failed: {name}")
        };

        let body = match error_message {
            Some(error) => error.to_string(),
            None => format!("{kind} {name} finished {operation} successfully."),
        };

        let mut message = NotificationMessage::new(title, body)
            .level(if succeeded {
                NotificationLevel::Info
            } else {
                NotificationLevel::Critical
            })
            .field("Resource", format!("{kind} {name}"))
            .field("Operation", operation)
            .field("Status", final_status)
            .field("Deployment", deployment_id.to_string());

        if let Some(error) = error_message {
            message = message.field("Error", error);
        }

        service
            .notify(
                NotificationScope::Organization(organization_id),
                trigger,
                &message,
            )
            .await;
    }

    /// Owning organization of whichever resource this deployment targeted.
    async fn deployment_organization_id(
        &self,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        database_kind: Option<&str>,
    ) -> Option<i64> {
        let repo = resolve::<crate::repository::DeploymentRepository>()
            .await
            .ok()?;

        let lookup = if let Some(app_id) = application_id {
            repo.application_organization_id(app_id).await
        } else if let Some(cmp_id) = compose_id {
            repo.compose_organization_id(cmp_id).await
        } else if let (Some(db_id), Some(kind)) = (database_id, database_kind) {
            repo.database_organization_id(db_id, kind).await
        } else {
            return None;
        };

        match lookup {
            Ok(id) => Some(id),
            Err(error) => {
                tracing::error!(error = %error, "could not resolve deployment organization");
                None
            }
        }
    }

    /// Resolves a human-readable "<kind> <name>" for the deployed resource,
    /// falling back to the id when the lookup fails — a notification with a
    /// weaker label is far better than no notification.
    async fn describe_deployment_target(
        &self,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        database_kind: Option<&str>,
    ) -> (&'static str, String) {
        if let Some(app_id) = application_id {
            let name = match resolve::<crate::repository::ApplicationRepository>().await {
                Ok(repo) => repo
                    .get_by_id(app_id)
                    .await
                    .ok()
                    .flatten()
                    .map(|app| app.name),
                Err(_) => None,
            };
            return ("Application", name.unwrap_or_else(|| app_id.to_string()));
        }

        if let Some(cmp_id) = compose_id {
            let name = match resolve::<crate::repository::ComposeProjectRepository>().await {
                Ok(repo) => repo
                    .get_by_id(cmp_id)
                    .await
                    .ok()
                    .flatten()
                    .map(|compose| compose.name),
                Err(_) => None,
            };
            return ("Compose", name.unwrap_or_else(|| cmp_id.to_string()));
        }

        if let (Some(db_id), Some(kind)) = (database_id, database_kind) {
            let name = match resolve::<crate::repository::DeploymentRepository>().await {
                Ok(repo) => repo.get_database_app_name(db_id, kind).await.ok(),
                Err(_) => None,
            };
            return ("Database", name.unwrap_or_else(|| db_id.to_string()));
        }

        ("Deployment", "unknown".to_string())
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

/// Which notification a finished deployment should raise, if any.
///
/// `CANCELLED` returns `None`: a cancellation is a deliberate user action, not
/// something worth paging anyone about.
pub fn deployment_notification_trigger(final_status: &str) -> Option<NotificationTrigger> {
    match final_status {
        "DONE" => Some(NotificationTrigger::AppDeploy),
        "CANCELLED" => None,
        _ => Some(NotificationTrigger::AppBuildError),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_successful_deployment_notifies_on_deploy() {
        assert_eq!(
            deployment_notification_trigger("DONE"),
            Some(NotificationTrigger::AppDeploy)
        );
    }

    #[test]
    fn a_failed_deployment_notifies_on_build_error() {
        assert_eq!(
            deployment_notification_trigger("ERROR"),
            Some(NotificationTrigger::AppBuildError)
        );
    }

    /// Cancelling is something the operator just did on purpose; telling them
    /// about it is noise.
    #[test]
    fn a_cancelled_deployment_notifies_nobody() {
        assert_eq!(deployment_notification_trigger("CANCELLED"), None);
    }

    /// An unrecognised status is treated as a failure rather than silently
    /// dropped — a deployment that ended in an unknown state is worth knowing
    /// about.
    #[test]
    fn an_unknown_status_is_treated_as_a_failure() {
        assert_eq!(
            deployment_notification_trigger("SOMETHING_ELSE"),
            Some(NotificationTrigger::AppBuildError)
        );
    }
}
