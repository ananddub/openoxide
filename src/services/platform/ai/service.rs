use auto_di::singleton;
use std::sync::Arc;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        ai_generations::{
            AiGenerationsCreateParams, AiGenerationsMarkDeployedParams, AiGenerationsReviewParams,
        },
        misc_services_extra::MiscSvcAiSettingsCreateReturnIdParams,
        models::{AiGenerations, AiSettings},
        mount::MountCreateComposeFileParams,
    },
    services::{
        compute::compose::{ComposeOperation, ComposeService, ComposeType, crud::CreateComposeDto},
        networking::domain::{CreateDomainDto, DomainService},
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
    queries: Arc<FlashQueries>,
    compose: Arc<ComposeService>,
    domain: Arc<DomainService>,
    client: AiClient,
}

#[singleton]
impl AiService {
    fn new(
        queries: Arc<FlashQueries>,
        compose: Arc<ComposeService>,
        domain: Arc<DomainService>,
    ) -> Self {
        Self {
            queries,
            compose,
            domain,
            client: AiClient,
        }
    }

    pub async fn list_settings(&self, organization_id: i64) -> Result<Vec<AiSettingView>, String> {
        Ok(self
            .queries
            .ai_settings_list_by_organization(organization_id)
            .await
            .map_err(db_error)?
            .into_iter()
            .map(setting_view)
            .collect())
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
        let item = AiSettings {
            id: 0,
            name: input.name.trim().to_string(),
            api_url: normalize_url(&input.api_url),
            api_key: input.api_key,
            model: input.model.trim().to_string(),
            is_enabled: i64::from(input.is_enabled),
            organization_id,
            created_at: now,
            updated_at: now,
        };
        let id = self
            .queries
            .misc_svc_ai_settings_create_return_id(&MiscSvcAiSettingsCreateReturnIdParams {
                name: item.name,
                api_url: item.api_url,
                api_key: item.api_key,
                model: item.model,
                is_enabled: item.is_enabled,
                organization_id: item.organization_id,
                created_at: item.created_at,
                updated_at: item.updated_at,
            })
            .await
            .map_err(db_error)?;
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
        let item = AiSettings {
            id,
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
        self.queries
            .ai_settings_update(&crate::flash_gen::ai_settings::AiSettingsUpdateParams {
                name: item.name.clone(),
                api_url: item.api_url.clone(),
                api_key: item.api_key.clone(),
                model: item.model.clone(),
                is_enabled: item.is_enabled,
                organization_id: item.organization_id,
                created_at: item.created_at,
                updated_at: item.updated_at,
                id,
            })
            .await
            .map_err(db_error)?;
        self.get_setting(id, organization_id).await
    }

    pub async fn delete_setting(&self, id: i64, organization_id: i64) -> Result<bool, String> {
        match self
            .queries
            .misc_svc_ai_settings_delete_for_organization(id, organization_id)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(db_error(error)),
        }
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
            .queries
            .ai_generations_create(&AiGenerationsCreateParams {
                ai_setting_id,
                organization_id,
                created_by: user_id,
                prompt: request.trim().to_string(),
                output_json,
            })
            .await
            .map_err(db_error)?;
        generation_view(generation)
    }

