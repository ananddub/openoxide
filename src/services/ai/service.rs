use auto_di::singleton;
use std::sync::Arc;

use crate::{
    api::dto::{compose::CreateComposeDto, domain::CreateDomainDto},
    db::models::{ai_generations::AiGeneration, ai_settings::AiSetting},
    repository::{
        AiGenerationRepository, AiSettingRepository, DomainRepository, EnvironmentRepository,
        MountRepository, ProjectRepository,
    },
    services::{
        compose::{ComposeOperation, ComposeService},
        domain::DomainService,
    },
    utils::ai::{AiClient, AiProviderConfig, AiProviderKind},
};

use super::{
    prompts,
    types::{
        AiDeployInput, AiDeploymentResult, AiGenerationOutput, AiGenerationView, AiLogContext,
        AiSettingInput, AiSettingUpdate, AiSettingView,
    },
    validation,
};

pub struct AiService {
    settings: Arc<AiSettingRepository>,
    generations: Arc<AiGenerationRepository>,
    environments: Arc<EnvironmentRepository>,
    projects: Arc<ProjectRepository>,
    domains: Arc<DomainRepository>,
    mounts: Arc<MountRepository>,
    compose: Arc<ComposeService>,
    domain: Arc<DomainService>,
    client: AiClient,
}

#[singleton]
impl AiService {
    fn new(
        settings: Arc<AiSettingRepository>,
        generations: Arc<AiGenerationRepository>,
        environments: Arc<EnvironmentRepository>,
        projects: Arc<ProjectRepository>,
        domains: Arc<DomainRepository>,
        mounts: Arc<MountRepository>,
        compose: Arc<ComposeService>,
        domain: Arc<DomainService>,
    ) -> Self {
        Self {
            settings,
            generations,
            environments,
            projects,
            domains,
            mounts,
            compose,
            domain,
            client: AiClient::default(),
        }
    }

    pub async fn list_settings(&self, organization_id: i64) -> Result<Vec<AiSettingView>, String> {
        self.settings
            .list_by_organization(organization_id)
            .await
            .map_err(db_error)
            .map(|items| items.into_iter().map(setting_view).collect())
    }

