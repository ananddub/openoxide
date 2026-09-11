use std::sync::Arc;

use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Compose, ComposeDependencies, ComposeExport, ComposeMountRequest, ComposeOperation,
    ComposeOperationRequest, ComposePatchRequest, ComposePreview, ComposeService,
    ComposeServiceExt, ComposeWebhookToken, CreateComposeRequest, DeleteComposeRequest,
    Empty, ExportComposeRequest, GetComposeRequest, ImportComposeRequest,
    InstallTemplateRequest, ListComposesRequest, ListComposesResponse,
    MoveComposeRequest, PreviewComposeRequest, ResourceListResponse, ResourceResponse,
    ServiceMutationRequest, SetComposeSourceRequest, SetRawSourceRequest,
    UpdateComposeRequest, UpdateComposeResourcesRequest, UploadComposeRequest,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::{Mounts, Patches, Services};
use crate::services::compute::compose::crud::{
    ComposeOperationResponseDto, ComposeResponseDto, CreateComposeDto, PatchComposeDto,
};
use crate::services::compute::compose::queries::ComposeDependencyCounts;
use crate::services::compute::compose::management::{
    ComposePreviewDto, ImportComposeDto, InstallComposeTemplateDto, UpsertComposePatchDto,
    UpsertComposeResourceDto,
};
use crate::services::compute::compose::source::{
    PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto, PatchComposeGiteaSourceDto,
    PatchComposeGithubSourceDto, PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};
use crate::services::compute::compose::ComposeOperation as AppComposeOperation;
use super::helper::*;
use super::init::ComposeController;

