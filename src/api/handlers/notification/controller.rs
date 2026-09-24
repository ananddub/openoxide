use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateNotificationBindingRequest, CreateNotificationBindingResponse,
    CreateNotificationRequest, CreateNotificationResponse, DeleteNotificationBindingRequest,
    DeleteNotificationRequest, Empty, GetNotificationRequest, GetNotificationResponse,
    ListNotificationBindingsRequest, ListNotificationBindingsResponse,
    ListNotificationDeliveryHistoryRequest, ListNotificationDeliveryHistoryResponse,
    ListNotificationsRequest, ListNotificationsResponse, NotificationService,
    NotificationServiceExt, ReplaceNotificationRequest, ReplaceNotificationResponse,
    TestNotificationRequest, TestNotificationResponse, UpdateNotificationRequest,
    UpdateNotificationResponse,
};
use crate::flash_gen::models::Notifications as Notification;
use crate::flash_gen::notification_delivery::NotificationDeliveryCreateBindingParams;
use crate::flash_gen::notifications::{
    CreateNotificationWithTriggersParams, UpdateNotificationFullParams,
};
use crate::services::integrations::notification::{NotificationLevel, NotificationMessage};

use super::builder::{create_provider_parts, replace_provider_parts};
use super::helper::*;
use super::init::NotificationController;

