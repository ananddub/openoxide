use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Application as ApplicationProto, ApplicationDependencies,
    ApplicationExport, ApplicationMiddlewares, ApplicationMounts, ApplicationNetworks,
    ApplicationOperation, ApplicationOperationRequest, ApplicationPatches, ApplicationPorts,
    ApplicationRedirects, ApplicationSecurity, ApplicationService, ApplicationServiceExt, ApplicationWebhookToken, CreateApplicationRequest,
    DeleteApplicationRequest, DeleteRollbackRequest, Empty, ExportApplicationRequest,
    GetApplicationRequest, ImportApplicationRequest, ListApplicationsRequest,
    ListApplicationsResponse, ListRollbacksRequest, ListRollbacksResponse, MoveApplicationRequest,
    OperationResult, ResourceListRequest, ResourceListResponse, ResourceRequest, ResourceResponse,
    Rollback, RollbackTrigger, SetBitbucketSourceRequest, SetCustomGitSourceRequest,
    SetDockerSourceRequest, SetDropSourceRequest, SetGiteaSourceRequest,
    SetGithubSourceRequest, SetGitlabSourceRequest, TriggerRollbackRequest,
    UpdateApplicationRequest, UpdateBuildConfigRequest, UpdatePreviewConfigRequest,
    UpdateResourceConfigRequest,
};
use crate::services::application::import_export::ImportApplicationDto;
use crate::services::application::ApplicationOperation as AppApplicationOperation;

