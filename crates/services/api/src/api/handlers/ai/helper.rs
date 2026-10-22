use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    AiComposeSuggestion as AiComposeSuggestionProto, AiDomain as AiDomainProto,
    AiEnvironmentVariable as AiEnvironmentVariableProto, AiGeneratedFile as AiGeneratedFileProto,
    AiGeneration as AiGenerationProto, AiGenerationOutput as AiGenerationOutputProto, AiProvider,
    AiSetting as AiSettingProto,
};
use crate::services::platform::ai::{AiGenerationView, AiSettingView};

pub fn map_error(error: String) -> ConnectError {
    let lower = error.to_ascii_lowercase();
    if lower.contains("not found") {
        ConnectError::not_found(error)
    } else if lower.contains("already") || lower.contains("concurrently") {
        ConnectError::new(ErrorCode::AlreadyExists, error)
    } else if lower.contains("does not belong") || lower.contains("authenticated organization") {
        ConnectError::new(ErrorCode::PermissionDenied, error)
    } else if lower.contains("database") || lower.contains("sqlite") {
        tracing::error!(error = %error, "AI backend database operation failed");
        ConnectError::new(ErrorCode::Internal, "database operation failed")
    } else {
        ConnectError::invalid_argument(error)
    }
}

fn provider_to_proto(provider: &str) -> ::buffa::EnumValue<AiProvider> {
    let known = match provider {
        "openai" => AiProvider::Openai,
        "anthropic" => AiProvider::Anthropic,
        "gemini" => AiProvider::Gemini,
        "ollama" => AiProvider::Ollama,
        "openai_compatible" => AiProvider::OpenaiCompatible,
        _ => AiProvider::Unspecified,
    };
    ::buffa::EnumValue::Known(known)
}

pub fn setting_to_proto(view: AiSettingView) -> AiSettingProto {
    AiSettingProto {
        id: view.id,
        name: view.name,
        api_url: view.api_url,
        model: view.model,
        is_enabled: view.is_enabled,
        provider: provider_to_proto(&view.provider),
        has_api_key: view.has_api_key,
        organization_id: view.organization_id,
        created_at: view.created_at,
        updated_at: view.updated_at,
    }
}

pub fn generation_to_proto(view: AiGenerationView) -> AiGenerationProto {
    AiGenerationProto {
        id: view.id,
        ai_setting_id: view.ai_setting_id,
        organization_id: view.organization_id,
        created_by: view.created_by,
        prompt: view.prompt,
        output: ::buffa::MessageField::some(AiGenerationOutputProto {
            suggestions: view
                .output
                .suggestions
                .into_iter()
                .map(|s| AiComposeSuggestionProto {
                    id: s.id,
                    name: s.name,
                    short_description: s.short_description,
                    description: s.description,
                    docker_compose: s.docker_compose,
                    env_variables: s
                        .env_variables
                        .into_iter()
                        .map(|e| AiEnvironmentVariableProto {
                            name: e.name,
                            value: e.value,
                        })
                        .collect(),
                    domains: s
                        .domains
                        .into_iter()
                        .map(|d| AiDomainProto {
                            host: d.host,
                            port: d.port,
                            service_name: d.service_name,
                        })
                        .collect(),
                    config_files: s
                        .config_files
                        .into_iter()
                        .map(|f| AiGeneratedFileProto {
                            file_path: f.file_path,
                            content: f.content,
                        })
                        .collect(),
                })
                .collect(),
        }),
        status: view.status,
        compose_id: view.compose_id,
        created_at: view.created_at,
        updated_at: view.updated_at,
    }
}