    pub async fn get_setting(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<AiSettingView, String> {
        self.setting(id, organization_id).await.map(setting_view)
    }

    pub async fn create_setting(
        &self,
        organization_id: i64,
        input: AiSettingInput,
    ) -> Result<AiSettingView, String> {
        validate_setting(&input.name, &input.api_url, &input.model)?;
        let now = chrono::Utc::now().timestamp();
        let item = AiSetting {
            id: None,
            name: input.name.trim().to_string(),
            api_url: normalize_url(&input.api_url),
            api_key: input.api_key,
            model: input.model.trim().to_string(),
            is_enabled: i64::from(input.is_enabled),
            organization_id,
            created_at: now,
            updated_at: now,
        };
        let id = self.settings.create(&item).await.map_err(db_error)?;
        self.get_setting(id, organization_id).await
    }

    pub async fn update_setting(
        &self,
        id: i64,
        organization_id: i64,
        input: AiSettingUpdate,
    ) -> Result<AiSettingView, String> {
        let current = self.setting(id, organization_id).await?;
        let name = input.name.unwrap_or(current.name);
        let api_url = input.api_url.unwrap_or(current.api_url);
        let model = input.model.unwrap_or(current.model);
        validate_setting(&name, &api_url, &model)?;
        let item = AiSetting {
            id: Some(id),
            name: name.trim().to_string(),
            api_url: normalize_url(&api_url),
            api_key: input.api_key.unwrap_or(current.api_key),
            model: model.trim().to_string(),
            is_enabled: input
                .is_enabled
                .map(i64::from)
                .unwrap_or(current.is_enabled),
            organization_id,
            created_at: current.created_at,
            updated_at: chrono::Utc::now().timestamp(),
        };
        self.settings.update(id, &item).await.map_err(db_error)?;
        self.get_setting(id, organization_id).await
    }

    pub async fn delete_setting(&self, id: i64, organization_id: i64) -> Result<bool, String> {
        self.settings
            .delete_for_organization(id, organization_id)
            .await
            .map_err(db_error)
    }

    pub async fn discover_models(
        &self,
        api_url: String,
        api_key: String,
    ) -> Result<Vec<String>, String> {
        self.client
            .discover_models(&AiProviderConfig {
                api_url,
                api_key,
                model: String::new(),
            })
            .await
    }

    pub async fn discover_setting_models(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<String>, String> {
        let setting = self.setting(id, organization_id).await?;
        self.client
            .discover_models(&provider_config(&setting))
            .await
    }

    pub async fn test_setting(&self, id: i64, organization_id: i64) -> Result<(), String> {
        let setting = self.setting(id, organization_id).await?;
        self.client
            .test_connection(&provider_config(&setting))
            .await
    }

    pub async fn test_connection(
        &self,
        api_url: String,
        api_key: String,
        model: String,
    ) -> Result<(), String> {
        self.client
            .test_connection(&AiProviderConfig {
                api_url,
                api_key,
                model,
            })
            .await
    }

    pub async fn generate(
        &self,
        ai_setting_id: i64,
        organization_id: i64,
        user_id: i64,
        request: String,
    ) -> Result<AiGenerationView, String> {
        if request.trim().is_empty() || request.len() > 8_000 {
            return Err("AI generation request must contain 1 to 8000 characters".into());
        }
        let setting = self.enabled_setting(ai_setting_id, organization_id).await?;
        let value = self
            .client
            .complete_json(
                &provider_config(&setting),
                &prompts::compose_generation(&request),
            )
            .await?;
        let output: AiGenerationOutput = serde_json::from_value(value)
            .map_err(|error| format!("AI output does not match the generation schema: {error}"))?;
        validation::validate_output(&output)?;
        let output_json = serde_json::to_string(&output)
            .map_err(|error| format!("could not serialize AI output: {error}"))?;
        let generation = self
            .generations
            .create(
                ai_setting_id,
                organization_id,
                user_id,
                request.trim(),
                &output_json,
            )
            .await
            .map_err(db_error)?;
        generation_view(generation)
    }

    pub async fn get_generation(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<AiGenerationView, String> {
        let generation = self
            .generations
            .get_for_organization(id, organization_id)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "AI generation not found".to_string())?;
        generation_view(generation)
    }

    pub async fn list_generations(
        &self,
        organization_id: i64,
        limit: i64,
    ) -> Result<Vec<AiGenerationView>, String> {
        let rows = self
            .generations
            .list_for_organization(organization_id, limit.clamp(1, 100))
            .await
            .map_err(db_error)?;
        rows.into_iter().map(generation_view).collect()
    }

    pub async fn review_generation(
        &self,
        id: i64,
        organization_id: i64,
        output: AiGenerationOutput,
    ) -> Result<AiGenerationView, String> {
        validation::validate_output(&output)?;
        let output_json = serde_json::to_string(&output)
            .map_err(|error| format!("could not serialize AI output: {error}"))?;
        let generation = self
            .generations
            .review(id, organization_id, &output_json)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "AI generation not found or already deployed".to_string())?;
        generation_view(generation)
    }

    pub async fn deploy_generation(
        &self,
        id: i64,
        organization_id: i64,
        input: AiDeployInput,
    ) -> Result<AiDeploymentResult, String> {
        let generation = self.get_generation(id, organization_id).await?;
        if generation.status != "REVIEWED" {
            return Err("AI generation must be reviewed before deployment".into());
        }
        let suggestion = generation
            .output
            .suggestions
            .iter()
            .find(|item| item.id == input.suggestion_id)
            .cloned()
            .ok_or_else(|| "selected AI suggestion not found".to_string())?;
        validation::validate_suggestion(&suggestion)?;
        self.verify_environment(input.environment_id, organization_id)
            .await?;
        for domain in &suggestion.domains {
            if self
                .domains
                .host_in_use(&domain.host)
                .await
                .map_err(db_error)?
            {
                return Err(format!("domain `{}` is already in use", domain.host));
            }
        }

        if !self
            .generations
            .claim_for_deploy(id, organization_id)
            .await
            .map_err(db_error)?
        {
            return Err("AI generation is already deploying or has been deployed".into());
        }

        let compose = match self
            .compose
            .create(CreateComposeDto {
                name: suggestion.name.clone(),
                description: Some(suggestion.description.clone()),
                environment_id: input.environment_id,
                server_id: input.server_id,
                source_type: "RAW".into(),
                compose_type: "DOCKER-COMPOSE".into(),
                compose_file: suggestion.docker_compose.clone(),
            })
            .await
        {
            Ok(compose) => compose,
            Err(error) => {
                self.release_deploy(id, organization_id).await;
                return Err(db_error(error));
            }
        };

        let result = self.finish_compose_setup(compose.id, &suggestion).await;
        if let Err(error) = result {
            let _ = self.compose.delete(compose.id).await;
            self.release_deploy(id, organization_id).await;
            return Err(error);
        }

        let deployment_id = if input.deploy_now {
            match self
                .compose
                .run_operation(compose.id, ComposeOperation::Deploy)
                .await
            {
                Ok(result) => match result.deployment_id {
                    Some(deployment_id) => Some(deployment_id),
                    None => {
                        let _ = self.compose.delete(compose.id).await;
                        self.release_deploy(id, organization_id).await;
                        return Err("deployment queue did not return an ID".into());
                    }
                },
                Err(error) => {
                    let _ = self.compose.delete(compose.id).await;
                    self.release_deploy(id, organization_id).await;
                    return Err(db_error(error));
                }
            }
        } else {
            None
        };
        let deployed = self
            .generations
            .mark_deployed(id, organization_id, compose.id)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "AI generation deployment state changed concurrently".to_string())?;

        Ok(AiDeploymentResult {
            generation: generation_view(deployed)?,
            compose_id: compose.id,
            deployment_id,
        })
    }

