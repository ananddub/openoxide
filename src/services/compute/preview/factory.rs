use auto_di::resolve;

use crate::{
    flash_gen::preview_deployments::{
        PreviewDeploymentsCreateParams, PreviewDeploymentsFindForPullRequestParams,
        PreviewDeploymentsReopenParams, PreviewDeploymentsUpdateSourceParams,
    },
    flash_gen::preview_env_extra::{
        PrevEnvCloneServiceGitParams, PrevEnvCloneServiceParams, PrevEnvCreateDomainParams,
        PrevEnvSetBitbucketSourceParams, PrevEnvSetGiteaSourceParams, PrevEnvSetGitlabSourceParams,
        PrevEnvSetGithubSourceParams, PrevEnvSyncServiceParams,
    },
    services::{
        compute::application::{ApplicationOperation, ApplicationRecord, ApplicationService},
        integrations::git::webhooks::{GitProviderKind, PullRequestEvent},
    },
};

use super::{
    types::{NewPreviewDeployment, PreviewTarget},
    PreviewDeploymentOutcome, PreviewDeploymentService,
};

impl PreviewDeploymentService {
    pub(super) async fn deploy_target(
        &self,
        target: &PreviewTarget,
        event: &PullRequestEvent,
    ) -> Result<(PreviewDeploymentOutcome, bool), String> {
        let provider = event.provider.as_str();
        let existing = match self
            .queries
            .preview_deployments_find_for_pull_request(&PreviewDeploymentsFindForPullRequestParams {
                base_service_id: target.base_service_id,
                provider_type: provider.to_string(),
                pull_request_number: event.number.clone(),
            })
            .await
        {
            Ok(row) => Some(row),
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error.to_string()),
        };
        let applications = resolve::<ApplicationService>()
            .await
            .map_err(|error| error.to_string())?;
        let base = applications
            .load_record(target.base_service_id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("Base application not found")?;

        let (preview_id, preview_service_id, created) = if let Some(existing) = existing {
            let Some(preview_service_id) = existing.preview_service_id else {
                let (preview_service_id, domain) =
                    self.create_preview_resources(&base, event).await?;
                if let Err(error) = self
                    .queries
                    .preview_deployments_reopen(&PreviewDeploymentsReopenParams {
                        preview_service_id,
                        source_branch: event.source_branch.clone(),
                        source_owner: event.source_owner.clone().unwrap_or_default(),
                        source_repository: event.source_repository.clone().unwrap_or_default(),
                        target_branch: event.target_branch.clone(),
                        commit_sha: event.commit.clone().unwrap_or_default(),
                        author: event.author.clone().unwrap_or_default(),
                        domain,
                        id: existing.id,
                    })
                    .await
                {
                    let _ = self.queries.delete_service(preview_service_id).await;
                    return Err(error.to_string());
                }
                return self
                    .queue_preview(existing.id, preview_service_id, true)
                    .await;
            };
            let current = applications
                .load_record(preview_service_id)
                .await
                .map_err(|error| error.to_string())?
                .ok_or("Preview application not found")?;
            self.sync_preview_resources(
                &base,
                event,
                preview_service_id,
                current.app_name.clone(),
                current.created_at,
            )
            .await?;
            self.queries
                .preview_deployments_update_source(&PreviewDeploymentsUpdateSourceParams {
                    source_branch: event.source_branch.clone(),
                    source_owner: event.source_owner.clone().unwrap_or_default(),
                    source_repository: event.source_repository.clone().unwrap_or_default(),
                    commit_sha: event.commit.clone().unwrap_or_default(),
                    author: event.author.clone().unwrap_or_default(),
                    id: existing.id,
                })
                .await
                .map_err(|error| error.to_string())?;
            (existing.id, preview_service_id, false)
        } else {
            let (preview_service_id, domain) =
                self.create_preview_resources(&base, event).await?;
            let new = NewPreviewDeployment {
                base_service_id: target.base_service_id,
                preview_service_id,
                provider_type: provider,
                provider_id: target.provider_id,
                owner: &event.owner,
                repository: &event.repository,
                pull_request_number: &event.number,
                source_branch: &event.source_branch,
                source_owner: event.source_owner.as_deref(),
                source_repository: event.source_repository.as_deref(),
                target_branch: &event.target_branch,
                commit_sha: event.commit.as_deref(),
                author: event.author.as_deref(),
                domain: &domain,
            };
            let now = chrono::Utc::now().timestamp();
            let preview_id = match self
                .queries
                .preview_deployments_create(&PreviewDeploymentsCreateParams {
                    base_service_id: new.base_service_id,
                    preview_service_id: new.preview_service_id,
                    provider_type: new.provider_type.to_string(),
                    provider_id: new.provider_id,
                    owner: new.owner.to_string(),
                    repository: new.repository.to_string(),
                    pull_request_number: new.pull_request_number.to_string(),
                    source_branch: new.source_branch.to_string(),
                    source_owner: new.source_owner.unwrap_or_default().to_string(),
                    source_repository: new.source_repository.unwrap_or_default().to_string(),
                    target_branch: new.target_branch.to_string(),
                    commit_sha: new.commit_sha.unwrap_or_default().to_string(),
                    author: new.author.unwrap_or_default().to_string(),
                    domain: new.domain.to_string(),
                    created_at: now,
                    updated_at: now,
                })
                .await
            {
                Ok(id) => id,
                Err(error) => {
                    let _ = self.queries.delete_service(preview_service_id).await;
                    return Err(error.to_string());
                }
            };
            (preview_id, preview_service_id, true)
        };

        self.queue_preview(preview_id, preview_service_id, created)
            .await
    }

