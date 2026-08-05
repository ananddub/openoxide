use auto_di::resolve;

use crate::services::{git_provider::GitProviderService, webhook::PullRequestEvent};

use super::{PreviewDeploymentOutcome, PreviewDeploymentService, PreviewLifecycleOutcome};

impl PreviewDeploymentService {
    pub async fn handle_pull_request(
        &self,
        event: &PullRequestEvent,
    ) -> Result<PreviewLifecycleOutcome, String> {
        let _guard = self.lifecycle_lock.lock().await;
        if is_close_action(&event.action) {
            let rows = self
                .previews
                .find_open_for_event(
                    event.provider.as_str(),
                    &event.owner,
                    &event.repository,
                    &event.number,
                )
                .await
                .map_err(|error| error.to_string())?;
            let mut outcome = PreviewLifecycleOutcome::default();
            for row in rows {
                self.remove_unlocked(row.id, false).await?;
                outcome.removed += 1;
            }
            return Ok(outcome);
        }

        let targets = self
            .previews
            .matching_targets(
                event.provider.as_str(),
                &event.owner,
                &event.repository,
                &event.target_branch,
            )
            .await
            .map_err(|error| error.to_string())?;
        let providers = resolve::<GitProviderService>()
            .await
            .map_err(|error| error.to_string())?;
        let mut outcome = PreviewLifecycleOutcome::default();

        for target in targets {
            if target.require_collaborator_permissions {
                let Some(author) = event.author.as_deref() else {
                    outcome.skipped_permission += 1;
                    continue;
                };
                let permission = providers
                    .collaborator_permission(
                        target.provider_id,
                        &event.owner,
                        &event.repository,
                        author,
                    )
                    .await?;
                if !permission.can_write {
                    outcome.skipped_permission += 1;
                    continue;
                }
            }

            let existing = self
                .previews
                .find_for_pull_request(
                    target.base_application_id,
                    event.provider.as_str(),
                    &event.number,
                )
                .await
                .map_err(|error| error.to_string())?;
            if existing.is_none()
                && self
                    .previews
                    .active_count(target.base_application_id)
                    .await
                    .map_err(|error| error.to_string())?
                    >= target.preview_limit.max(1)
            {
                outcome.skipped_limit += 1;
                continue;
            }

            match self.deploy_target(&target, event).await {
                Ok((_, true)) => outcome.created += 1,
                Ok((_, false)) => outcome.redeployed += 1,
                Err(error) if error.contains("already queued or running") => {
                    outcome.already_running += 1;
                }
                Err(error) => return Err(error),
            }
        }
        Ok(outcome)
    }

    pub async fn deploy_application(
        &self,
        base_application_id: i64,
        event: &PullRequestEvent,
    ) -> Result<PreviewDeploymentOutcome, String> {
        let _guard = self.lifecycle_lock.lock().await;
        let target = self
            .previews
            .matching_targets(
                event.provider.as_str(),
                &event.owner,
                &event.repository,
                &event.target_branch,
            )
            .await
            .map_err(|error| error.to_string())?
            .into_iter()
            .find(|target| target.base_application_id == base_application_id)
            .ok_or("Application is not configured for this pull request preview")?;
        let existing = self
            .previews
            .find_for_pull_request(base_application_id, event.provider.as_str(), &event.number)
            .await
            .map_err(|error| error.to_string())?;
        if existing.as_ref().is_none_or(|row| row.status == "CLOSED")
            && self
                .previews
                .active_count(base_application_id)
                .await
                .map_err(|error| error.to_string())?
                >= target.preview_limit.max(1)
        {
            return Err("Preview deployment limit reached".into());
        }
        self.deploy_target(&target, event)
            .await
            .map(|(outcome, _)| outcome)
    }

    pub async fn redeploy(&self, id: i64) -> Result<PreviewDeploymentOutcome, String> {
        let row = self
            .previews
            .get_by_id(id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("Preview deployment not found")?;
        if row.status == "CLOSED" {
            return Err("Closed preview deployment cannot be redeployed".into());
        }
        let event = PullRequestEvent {
            provider: provider_kind(&row.provider_type)?,
            owner: row.owner,
            repository: row.repository,
            number: row.pull_request_number,
            action: "update".into(),
            source_branch: row.source_branch,
            source_owner: row.source_owner,
            source_repository: row.source_repository,
            target_branch: row.target_branch,
            commit: row.commit_sha,
            author: row.author,
        };
        self.deploy_application(row.base_application_id, &event)
            .await
    }
}

fn is_close_action(action: &str) -> bool {
    matches!(
        action.to_ascii_lowercase().as_str(),
        "close" | "closed" | "merge" | "merged" | "fulfilled" | "rejected" | "declined"
    )
}

fn provider_kind(value: &str) -> Result<crate::services::webhook::GitProviderKind, String> {
    match value.to_ascii_lowercase().as_str() {
        "github" => Ok(crate::services::webhook::GitProviderKind::Github),
        "gitlab" => Ok(crate::services::webhook::GitProviderKind::Gitlab),
        "gitea" => Ok(crate::services::webhook::GitProviderKind::Gitea),
        "bitbucket" => Ok(crate::services::webhook::GitProviderKind::Bitbucket),
        _ => Err("Unsupported preview provider".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_provider_close_actions_cleanup_previews() {
        for action in ["closed", "merged", "fulfilled", "rejected", "declined"] {
            assert!(is_close_action(action));
        }
        assert!(!is_close_action("synchronize"));
    }
}
