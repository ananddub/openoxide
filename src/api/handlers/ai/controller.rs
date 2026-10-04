use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AiAnalyzeLogsRequest, AiAnalyzeLogsResponse, AiCreateGenerationRequest,
    AiCreateGenerationResponse, AiCreateSettingsRequest, AiCreateSettingsResponse,
    AiDeleteSettingsRequest, AiDeployGenerationRequest, AiDeployGenerationResponse,
    AiDiscoverModelsRequest, AiDiscoverModelsResponse, AiGetGenerationRequest,
    AiGetGenerationResponse, AiGetSettingsRequest, AiGetSettingsResponse, AiListGenerationsRequest,
    AiListGenerationsResponse, AiListModelsRequest, AiListModelsResponse, AiListSettingsRequest,
    AiListSettingsResponse, AiLogContext, AiReviewGenerationRequest, AiReviewGenerationResponse,
    AiService, AiServiceExt, AiTestConnectionRequest, AiTestConnectionResponse,
    AiTestSettingsRequest, AiTestSettingsResponse, AiUpdateSettingsRequest,
    AiUpdateSettingsResponse, Empty,
};
use crate::services::platform::ai::{
    AiDeployInput, AiGenerationOutput as AppAiGenerationOutput, AiLogContext as AppAiLogContext,
    AiSettingInput, AiSettingUpdate,
};

use super::helper::*;
use super::init::AiController;