    async fn queue_preview(
        &self,
        preview_id: i64,
        preview_service_id: i64,
        created: bool,
    ) -> Result<(PreviewDeploymentOutcome, bool), String> {
        let application_service = resolve::<ApplicationService>()
            .await
            .map_err(|error| error.to_string())?;
        let deployment = match application_service
            .run_operation(
                preview_service_id,
                if created {
                    ApplicationOperation::Deploy
                } else {
                    ApplicationOperation::Redeploy
                },
                0,
            )
            .await
        {
            Ok(deployment) => deployment,
            Err(error) => {
                if !error.to_string().contains("already queued or running") {
                    let _ = self
                        .queries
                        .preview_deployments_set_status("ERROR", preview_id)
                        .await;
                }
                return Err(error.to_string());
            }
        };
        let deployment_id = deployment
            .deployment_id
            .ok_or("Preview deployment was not queued")?;
        self.queries
            .preview_deployments_link_deployment1(deployment_id)
            .await
            .map_err(|error| error.to_string())?;
        self.queries
            .preview_deployments_link_deployment2(deployment_id, preview_id)
            .await
            .map_err(|error| error.to_string())?;
        Ok((
            PreviewDeploymentOutcome {
                preview: self
                    .get(preview_id)
                    .await
                    .map_err(|error| error.to_string())?,
                deployment_id: Some(deployment_id),
            },
            created,
        ))
    }

    async fn create_preview_resources(
        &self,
        base: &ApplicationRecord,
        event: &PullRequestEvent,
    ) -> Result<(i64, String), String> {
        let app_name = preview_app_name(&base.app_name, &event.number);
        let domain = preview_domain(base, &event.number)?;
        if self
            .queries
            .domains_host_in_use(&domain)
            .await
            .map_err(|error| error.to_string())?
            > 0
        {
            return Err(format!("Preview domain is already in use: {domain}"));
        }
        let preview_service_id = self
            .clone_preview_service(base, event, &app_name)
            .await?;
        if let Err(error) = self.create_preview_domain(base, &domain, preview_service_id).await {
            let _ = self.queries.delete_service(preview_service_id).await;
            return Err(error);
        }
        Ok((preview_service_id, domain))
    }