    pub async fn get_generation(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<AiGenerationView, String> {
        let generation = match self
            .queries
            .ai_generations_get_for_organization(id, organization_id)
            .await
        {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Err("AI generation not found".to_string()),
            Err(error) => return Err(db_error(error)),
        };
        generation_view(generation)
    }

    pub async fn list_generations(
        &self,
        organization_id: i64,
        limit: i64,
    ) -> Result<Vec<AiGenerationView>, String> {
        self.queries
            .ai_generations_list_for_organization(organization_id, limit.clamp(1, 100))
            .await
            .map_err(db_error)?
            .into_iter()
            .map(generation_view)
            .collect()
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
        let generation = match self
            .queries
            .ai_generations_review(&AiGenerationsReviewParams {
                output_json,
                id,
                organization_id,
            })
            .await
        {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => {
                return Err("AI generation not found or already deployed".to_string());
            }
            Err(error) => return Err(db_error(error)),
        };
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
                .queries
                .domains_host_in_use(&domain.host)
                .await
                .map_err(db_error)?
                > 0
            {
                return Err(format!("domain `{}` is already in use", domain.host));
            }
        }

        let claimed = match self
            .queries
            .ai_generations_claim_for_deploy(id, organization_id)
            .await
        {
            Ok(_) => true,
            Err(sqlx::Error::RowNotFound) => false,
            Err(error) => return Err(db_error(error)),
        };
        if !claimed {
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
                compose_type: ComposeType::DockerCompose,
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

        let result = self
            .finish_compose_setup(compose.id, &suggestion, organization_id)
            .await;
        if let Err(error) = result {
            let _ = self.compose.delete(compose.id).await;
            self.release_deploy(id, organization_id).await;
            return Err(error);
        }

        let deployment_id = if input.deploy_now {
            match self
                .compose
                .run_operation(compose.id, ComposeOperation::Deploy, 0)
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
        let deployed = match self
            .queries
            .ai_generations_mark_deployed(&AiGenerationsMarkDeployedParams {
                service_id: compose.id,
                id,
                organization_id,
            })
            .await
        {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => {
                return Err("AI generation deployment state changed concurrently".to_string());
            }
            Err(error) => return Err(db_error(error)),
        };

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
        if let Err(error) = self
            .queries
            .ai_generations_release_deploy(id, organization_id)
            .await
        {
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
        organization_id: i64,
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
                    crate::services::compute::compose::crud::PatchComposeDto {
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
                .create(
                    CreateDomainDto {
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
                    },
                    organization_id,
                )
                .await
                .map_err(db_error)?;
        }
        for file in &suggestion.config_files {
            self.queries
                .mount_create_compose_file(&MountCreateComposeFileParams {
                    file_path: file.file_path.clone(),
                    content: file.content.clone(),
                    service_id: compose_id,
                })
                .await
                .map_err(db_error)?;
        }
        Ok(())
    }

    async fn setting(&self, id: i64, organization_id: i64) -> Result<AiSettings, String> {
        match self
            .queries
            .ai_settings_get_for_organization(id, organization_id)
            .await
        {
            Ok(row) => Ok(row),
            Err(sqlx::Error::RowNotFound) => Err("AI setting not found".into()),
            Err(error) => Err(db_error(error)),
        }
    }

    async fn enabled_setting(&self, id: i64, organization_id: i64) -> Result<AiSettings, String> {
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
        let environment = match self.queries.environments_get_by_id(environment_id).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Err("environment not found".into()),
            Err(error) => return Err(db_error(error)),
        };
        let project = match self
            .queries
            .projects_get_by_id(environment.project_id)
            .await
        {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Err("environment project not found".into()),
            Err(error) => return Err(db_error(error)),
        };
        if project.organization_id != organization_id {
            return Err("environment does not belong to the authenticated organization".into());
        }
        Ok(())
    }
}

fn provider_config(setting: &AiSettings) -> AiProviderConfig {
    AiProviderConfig {
        api_url: setting.api_url.clone(),
        api_key: setting.api_key.clone(),
        model: setting.model.clone(),
    }
}

fn setting_view(setting: AiSettings) -> AiSettingView {
    let provider = match AiProviderKind::detect(&setting.api_url) {
        AiProviderKind::OpenAi => "openai",
        AiProviderKind::Anthropic => "anthropic",
        AiProviderKind::Gemini => "gemini",
        AiProviderKind::Ollama => "ollama",
        AiProviderKind::OpenAiCompatible => "openai_compatible",
    };
    AiSettingView {
        id: setting.id,
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

fn generation_view(generation: AiGenerations) -> Result<AiGenerationView, String> {
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
        compose_id: generation.service_id,
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