    pub async fn analyze_logs(
        &self,
        ai_setting_id: i64,
        organization_id: i64,
        context: AiLogContext,
        logs: String,
    ) -> Result<String, String> {
        if logs.trim().is_empty() || logs.len() > 250_000 {
            return Err("logs must contain 1 to 250000 characters".into());
        }
        let setting = self.enabled_setting(ai_setting_id, organization_id).await?;
        self.client
            .complete_text(
                &provider_config(&setting),
                &prompts::log_analysis(context.label(), &logs),
            )
            .await
    }

    async fn release_deploy(&self, id: i64, organization_id: i64) {
        if let Err(error) = self.generations.release_deploy(id, organization_id).await {
            tracing::error!(
                id,
                organization_id,
                error = %error,
                "could not release AI deployment claim"
            );
        }
    }

    async fn finish_compose_setup(
        &self,
        compose_id: i64,
        suggestion: &super::types::AiComposeSuggestion,
    ) -> Result<(), String> {
        if !suggestion.env_variables.is_empty() {
            let env_var = suggestion
                .env_variables
                .iter()
                .map(|item| format!("{}={}", item.name, item.value.replace('\n', "\\n")))
                .collect::<Vec<_>>()
                .join("\n");
            self.compose
                .patch(
                    compose_id,
                    crate::api::dto::compose::PatchComposeDto {
                        name: None,
                        description: None,
                        env_var: Some(env_var),
                        compose_file: None,
                        compose_type: None,
                        trigger_type: None,
                        command: None,
                        enable_submodules: None,
                        compose_path: None,
                        suffix: None,
                        randomize: None,
                        isolated_deployment: None,
                        isolated_deployments_volume: None,
                        watch_paths: None,
                        service_networks: None,
                        server_id: None,
                    },
                )
                .await
                .map_err(db_error)?;
        }
        for domain in &suggestion.domains {
            self.domain
                .create(CreateDomainDto {
                    host: domain.host.clone(),
                    https: false,
                    port: Some(domain.port),
                    path: "/".into(),
                    internal_path: "/".into(),
                    custom_entrypoint: None,
                    service_name: Some(domain.service_name.clone()),
                    custom_cert_resolver: None,
                    strip_path: false,
                    middlewares: "[]".into(),
                    domain_type: "COMPOSE".into(),
                    certificate_type: "NONE".into(),
                    application_id: None,
                    compose_id: Some(compose_id),
                })
                .await
                .map_err(db_error)?;
        }
        for file in &suggestion.config_files {
            self.mounts
                .create_compose_file(compose_id, &file.file_path, &file.content)
                .await
                .map_err(db_error)?;
        }
        Ok(())
    }