    /// Clones the base (services, service_git, service_application) rows into a
    /// fresh preview service and overrides the pull-request source fields.
    async fn clone_preview_service(
        &self,
        base: &ApplicationRecord,
        event: &PullRequestEvent,
        app_name: &str,
    ) -> Result<i64, String> {
        let now = chrono::Utc::now().timestamp();
        let name = format!("{} · PR #{}", base.name, event.number);
        let env_var =
            merge_environment(base.env_var.as_deref(), base.preview_env.as_deref())
                .unwrap_or_default();
        let preview_service_id = self
            .queries
            .prev_env_clone_service(&PrevEnvCloneServiceParams {
                name,
                app_name: app_name.to_string(),
                env_var,
                created_at: now,
                updated_at: now,
                id: base.id,
            })
            .await
            .map_err(|error| error.to_string())?;
        if let Err(error) = self
            .queries
            .prev_env_clone_service_git(&PrevEnvCloneServiceGitParams {
                service_id: preview_service_id,
                created_at: now,
                updated_at: now,
                service_id2: base.id,
            })
            .await
        {
            let _ = self.queries.delete_service(preview_service_id).await;
            return Err(error.to_string());
        }
        if let Err(error) = self
            .queries
            .prev_env_clone_service_application(preview_service_id, base.id)
            .await
        {
            let _ = self.queries.delete_service(preview_service_id).await;
            return Err(error.to_string());
        }
        if let Err(error) = self.set_preview_source(preview_service_id, event).await {
            let _ = self.queries.delete_service(preview_service_id).await;
            return Err(error.to_string());
        }
        Ok(preview_service_id)
    }

    /// Re-syncs an existing preview service with the current base configuration
    /// before redeploying, mirroring the old "update the cloned application row"
    /// behaviour on the unified services/service_git/service_application schema.
    async fn sync_preview_resources(
        &self,
        base: &ApplicationRecord,
        event: &PullRequestEvent,
        preview_service_id: i64,
        app_name: String,
        created_at: i64,
    ) -> Result<(), String> {
        let name = format!("{} · PR #{}", base.name, event.number);
        let env_var =
            merge_environment(base.env_var.as_deref(), base.preview_env.as_deref())
                .unwrap_or_default();
        self.queries
            .prev_env_sync_service(&PrevEnvSyncServiceParams {
                name,
                app_name,
                env_var,
                created_at,
                updated_at: chrono::Utc::now().timestamp(),
                id: preview_service_id,
                id2: base.id,
            })
            .await
            .map_err(|error| error.to_string())?;
        self.queries
            .prev_env_sync_service_git(preview_service_id, base.id)
            .await
            .map_err(|error| error.to_string())?;
        self.queries
            .prev_env_sync_service_application(preview_service_id, base.id)
            .await
            .map_err(|error| error.to_string())?;
        self.set_preview_source(preview_service_id, event)
            .await
            .map_err(|error| error.to_string())
    }

    async fn set_preview_source(
        &self,
        preview_service_id: i64,
        event: &PullRequestEvent,
    ) -> Result<(), sqlx::Error> {
        let owner = event
            .source_owner
            .clone()
            .unwrap_or_else(|| event.owner.clone());
        let repository = event
            .source_repository
            .clone()
            .unwrap_or_else(|| event.repository.clone());
        match event.provider {
            GitProviderKind::Github => {
                self.queries
                    .prev_env_set_github_source(&PrevEnvSetGithubSourceParams {
                        repository,
                        owner,
                        branch: event.source_branch.clone(),
                        service_id: preview_service_id,
                    })
                    .await
            }
            GitProviderKind::Gitlab => {
                self.queries
                    .prev_env_set_gitlab_source(&PrevEnvSetGitlabSourceParams {
                        gitlab_repository: repository,
                        gitlab_owner: owner,
                        gitlab_branch: event.source_branch.clone(),
                        service_id: preview_service_id,
                    })
                    .await
            }
            GitProviderKind::Gitea => {
                self.queries
                    .prev_env_set_gitea_source(&PrevEnvSetGiteaSourceParams {
                        gitea_repository: repository,
                        gitea_owner: owner,
                        gitea_branch: event.source_branch.clone(),
                        service_id: preview_service_id,
                    })
                    .await
            }
            GitProviderKind::Bitbucket => {
                self.queries
                    .prev_env_set_bitbucket_source(&PrevEnvSetBitbucketSourceParams {
                        bitbucket_repository: repository,
                        bitbucket_owner: owner,
                        bitbucket_branch: event.source_branch.clone(),
                        service_id: preview_service_id,
                    })
                    .await
            }
        }
    }