#[auto_grpc::service(connect = true, auth = true)]
impl AiService for AiController {
    #[permission(Application, CanRead)]
    async fn list_settings<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, AiListSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiListSettingsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let items = self
            .service
            .list_settings(organization_id)
            .await
            .map_err(map_error)?;
        let settings = items.into_iter().map(setting_to_proto).collect();
        Response::ok(AiListSettingsResponse { settings })
    }

    #[permission(Application, CanRead)]
    async fn get_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiGetSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiGetSettingsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let setting = self
            .service
            .get_setting(request.id, organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(AiGetSettingsResponse {
            setting: ::buffa::MessageField::some(setting_to_proto(setting)),
        })
    }

    #[permission(Organization, CanUpdate)]
    async fn create_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiCreateSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiCreateSettingsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let input = AiSettingInput {
            name: request.name.to_owned(),
            api_url: request.api_url.to_owned(),
            api_key: request.api_key.to_owned(),
            model: request.model.to_owned(),
            is_enabled: request.is_enabled,
        };
        let setting = self
            .service
            .create_setting(organization_id, input)
            .await
            .map_err(map_error)?;
        Response::ok(AiCreateSettingsResponse {
            setting: ::buffa::MessageField::some(setting_to_proto(setting)),
        })
    }

    #[permission(Organization, CanUpdate)]
    async fn update_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiUpdateSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiUpdateSettingsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let input = AiSettingUpdate {
            name: request.name.map(str::to_owned),
            api_url: request.api_url.map(str::to_owned),
            api_key: request.api_key.map(str::to_owned),
            model: request.model.map(str::to_owned),
            is_enabled: request.is_enabled,
        };
        let setting = self
            .service
            .update_setting(request.id, organization_id, input)
            .await
            .map_err(map_error)?;
        Response::ok(AiUpdateSettingsResponse {
            setting: ::buffa::MessageField::some(setting_to_proto(setting)),
        })
    }

    #[permission(Organization, CanUpdate)]
    async fn delete_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiDeleteSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let deleted = self
            .service
            .delete_setting(request.id, organization_id)
            .await
            .map_err(map_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("AI setting not found"))
        }
    }

    #[permission(Application, CanRead)]
    async fn list_models<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiListModelsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiListModelsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let models = self
            .service
            .discover_setting_models(request.setting_id, organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(AiListModelsResponse { models })
    }

    #[permission(Application, CanRead)]
    async fn test_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiTestSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiTestSettingsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        self.service
            .test_setting(request.id, organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(AiTestSettingsResponse {
            success: true,
            message: "connection successful".into(),
        })
    }

    #[permission(Application, CanRead)]
    async fn discover_models<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiDiscoverModelsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiDiscoverModelsResponse> + Send + use<'a>> {
        let models = self
            .service
            .discover_models(request.api_url.to_owned(), request.api_key.to_owned())
            .await
            .map_err(map_error)?;
        Response::ok(AiDiscoverModelsResponse { models })
    }

    #[permission(Application, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiTestConnectionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiTestConnectionResponse> + Send + use<'a>> {
        self.service
            .test_connection(
                request.api_url.to_owned(),
                request.api_key.to_owned(),
                request.model.to_owned(),
            )
            .await
            .map_err(map_error)?;
        Response::ok(AiTestConnectionResponse {
            success: true,
            message: "connection successful".into(),
        })
    }

    #[permission(Application, CanRead)]
    async fn list_generations<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiListGenerationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiListGenerationsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let limit = request.limit.unwrap_or(50);
        let items = self
            .service
            .list_generations(organization_id, limit)
            .await
            .map_err(map_error)?;
        let generations = items.into_iter().map(generation_to_proto).collect();
        Response::ok(AiListGenerationsResponse { generations })
    }

    #[permission(Application, CanRead)]
    async fn get_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiGetGenerationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiGetGenerationResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let generation = self
            .service
            .get_generation(request.id, organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(AiGetGenerationResponse {
            generation: ::buffa::MessageField::some(generation_to_proto(generation)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiCreateGenerationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiCreateGenerationResponse> + Send + use<'a>>
    {
        let auth = Self::extract_auth(&ctx).await?;
        let organization_id = auth.organization_id;
        let generation = self
            .service
            .generate(
                request.ai_setting_id,
                organization_id,
                auth.user_id(),
                request.request.to_owned(),
            )
            .await
            .map_err(map_error)?;
        Response::ok(AiCreateGenerationResponse {
            generation: ::buffa::MessageField::some(generation_to_proto(generation)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn review_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiReviewGenerationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiReviewGenerationResponse> + Send + use<'a>>
    {
        let organization_id = auth.organization_id;
        let output_view = request.output.as_option().ok_or_else(|| {
            ConnectError::invalid_argument("review generation requires an output")
        })?;
        let output = output_view_to_dto(output_view);
        let generation = self
            .service
            .review_generation(request.id, organization_id, output)
            .await
            .map_err(map_error)?;
        Response::ok(AiReviewGenerationResponse {
            generation: ::buffa::MessageField::some(generation_to_proto(generation)),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn deploy_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiDeployGenerationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiDeployGenerationResponse> + Send + use<'a>>
    {
        let organization_id = auth.organization_id;
        let input = AiDeployInput {
            suggestion_id: request.suggestion_id.to_owned(),
            environment_id: request.environment_id,
            server_id: request.server_id,
            deploy_now: request.deploy_now,
        };
        let result = self
            .service
            .deploy_generation(request.id, organization_id, input)
            .await
            .map_err(map_error)?;
        Response::ok(AiDeployGenerationResponse {
            generation: ::buffa::MessageField::some(generation_to_proto(result.generation)),
            compose_id: result.compose_id,
            deployment_id: result.deployment_id,
        })
    }

    #[permission(Application, CanRead)]
    async fn analyze_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiAnalyzeLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiAnalyzeLogsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let context = match request.context {
            ::buffa::EnumValue::Known(AiLogContext::Build) => AppAiLogContext::Build,
            ::buffa::EnumValue::Known(AiLogContext::Runtime) => AppAiLogContext::Runtime,
            _ => return Err(ConnectError::invalid_argument("unsupported log context")),
        };
        let analysis = self
            .service
            .analyze_logs(
                request.ai_setting_id,
                organization_id,
                context,
                request.logs.to_owned(),
            )
            .await
            .map_err(map_error)?;
        Response::ok(AiAnalyzeLogsResponse { analysis })
    }
}

/// Wire `AiGenerationOutput` view → service DTO for the review mutation.
///
/// The service inner suggestion types (`AiComposeSuggestion` et al.) are not
/// re-exported from `services::platform::ai`, so the DTO is reconstructed via
/// its `Deserialize` impl from JSON built field-by-field off the zero-copy view.
fn output_view_to_dto(
    view: &crate::api::connect::generated::openoxide::api::v1::__buffa::view::AiGenerationOutputView<'_>,
) -> AppAiGenerationOutput {
    let value = serde_json::json!({
        "suggestions": view
            .suggestions
            .iter()
            .map(|s| serde_json::json!({
                "id": s.id,
                "name": s.name,
                "short_description": s.short_description,
                "description": s.description,
                "docker_compose": s.docker_compose,
                "env_variables": s
                    .env_variables
                    .iter()
                    .map(|e| serde_json::json!({ "name": e.name, "value": e.value }))
                    .collect::<Vec<_>>(),
                "domains": s
                    .domains
                    .iter()
                    .map(|d| serde_json::json!({
                        "host": d.host,
                        "port": d.port,
                        "service_name": d.service_name,
                    }))
                    .collect::<Vec<_>>(),
                "config_files": s
                    .config_files
                    .iter()
                    .map(|f| serde_json::json!({
                        "file_path": f.file_path,
                        "content": f.content,
                    }))
                    .collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>(),
    });
    match serde_json::from_value::<AppAiGenerationOutput>(value) {
        Ok(output) => output,
        Err(error) => {
            tracing::error!(error = %error, "failed to decode AI review output");
            AppAiGenerationOutput {
                suggestions: vec![],
            }
        }
    }
}