use super::helper::*;
use super::init::ApplicationController;

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
        Response::ok(ListApplicationsResponse {
            applications,
            ..Default::default()
        })
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
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Import / Export ────────────────────────────────────────────────

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
        let dto: ImportApplicationDto = match serde_json::from_str(&request.archive) {
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
            ConnectError::new(ErrorCode::Internal, "could not serialize application export")
        })?;
        Response::ok(ApplicationExport {
            format: "openoxide.application+json".into(),
            schema_version: i64::from(bundle.schema_version),
            archive,
            ..Default::default()
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
    // NOTE: upload_source is a client-streaming RPC. Each streamed chunk is
    // an UploadSourceRequest carrying a base64 file segment. Multipart
    // upload aggregation is not yet wired; reject until a chunked upload
    // protocol is defined.

    #[permission(Application, CanCreate)]
    async fn upload_source<'a>(
        &'a self,
        _ctx: RequestContext,
        _requests: connectrpc::InboundStream<
            crate::api::connect::generated::openoxide::api::v1::UploadSourceRequest,
        >,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationProto> + Send + use<'a>> {
        let error: ServiceResult<ApplicationProto> = Err(ConnectError::new(
            ErrorCode::Unimplemented,
            "upload_source requires client-streaming; use the REST endpoint instead",
        ));
        error
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

    #[permission(Application, CanDeploy)]
    async fn deploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Deploy)
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanDeploy)]
    async fn redeploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Redeploy)
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanDeploy)]
    async fn rebuild<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Rebuild)
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanDeploy)]
    async fn reload<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Reload)
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanDeploy)]
    async fn start<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ApplicationOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ApplicationOperation> + Send + use<'a>> {
        let result = self
            .service
            .run_operation(request.id, AppApplicationOperation::Start)
            .await
            .map_err(map_db_error)?;
        Response::ok(operation_result(result))
    }

    #[permission(Application, CanDeploy)]
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
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
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
        Response::ok(ApplicationWebhookToken {
            token,
            ..Default::default()
        })
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

    #[permission(Application, CanDeploy)]
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

    #[permission(Application, CanDeploy)]
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
                ..Default::default()
            })
            .collect();
        Response::ok(ListRollbacksResponse {
            rollbacks,
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
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
            ..Default::default()
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
            items.push(format!("preview_deployments:{}", counts.preview_deployments));
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
        Response::ok(ApplicationDependencies {
            items,
            ..Default::default()
        })
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
            .map(|p| serde_json::to_string(&crate::services::application::port::ApplicationPortResponseDto::from(p)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationPorts {
            items,
            ..Default::default()
        })
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
            .map(|m| serde_json::to_string(&crate::services::application::mount::ApplicationMountResponseDto::from(m)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationMounts {
            items,
            ..Default::default()
        })
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
                let dto = crate::services::database_network::DatabaseNetworkResponseDto {
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
        Response::ok(ApplicationNetworks {
            items,
            ..Default::default()
        })
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
            .map(|p| serde_json::to_string(&crate::services::application::patch::ApplicationPatchResponseDto::from(p)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationPatches {
            items,
            ..Default::default()
        })
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
        Response::ok(ApplicationMiddlewares {
            items,
            ..Default::default()
        })
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
            .map(|r| serde_json::to_string(&crate::services::application::redirect::ApplicationRedirectResponseDto::from(r)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationRedirects {
            items,
            ..Default::default()
        })
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
            .map(|s| serde_json::to_string(&crate::services::application::security::ApplicationSecurityResponseDto::from(s)).unwrap_or_default())
            .collect();
        Response::ok(ApplicationSecurity {
            items,
            ..Default::default()
        })
    }

    // ── Ports (ResourceRequest envelope) ───────────────────────────────
    // ResourceRequest.id = application_id for list/create,
    // ResourceRequest.id = resource_id, .organization_id = application_id for update/delete

    #[permission(Application, CanRead)]
    async fn list_ports<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = request.id;
        let items = self.ports.list(application_id).await.map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(port_to_resource)
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_port<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::port::UpsertApplicationPortDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .ports
            .create(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(port_to_resource(item))
    }

    #[permission(Application, CanUpdate)]
    async fn update_port<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        let dto: crate::services::application::port::UpsertApplicationPortDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .ports
            .update(application_id, resource_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(port_to_resource(item))
    }

    #[permission(Application, CanDelete)]
    async fn delete_port<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        self.ports
            .delete(application_id, resource_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Mounts (ResourceRequest envelope) ──────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_mounts<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = request.id;
        let items = self.mounts.list(application_id).await.map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(mount_to_resource)
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::mount::UpsertApplicationMountDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .mounts
            .create(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(mount_to_resource(item))
    }

    #[permission(Application, CanUpdate)]
    async fn update_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        let dto: crate::services::application::mount::UpsertApplicationMountDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .mounts
            .update(application_id, resource_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(mount_to_resource(item))
    }

    #[permission(Application, CanDelete)]
    async fn delete_mount<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        self.mounts
            .delete(application_id, resource_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Patches (ResourceRequest envelope) ─────────────────────────────

    #[permission(Application, CanRead)]
    async fn list_patches<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = request.id;
        let items = self.patches.list(application_id).await.map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(patch_to_resource)
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::patch::UpsertApplicationPatchDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .patches
            .create(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(patch_to_resource(item))
    }

    #[permission(Application, CanUpdate)]
    async fn update_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        let dto: crate::services::application::patch::UpsertApplicationPatchDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .patches
            .update(application_id, resource_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(patch_to_resource(item))
    }

    #[permission(Application, CanDelete)]
    async fn delete_patch<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        self.patches
            .delete(application_id, resource_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Middlewares (ResourceRequest envelope) ─────────────────────────

    #[permission(Application, CanRead)]
    async fn list_middlewares<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = request.id;
        let items = self.middles.list(application_id).await.map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(middleware_to_resource)
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_middleware<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::middleware::UpsertApplicationMiddlewareDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .middles
            .create(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(middleware_to_resource(item))
    }

    #[permission(Application, CanUpdate)]
    async fn update_middleware<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        let dto: crate::services::application::middleware::UpsertApplicationMiddlewareDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .middles
            .update(application_id, resource_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(middleware_to_resource(item))
    }

    #[permission(Application, CanDelete)]
    async fn delete_middleware<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        self.middles
            .delete(application_id, resource_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Redirects (ResourceRequest envelope) ───────────────────────────

    #[permission(Application, CanRead)]
    async fn list_redirects<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = request.id;
        let items = self
            .redirects
            .list(application_id)
            .await
            .map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(redirect_to_resource)
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_redirect<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::redirect::UpsertApplicationRedirectDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .redirects
            .create(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(redirect_to_resource(item))
    }

    #[permission(Application, CanUpdate)]
    async fn update_redirect<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        let dto: crate::services::application::redirect::UpsertApplicationRedirectDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .redirects
            .update(application_id, resource_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(redirect_to_resource(item))
    }

    #[permission(Application, CanDelete)]
    async fn delete_redirect<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        self.redirects
            .delete(application_id, resource_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Security (ResourceRequest envelope) ────────────────────────────

    #[permission(Application, CanRead)]
    async fn get_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let items = self
            .securities
            .list(application_id)
            .await
            .map_err(map_db_error)?;
        let payload = serde_json::to_string(
            &items
                .into_iter()
                .map(crate::services::application::security::ApplicationSecurityResponseDto::from)
                .collect::<Vec<_>>(),
        )
        .unwrap_or_default();
        Response::ok(ResourceResponse {
            id: application_id,
            kind: "application_security".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::security::UpsertApplicationSecurityDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .securities
            .create(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(security_to_resource(item))
    }

    #[permission(Application, CanUpdate)]
    async fn update_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        let dto: crate::services::application::security::UpsertApplicationSecurityDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self
            .securities
            .update(application_id, resource_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(security_to_resource(item))
    }

    #[permission(Application, CanDelete)]
    async fn delete_security<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let resource_id = request.id;
        let application_id = request.organization_id.unwrap_or(0);
        self.securities
            .delete(application_id, resource_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    // ── Networks (ResourceRequest envelope) ────────────────────────────

    #[permission(Application, CanUpdate)]
    async fn update_networks<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let application_id = request.id;
        let dto: crate::services::application::network::UpdateApplicationNetworksDto =
            serde_json::from_str(request.payload)
                .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (_detach, _networks) = self
            .networks
            .update(application_id, dto)
            .await
            .map_err(map_db_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "update_networks".into(),
            ..Default::default()
        })
    }
}