#[auto_grpc::service(connect = true, auth = true)]
impl ComposeService for ComposeController {
    #[permission(Application, CanRead)]
    async fn get_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, GetComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let item = self.service.get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanRead)]
    async fn stream_compose(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetComposeRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<Compose> + Send + use<>,
        >,
    > {
        let id = request.id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT s.* FROM services s JOIN service_compose c ON c.service_id = s.id WHERE s.id = {id} AND s.service_type = 'COMPOSE'"
            );
            realtime()
                .watch_query::<Services>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.then(move |mut rows| {
            let service = Arc::clone(&service);
            async move {
                match rows.pop() {
                    Some(row) => match service.get_by_id(row.id).await {
                        Ok(item) => Ok(compose_to_proto(&ComposeResponseDto::from(item))),
                        _ => Err(ConnectError::not_found("compose project not found")),
                    },
                    None => Err(ConnectError::not_found("compose project not found")),
                }
            }
        }))
    }

    #[permission(Application, CanRead)]
    async fn list_composes<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ListComposesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListComposesResponse> + Send + use<'a>> {
        let items = self.service.list_by_environment(request.environment_id).await.map_err(map_error)?;
        let composes = items.into_iter().map(ComposeResponseDto::from).map(|c| compose_to_proto(&c)).collect();
        Response::ok(ListComposesResponse { composes, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn stream_list_composes(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListComposesRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ListComposesResponse> + Send + use<>,
        >,
    > {
        let environment_id = request.environment_id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT s.* FROM services s JOIN service_compose c ON c.service_id = s.id WHERE s.environment_id = {environment_id} AND s.service_type = 'COMPOSE'"
            );
            realtime()
                .watch_query::<Services>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.then(move |rows| {
            let service = Arc::clone(&service);
            async move {
                let mut items = Vec::with_capacity(rows.len());
                for row in rows {
                    if let Ok(item) = service.get_by_id(row.id).await {
                        items.push(compose_to_proto(&ComposeResponseDto::from(item)));
                    }
                }
                Ok(ListComposesResponse { composes: items, ..Default::default() })
            }
        }))
    }

    #[permission(Application, CanCreate)]
    async fn create_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, CreateComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = CreateComposeDto {
            name: request.name.to_owned(),
            description: request.description.map(str::to_owned),
            environment_id: request.environment_id,
            server_id: request.server_id,
            source_type: "GITHUB".into(),
            compose_type: "DOCKER_COMPOSE".into(),
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
            }).to_string(),
        ).map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.service.patch(request.id, dto).await.map_err(map_error)?;
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
            branch: request.branch.map(str::to_owned).unwrap_or_else(|| "main".into()),
            github_provider_id: request.provider_id,
            auto_deploy: None,
        };
        let item = self.service.set_github_source(request.id, dto).await.map_err(map_error)?;
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
            gitlab_branch: request.branch.map(str::to_owned).unwrap_or_else(|| "main".into()),
            gitlab_path_namespace: None,
            gitlab_provider_id: request.provider_id,
            gitlab_build_path: request.build_path.map(str::to_owned),
        };
        let item = self.service.set_gitlab_source(request.id, dto).await.map_err(map_error)?;
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
            bitbucket_branch: request.branch.map(str::to_owned).unwrap_or_else(|| "main".into()),
            bitbucket_provider_id: request.provider_id,
            bitbucket_build_path: request.build_path.map(str::to_owned),
        };
        let item = self.service.set_bitbucket_source(request.id, dto).await.map_err(map_error)?;
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
            gitea_branch: request.branch.map(str::to_owned).unwrap_or_else(|| "main".into()),
            gitea_provider_id: request.provider_id,
            gitea_build_path: request.build_path.map(str::to_owned),
        };
        let item = self.service.set_gitea_source(request.id, dto).await.map_err(map_error)?;
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
            custom_git_branch: request.branch.map(str::to_owned).unwrap_or_else(|| "main".into()),
            custom_git_ssh_key_id: request.ssh_key_id,
            custom_git_build_path: request.build_path.map(str::to_owned),
        };
        let item = self.service.set_custom_git_source(request.id, dto).await.map_err(map_error)?;
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
        let item = self.service.set_raw_source(request.id, dto).await.map_err(map_error)?;
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

    #[permission(Application, CanDeploy)]
    async fn deploy<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self.service.run_operation(request.id, AppComposeOperation::Deploy).await.map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn redeploy<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self.service.run_operation(request.id, AppComposeOperation::Redeploy).await.map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn reload<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self.service.run_operation(request.id, AppComposeOperation::Reload).await.map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn start<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        let result = self.service.run_operation(request.id, AppComposeOperation::Start).await.map_err(map_error)?;
        let dto = ComposeOperationResponseDto::from(result);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&dto.compose)),
            operation: dto.operation,
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn stop<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeOperation> + Send + use<'a>> {
        self.service.stop_operation(request.id).await.map_err(map_error)?;
        let compose = self.service.get_by_id(request.id).await.map_err(map_error)?;
        let compose_dto = ComposeResponseDto::from(compose);
        Response::ok(ComposeOperation {
            compose: ::buffa::MessageField::some(compose_to_proto(&compose_dto)),
            operation: AppComposeOperation::Stop.as_str().to_string(),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn cancel<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.cancel_operation(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanCreate)]
    async fn move_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, MoveComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let item = self.management
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
        let token = self.management.rotate_webhook_token(request.id).await.map_err(map_error)?;
        Response::ok(ComposeWebhookToken { token, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeDependencies> + Send + use<'a>> {
        let deps = self.service.dependencies(request.id).await.map_err(map_error)?;
        let ComposeDependencyCounts {
            active_deployments, deployments, domains, mounts, patches,
            schedules, backups, enabled_backups, preview_deployments,
            middleware, ports, redirects, security_rules, ..
        } = deps;
        let mut items = Vec::new();
        if active_deployments > 0 { items.push(format!("active_deployments: {active_deployments}")); }
        if deployments > 0 { items.push(format!("deployments: {deployments}")); }
        if domains > 0 { items.push(format!("domains: {domains}")); }
        if mounts > 0 { items.push(format!("mounts: {mounts}")); }
        if patches > 0 { items.push(format!("patches: {patches}")); }
        if schedules > 0 { items.push(format!("schedules: {schedules}")); }
        if backups > 0 { items.push(format!("backups: {backups}")); }
        if enabled_backups > 0 { items.push(format!("enabled_backups: {enabled_backups}")); }
        if preview_deployments > 0 { items.push(format!("preview_deployments: {preview_deployments}")); }
        if middleware > 0 { items.push(format!("middleware: {middleware}")); }
        if ports > 0 { items.push(format!("ports: {ports}")); }
        if redirects > 0 { items.push(format!("redirects: {redirects}")); }
        if security_rules > 0 { items.push(format!("security_rules: {security_rules}")); }
        Response::ok(ComposeDependencies { items, ..Default::default() })
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
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn install_template<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, InstallTemplateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Compose> + Send + use<'a>> {
        let dto = InstallComposeTemplateDto {
            name: request.name.map(str::to_owned).unwrap_or_else(|| "compose".into()),
            description: None,
            environment_id: 0,
            server_id: None,
            compose_file: request.template.to_owned(),
            env_var: None,
        };
        let item = self.management.install_template(dto).await.map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanRead)]
    async fn export_compose<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ExportComposeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ComposeExport> + Send + use<'a>> {
        let bundle = self.transfer.export(request.id, request.include_secrets).await.map_err(map_error)?;
        let archive = serde_json::to_string_pretty(&bundle)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(ComposeExport { archive, ..Default::default() })
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
        let _ = self.management.remove_service(request.id, &service_name).await.map_err(map_error)?;
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
        }).to_string();
        let dto: UpsertComposeResourceDto = serde_json::from_str(&payload_json)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.management.upsert_resource(request.id, dto).await.map_err(map_error)?;
        Response::ok(compose_to_proto(&ComposeResponseDto::from(item)))
    }

    #[permission(Application, CanDelete)]
    async fn clear_deployment_history<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.management.clear_history(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn cleanup_deployment_queue<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.management.cleanup_queue(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn force_kill_deployment<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.management.force_kill(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn list_mounts<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.mounts.list(request.compose_id).await.map_err(map_error)?;
        let items = items.into_iter()
            .map(ComposeMountResponseDto::from)
            .map(mount_to_resource)
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn stream_list_mounts(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeMountRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let compose_id = request.compose_id;
        let stream = {
            let sql = format!("SELECT * FROM mounts WHERE service_id = {compose_id}");
            realtime()
                .watch_query::<Mounts>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(ComposeMountResponseDto::from)
                    .map(mount_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Application, CanCreate)]
    async fn create_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let item = self
            .mounts
            .create(request.compose_id, request.to_owned_message())
            .await
            .map_err(map_error)?;
        Response::ok(mount_to_resource(ComposeMountResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn update_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let id = request.id.ok_or_else(|| ConnectError::invalid_argument("mount id is required"))?;
        let item = self
            .mounts
            .update(request.compose_id, id, request.to_owned_message())
            .await
            .map_err(map_error)?;
        Response::ok(mount_to_resource(ComposeMountResponseDto::from(item)))
    }

    #[permission(Application, CanDelete)]
    async fn delete_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposeMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let id = request.id.ok_or_else(|| ConnectError::invalid_argument("mount id is required"))?;
        let deleted = self.mounts.delete(request.compose_id, id).await.map_err(map_error)?;
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
        request: ServiceRequest<'_, ComposePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.patches.list(request.compose_id).await.map_err(map_error)?;
        let items = items.into_iter()
            .map(ComposePatchResponseDto::from)
            .map(patch_to_resource)
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn stream_list_patches(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposePatchRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let compose_id = request.compose_id;
        let stream = {
            let sql = format!("SELECT * FROM patches WHERE service_id = {compose_id}");
            realtime()
                .watch_query::<Patches>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(ComposePatchResponseDto::from)
                    .map(patch_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Application, CanCreate)]
    async fn create_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: UpsertComposePatchDto = serde_json::from_str(
            &serde_json::json!({
                "patch_type": request.patch_type,
                "payload": request.payload,
            }).to_string(),
        ).map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.patches.create(request.compose_id, dto).await.map_err(map_error)?;
        Response::ok(patch_to_resource(ComposePatchResponseDto::from(item)))
    }

    #[permission(Application, CanCreate)]
    async fn update_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let id = request.id.ok_or_else(|| ConnectError::invalid_argument("patch id is required"))?;
        let dto: UpsertComposePatchDto = serde_json::from_str(
            &serde_json::json!({
                "patch_type": request.patch_type,
                "payload": request.payload,
            }).to_string(),
        ).map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.patches.update(request.compose_id, id, dto).await.map_err(map_error)?;
        Response::ok(patch_to_resource(ComposePatchResponseDto::from(item)))
    }

    #[permission(Application, CanDelete)]
    async fn delete_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ComposePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let id = request.id.ok_or_else(|| ConnectError::invalid_argument("patch id is required"))?;
        let deleted = self.patches.delete(request.compose_id, id).await.map_err(map_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("compose patch not found"))
        }
    }
}