#[auto_grpc::service(connect = true, auth = true)]
impl NotificationService for NotificationController {
    #[permission(Notification, CanRead)]
    async fn list_notifications<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListNotificationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListNotificationsResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let items = self
            .queries
            .list_notifications_by_organization(organization_id)
            .await
            .map_err(internal)?;
        Response::ok(ListNotificationsResponse {
            notifications: items.into_iter().map(notif_to_proto).collect(),
            ..Default::default()
        })
    }

    #[permission(Notification, CanRead)]
    async fn get_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetNotificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetNotificationResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let item = self
            .queries
            .get_notification_by_i_d_for_organization(request.id, organization_id)
            .await
            .map_err(map_sqlx_not_found)?;
        Response::ok(GetNotificationResponse {
            notification: ::buffa::MessageField::some(notif_to_proto(item)),
            ..Default::default()
        })
    }

    #[permission(Notification, CanCreate)]
    async fn create_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateNotificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateNotificationResponse> + Send + use<'a>> {
        verify_scope(auth.organization_id, request.organization_id)?;
        // Exactly one provider config must be set; it also fixes the stored
        // notification_type.
        let (provider, config) = create_provider_parts(
            request
                .provider
                .as_ref()
                .ok_or_else(|| ConnectError::invalid_argument("a provider config must be set"))?,
        );
        let now = chrono::Utc::now().timestamp();
        let mut notif = Notification {
            id: 0,
            name: request.name.to_owned(),
            notification_type: String::new(),
            config_json: String::new(),
            on_app_deploy: request.on_app_deploy as i64,
            on_app_build_error: request.on_app_build_error as i64,
            on_database_backup: request.on_database_backup as i64,
            on_volume_backup: request.on_volume_backup as i64,
            on_panel_restart: request.on_panel_restart as i64,
            on_docker_cleanup: request.on_docker_cleanup as i64,
            on_server_threshold: request.on_server_threshold as i64,
            on_server_migration: 0,
            on_security_alert: 0,
            on_panel_backup: request.on_panel_backup as i64,
            on_schedule_success: request.on_schedule_success as i64,
            on_schedule_failure: request.on_schedule_failure as i64,
            organization_id: request.organization_id,
            created_at: now,
            updated_at: now,
        };
        self.builder
            .bind_provider(&mut notif, provider, config)
            .await
            .map_err(internal)?;
        let created = self
            .queries
            .create_notification_with_triggers(&CreateNotificationWithTriggersParams {
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
            })
            .await
            .map_err(internal)?;
        notif.id = created.id;
        Response::ok(CreateNotificationResponse {
            notification: ::buffa::MessageField::some(notif_to_proto(notif)),
            ..Default::default()
        })
    }

    #[permission(Notification, CanCreate)]
    async fn update_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateNotificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateNotificationResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let mut notif = self
            .queries
            .get_notification_by_i_d_for_organization(request.id, organization_id)
            .await
            .map_err(map_sqlx_not_found)?;
        // PATCH semantics: absent fields keep their stored values. The
        // provider kind itself is immutable here (ReplaceNotification
        // changes it).
        if let Some(name) = request.name {
            notif.name = name.to_owned();
        }
        if let Some(val) = request.on_app_deploy {
            notif.on_app_deploy = val as i64;
        }
        if let Some(val) = request.on_app_build_error {
            notif.on_app_build_error = val as i64;
        }
        if let Some(val) = request.on_database_backup {
            notif.on_database_backup = val as i64;
        }
        if let Some(val) = request.on_volume_backup {
            notif.on_volume_backup = val as i64;
        }
        if let Some(val) = request.on_panel_restart {
            notif.on_panel_restart = val as i64;
        }
        if let Some(val) = request.on_docker_cleanup {
            notif.on_docker_cleanup = val as i64;
        }
        if let Some(val) = request.on_server_threshold {
            notif.on_server_threshold = val as i64;
        }
        if let Some(val) = request.on_panel_backup {
            notif.on_panel_backup = val as i64;
        }
        if let Some(val) = request.on_schedule_success {
            notif.on_schedule_success = val as i64;
        }
        if let Some(val) = request.on_schedule_failure {
            notif.on_schedule_failure = val as i64;
        }
        // Provider config fields update the stored channel config in place.
        // `endpoint` is stored under the config key `url`.
        let mut config_fields: Vec<(&'static str, serde_json::Value)> = Vec::new();
        macro_rules! patch {
            ($field:expr, $key:literal) => {
                if let Some(value) = $field {
                    config_fields.push(($key, serde_json::json!(value)));
                }
            };
        }
        patch!(request.webhook_url, "webhook_url");
        patch!(request.channel, "channel");
        patch!(request.username, "username");
        patch!(request.decoration, "decoration");
        patch!(request.bot_token, "bot_token");
        patch!(request.chat_id, "chat_id");
        patch!(request.message_thread_id, "message_thread_id");
        patch!(request.smtp_server, "smtp_server");
        patch!(request.smtp_port, "smtp_port");
        patch!(request.password, "password");
        patch!(request.from_address, "from_address");
        patch!(request.api_key, "api_key");
        patch!(request.server_url, "server_url");
        patch!(request.app_token, "app_token");
        patch!(request.priority, "priority");
        patch!(request.topic, "topic");
        patch!(request.access_token, "access_token");
        patch!(request.endpoint, "url");
        patch!(request.headers, "headers");
        patch!(request.user_key, "user_key");
        patch!(request.api_token, "api_token");
        patch!(request.retry, "retry");
        patch!(request.expire, "expire");
        let to_addresses: Vec<String> = request
            .to_addresses
            .iter()
            .map(|address| (*address).to_owned())
            .collect();
        apply_provider_patch(&mut notif.config_json, config_fields, to_addresses)?;
        notif.updated_at = chrono::Utc::now().timestamp();
        self.queries
            .update_notification_full(&UpdateNotificationFullParams {
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
            })
            .await
            .map_err(internal)?;
        Response::ok(UpdateNotificationResponse {
            notification: ::buffa::MessageField::some(notif_to_proto(notif)),
            ..Default::default()
        })
    }

    #[permission(Notification, CanCreate)]
    async fn replace_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ReplaceNotificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ReplaceNotificationResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let (provider, config) = replace_provider_parts(
            request
                .provider
                .as_ref()
                .ok_or_else(|| ConnectError::invalid_argument("a provider config must be set"))?,
        );
        let existing = self
            .queries
            .get_notification_by_i_d_for_organization(request.id, organization_id)
            .await
            .map_err(map_sqlx_not_found)?;
        let mut replacement = Notification {
            id: request.id,
            name: request.name.to_owned(),
            notification_type: String::new(),
            config_json: String::new(),
            on_app_deploy: request.on_app_deploy as i64,
            on_app_build_error: request.on_app_build_error as i64,
            on_database_backup: request.on_database_backup as i64,
            on_volume_backup: request.on_volume_backup as i64,
            on_panel_restart: request.on_panel_restart as i64,
            on_docker_cleanup: request.on_docker_cleanup as i64,
            on_server_threshold: request.on_server_threshold as i64,
            on_server_migration: 0,
            on_security_alert: 0,
            on_panel_backup: request.on_panel_backup as i64,
            on_schedule_success: request.on_schedule_success as i64,
            on_schedule_failure: request.on_schedule_failure as i64,
            organization_id,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now().timestamp(),
        };
        self.builder
            .bind_provider(&mut replacement, provider, config)
            .await
            .map_err(internal)?;
        self.queries
            .update_notification_full(&UpdateNotificationFullParams {
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
            })
            .await
            .map_err(internal)?;
        Response::ok(ReplaceNotificationResponse {
            notification: ::buffa::MessageField::some(notif_to_proto(replacement)),
            ..Default::default()
        })
    }

    #[permission(Notification, CanDelete)]
    async fn delete_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteNotificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        match self
            .queries
            .delete_notification_for_organization(request.id, organization_id)
            .await
        {
            Ok(_) => Response::ok(Empty::default()),
            Err(sqlx::Error::RowNotFound) => Err(ConnectError::not_found("notification not found")),
            Err(e) => Err(internal(e)),
        }
    }

    #[permission(Notification, CanCreate)]
    async fn test_notification<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestNotificationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestNotificationResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        self.queries
            .get_notification_by_i_d_for_organization(request.id, organization_id)
            .await
            .map_err(map_sqlx_not_found)?;
        let msg = NotificationMessage::new(
            "OpenOxide Test Notification",
            "Test dispatch from OpenOxide Notification Manager",
        )
        .level(NotificationLevel::Info)
        .field("Status", "Operational")
        .field("Test", "Successful");
        match self.service.send_test(request.id, &msg).await {
            Ok(()) => Response::ok(TestNotificationResponse {
                success: true,
                message: "notification delivered".to_owned(),
                ..Default::default()
            }),
            Err(error) => Response::ok(TestNotificationResponse {
                success: false,
                message: error,
                ..Default::default()
            }),
        }
    }

    #[permission(Notification, CanRead)]
    async fn list_delivery_history<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListNotificationDeliveryHistoryRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ListNotificationDeliveryHistoryResponse> + Send + use<'a>,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let rows = self
            .queries
            .notification_delivery_list(organization_id, 200)
            .await
            .map_err(internal)?;
        Response::ok(ListNotificationDeliveryHistoryResponse {
            attempts: rows.into_iter().map(delivery_to_proto).collect(),
            ..Default::default()
        })
    }

    #[permission(Notification, CanCreate)]
    async fn create_binding<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateNotificationBindingRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<CreateNotificationBindingResponse> + Send + use<'a>,
    > {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let resource_type = proto_resource_type_name(request.resource_type)?;
        self.queries
            .get_notification_by_i_d_for_organization(request.notification_id, organization_id)
            .await
            .map_err(map_sqlx_not_found)?;
        let id = self
            .queries
            .notification_delivery_create_binding(&NotificationDeliveryCreateBindingParams {
                notification_id: request.notification_id,
                organization_id,
                resource_type,
                resource_id: request.resource_id,
            })
            .await
            .map_err(internal)?;
        Response::ok(CreateNotificationBindingResponse {
            id,
            ..Default::default()
        })
    }

    #[permission(Notification, CanRead)]
    async fn list_bindings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListNotificationBindingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListNotificationBindingsResponse> + Send + use<'a>>
    {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let rows = self
            .queries
            .notification_delivery_bindings(organization_id)
            .await
            .map_err(internal)?;
        Response::ok(ListNotificationBindingsResponse {
            bindings: rows.into_iter().map(binding_to_proto).collect(),
            ..Default::default()
        })
    }

    #[permission(Notification, CanDelete)]
    async fn delete_binding<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteNotificationBindingRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(auth.organization_id);
        verify_scope(auth.organization_id, organization_id)?;
        let deleted = self
            .queries
            .notification_delivery_delete_binding(request.id, organization_id)
            .await
            .map_err(internal)?;
        if deleted > 0 {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("binding not found"))
        }
    }
}

fn map_sqlx_not_found(error: sqlx::Error) -> ConnectError {
    match error {
        sqlx::Error::RowNotFound => ConnectError::not_found("notification not found"),
        e => internal(e),
    }
}