    async fn setting(&self, id: i64, organization_id: i64) -> Result<AiSetting, String> {
        self.settings
            .get_for_organization(id, organization_id)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "AI setting not found".into())
    }

    async fn enabled_setting(&self, id: i64, organization_id: i64) -> Result<AiSetting, String> {
        let setting = self.setting(id, organization_id).await?;
        if setting.is_enabled == 0 {
            return Err("AI setting is disabled".into());
        }
        Ok(setting)
    }

    async fn verify_environment(
        &self,
        environment_id: i64,
        organization_id: i64,
    ) -> Result<(), String> {
        let environment = self
            .environments
            .get_by_id(environment_id)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "environment not found".to_string())?;
        let project = self
            .projects
            .get_by_id(environment.project_id)
            .await
            .map_err(db_error)?
            .ok_or_else(|| "environment project not found".to_string())?;
        if project.organization_id != organization_id {
            return Err("environment does not belong to the authenticated organization".into());
        }
        Ok(())
    }
}

fn provider_config(setting: &AiSetting) -> AiProviderConfig {
    AiProviderConfig {
        api_url: setting.api_url.clone(),
        api_key: setting.api_key.clone(),
        model: setting.model.clone(),
    }
}

fn setting_view(setting: AiSetting) -> AiSettingView {
    let provider = match AiProviderKind::detect(&setting.api_url) {
        AiProviderKind::OpenAi => "openai",
        AiProviderKind::Anthropic => "anthropic",
        AiProviderKind::Gemini => "gemini",
        AiProviderKind::Ollama => "ollama",
        AiProviderKind::OpenAiCompatible => "openai_compatible",
    };
    AiSettingView {
        id: setting.id.unwrap_or_default(),
        name: setting.name,
        api_url: setting.api_url,
        model: setting.model,
        is_enabled: setting.is_enabled != 0,
        provider: provider.into(),
        has_api_key: !setting.api_key.is_empty(),
        organization_id: setting.organization_id,
        created_at: setting.created_at,
        updated_at: setting.updated_at,
    }
}

fn generation_view(generation: AiGeneration) -> Result<AiGenerationView, String> {
    let output = serde_json::from_str(&generation.output_json)
        .map_err(|error| format!("stored AI generation is invalid: {error}"))?;
    Ok(AiGenerationView {
        id: generation.id,
        ai_setting_id: generation.ai_setting_id,
        organization_id: generation.organization_id,
        created_by: generation.created_by,
        prompt: generation.prompt,
        output,
        status: generation.status,
        compose_id: generation.compose_id,
        created_at: generation.created_at,
        updated_at: generation.updated_at,
    })
}

fn validate_setting(name: &str, api_url: &str, model: &str) -> Result<(), String> {
    if name.trim().is_empty() || name.len() > 100 {
        return Err("AI setting name must contain 1 to 100 characters".into());
    }
    if model.trim().is_empty() || model.len() > 200 {
        return Err("AI model must contain 1 to 200 characters".into());
    }
    let url = reqwest::Url::parse(api_url.trim())
        .map_err(|error| format!("invalid AI API URL: {error}"))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err("AI API URL must use http or https".into());
    }
    Ok(())
}

fn normalize_url(value: &str) -> String {
    value.trim().trim_end_matches('/').to_string()
}

fn db_error(error: sqlx::Error) -> String {
    match &error {
        sqlx::Error::Database(database) if database.is_unique_violation() => {
            "an AI setting with this name already exists in the organization".into()
        }
        _ => error.to_string(),
    }
}
