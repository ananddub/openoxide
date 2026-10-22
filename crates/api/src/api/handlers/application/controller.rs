use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use crate::api::connect::generated::openoxide::api::v1::{
    Application as ApplicationProto, ApplicationCreateMiddlewareRequest,
    ApplicationCreateMiddlewareResponse, ApplicationCreateMountRequest,
    ApplicationCreateMountResponse, ApplicationCreatePatchRequest, ApplicationCreatePatchResponse,
    ApplicationCreatePortRequest, ApplicationCreatePortResponse, ApplicationCreateRedirectRequest,
    ApplicationCreateRedirectResponse, ApplicationCreateSecurityRequest,
    ApplicationCreateSecurityResponse, ApplicationDeleteMiddlewareRequest,
    ApplicationDeleteMountRequest, ApplicationDeletePatchRequest, ApplicationDeletePortRequest,
    ApplicationDeleteRedirectRequest, ApplicationDeleteSecurityRequest, ApplicationDependencies,
    ApplicationExport, ApplicationGetSecurityRequest, ApplicationGetSecurityResponse,
    ApplicationListMiddlewaresRequest, ApplicationListMiddlewaresResponse,
    ApplicationListMountsRequest, ApplicationListMountsResponse, ApplicationListPatchesRequest,
    ApplicationListPatchesResponse, ApplicationListPortsRequest, ApplicationListPortsResponse,
    ApplicationListRedirectsRequest, ApplicationListRedirectsResponse, ApplicationMiddlewares,
    ApplicationMounts, ApplicationNetworks, ApplicationOperation, ApplicationOperationRequest,
    ApplicationPatches, ApplicationPorts, ApplicationRedirects, ApplicationSecurity,
    ApplicationService, ApplicationServiceExt, ApplicationUpdateMiddlewareRequest,
    ApplicationUpdateMiddlewareResponse, ApplicationUpdateMountRequest,
    ApplicationUpdateMountResponse, ApplicationUpdateNetworksRequest,
    ApplicationUpdateNetworksResponse, ApplicationUpdatePatchRequest,
    ApplicationUpdatePatchResponse, ApplicationUpdatePortRequest, ApplicationUpdatePortResponse,
    ApplicationUpdateRedirectRequest, ApplicationUpdateRedirectResponse,
    ApplicationUpdateSecurityRequest, ApplicationUpdateSecurityResponse, ApplicationWebhookToken,
    CreateApplicationRequest, DeleteApplicationRequest, DeleteRollbackRequest, Empty,
    ExportApplicationRequest, GetApplicationRequest, ImportApplicationRequest,
    ListApplicationsRequest, ListApplicationsResponse, ListRollbacksRequest, ListRollbacksResponse,
    MoveApplicationRequest, ResourceListRequest, Rollback, RollbackTrigger,
    SetBitbucketSourceRequest, SetCustomGitSourceRequest, SetDockerSourceRequest,
    SetDropSourceRequest, SetGiteaSourceRequest, SetGithubSourceRequest, SetGitlabSourceRequest,
    TriggerRollbackRequest, UpdateApplicationRequest, UpdateBuildConfigRequest,
    UpdatePreviewConfigRequest, UpdateResourceConfigRequest, UploadSourceRequest,
};
use crate::flash_gen::models::Mounts;
use crate::services::compute::application::ApplicationOperation as AppApplicationOperation;
use crate::services::compute::application::import_export::ImportApplicationDto;

use super::helper::*;
use super::init::ApplicationController;

/// Builds the middleware upsert DTO from already-converted owned pieces,
/// folding empty collections to `None` so the service validation paths
/// (which `unwrap_or_default()`) behave identically to the JSON envelope.
fn build_middleware_dto(
    name: String,
    middleware_type: crate::services::compute::application::middleware::ApplicationMiddlewareType,
    enabled: bool,
    headers: Vec<(String, String)>,
    average: Option<i64>,
    burst: Option<i64>,
    source_ranges: Vec<String>,
) -> crate::services::compute::application::middleware::UpsertApplicationMiddlewareDto {
    use crate::services::compute::application::middleware::{
        MiddlewareHeaderDto, UpsertApplicationMiddlewareDto,
    };
    UpsertApplicationMiddlewareDto {
        name,
        middleware_type,
        enabled,
        headers: (!headers.is_empty()).then(|| {
            headers
                .into_iter()
                .map(|(name, value)| MiddlewareHeaderDto { name, value })
                .collect()
        }),
        average,
        burst,
        source_ranges: (!source_ranges.is_empty()).then_some(source_ranges),
    }
}