    async fn create_preview_domain(
        &self,
        base: &ApplicationRecord,
        domain: &str,
        preview_service_id: i64,
    ) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        self.queries
            .prev_env_create_domain(&PrevEnvCreateDomainParams {
                host: domain.to_string(),
                https: i64::from(base.preview_https),
                port: base.preview_port.unwrap_or(3000),
                path: base.preview_path.clone().unwrap_or_else(|| "/".into()),
                internal_path: "/".into(),
                custom_entrypoint: String::new(),
                service_name: String::new(),
                custom_cert_resolver: base
                    .preview_custom_cert_resolver
                    .clone()
                    .unwrap_or_default(),
                strip_path: 0,
                middlewares: "[]".into(),
                domain_type: "APPLICATION".into(),
                certificate_type: base.preview_certificate_type.clone(),
                service_id: preview_service_id,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

fn merge_environment(base: Option<&str>, preview: Option<&str>) -> Option<String> {
    match (
        base.filter(|v| !v.trim().is_empty()),
        preview.filter(|v| !v.trim().is_empty()),
    ) {
        (Some(base), Some(preview)) => Some(format!("{base}\n{preview}")),
        (Some(base), None) => Some(base.to_owned()),
        (None, Some(preview)) => Some(preview.to_owned()),
        (None, None) => None,
    }
}

fn preview_app_name(base: &str, number: &str) -> String {
    let suffix = short_label(number, 16);
    let max_base = 40_usize;
    format!(
        "{}-pr-{suffix}",
        base.chars().take(max_base).collect::<String>()
    )
    .trim_matches('-')
    .to_owned()
}

fn preview_domain(base: &ApplicationRecord, number: &str) -> Result<String, String> {
    let wildcard = base
        .preview_wildcard
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or("Preview wildcard domain is required")?
        .trim_start_matches("*.");
    let app = sanitize_label(&base.app_name);
    let pr = short_label(number, 20);
    let host = if wildcard.contains("{app}") || wildcard.contains("{pr}") {
        wildcard.replace("{app}", &app).replace("{pr}", &pr)
    } else {
        format!("{app}-pr-{pr}.{wildcard}")
    };
    if host.len() > 253
        || host
            .split('.')
            .any(|label| label.is_empty() || label.len() > 63)
    {
        return Err("Generated preview domain is invalid".into());
    }
    Ok(host)
}

fn sanitize_label(value: &str) -> String {
    let value = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    value.trim_matches('-').to_owned()
}

fn short_label(value: &str, limit: usize) -> String {
    let value = sanitize_label(value)
        .chars()
        .take(limit)
        .collect::<String>();
    if value.is_empty() {
        "preview".into()
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_names_are_stable_and_safe() {
        assert_eq!(
            preview_app_name("api_service", "42/test"),
            "api_service-pr-42-test"
        );
    }

    #[test]
    fn preview_environment_appends_overrides() {
        assert_eq!(
            merge_environment(Some("A=1"), Some("A=2\nB=3")).as_deref(),
            Some("A=1\nA=2\nB=3")
        );
    }
}
