use auto_di::{resolve, singleton};
use serde::Serialize;
use std::sync::Arc;

use crate::{
    repository::WebhookRepository,
    services::{
        application::{ApplicationOperation, ApplicationService},
        compose::{ComposeOperation, ComposeService},
        preview::PreviewDeploymentService,
    },
    utils::watch_paths::should_deploy,
};

use super::{GitTrigger, PullRequestEvent, PushEvent};

#[derive(Debug, Default, Serialize, poem_openapi::Object)]
pub struct DispatchOutcome {
    pub event_type: String,
    pub provider: Option<String>,
    pub owner: Option<String>,
    pub repository: Option<String>,
    pub applications_queued: usize,
    pub compose_projects_queued: usize,
    pub skipped_by_watch_paths: usize,
    pub already_running: usize,
    pub pull_request_number: Option<String>,
    pub pull_request_action: Option<String>,
    pub source_branch: Option<String>,
    pub source_owner: Option<String>,
    pub source_repository: Option<String>,
    pub target_branch: Option<String>,
    pub commit: Option<String>,
    pub author: Option<String>,
    pub previews_created: usize,
    pub previews_redeployed: usize,
    pub previews_removed: usize,
    pub previews_skipped_permission: usize,
    pub previews_skipped_limit: usize,
}

pub struct WebhookDispatcher {
    repository: Arc<WebhookRepository>,
}

#[singleton]
impl WebhookDispatcher {
    fn new(repository: Arc<WebhookRepository>) -> Self {
        Self { repository }
    }

    pub async fn dispatch(&self, event: &PushEvent) -> Result<DispatchOutcome, String> {
        let applications = self
            .repository
            .matching_applications(event)
            .await
            .map_err(|error| error.to_string())?;
        let compose_projects = self
            .repository
            .matching_compose_projects(event)
            .await
            .map_err(|error| error.to_string())?;
        let app_service = resolve::<ApplicationService>()
            .await
            .map_err(|error| error.to_string())?;
        let compose_service = resolve::<ComposeService>()
            .await
            .map_err(|error| error.to_string())?;
        let mut outcome = DispatchOutcome::default();
        outcome.event_type = event.trigger.as_str().to_ascii_lowercase();
        outcome.provider = Some(event.provider.as_str().into());
        outcome.owner = Some(event.owner.clone());
        outcome.repository = Some(event.repository.clone());

        for resource in applications {
            if event.trigger == GitTrigger::Push
                && !should_deploy(resource.watch_paths.as_deref(), &event.changed_paths)?
            {
                outcome.skipped_by_watch_paths += 1;
                continue;
            }
            match app_service
                .run_operation(resource.id, ApplicationOperation::Redeploy)
                .await
            {
                Ok(_) => outcome.applications_queued += 1,
                Err(error) if error.to_string().contains("already queued or running") => {
                    outcome.already_running += 1;
                }
                Err(error) => return Err(error.to_string()),
            }
        }

        for resource in compose_projects {
            if event.trigger == GitTrigger::Push
                && !should_deploy(resource.watch_paths.as_deref(), &event.changed_paths)?
            {
                outcome.skipped_by_watch_paths += 1;
                continue;
            }
            match compose_service
                .run_operation(resource.id, ComposeOperation::Redeploy)
                .await
            {
                Ok(_) => outcome.compose_projects_queued += 1,
                Err(error) if error.to_string().contains("already queued or running") => {
                    outcome.already_running += 1;
                }
                Err(error) => return Err(error.to_string()),
            }
        }

        Ok(outcome)
    }

    pub async fn dispatch_pull_request(
        &self,
        event: &PullRequestEvent,
    ) -> Result<DispatchOutcome, String> {
        let lifecycle = resolve::<PreviewDeploymentService>()
            .await
            .map_err(|error| error.to_string())?
            .handle_pull_request(event)
            .await?;
        Ok(DispatchOutcome {
            event_type: "pull_request".into(),
            provider: Some(event.provider.as_str().into()),
            owner: Some(event.owner.clone()),
            repository: Some(event.repository.clone()),
            pull_request_number: Some(event.number.clone()),
            pull_request_action: Some(event.action.clone()),
            source_branch: Some(event.source_branch.clone()),
            source_owner: event.source_owner.clone(),
            source_repository: event.source_repository.clone(),
            target_branch: Some(event.target_branch.clone()),
            commit: event.commit.clone(),
            author: event.author.clone(),
            previews_created: lifecycle.created,
            previews_redeployed: lifecycle.redeployed,
            previews_removed: lifecycle.removed,
            previews_skipped_permission: lifecycle.skipped_permission,
            previews_skipped_limit: lifecycle.skipped_limit,
            already_running: lifecycle.already_running,
            ..Default::default()
        })
    }
}