/// Mirrors `mount.rs normalize_row`: Flash bind params write `None` as `""`,
/// so empty strings fold back to `None` before DTO conversion.
fn normalize_mount_row(row: Mounts) -> Mounts {
    let non_empty = |value: Option<String>| value.filter(|value| !value.is_empty());
    Mounts {
        host_path: non_empty(row.host_path),
        volume_name: non_empty(row.volume_name),
        file_path: non_empty(row.file_path),
        content: non_empty(row.content),
        ..row
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl ApplicationService for ApplicationController {
    // ── Core CRUD ──────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn get_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanRead)]
    async fn list_applications<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListApplicationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListApplicationsResponse> + Send + use<'a>> {
        let items = self
            .service
            .list_by_environment(request.environment_id)
            .await
            .map_err(map_db_error)?;
        let applications = items.into_iter().map(app_to_proto).collect();
        Response::ok(ListApplicationsResponse { applications })
    }

    #[permission(Application, CanCreate)]
    async fn create_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .create(request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn update_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .patch(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanDelete)]
    async fn delete_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Import / Export ────────────────────────────────────────────────
    //Todo: Incomplete Import Application
    #[permission(Application, CanCreate)]
    async fn import_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ImportApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        // ImportApplicationRequest only carries the archive string.
        // The full ImportApplicationDto (with target_environment_id etc.)
        // must be encoded in the archive JSON or passed via a wrapper.
        // Try to parse the archive as the full DTO first; fall back to
        // using the archive field directly.
        let dto: ImportApplicationDto = match serde_json::from_str(request.archive) {
            Ok(dto) => dto,
            Err(_) => {
                return Err(ConnectError::invalid_argument(
                    "archive must be a JSON-encoded ImportApplicationDto with target_environment_id",
                ));
            }
        };
        let app = self.transfer.import(dto).await.map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanRead)]
    async fn export_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ExportApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationExport> + Send + use<'a>> {
        let bundle = self
            .transfer
            .export(request.id, request.include_secrets)
            .await
            .map_err(map_db_error)?;
        let archive = serde_json::to_string_pretty(&bundle).map_err(|error| {
            tracing::error!(%error, "could not serialize application export");
            ConnectError::new(
                ErrorCode::Internal,
                "could not serialize application export",
            )
        })?;
        Response::ok(ApplicationExport {
            format: "openoxide.application+json".into(),
            schema_version: i64::from(bundle.schema_version),
            archive,
        })
    }

    // ── Source configuration ───────────────────────────────────────────

    #[permission(Application, CanUpdate)]
    async fn set_github_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetGithubSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_github_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn set_gitlab_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetGitlabSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_gitlab_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn set_bitbucket_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetBitbucketSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_bitbucket_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn set_gitea_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetGiteaSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_gitea_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn set_custom_git_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetCustomGitSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_custom_git_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn set_docker_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetDockerSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_docker_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn set_drop_source<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SetDropSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .set_drop_source(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    // ── Upload source (client-streaming) ───────────────────────────────
    // First frame carries the header (application id + build path); the
    // rest are raw ZIP segments (≤ 4 MiB each). The archive is staged on the
    // panel, then installed through the Agent: upload → sanitize → extract
    // into the application's code directory.

    #[permission(Application, CanCreate)]
    async fn upload_source<'a>(
        &'a self,
        _ctx: RequestContext,
        mut requests: connectrpc::InboundStream<UploadSourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        use futures::StreamExt;

        use crate::api::connect::generated::openoxide::api::v1::upload_source_request::Payload;

        let first = requests
            .next()
            .await
            .ok_or_else(|| ConnectError::invalid_argument("upload stream is empty"))?
            .map_err(|error| ConnectError::new(ErrorCode::Internal, error.to_string()))?;
        let Payload::Header(header) = first
            .to_owned_message()
            .payload
            .ok_or_else(|| ConnectError::invalid_argument("first frame must carry the header"))?
        else {
            return Err(ConnectError::invalid_argument(
                "first frame must carry the header",
            ));
        };

        const MAX_UPLOAD_BYTES: u64 = 512 * 1024 * 1024;
        let application_id = header.application_id;
        let _app = self
            .service
            .get_by_id(application_id)
            .await
            .map_err(map_db_error)?;

        let staging =
            std::env::temp_dir().join(format!("openoxide-drop-{}.zip", uuid::Uuid::new_v4()));
        let staged = async {
            let mut file = tokio::fs::File::create(&staging)
                .await
                .map_err(|error| ConnectError::internal(format!("stage upload: {error}")))?;
            let mut written = 0u64;
            while let Some(message) = requests.next().await {
                let message = message
                    .map_err(|error| ConnectError::new(ErrorCode::Internal, error.to_string()))?;
                match message.to_owned_message().payload {
                    Some(Payload::Chunk(chunk)) => {
                        written += chunk.len() as u64;
                        if written > MAX_UPLOAD_BYTES {
                            return Err(ConnectError::invalid_argument(
                                "upload exceeds the 512 MiB limit",
                            ));
                        }
                        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk)
                            .await
                            .map_err(|error| {
                                ConnectError::internal(format!("stage chunk: {error}"))
                            })?;
                    }
                    Some(Payload::Header(_)) => {
                        return Err(ConnectError::invalid_argument(
                            "only one header frame is allowed",
                        ));
                    }
                    None => {}
                }
            }
            if written == 0 {
                return Err(ConnectError::invalid_argument("upload contains no data"));
            }
            tokio::io::AsyncWriteExt::shutdown(&mut file)
                .await
                .map_err(|error| ConnectError::internal(format!("close upload: {error}")))?;
            Ok::<u64, ConnectError>(written)
        }
        .await;
        if staged.is_err() {
            let _ = tokio::fs::remove_file(&staging).await;
        }
        staged?;

        // Record the drop source first: it clears any previous git/docker
        // source so the builder switches to the on-disk code directory.
        let app = self
            .service
            .set_drop_source(
                application_id,
                SetDropSourceRequest {
                    id: application_id,
                    build_path: header.build_path,
                },
            )
            .await
            .map_err(map_db_error)?;

        let install = crate::services::compute::application::source::install_drop_zip(
            &app.app_name,
            app.server_id,
            &staging,
        )
        .await;
        let _ = tokio::fs::remove_file(&staging).await;
        install.map_err(ConnectError::internal)?;

        Response::ok(app_to_proto(app))
    }

    // ── Build / Resource / Preview config ──────────────────────────────

    #[permission(Application, CanUpdate)]
    async fn update_build_config<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateBuildConfigRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .patch_build_config(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn update_resource_config<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateResourceConfigRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .patch_resource_config(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    #[permission(Application, CanUpdate)]
    async fn update_preview_config<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdatePreviewConfigRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        self.service
            .patch_preview_config(request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        let app = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    // ── Deploy operations ──────────────────────────────────────────────

    #[permission(Application, CanUpdate)]
    async fn deploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Deploy, auth.user_id())
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanUpdate)]
    async fn redeploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(
                request.id,
                AppApplicationOperation::Redeploy,
                auth.user_id(),
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanUpdate)]
    async fn rebuild<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Rebuild, auth.user_id())
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanUpdate)]
    async fn reload<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Reload, auth.user_id())
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanUpdate)]
    async fn start<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Start, auth.user_id())
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanUpdate)]
    async fn stop<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        self.service
            .stop_operation(request.id)
            .await
            .map_err(map_db_error)?;
        let app = self
            .service
            .get_by_id(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationOperation {
            application: app_to_proto(app).into(),
            operation: "stop".into(),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn cancel<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .cancel_operation(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanUpdate)]
    async fn rotate_webhook_token<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationWebhookToken> + Send + use<'a>> {
        let token = self
            .service
            .rotate_webhook_token(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationWebhookToken { token })
    }

    #[permission(Application, CanUpdate)]
    async fn move_application<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, MoveApplicationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let app = self
            .service
            .move_to_environment(request.id, request.target_environment_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(app_to_proto(app))
    }

    // ── Deployment management ──────────────────────────────────────────

    #[permission(Application, CanDelete)]
    async fn clear_deployment_history<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .clear_deployment_history(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanUpdate)]
    async fn cleanup_deployment_queue<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .cleanup_deployment_queue(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanUpdate)]
    async fn force_kill_deployment<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .force_kill_deployment(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Rollbacks ──────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_rollbacks<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListRollbacksRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListRollbacksResponse> + Send + use<'a>> {
        let items = self
            .service
            .list_rollbacks(request.application_id)
            .await
            .map_err(map_db_error)?;
        let rollbacks = items
            .into_iter()
            .map(|r| Rollback {
                id: r.id,
                application_id: request.application_id,
                status: "AVAILABLE".into(),
                commit: r.image.clone(),
                created_at: r.created_at,
            })
            .collect();
        Response::ok(ListRollbacksResponse { rollbacks })
    }

    #[permission(Application, CanUpdate)]
    async fn trigger_rollback<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TriggerRollbackRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RollbackTrigger> + Send + use<'a>> {
        let _result = self
            .service
            .trigger_rollback(request.application_id, request.rollback_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RollbackTrigger {
            rollback_id: request.rollback_id,
            status: "TRIGGERED".into(),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_rollback<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteRollbackRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete_rollback(request.application_id, request.rollback_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Dependencies ───────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationDependencies> + Send + use<'a>> {
        let counts = self
            .service
            .dependencies(request.id)
            .await
            .map_err(map_db_error)?;
        let mut items = Vec::new();
        if counts.active_deployments > 0 {
            items.push(format!("active_deployments:{}", counts.active_deployments));
        }
        if counts.deployments > 0 {
            items.push(format!("deployments:{}", counts.deployments));
        }
        if counts.domains > 0 {
            items.push(format!("domains:{}", counts.domains));
        }
        if counts.mounts > 0 {
            items.push(format!("mounts:{}", counts.mounts));
        }
        if counts.patches > 0 {
            items.push(format!("patches:{}", counts.patches));
        }
        if counts.schedules > 0 {
            items.push(format!("schedules:{}", counts.schedules));
        }
        if counts.backups > 0 {
            items.push(format!("backups:{}", counts.backups));
        }
        if counts.enabled_backups > 0 {
            items.push(format!("enabled_backups:{}", counts.enabled_backups));
        }
        if counts.preview_deployments > 0 {
            items.push(format!(
                "preview_deployments:{}",
                counts.preview_deployments
            ));
        }
        if counts.middleware > 0 {
            items.push(format!("middleware:{}", counts.middleware));
        }
        if counts.ports > 0 {
            items.push(format!("ports:{}", counts.ports));
        }
        if counts.redirects > 0 {
            items.push(format!("redirects:{}", counts.redirects));
        }
        if counts.security_rules > 0 {
            items.push(format!("security_rules:{}", counts.security_rules));
        }
        Response::ok(ApplicationDependencies { items })
    }

    // ── Application-level resource list methods ────────────────────────

    #[permission(Application, CanRead)]
    async fn list_application_ports<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationPorts> + Send + use<'a>> {
        let items = self
            .ports
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|p| {
                serde_json::to_string(
                    &crate::services::compute::application::port::ApplicationPortResponseDto::from(
                        p,
                    ),
                )
                .unwrap_or_default()
            })
            .collect();
        Response::ok(ApplicationPorts { items })
    }

    #[permission(Application, CanRead)]
    async fn list_application_mounts<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationMounts> + Send + use<'a>> {
        let items = self
            .mounts
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|m| serde_json::to_string(&crate::services::compute::application::mount::ApplicationMountResponseDto::from(m)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationMounts { items })
    }

    #[permission(Application, CanRead)]
    async fn list_application_networks<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationNetworks> + Send + use<'a>> {
        let (_detach, networks) = self
            .networks
            .get(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = networks
            .into_iter()
            .map(|n| {
                // Field-wise mapping: works for both the hand-written
                // DatabaseNetwork model and the flash-generated
                // DatabaseNetworks model (identical field layout).
                let dto =
                    crate::services::networking::database_network::DatabaseNetworkResponseDto {
                        id: n.id,
                        name: n.name,
                        docker_network_name: n.docker_network_name,
                        description: n.description,
                        external: n.external,
                        server_id: n.server_id,
                        created_at: n.created_at,
                        updated_at: n.updated_at,
                    };
                serde_json::to_string(&dto).unwrap_or_default()
            })
            .collect();
        Response::ok(ApplicationNetworks { items })
    }

    #[permission(Application, CanRead)]
    async fn list_application_patches<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationPatches> + Send + use<'a>> {
        let items = self
            .patches
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|p| serde_json::to_string(&crate::services::compute::application::patch::ApplicationPatchResponseDto::from(p)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationPatches { items })
    }

    #[permission(Application, CanRead)]
    async fn list_application_middlewares<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationMiddlewares> + Send + use<'a>> {
        let items = self
            .middles
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|m| serde_json::to_string(&m).unwrap_or_default())
            .collect();
        Response::ok(ApplicationMiddlewares { items })
    }

    #[permission(Application, CanRead)]
    async fn list_application_redirects<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationRedirects> + Send + use<'a>> {
        let items = self
            .redirects
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|r| serde_json::to_string(&crate::services::compute::application::redirect::ApplicationRedirectResponseDto::from(r)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationRedirects { items })
    }

    #[permission(Application, CanRead)]
    async fn list_application_security<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationSecurity> + Send + use<'a>> {
        let items = self
            .securities
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(|s| serde_json::to_string(&crate::services::compute::application::security::ApplicationSecurityResponseDto::from(s)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationSecurity { items })
    }

    // ── Ports ──────────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_ports<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationListPortsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationListPortsResponse> + Send + use<'a>>
    {
        let items = self
            .ports
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let ports = items
            .into_iter()
            .map(|p| {
                port_to_proto(
                    crate::services::compute::application::port::ApplicationPortResponseDto::from(
                        p,
                    ),
                )
            })
            .collect();
        Response::ok(ApplicationListPortsResponse { ports })
    }

    #[permission(Application, CanCreate)]
    async fn create_port<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationCreatePortRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationCreatePortResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::port::UpsertApplicationPortDto {
            published_port: request.published_port,
            target_port: request.target_port,
            protocol: protocol_from_proto(request.protocol),
            publish_mode: publish_mode_from_proto(request.publish_mode),
        };
        let item = self
            .ports
            .create(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationCreatePortResponse {
            port: ::buffa::MessageField::some(port_to_proto(
                crate::services::compute::application::port::ApplicationPortResponseDto::from(item),
            )),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn update_port<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdatePortRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationUpdatePortResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::port::UpsertApplicationPortDto {
            published_port: request.published_port,
            target_port: request.target_port,
            protocol: protocol_from_proto(request.protocol),
            publish_mode: publish_mode_from_proto(request.publish_mode),
        };
        let item = self
            .ports
            .update(request.application_id, request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdatePortResponse {
            port: ::buffa::MessageField::some(port_to_proto(
                crate::services::compute::application::port::ApplicationPortResponseDto::from(item),
            )),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_port<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationDeletePortRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.ports
            .delete(request.application_id, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Mounts ─────────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_mounts<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationListMountsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationListMountsResponse> + Send + use<'a>>
    {
        let items = self
            .mounts
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let mounts = items
            .into_iter()
            .map(normalize_mount_row)
            .map(|m| {
                mount_to_proto(
                    crate::services::compute::application::mount::ApplicationMountResponseDto::from(
                        m,
                    ),
                )
            })
            .collect();
        Response::ok(ApplicationListMountsResponse { mounts })
    }

    #[permission(Application, CanCreate)]
    async fn create_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationCreateMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationCreateMountResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::mount::UpsertApplicationMountDto {
            mount_type: mount_type_from_proto(request.mount_type),
            host_path: request.host_path.map(str::to_owned),
            volume_name: request.volume_name.map(str::to_owned),
            file_path: request.file_path.map(str::to_owned),
            content: request.content.map(str::to_owned),
            mount_path: request.mount_path.to_owned(),
        };
        let item = self
            .mounts
            .create(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationCreateMountResponse {
            mount: ::buffa::MessageField::some(mount_to_proto(
                crate::services::compute::application::mount::ApplicationMountResponseDto::from(
                    item,
                ),
            )),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn update_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdateMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationUpdateMountResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::mount::UpsertApplicationMountDto {
            mount_type: mount_type_from_proto(request.mount_type),
            host_path: request.host_path.map(str::to_owned),
            volume_name: request.volume_name.map(str::to_owned),
            file_path: request.file_path.map(str::to_owned),
            content: request.content.map(str::to_owned),
            mount_path: request.mount_path.to_owned(),
        };
        let item = self
            .mounts
            .update(request.application_id, request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdateMountResponse {
            mount: ::buffa::MessageField::some(mount_to_proto(
                crate::services::compute::application::mount::ApplicationMountResponseDto::from(
                    item,
                ),
            )),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationDeleteMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.mounts
            .delete(request.application_id, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Patches ────────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_patches<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationListPatchesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationListPatchesResponse> + Send + use<'a>>
    {
        let items = self
            .patches
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let patches = items
            .into_iter()
            .map(|p| {
                patch_to_proto(
                    crate::services::compute::application::patch::ApplicationPatchResponseDto::from(
                        p,
                    ),
                )
            })
            .collect();
        Response::ok(ApplicationListPatchesResponse { patches })
    }

    #[permission(Application, CanCreate)]
    async fn create_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationCreatePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationCreatePatchResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::patch::UpsertApplicationPatchDto {
            patch_type: patch_type_from_proto(request.patch_type),
            file_path: request.file_path.to_owned(),
            enabled: request.enabled,
            content: request.content.to_owned(),
        };
        let item = self
            .patches
            .create(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationCreatePatchResponse {
            patch: ::buffa::MessageField::some(patch_to_proto(
                crate::services::compute::application::patch::ApplicationPatchResponseDto::from(
                    item,
                ),
            )),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn update_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdatePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationUpdatePatchResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::patch::UpsertApplicationPatchDto {
            patch_type: patch_type_from_proto(request.patch_type),
            file_path: request.file_path.to_owned(),
            enabled: request.enabled,
            content: request.content.to_owned(),
        };
        let item = self
            .patches
            .update(request.application_id, request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdatePatchResponse {
            patch: ::buffa::MessageField::some(patch_to_proto(
                crate::services::compute::application::patch::ApplicationPatchResponseDto::from(
                    item,
                ),
            )),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationDeletePatchRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.patches
            .delete(request.application_id, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Middlewares ────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_middlewares<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationListMiddlewaresRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ApplicationListMiddlewaresResponse> + Send + use<'a>,
    > {
        let items = self
            .middles
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let middlewares = items.into_iter().map(middleware_to_proto).collect();
        Response::ok(ApplicationListMiddlewaresResponse { middlewares })
    }

    #[permission(Application, CanCreate)]
    async fn create_middleware<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationCreateMiddlewareRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ApplicationCreateMiddlewareResponse> + Send + use<'a>,
    > {
        let dto = build_middleware_dto(
            request.name.to_owned(),
            middleware_type_from_proto(request.middleware_type),
            request.enabled,
            request
                .headers
                .iter()
                .map(|h| (h.name.to_owned(), h.value.to_owned()))
                .collect(),
            request.average,
            request.burst,
            request
                .source_ranges
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        let item = self
            .middles
            .create(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationCreateMiddlewareResponse {
            middleware: ::buffa::MessageField::some(middleware_to_proto(item)),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn update_middleware<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdateMiddlewareRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ApplicationUpdateMiddlewareResponse> + Send + use<'a>,
    > {
        let dto = build_middleware_dto(
            request.name.to_owned(),
            middleware_type_from_proto(request.middleware_type),
            request.enabled,
            request
                .headers
                .iter()
                .map(|h| (h.name.to_owned(), h.value.to_owned()))
                .collect(),
            request.average,
            request.burst,
            request
                .source_ranges
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        let item = self
            .middles
            .update(request.application_id, request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdateMiddlewareResponse {
            middleware: ::buffa::MessageField::some(middleware_to_proto(item)),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_middleware<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationDeleteMiddlewareRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.middles
            .delete(request.application_id, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Redirects ──────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_redirects<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationListRedirectsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationListRedirectsResponse> + Send + use<'a>>
    {
        let items = self
            .redirects
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let redirects = items
            .into_iter()
            .map(|r| {
                redirect_to_proto(
                    crate::services::compute::application::redirect::ApplicationRedirectResponseDto::from(r),
                )
            })
            .collect();
        Response::ok(ApplicationListRedirectsResponse { redirects })
    }

    #[permission(Application, CanCreate)]
    async fn create_redirect<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationCreateRedirectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationCreateRedirectResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::redirect::UpsertApplicationRedirectDto {
            regex: request.regex.to_owned(),
            replacement: request.replacement.to_owned(),
            permanent: i64::from(request.permanent),
            unique_config_key: request.unique_config_key,
        };
        let item = self
            .redirects
            .create(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationCreateRedirectResponse {
            redirect: ::buffa::MessageField::some(redirect_to_proto(
                crate::services::compute::application::redirect::ApplicationRedirectResponseDto::from(item),
            )),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn update_redirect<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdateRedirectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationUpdateRedirectResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::redirect::UpsertApplicationRedirectDto {
            regex: request.regex.to_owned(),
            replacement: request.replacement.to_owned(),
            permanent: i64::from(request.permanent),
            unique_config_key: request.unique_config_key,
        };
        let item = self
            .redirects
            .update(request.application_id, request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdateRedirectResponse {
            redirect: ::buffa::MessageField::some(redirect_to_proto(
                crate::services::compute::application::redirect::ApplicationRedirectResponseDto::from(item),
            )),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_redirect<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationDeleteRedirectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.redirects
            .delete(request.application_id, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Security ───────────────────────────────────────────────────────

    #[permission(Application, CanRead)]
    async fn get_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationGetSecurityRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationGetSecurityResponse> + Send + use<'a>>
    {
        let items = self
            .securities
            .list(request.application_id)
            .await
            .map_err(map_db_error)?;
        let security = items
            .into_iter()
            .map(|s| {
                security_to_proto(
                    crate::services::compute::application::security::ApplicationSecurityResponseDto::from(s),
                )
            })
            .collect();
        Response::ok(ApplicationGetSecurityResponse { security })
    }

    #[permission(Application, CanCreate)]
    async fn create_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationCreateSecurityRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationCreateSecurityResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::security::UpsertApplicationSecurityDto {
            username: request.username.to_owned(),
            password: request.password.to_owned(),
        };
        let item = self
            .securities
            .create(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationCreateSecurityResponse {
            security: ::buffa::MessageField::some(security_to_proto(
                crate::services::compute::application::security::ApplicationSecurityResponseDto::from(item),
            )),
        })
    }

    #[permission(Application, CanUpdate)]
    async fn update_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdateSecurityRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationUpdateSecurityResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::security::UpsertApplicationSecurityDto {
            username: request.username.to_owned(),
            password: request.password.to_owned(),
        };
        let item = self
            .securities
            .update(request.application_id, request.id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdateSecurityResponse {
            security: ::buffa::MessageField::some(security_to_proto(
                crate::services::compute::application::security::ApplicationSecurityResponseDto::from(item),
            )),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationDeleteSecurityRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.securities
            .delete(request.application_id, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Networks ───────────────────────────────────────────────────────

    #[permission(Application, CanUpdate)]
    async fn update_networks<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationUpdateNetworksRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationUpdateNetworksResponse> + Send + use<'a>>
    {
        let dto = crate::services::compute::application::network::UpdateApplicationNetworksDto {
            network_ids: request.network_ids.iter().copied().collect(),
            detach_rustploy_network: request.detach_rustploy_network,
        };
        let (_detach, _networks) = self
            .networks
            .update(request.application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(ApplicationUpdateNetworksResponse {
            success: true,
            operation: "update_networks".into(),
        })
    }
}
