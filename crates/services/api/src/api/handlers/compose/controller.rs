use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use super::helper::*;
use super::init::ComposeController;
use crate::api::connect::generated::openoxide::api::v1::{
    Compose, ComposeCreateMountRequest, ComposeCreateMountResponse, ComposeCreatePatchRequest,
    ComposeCreatePatchResponse, ComposeDependencies, ComposeExport, ComposeListMountsRequest,
    ComposeListMountsResponse, ComposeListPatchesRequest, ComposeListPatchesResponse,
    ComposeMountRequest, ComposeOperation, ComposeOperationRequest, ComposePatchRequest,
    ComposePatchType, ComposePreview, ComposeService, ComposeServiceExt, ComposeUpdateMountRequest,
    ComposeUpdateMountResponse, ComposeUpdatePatchRequest, ComposeUpdatePatchResponse,
    ComposeWebhookToken, CreateComposeRequest, DeleteComposeRequest, Empty, ExportComposeRequest,
    GetComposeRequest, ImportComposeRequest, InstallTemplateRequest, ListComposesRequest,
    ListComposesResponse, MoveComposeRequest, PreviewComposeRequest, ServiceMutationRequest,
    SetComposeSourceRequest, SetRawSourceRequest, UpdateComposeRequest,
    UpdateComposeResourcesRequest, UploadComposeRequest,
};
use crate::services::compute::compose::crud::{
    ComposeOperationResponseDto, ComposeResponseDto, CreateComposeDto, PatchComposeDto,
};
use crate::services::compute::compose::management::{
    ComposePreviewDto, ImportComposeDto, InstallComposeTemplateDto, UpsertComposePatchDto,
    UpsertComposeResourceDto,
};
use crate::services::compute::compose::queries::ComposeDependencyCounts;
use crate::services::compute::compose::source::{
    PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto, PatchComposeGiteaSourceDto,
    PatchComposeGithubSourceDto, PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};
use crate::services::compute::compose::{ComposeOperation as AppComposeOperation, ComposeType};

