use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, NotificationOperationsService, NotificationOperationsServiceExt,
    OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::{
    NotificationDeliveryAttempts, NotificationResourceBindings,
    Notifications as Notification,
};
use crate::flash_gen::notification_delivery::NotificationDeliveryCreateBindingParams;
use crate::flash_gen::notifications::{
    CreateNotificationWithTriggersParams, UpdateNotificationFullParams,
};
use crate::services::integrations::notification::{NotificationLevel, NotificationMessage};
use super::helper::*;
use super::init::NotificationController;

#[auto_grpc::service(connect = true, auth = true)]
impl NotificationOperationsService for NotificationController {
    #[permission(Notification, CanRead)]
    async fn list_delivery_history<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let rows = self.queries.notification_delivery_list(organization_id, 200).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let items = rows.into_iter().map(NotificationDeliveryAttemptDto::from).map(delivery_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Notification, CanRead)]
    async fn stream_list_delivery_history(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let stream = {
            let sql = format!(
                "SELECT * FROM notification_delivery_attempts WHERE organization_id = {organization_id} ORDER BY created_at DESC LIMIT 200"
            );
            realtime()
                .watch_query::<NotificationDeliveryAttempts>(sql)
                .await
                .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(NotificationDeliveryAttemptDto::from)
                    .map(delivery_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Notification, CanCreate)]
    async fn create_binding<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let dto: CreateNotificationBindingDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.queries
            .get_notification_by_i_d_for_organization(dto.notification_id, organization_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ConnectError::not_found("notification not found"),
                e => ConnectError::new(ErrorCode::Internal, e.to_string()),
            })?;
        let id = self.queries
            .notification_delivery_create_binding(&NotificationDeliveryCreateBindingParams {
                notification_id: dto.notification_id,
                organization_id,
                resource_type: dto.resource_type.as_str().to_string(),
                resource_id: dto.resource_id,
            })
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let payload = serde_json::to_string(&serde_json::json!({ "id": id })).unwrap_or_default();
        Response::ok(ResourceResponse { id, kind: "notification_binding".into(), payload, ..Default::default() })
    }

    #[permission(Notification, CanRead)]
    async fn list_bindings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let rows = self.queries.notification_delivery_bindings(organization_id).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let items = rows.into_iter().map(NotificationResourceBindingDto::from).map(binding_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Notification, CanRead)]
    async fn stream_list_bindings(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let stream = {
            let sql = format!(
                "SELECT * FROM notification_resource_bindings WHERE organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<NotificationResourceBindings>(sql)
                .await
                .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(NotificationResourceBindingDto::from)
                    .map(binding_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Notification, CanDelete)]
    async fn delete_binding<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let deleted = self.queries.notification_delivery_delete_binding(request.id, organization_id).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        if deleted > 0 {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("binding not found"))
        }
    }

    #[permission(Notification, CanRead)]
    async fn list_notifications<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        let items = self.queries.list_notifications_by_organization(organization_id).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let items = items.into_iter().map(NotificationResponseDto::from).map(notif_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Notification, CanRead)]
    async fn stream_list_notifications(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        let stream = {
            let sql = format!("SELECT * FROM notifications WHERE organization_id = {organization_id}");
            realtime()
                .watch_query::<Notification>(sql)
                .await
                .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(NotificationResponseDto::from)
                    .map(notif_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Notification, CanRead)]
    async fn get_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        let item = self.queries.get_notification_by_i_d_for_organization(request.id, organization_id).await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ConnectError::not_found(&format!("Notification {} not found", request.id)),
                e => ConnectError::new(ErrorCode::Internal, e.to_string()),
            })?;
        Response::ok(notif_to_resource(NotificationResponseDto::from(item)))
    }

    #[permission(Notification, CanRead)]
    async fn stream_notification(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        let id = request.id;
        let stream = {
            let sql = format!(
                "SELECT * FROM notifications WHERE id = {id} AND organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<Notification>(sql)
                .await
                .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(notif_to_resource(NotificationResponseDto::from(item))),
            None => Err(ConnectError::not_found(&format!("Notification {id} not found"))),
        }))
    }

    #[permission(Notification, CanCreate)]
    async fn create_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateNotificationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let now = chrono::Utc::now().timestamp();
        let provider = provider_of(&dto);
        let mut notif = Notification {
            id: 0,
            name: dto.settings.name.clone(),
            notification_type: provider.as_str().to_string(),
            config_json: String::new(),
            on_app_deploy: dto.settings.on_app_deploy as i64,
            on_app_build_error: dto.settings.on_app_build_error as i64,
            on_database_backup: dto.settings.on_database_backup as i64,
            on_volume_backup: dto.settings.on_volume_backup as i64,
            on_panel_restart: dto.settings.on_panel_restart as i64,
            on_docker_cleanup: dto.settings.on_docker_cleanup as i64,
            on_server_threshold: dto.settings.on_server_threshold as i64,
            on_server_migration: 0,
            on_security_alert: 0,
            on_panel_backup: dto.settings.on_panel_backup as i64,
            on_schedule_success: dto.settings.on_schedule_success as i64,
            on_schedule_failure: dto.settings.on_schedule_failure as i64,
            organization_id: dto.settings.organization_id,
            created_at: now,
            updated_at: now,
        };
        self.builder.bind_provider(&mut notif, &dto).await
            .map_err(|e| e)?;
        let created = self.queries.create_notification_with_triggers(&CreateNotificationWithTriggersParams {
            name: notif.name.clone(),
            notification_type: notif.notification_type.clone(),
            config_json: notif.config_json.clone(),
            on_app_deploy: notif.on_app_deploy,
            on_app_build_error: notif.on_app_build_error,
            on_database_backup: notif.on_database_backup,
            on_volume_backup: notif.on_volume_backup,
            on_panel_backup: notif.on_panel_backup,
            on_panel_restart: notif.on_panel_restart,
            on_docker_cleanup: notif.on_docker_cleanup,
            on_server_threshold: notif.on_server_threshold,
            on_server_migration: notif.on_server_migration,
            on_security_alert: notif.on_security_alert,
            on_schedule_success: notif.on_schedule_success,
            on_schedule_failure: notif.on_schedule_failure,
            organization_id: notif.organization_id,
            created_at: notif.created_at,
            updated_at: notif.updated_at,
        }).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        notif.id = created.id;
        Response::ok(notif_to_resource(NotificationResponseDto::from(notif)))
    }

    #[permission(Notification, CanCreate)]
    async fn update_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        let dto: PatchNotificationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let mut notif = self.queries
            .get_notification_by_i_d_for_organization(request.id, organization_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ConnectError::not_found(&format!("Notification {} not found", request.id)),
                e => ConnectError::new(ErrorCode::Internal, e.to_string()),
            })?;
        if let Some(name) = dto.name { notif.name = name; }
        if let Some(val) = dto.on_app_deploy { notif.on_app_deploy = val as i64; }
        if let Some(val) = dto.on_app_build_error { notif.on_app_build_error = val as i64; }
        if let Some(val) = dto.on_database_backup { notif.on_database_backup = val as i64; }
        if let Some(val) = dto.on_volume_backup { notif.on_volume_backup = val as i64; }
        if let Some(val) = dto.on_panel_restart { notif.on_panel_restart = val as i64; }
        if let Some(val) = dto.on_docker_cleanup { notif.on_docker_cleanup = val as i64; }
        if let Some(val) = dto.on_server_threshold { notif.on_server_threshold = val as i64; }
        if let Some(val) = dto.on_panel_backup { notif.on_panel_backup = val as i64; }
        if let Some(val) = dto.on_schedule_success { notif.on_schedule_success = val as i64; }
        if let Some(val) = dto.on_schedule_failure { notif.on_schedule_failure = val as i64; }
        notif.updated_at = chrono::Utc::now().timestamp();
        self.queries.update_notification_full(&UpdateNotificationFullParams {
            name: notif.name.clone(),
            notification_type: notif.notification_type.clone(),
            config_json: notif.config_json.clone(),
            on_app_deploy: notif.on_app_deploy,
            on_app_build_error: notif.on_app_build_error,
            on_database_backup: notif.on_database_backup,
            on_volume_backup: notif.on_volume_backup,
            on_panel_backup: notif.on_panel_backup,
            on_panel_restart: notif.on_panel_restart,
            on_docker_cleanup: notif.on_docker_cleanup,
            on_server_threshold: notif.on_server_threshold,
            on_server_migration: notif.on_server_migration,
            on_security_alert: notif.on_security_alert,
            on_schedule_success: notif.on_schedule_success,
            on_schedule_failure: notif.on_schedule_failure,
            organization_id: notif.organization_id,
            created_at: notif.created_at,
            updated_at: notif.updated_at,
            id: request.id,
        }).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(notif_to_resource(NotificationResponseDto::from(notif)))
    }

    #[permission(Notification, CanCreate)]
    async fn replace_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        let dto: CreateNotificationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        if dto.settings.organization_id != organization_id {
            return Err(ConnectError::invalid_argument("organization_id must match authenticated scope"));
        }
        let existing = self.queries
            .get_notification_by_i_d_for_organization(request.id, organization_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ConnectError::not_found("notification not found"),
                e => ConnectError::new(ErrorCode::Internal, e.to_string()),
            })?;
        let mut replacement = Notification {
            id: request.id,
            name: dto.settings.name.clone(),
            notification_type: String::new(),
            config_json: String::new(),
            on_app_deploy: dto.settings.on_app_deploy as i64,
            on_app_build_error: dto.settings.on_app_build_error as i64,
            on_database_backup: dto.settings.on_database_backup as i64,
            on_volume_backup: dto.settings.on_volume_backup as i64,
            on_panel_restart: dto.settings.on_panel_restart as i64,
            on_docker_cleanup: dto.settings.on_docker_cleanup as i64,
            on_server_threshold: dto.settings.on_server_threshold as i64,
            on_server_migration: 0,
            on_security_alert: 0,
            on_panel_backup: dto.settings.on_panel_backup as i64,
            on_schedule_success: dto.settings.on_schedule_success as i64,
            on_schedule_failure: dto.settings.on_schedule_failure as i64,
            organization_id,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now().timestamp(),
        };
        replacement.notification_type = provider_of(&dto).as_str().to_owned();
        self.builder.bind_provider(&mut replacement, &dto).await
            .map_err(|e| e)?;
        self.queries.update_notification_full(&UpdateNotificationFullParams {
            name: replacement.name.clone(),
            notification_type: replacement.notification_type.clone(),
            config_json: replacement.config_json.clone(),
            on_app_deploy: replacement.on_app_deploy,
            on_app_build_error: replacement.on_app_build_error,
            on_database_backup: replacement.on_database_backup,
            on_volume_backup: replacement.on_volume_backup,
            on_panel_backup: replacement.on_panel_backup,
            on_panel_restart: replacement.on_panel_restart,
            on_docker_cleanup: replacement.on_docker_cleanup,
            on_server_threshold: replacement.on_server_threshold,
            on_server_migration: replacement.on_server_migration,
            on_security_alert: replacement.on_security_alert,
            on_schedule_success: replacement.on_schedule_success,
            on_schedule_failure: replacement.on_schedule_failure,
            organization_id: replacement.organization_id,
            created_at: replacement.created_at,
            updated_at: replacement.updated_at,
            id: request.id,
        }).await
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(notif_to_resource(NotificationResponseDto::from(replacement)))
    }

    #[permission(Notification, CanDelete)]
    async fn delete_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        match self.queries.delete_notification_for_organization(request.id, organization_id).await {
            Ok(_) => Response::ok(Empty::default()),
            Err(sqlx::Error::RowNotFound) => Err(ConnectError::not_found("notification not found")),
            Err(e) => Err(ConnectError::new(ErrorCode::Internal, e.to_string())),
        }
    }

    #[permission(Notification, CanCreate)]
    async fn test_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        self.queries.get_notification_by_i_d_for_organization(request.id, organization_id).await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ConnectError::not_found("notification not found"),
                e => ConnectError::new(ErrorCode::Internal, e.to_string()),
            })?;
        let msg = NotificationMessage::new(
            "OpenOxide Test Notification",
            "Test dispatch from OpenOxide Notification Manager",
        )
        .level(NotificationLevel::Info)
        .field("Status", "Operational")
        .field("Test", "Successful");
        self.service.send_test(request.id, &msg).await
            .map_err(|e| ConnectError::invalid_argument(e))?;
        Response::ok(OperationResult {
            success: true,
            operation: "test_notification".into(),
            ..Default::default()
        })
    }
}
