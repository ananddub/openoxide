use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AiAnalyzeLogsRequest, AiAnalyzeLogsResponse, AiApplyFixRequest, AiApplyFixResponse,
    AiAssistantEvent, AiAssistantEventKind, AiAssistantRequest, AiAssistantResponse,
    AiCreateGenerationRequest, AiCreateGenerationResponse, AiCreateSettingsRequest,
    AiCreateSettingsResponse, AiDeleteSettingsRequest, AiDeployGenerationRequest,
    AiDeployGenerationResponse, AiDiscoverModelsRequest, AiDiscoverModelsResponse,
    AiGetBuildDiagnosisRequest, AiGetBuildDiagnosisResponse, AiGetGenerationRequest,
    AiGetGenerationResponse, AiGetSettingsRequest, AiGetSettingsResponse, AiInteraction,
    AiInteractionKind, AiInteractionOption, AiListGenerationsRequest, AiListGenerationsResponse,
    AiListModelsRequest, AiListModelsResponse, AiListSettingsRequest, AiListSettingsResponse,
    AiLogContext, AiReviewGenerationRequest, AiReviewGenerationResponse, AiService, AiServiceExt,
    AiTestConnectionRequest, AiTestConnectionResponse, AiTestSettingsRequest,
    AiTestSettingsResponse, AiUpdateSettingsRequest, AiUpdateSettingsResponse, Empty,
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
    async fn get_build_diagnosis<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiGetBuildDiagnosisRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiGetBuildDiagnosisResponse> + Send + use<'a>>
    {
        let _ = &ctx;
        match self
            .service
            .get_build_diagnosis(request.deployment_id, request.force)
            .await
        {
            Some(d) => Response::ok(AiGetBuildDiagnosisResponse {
                available: true,
                classification: d.classification.as_str().to_string(),
                root_cause: d.root_cause.clone(),
                explanation: d.explanation.clone(),
                fixable: d.is_fixable(),
                fix_target: d.fix_target.clone().unwrap_or_default(),
            }),
            None => Response::ok(AiGetBuildDiagnosisResponse {
                available: false,
                classification: String::new(),
                root_cause: String::new(),
                explanation: String::new(),
                fixable: false,
                fix_target: String::new(),
            }),
        }
    }

    #[permission(Application, CanUpdate)]
    async fn apply_ai_fix<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiApplyFixRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiApplyFixResponse> + Send + use<'a>> {
        let auth = Self::extract_auth(&ctx).await?;
        match self
            .service
            .apply_ai_fix(request.deployment_id, auth.user_id())
            .await
        {
            Ok(message) => Response::ok(AiApplyFixResponse {
                applied: true,
                message,
            }),
            Err(message) => Response::ok(AiApplyFixResponse {
                applied: false,
                message,
            }),
        }
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

    #[permission(Application, CanCreate)]
    async fn assistant<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiAssistantRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AiAssistantResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        // Forward the caller's own bearer token so the assistant's action calls run under the same identity.
        let token = ctx
            .header("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.strip_prefix("Bearer ")
                    .or_else(|| v.strip_prefix("bearer "))
            })
            .map(str::to_owned)
            .ok_or_else(|| ConnectError::unauthenticated("missing bearer token"))?;
        let history: Vec<(String, String)> = request
            .history
            .iter()
            .map(|t| (t.role.to_owned(), t.content.to_owned()))
            .collect();
        let reply = self
            .service
            .assistant(
                organization_id,
                auth.user_id(),
                token,
                request.ai_setting_id,
                history,
                request
                    .pending_actions
                    .map(str::to_owned)
                    .unwrap_or_default(),
                request.prompt.to_owned(),
            )
            .await
            .map_err(map_error)?;
        let interaction = reply.interaction.map(|i| {
            use crate::services::platform::ai::agent::InteractionKind;
            AiInteraction {
                kind: match i.kind {
                    InteractionKind::Confirm => AiInteractionKind::Confirm,
                    InteractionKind::Choice => AiInteractionKind::Choice,
                }
                .into(),
                question: i.question,
                options: i
                    .options
                    .into_iter()
                    .map(|o| AiInteractionOption {
                        id: o.id,
                        label: o.label,
                        description: o.description,
                    })
                    .collect(),
                pending_actions: i.pending_actions,
            }
        });
        // Keep the legacy field populated for a CONFIRM so older clients still work.
        let pending_confirmation = interaction.as_ref().and_then(|i| {
            matches!(
                i.kind,
                ::buffa::EnumValue::Known(AiInteractionKind::Confirm)
            )
            .then(|| i.question.clone())
        });
        Response::ok(AiAssistantResponse {
            reply: reply.text,
            pending_confirmation,
            interaction: interaction.into(),
        })
    }

    #[permission(Application, CanCreate)]
    async fn assistant_stream(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, AiAssistantRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<impl connectrpc::Encodable<AiAssistantEvent> + Send + use<>>,
    > {
        let auth = Self::extract_auth(&ctx).await?;
        let organization_id = auth.organization_id;
        let token = ctx
            .header("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                v.strip_prefix("Bearer ")
                    .or_else(|| v.strip_prefix("bearer "))
            })
            .map(str::to_owned)
            .ok_or_else(|| ConnectError::unauthenticated("missing bearer token"))?;

        let (session_id, mut rx) = self
            .service
            .assistant_stream(
                organization_id,
                auth.user_id(),
                token,
                request.ai_setting_id,
                request.session_id.map(str::to_owned),
                request
                    .pending_actions
                    .map(str::to_owned)
                    .unwrap_or_default(),
                request.prompt.to_owned(),
            )
            .await
            .map_err(map_error)?;

        // Bridge the agent's AgentEvent channel to the proto event stream.
        let (out_tx, out_rx) =
            tokio::sync::mpsc::channel::<Result<AiAssistantEvent, ConnectError>>(64);
        let sid = session_id.clone();
        tokio::spawn(async move {
            // First event carries the session id so the client can store it.
            let _ = out_tx
                .send(Ok(AiAssistantEvent {
                    kind: AiAssistantEventKind::Thinking.into(),
                    text: String::new(),
                    interaction: ::buffa::MessageField::none(),
                    session_id: Some(sid.clone()),
                }))
                .await;
            while let Some(event) = rx.recv().await {
                let proto = agent_event_to_proto(event, &sid);
                if out_tx.send(Ok(proto)).await.is_err() {
                    break;
                }
            }
        });

        Response::stream_ok(tokio_stream::wrappers::ReceiverStream::new(out_rx))
    }
}

