use auto_di::resolve;

use crate::{
    flash_gen::preview_deployments::{
        PreviewDeploymentsFindForPullRequestParams, PreviewDeploymentsFindOpenForEventParams,
        PreviewDeploymentsMatchingTargets1Params, PreviewDeploymentsMatchingTargets2Params,
        PreviewDeploymentsMatchingTargets3Params, PreviewDeploymentsMatchingTargets4Params,
    },
    services::{git_provider::GitProviderService, webhook::PullRequestEvent},
};

use super::{
    types::{optional_field, PreviewTarget},
    PreviewDeploymentOutcome, PreviewDeploymentService, PreviewLifecycleOutcome,
};

impl PreviewDeploymentService {
    pub async fn handle_pull_request(
        &self,
        event: &PullRequestEvent,
    ) -> Result<PreviewLifecycleOutcome, String> {
        let _guard = self.lifecycle_lock.lock().await;
        if is_close_action(&event.action) {
            let rows = self
                .queries
                .preview_deployments_find_open_for_event(&PreviewDeploymentsFindOpenForEventParams {
                    provider_type: event.provider.as_str().to_string(),
                    owner: event.owner.clone(),
                    repository: event.repository.clone(),
                    pull_request_number: event.number.clone(),
                })
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
            .matching_targets(
                event.provider,
                &event.owner,
                &event.repository,
                &event.target_branch,
            )
            .await?;
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

            let existing = match self
                .queries
                .preview_deployments_find_for_pull_request(&PreviewDeploymentsFindForPullRequestParams {
                    base_service_id: target.base_service_id,
                    provider_type: event.provider.as_str().to_string(),
                    pull_request_number: event.number.clone(),
                })
                .await
            {
                Ok(row) => Some(row),
                Err(sqlx::Error::RowNotFound) => None,
                Err(error) => return Err(error.to_string()),
            };
            if existing.is_none()
                && self
                    .queries
                    .preview_deployments_active_count(target.base_service_id)
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
            .matching_targets(
                event.provider,
                &event.owner,
                &event.repository,
                &event.target_branch,
            )
            .await?
            .into_iter()
            .find(|target| target.base_service_id == base_application_id)
            .ok_or("Application is not configured for this pull request preview")?;
        let existing = match self
            .queries
            .preview_deployments_find_for_pull_request(&PreviewDeploymentsFindForPullRequestParams {
                base_service_id: base_application_id,
                provider_type: event.provider.as_str().to_string(),
                pull_request_number: event.number.clone(),
            })
            .await
        {
            Ok(row) => Some(row),
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error.to_string()),
        };
        if existing.as_ref().is_none_or(|row| row.status == "CLOSED")
            && self
                .queries
                .preview_deployments_active_count(base_application_id)
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
        let row = match self.queries.preview_deployments_get_by_id(id).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Err("Preview deployment not found".into()),
            Err(error) => return Err(error.to_string()),
        };
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
            source_owner: optional_field(row.source_owner),
            source_repository: optional_field(row.source_repository),
            target_branch: row.target_branch,
            commit: optional_field(row.commit_sha),
            author: optional_field(row.author),
        };
        self.deploy_application(row.base_service_id, &event)
            .await
    }

    async fn matching_targets(
        &self,
        provider: crate::services::webhook::GitProviderKind,
        owner: &str,
        repository: &str,
        target_branch: &str,
    ) -> Result<Vec<PreviewTarget>, String> {
        use crate::services::webhook::GitProviderKind;

        let targets = match provider {
            GitProviderKind::Github => {
                let rows = self
                    .queries
                    .preview_deployments_matching_targets1(&PreviewDeploymentsMatchingTargets1Params {
                        repository: repository.to_string(),
                        owner: owner.to_string(),
                        branch: target_branch.to_string(),
                    })
                    .await
                    .map_err(|error| error.to_string())?;
                rows.into_iter().map(preview_target).collect()
            }
            GitProviderKind::Gitlab => {
                let rows = self
                    .queries
                    .preview_deployments_matching_targets2(&PreviewDeploymentsMatchingTargets2Params {
                        gitlab_repository: repository.to_string(),
                        gitlab_owner: owner.to_string(),
                        gitlab_branch: target_branch.to_string(),
                    })
                    .await
                    .map_err(|error| error.to_string())?;
                rows.into_iter().map(preview_target).collect()
            }
            GitProviderKind::Gitea => {
                let rows = self
                    .queries
                    .preview_deployments_matching_targets3(&PreviewDeploymentsMatchingTargets3Params {
                        gitea_repository: repository.to_string(),
                        gitea_owner: owner.to_string(),
                        gitea_branch: target_branch.to_string(),
                    })
                    .await
                    .map_err(|error| error.to_string())?;
                rows.into_iter().map(preview_target).collect()
            }
            GitProviderKind::Bitbucket => {
                let rows = self
                    .queries
                    .preview_deployments_matching_targets4(&PreviewDeploymentsMatchingTargets4Params {
                        bitbucket_repository: repository.to_string(),
                        bitbucket_owner: owner.to_string(),
                        bitbucket_branch: target_branch.to_string(),
                    })
                    .await
                    .map_err(|error| error.to_string())?;
                rows.into_iter().map(preview_target).collect()
            }
        };
        Ok(targets)
    }
}

fn preview_target<R>(row: R) -> PreviewTarget
where
    R: MatchingTargetRow,
{
    PreviewTarget {
        base_service_id: row.base_service_id(),
        provider_id: row.provider_id(),
        preview_limit: row.preview_limit().unwrap_or(3),
        require_collaborator_permissions: row.require_collaborator_permissions().unwrap_or(0) != 0,
    }
}

trait MatchingTargetRow {
    fn base_service_id(&self) -> i64;
    fn provider_id(&self) -> i64;
    fn preview_limit(&self) -> Option<i64>;
    fn require_collaborator_permissions(&self) -> Option<i64>;
}

macro_rules! impl_matching_target_row {
    ($row:ty) => {
        impl MatchingTargetRow for $row {
            fn base_service_id(&self) -> i64 {
                self.base_service_id
            }
            fn provider_id(&self) -> i64 {
                self.provider_id
            }
            fn preview_limit(&self) -> Option<i64> {
                self.preview_limit
            }
            fn require_collaborator_permissions(&self) -> Option<i64> {
                self.require_collaborator_permissions
            }
        }
    };
}

impl_matching_target_row!(crate::flash_gen::preview_deployments::PreviewDeploymentsMatchingTargets1Row);
impl_matching_target_row!(crate::flash_gen::preview_deployments::PreviewDeploymentsMatchingTargets2Row);
impl_matching_target_row!(crate::flash_gen::preview_deployments::PreviewDeploymentsMatchingTargets3Row);
impl_matching_target_row!(crate::flash_gen::preview_deployments::PreviewDeploymentsMatchingTargets4Row);

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
