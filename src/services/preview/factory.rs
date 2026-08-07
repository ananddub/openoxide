use auto_di::resolve;

use crate::{
    db::models::applications::Application,
    repository::{NewPreviewDeployment, PreviewTarget},
    services::{
        application::{ApplicationOperation, ApplicationService},
        webhook::{GitProviderKind, PullRequestEvent},
    },
};

use super::{PreviewDeploymentOutcome, PreviewDeploymentService};

impl PreviewDeploymentService {
    pub(super) async fn deploy_target(
        &self,
        target: &PreviewTarget,
        event: &PullRequestEvent,
    ) -> Result<(PreviewDeploymentOutcome, bool), String> {
        let provider = event.provider.as_str();
        let existing = self
            .previews
            .find_for_pull_request(target.base_application_id, provider, &event.number)
            .await
            .map_err(|error| error.to_string())?;
        let base = self
            .applications
            .get_by_id(target.base_application_id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("Base application not found")?;

        let (preview_id, preview_application_id, created) = if let Some(existing) = existing {
            let Some(preview_application_id) = existing.preview_application_id else {
                let (preview_application_id, domain) =
                    self.create_preview_resources(&base, event).await?;
                if let Err(error) = self
                    .previews
                    .reopen(
                        existing.id,
                        preview_application_id,
                        &event.source_branch,
                        event.source_owner.as_deref(),
                        event.source_repository.as_deref(),
                        &event.target_branch,
                        event.commit.as_deref(),
                        event.author.as_deref(),
                        &domain,
                    )
                    .await
                {
                    let _ = self.applications.delete(preview_application_id).await;
                    return Err(error.to_string());
                }
                return self
                    .queue_preview(existing.id, preview_application_id, true)
                    .await;
            };
            let current = self
                .applications
                .get_by_id(preview_application_id)
                .await
                .map_err(|error| error.to_string())?
                .ok_or("Preview application not found")?;
            let clone = preview_application(&base, event, current.app_name, current.created_at)?;
            self.applications
                .update(preview_application_id, &clone)
                .await
                .map_err(|error| error.to_string())?;
            self.previews
                .update_source(
                    existing.id,
                    &event.source_branch,
                    event.source_owner.as_deref(),
                    event.source_repository.as_deref(),
                    event.commit.as_deref(),
                    event.author.as_deref(),
                )
                .await
                .map_err(|error| error.to_string())?;
            (existing.id, preview_application_id, false)
        } else {
            let (preview_application_id, domain) =
                self.create_preview_resources(&base, event).await?;
            let preview_id = match self
                .previews
                .create(NewPreviewDeployment {
                    base_application_id: target.base_application_id,
                    preview_application_id,
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
                })
                .await
            {
                Ok(id) => id,
                Err(error) => {
                    let _ = self.applications.delete(preview_application_id).await;
                    return Err(error.to_string());
                }
            };
            (preview_id, preview_application_id, true)
        };

        self.queue_preview(preview_id, preview_application_id, created)
            .await
    }

    async fn queue_preview(
        &self,
        preview_id: i64,
        preview_application_id: i64,
        created: bool,
    ) -> Result<(PreviewDeploymentOutcome, bool), String> {
        let application_service = resolve::<ApplicationService>()
            .await
            .map_err(|error| error.to_string())?;
        let deployment = match application_service
            .run_operation(
                preview_application_id,
                if created {
                    ApplicationOperation::Deploy
                } else {
                    ApplicationOperation::Redeploy
                },
            )
            .await
        {
            Ok(deployment) => deployment,
            Err(error) => {
                if !error.to_string().contains("already queued or running") {
                    let _ = self.previews.set_status(preview_id, "ERROR").await;
                }
                return Err(error.to_string());
            }
        };
        let deployment_id = deployment
            .deployment_id
            .ok_or("Preview deployment was not queued")?;
        self.previews
            .link_deployment(preview_id, deployment_id)
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
        base: &Application,
        event: &PullRequestEvent,
    ) -> Result<(i64, String), String> {
        let app_name = preview_app_name(&base.app_name, &event.number);
        let clone = preview_application(base, event, app_name, chrono::Utc::now().timestamp())?;
        let domain = preview_domain(base, &event.number)?;
        if self
            .domains
            .host_in_use(&domain)
            .await
            .map_err(|error| error.to_string())?
        {
            return Err(format!("Preview domain is already in use: {domain}"));
        }
        let preview_application_id = self
            .applications
            .create(&clone)
            .await
            .map_err(|error| error.to_string())?;
        if let Err(error) = self
            .domains
            .create_and_return(
                domain.clone(),
                base.preview_https,
                base.preview_port.or(Some(3000)),
                base.preview_path.clone().or(Some("/".into())),
                Some("/".into()),
                None,
                None,
                base.preview_custom_cert_resolver.clone(),
                0,
                "[]".into(),
                "APPLICATION".into(),
                base.preview_certificate_type.clone(),
                Some(preview_application_id),
                None,
            )
            .await
        {
            let _ = self.applications.delete(preview_application_id).await;
            return Err(error.to_string());
        }
        Ok((preview_application_id, domain))
    }
}

fn preview_application(
    base: &Application,
    event: &PullRequestEvent,
    app_name: String,
    created_at: i64,
) -> Result<Application, String> {
    let mut preview = base.clone();
    preview.id = None;
    preview.name = format!("{} · PR #{}", base.name, event.number);
    preview.app_name = app_name;
    preview.app_status = "IDLE".into();
    preview.auto_deploy = Some(0);
    preview.env_var = merge_environment(base.env_var.as_deref(), base.preview_env.as_deref());
    preview.build_args = base.preview_build_args.clone().or(base.build_args.clone());
    preview.build_secrets = base
        .preview_build_secrets
        .clone()
        .or(base.build_secrets.clone());
    preview.labels_swarm = base.preview_labels.clone().or(base.labels_swarm.clone());
    preview.is_preview_deployments_active = 0;
    preview.preview_require_collaborator_permissions = 0;
    preview.created_at = created_at;
    preview.updated_at = chrono::Utc::now().timestamp();
    match event.provider {
        GitProviderKind::Github => {
            preview.owner = Some(
                event
                    .source_owner
                    .clone()
                    .unwrap_or_else(|| event.owner.clone()),
            );
            preview.repository = Some(
                event
                    .source_repository
                    .clone()
                    .unwrap_or_else(|| event.repository.clone()),
            );
            preview.branch = Some(event.source_branch.clone());
        }
        GitProviderKind::Gitlab => {
            preview.gitlab_owner = Some(
                event
                    .source_owner
                    .clone()
                    .unwrap_or_else(|| event.owner.clone()),
            );
            preview.gitlab_repository = Some(
                event
                    .source_repository
                    .clone()
                    .unwrap_or_else(|| event.repository.clone()),
            );
            preview.gitlab_branch = Some(event.source_branch.clone());
        }
        GitProviderKind::Gitea => {
            preview.gitea_owner = Some(
                event
                    .source_owner
                    .clone()
                    .unwrap_or_else(|| event.owner.clone()),
            );
            preview.gitea_repository = Some(
                event
                    .source_repository
                    .clone()
                    .unwrap_or_else(|| event.repository.clone()),
            );
            preview.gitea_branch = Some(event.source_branch.clone());
        }
        GitProviderKind::Bitbucket => {
            preview.bitbucket_owner = Some(
                event
                    .source_owner
                    .clone()
                    .unwrap_or_else(|| event.owner.clone()),
            );
            preview.bitbucket_repository = Some(
                event
                    .source_repository
                    .clone()
                    .unwrap_or_else(|| event.repository.clone()),
            );
            preview.bitbucket_branch = Some(event.source_branch.clone());
        }
    }
    Ok(preview)
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

fn preview_domain(base: &Application, number: &str) -> Result<String, String> {
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