/// Map a runner AgentEvent to the proto streaming event.
fn agent_event_to_proto(
    event: crate::services::platform::ai::agent::AgentEvent,
    session_id: &str,
) -> AiAssistantEvent {
    use crate::services::platform::ai::agent::{AgentEvent, InteractionKind};
    let (kind, text, interaction) = match event {
        AgentEvent::Thinking(t) => (AiAssistantEventKind::Thinking, t, None),
        AgentEvent::Status(t) => (AiAssistantEventKind::Status, t, None),
        AgentEvent::ActionResult(t) => (AiAssistantEventKind::ActionResult, t, None),
        AgentEvent::Reply(t) => (AiAssistantEventKind::Reply, t, None),
        AgentEvent::Done => (AiAssistantEventKind::Done, String::new(), None),
        AgentEvent::Interaction(i) => {
            let proto = AiInteraction {
                kind: match i.kind {
                    InteractionKind::Confirm => AiInteractionKind::Confirm,
                    InteractionKind::Choice => AiInteractionKind::Choice,
                }
                .into(),
                question: i.question.clone(),
                options: i
                    .options
                    .into_iter()
                    .map(|o| AiInteractionOption {
                        id: o.id,
                        label: o.label,
                        description: o.description,
                    })
                    .collect(),
                pending_actions: i.pending_actions,
            };
            (AiAssistantEventKind::Interaction, i.question, Some(proto))
        }
    };
    AiAssistantEvent {
        kind: kind.into(),
        text,
        interaction: interaction.into(),
        session_id: Some(session_id.to_owned()),
    }
}

/// Rebuilds the review DTO via its `Deserialize` impl, since the service's inner suggestion types aren't re-exported from `services::platform::ai`.
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