#[auto_grpc::service(connect = true, auth = true)]
impl ComposeService for ComposeController {
    #[permission(Application, CanRead)]
    async fn get_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, GetComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let item = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanRead)]
    async fn list_composes<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ListComposesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListComposesResponse> + Send + use<'a>> {
        let items = self
            .service
            .list_by_environment(request.environment_id)
            .await
            .map_err(map_error)?;
        let composes = items
            .into_iter()
            .map(ComposeResponseDto::from)
            .map(|c| compose_to_proto(&c))
            .collect();
        Response::ok(ListComposesResponse { composes })
    }

    #[permission(Application, CanCreate)]
    async fn create_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, CreateComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        // Canonical spelling comes from the wire or the enum default; the
        // DTO is typed so no caller can smuggle a non-canonical string in.
        let compose_type = match request.compose_type {
            None => ComposeType::DockerCompose,
            Some(value) => ComposeType::from_str_opt(value).ok_or_else(|| {
                ConnectError::invalid_argument(format!("invalid compose_type: {value}"))
            })?,
        };
        let dto = CreateComposeDto {
            name: request.name.to_owned(),
            description: request.description.map(str::to_owned),
            environment_id: request.environment_id,
            server_id: request.server_id,
            source_type: "GITHUB".into(),
            compose_type,
            compose_file: request.compose_file.map(str::to_owned).unwrap_or_default(),
        };
        let item = self.service.create(dto).await.map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn update_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, UpdateComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto: PatchComposeDto = serde_json::from_str(
            &serde_json::json!({
                "name": request.name,
                "description": request.description,
                "compose_file": request.compose_file,
                "server_id": request.server_id,
                "command": request.command,
                "randomize": request.randomize.map(i64::from),
                "env_var": request.env_var,
            })
            .to_string(),
        )
        .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .service
            .patch(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanDelete)]
    async fn delete_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, DeleteComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanCreate)]
    async fn set_github_source<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SetComposeSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = PatchComposeGithubSourceDto {
            repository: request.repository.to_owned(),
            owner: request.owner.to_owned(),
            branch: request
                .branch
                .map(str::to_owned)
                .unwrap_or_else(|| "main".into()),
            github_provider_id: request.provider_id,
            auto_deploy: None,
        };
        let item = self
            .service
            .set_github_source(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn set_gitlab_source<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SetComposeSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = PatchComposeGitlabSourceDto {
            gitlab_project_id: None,
            gitlab_repository: request.repository.to_owned(),
            gitlab_owner: request.owner.to_owned(),
            gitlab_branch: request
                .branch
                .map(str::to_owned)
                .unwrap_or_else(|| "main".into()),
            gitlab_path_namespace: None,
            gitlab_provider_id: request.provider_id,
            gitlab_build_path: request.build_path.map(str::to_owned),
        };
        let item = self
            .service
            .set_gitlab_source(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn set_bitbucket_source<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SetComposeSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = PatchComposeBitbucketSourceDto {
            bitbucket_repository: request.repository.to_owned(),
            bitbucket_repository_slug: None,
            bitbucket_owner: request.owner.to_owned(),
            bitbucket_branch: request
                .branch
                .map(str::to_owned)
                .unwrap_or_else(|| "main".into()),
            bitbucket_provider_id: request.provider_id,
            bitbucket_build_path: request.build_path.map(str::to_owned),
        };
        let item = self
            .service
            .set_bitbucket_source(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn set_gitea_source<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SetComposeSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = PatchComposeGiteaSourceDto {
            gitea_repository: request.repository.to_owned(),
            gitea_owner: request.owner.to_owned(),
            gitea_branch: request
                .branch
                .map(str::to_owned)
                .unwrap_or_else(|| "main".into()),
            gitea_provider_id: request.provider_id,
            gitea_build_path: request.build_path.map(str::to_owned),
        };
        let item = self
            .service
            .set_gitea_source(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn set_custom_git_source<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SetComposeSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = PatchComposeCustomGitSourceDto {
            custom_git_url: request.repository.to_owned(),
            custom_git_branch: request
                .branch
                .map(str::to_owned)
                .unwrap_or_else(|| "main".into()),
            custom_git_ssh_key_id: request.ssh_key_id,
            custom_git_build_path: request.build_path.map(str::to_owned),
        };
        let item = self
            .service
            .set_custom_git_source(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn set_raw_source<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SetRawSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = PatchComposeRawSourceDto {
            compose_file: request.content.to_owned(),
        };
        let item = self
            .service
            .set_raw_source(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn upload_source<'a>(
        &'a self,
        _ctx: RequestContext,
        _requests: connectrpc::InboundStream<UploadComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        // Client-streaming upload aggregation is not yet wired; reject
        // until a chunked upload protocol is defined.
        let error: ServiceResult<Compose> = Err(ConnectError::new(
            ErrorCode::Unimplemented,
            "upload_source streaming not yet implemented",
        ));
        error
    }

    #[permission(Application, CanUpdate)]
    async fn deploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppComposeOperation::Deploy, auth.user_id())
            .await
            .map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
        })
    }

    #[permission(Application, CanUpdate)]
    async fn redeploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppComposeOperation::Redeploy, auth.user_id())
            .await
            .map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
        })
    }

    #[permission(Application, CanUpdate)]
    async fn reload<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppComposeOperation::Reload, auth.user_id())
            .await
            .map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
        })
    }

    #[permission(Application, CanUpdate)]
    async fn start<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppComposeOperation::Start, auth.user_id())
            .await
            .map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
        })
    }

    #[permission(Application, CanUpdate)]
    async fn stop<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        self.service
            .stop_operation(request.id)
            .await
            .map_err(map_error)?;
        let compose = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_error)?;
        let compose_dto = ComposeResponseDto::from(compose);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&compose_dto)),
            operation: AppComposeOperation::Stop.as_str().to_string(),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn cancel<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .cancel_operation(request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanCreate)]
    async fn move_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, MoveComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let item = self
            .management
            .move_to_environment(request.id, request.target_environment_id)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn rotate_webhook_token<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeWebhookToken> + Send + use<'a>> {
        let token = self
            .management
            .rotate_webhook_token(request.id)
            .await
            .map_err(map_error)?;
        Response::ok(ComposeWebhookToken { token })
    }

    #[permission(Application, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeDependencies> + Send + use<'a>> {
        let deps = self
            .service
            .dependencies(request.id)
            .await
            .map_err(map_error)?;
        let ComposeDependencyCounts {
            active_deployments,
            deployments,
            domains,
            mounts,
            patches,
            schedules,
            backups,
            enabled_backups,
            preview_deployments,
            middleware,
            ports,
            redirects,
            security_rules,
            ..
        } = deps;
        let mut items = Vec::new();
        if active_deployments > 0 {
            items.push(format!("active_deployments: {active_deployments}"));
        }
        if deployments > 0 {
            items.push(format!("deployments: {deployments}"));
        }
        if domains > 0 {
            items.push(format!("domains: {domains}"));
        }
        if mounts > 0 {
            items.push(format!("mounts: {mounts}"));
        }
        if patches > 0 {
            items.push(format!("patches: {patches}"));
        }
        if schedules > 0 {
            items.push(format!("schedules: {schedules}"));
        }
        if backups > 0 {
            items.push(format!("backups: {backups}"));
        }
        if enabled_backups > 0 {
            items.push(format!("enabled_backups: {enabled_backups}"));
        }
        if preview_deployments > 0 {
            items.push(format!("preview_deployments: {preview_deployments}"));
        }
        if middleware > 0 {
            items.push(format!("middleware: {middleware}"));
        }
        if ports > 0 {
            items.push(format!("ports: {ports}"));
        }
        if redirects > 0 {
            items.push(format!("redirects: {redirects}"));
        }
        if security_rules > 0 {
            items.push(format!("security_rules: {security_rules}"));
        }
        Response::ok(ComposeDependencies { items })
    }

    #[permission(Application, CanRead)]
    async fn preview_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, PreviewComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposePreview> + Send + use<'a>> {
        let dto = ComposePreviewDto {
            compose_file: request.content.to_owned(),
            randomize: false,
            suffix: None,
            isolated_deployment: false,
            isolated_deployments_volume: false,
            app_name: None,
        };
        let result = self.management.preview(dto).map_err(map_error)?;
        Response::ok(ComposePreview {
            content: result.compose_file,
            services: result.services,
        })
    }

    #[permission(Application, CanCreate)]
    async fn install_template<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, InstallTemplateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = InstallComposeTemplateDto {
            name: request
                .name
                .map(str::to_owned)
                .unwrap_or_else(|| "compose".into()),
            description: None,
            environment_id: request.environment_id,
            server_id: request.server_id,
            compose_file: request.template.to_owned(),
            env_var: request.env_var.map(str::to_owned),
        };
        let item = self
            .management
            .install_template(dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanRead)]
    async fn export_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ExportComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeExport> + Send + use<'a>> {
        let bundle = self
            .transfer
            .export(request.id, request.include_secrets)
            .await
            .map_err(map_error)?;
        let archive = serde_json::to_string_pretty(&bundle)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(ComposeExport { archive })
    }

    #[permission(Application, CanCreate)]
    async fn import_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ImportComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = ImportComposeDto {
            target_environment_id: 0,
            target_server_id: None,
            name: None,
            archive: request.archive.to_owned(),
        };
        let item = self.transfer.import(dto).await.map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn remove_services<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ServiceMutationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let service_name = request.services.first().cloned().unwrap_or_default();
        let _ = self
            .management
            .remove_service(request.id, service_name)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanCreate)]
    async fn update_resources<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, UpdateComposeResourcesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        // The proto UpdateComposeResourcesRequest has simplified fields;
        // for full resource upsert, the payload JSON carries the DTO.
        let payload_json = serde_json::json!({
            "memory_limit": request.memory_limit,
            "cpu_limit": request.cpu_limit,
            "replicas": request.replicas,
        })
        .to_string();
        let dto: UpsertComposeResourceDto = serde_json::from_str(&payload_json)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .management
            .upsert_resource(request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanDelete)]
    async fn clear_deployment_history<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.management
            .clear_history(request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanUpdate)]
    async fn cleanup_deployment_queue<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.management
            .cleanup_queue(request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanUpdate)]
    async fn force_kill_deployment<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.management
            .force_kill(request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn list_mounts<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeListMountsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeListMountsResponse> + Send + use<'a>> {
        let items = self
            .mounts
            .list(request.compose_id)
            .await
            .map_err(map_error)?;
        let mounts = items
            .into_iter()
            .map(ComposeMountResponseDto::from)
            .map(mount_to_proto)
            .collect();
        Response::ok(ComposeListMountsResponse { mounts })
    }

    #[permission(Application, CanCreate)]
    async fn create_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeCreateMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeCreateMountResponse> + Send + use<'a>>
    {
        let input = mount_request_from_create(&request);
        let item = self
            .mounts
            .create(request.compose_id, input)
            .await
            .map_err(map_error)?;
        Response::ok(ComposeCreateMountResponse {
            mount: ::buffa::MessageField::some(mount_to_proto(ComposeMountResponseDto::from(item))),
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeUpdateMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeUpdateMountResponse> + Send + use<'a>>
    {
        let input = mount_request_from_update(&request);
        let item = self
            .mounts
            .update(request.compose_id, request.id, input)
            .await
            .map_err(map_error)?;
        Response::ok(ComposeUpdateMountResponse {
            mount: ::buffa::MessageField::some(mount_to_proto(ComposeMountResponseDto::from(item))),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let id = request
            .id
            .ok_or_else(|| ConnectError::invalid_argument("mount id is required"))?;
        let deleted = self
            .mounts
            .delete(request.compose_id, id)
            .await
            .map_err(map_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("compose mount not found"))
        }
    }

    #[permission(Application, CanRead)]
    async fn list_patches<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeListPatchesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeListPatchesResponse> + Send + use<'a>>
    {
        let items = self
            .patches
            .list(request.compose_id)
            .await
            .map_err(map_error)?;
        let patches = items
            .into_iter()
            .map(ComposePatchResponseDto::from)
            .map(patch_to_proto)
            .collect();
        Response::ok(ComposeListPatchesResponse { patches })
    }

    #[permission(Application, CanCreate)]
    async fn create_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeCreatePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeCreatePatchResponse> + Send + use<'a>>
    {
        let dto = patch_dto_from_request(&request)?;
        let item = self
            .patches
            .create(request.compose_id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(ComposeCreatePatchResponse {
            patch: ::buffa::MessageField::some(patch_to_proto(ComposePatchResponseDto::from(item))),
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeUpdatePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeUpdatePatchResponse> + Send + use<'a>>
    {
        let dto = patch_dto_from_request(&request)?;
        let item = self
            .patches
            .update(request.compose_id, request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(ComposeUpdatePatchResponse {
            patch: ::buffa::MessageField::some(patch_to_proto(ComposePatchResponseDto::from(item))),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let id = request
            .id
            .ok_or_else(|| ConnectError::invalid_argument("patch id is required"))?;
        let deleted = self
            .patches
            .delete(request.compose_id, id)
            .await
            .map_err(map_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("compose patch not found"))
        }
    }
}

/// Typed create-mount request → the string-based service `ComposeMountRequest`.
fn mount_request_from_create(
    request: &ServiceRequest<'_, ComposeCreateMountRequest>,
) -> ComposeMountRequest {
    ComposeMountRequest {
        compose_id: request.compose_id,
        id: None,
        mount_type: compose_mount_type_to_spec_str(request.mount_type),
        host_path: request.host_path.map(str::to_owned),
        volume_name: request.volume_name.map(str::to_owned),
        file_path: request.file_path.map(str::to_owned),
        content: request.content.map(str::to_owned),
        mount_path: request.mount_path.to_owned(),
    }
}

/// Typed update-mount request → the string-based service `ComposeMountRequest`.
fn mount_request_from_update(
    request: &ServiceRequest<'_, ComposeUpdateMountRequest>,
) -> ComposeMountRequest {
    ComposeMountRequest {
        compose_id: request.compose_id,
        id: Some(request.id),
        mount_type: compose_mount_type_to_spec_str(request.mount_type),
        host_path: request.host_path.map(str::to_owned),
        volume_name: request.volume_name.map(str::to_owned),
        file_path: request.file_path.map(str::to_owned),
        content: request.content.map(str::to_owned),
        mount_path: request.mount_path.to_owned(),
    }
}

/// Typed patch request → service `UpsertComposePatchDto`. Both create and
/// update requests share the same field layout, so a single generic helper
/// covers them via the trait the generated messages implement.
fn patch_dto_from_request<R>(request: &R) -> Result<UpsertComposePatchDto, ConnectError>
where
    R: PatchRequestFields,
{
    let patch_type = request.patch_type();
    let parsed = match patch_type {
        buffa::EnumValue::Known(ComposePatchType::Delete) => "DELETE",
        _ => "UPDATE",
    };
    Ok(UpsertComposePatchDto {
        patch_type: parsed.parse().map_err(ConnectError::invalid_argument)?,
        file_path: request.file_path(),
        enabled: request.enabled(),
        content: request.content(),
    })
}

/// Minimal accessor trait so `patch_dto_from_request` can read the identical
/// fields off the create and update patch request views without duplicating
/// the body.
trait PatchRequestFields {
    fn patch_type(&self) -> buffa::EnumValue<ComposePatchType>;
    fn file_path(&self) -> String;
    fn enabled(&self) -> bool;
    fn content(&self) -> String;
}

impl PatchRequestFields for ServiceRequest<'_, ComposeCreatePatchRequest> {
    fn patch_type(&self) -> buffa::EnumValue<ComposePatchType> {
        self.patch_type
    }
    fn file_path(&self) -> String {
        self.file_path.to_owned()
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn content(&self) -> String {
        self.content.to_owned()
    }
}

impl PatchRequestFields for ServiceRequest<'_, ComposeUpdatePatchRequest> {
    fn patch_type(&self) -> buffa::EnumValue<ComposePatchType> {
        self.patch_type
    }
    fn file_path(&self) -> String {
        self.file_path.to_owned()
    }
    fn enabled(&self) -> bool {
        self.enabled
    }
    fn content(&self) -> String {
        self.content.to_owned()
    }
}
