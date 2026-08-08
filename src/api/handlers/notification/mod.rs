use crate::core::middleware::permission::{CanCreate, CanDelete, CanRead, Server};
pub mod builder;

use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use builder::NotificationProviderBuilder;

use crate::{
    api::dto::notification::{
        CreateNotificationBindingDto, CreateNotificationDto, NotificationDeliveryAttemptDto,
        NotificationResourceBindingDto, NotificationResponseDto, PatchNotificationDto,
    },
    core::middleware::permission::RequirePermission,
    core::middleware::validator::ValidatedJson,
    db::{
        models::notifications::Notification,
        repository::{
            NotifCustomRepository, NotifDiscordRepository, NotifEmailRepository,
            NotifGotifyRepository, NotifLarkRepository, NotifMattermostRepository,
            NotifNtfyRepository, NotifPushoverRepository, NotifResendRepository,
            NotifSlackRepository, NotifTeamRepository, NotifTelegramRepository,
            NotificationDeliveryRepository, NotificationRepository,
        },
    },
    services::notification::{
        NotificationLevel, NotificationMessage, NotificationProvider, NotificationService,
    },
};

type ApiError = (StatusCode, String);

pub struct NotificationController {
    repo: Arc<NotificationRepository>,
    service: Arc<NotificationService>,
    builder: NotificationProviderBuilder,
    delivery: Arc<NotificationDeliveryRepository>,
}

#[controller("/notifications")]
impl NotificationController {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        repo: Arc<NotificationRepository>,
        service: Arc<NotificationService>,
        slack: Arc<NotifSlackRepository>,
        telegram: Arc<NotifTelegramRepository>,
        discord: Arc<NotifDiscordRepository>,
        email: Arc<NotifEmailRepository>,
        resend: Arc<NotifResendRepository>,
        gotify: Arc<NotifGotifyRepository>,
        ntfy: Arc<NotifNtfyRepository>,
        mattermost: Arc<NotifMattermostRepository>,
        custom: Arc<NotifCustomRepository>,
        lark: Arc<NotifLarkRepository>,
        pushover: Arc<NotifPushoverRepository>,
        teams: Arc<NotifTeamRepository>,
        delivery: Arc<NotificationDeliveryRepository>,
    ) -> Self {
        let builder = NotificationProviderBuilder {
            slack,
            telegram,
            discord,
            email,
            resend,
            gotify,
            ntfy,
            mattermost,
            custom,
            lark,
            pushover,
            teams,
        };

        Self {
            repo,
            service,
            builder,
            delivery,
        }
    }

    #[get("/delivery-history/organization/{organization_id}")]
    async fn delivery_history(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<NotificationDeliveryAttemptDto>>, ApiError> {
        verify_scope(claims.user.group_id, organization_id)?;
        self.delivery
            .list(organization_id, 200)
            .await
            .map(|rows| Json(rows.into_iter().map(Into::into).collect()))
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
    }

    #[post("/bindings/organization/{organization_id}")]
    async fn create_binding(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanCreate>,
        Path(organization_id): Path<i64>,
        Json(body): Json<CreateNotificationBindingDto>,
    ) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
        verify_scope(claims.user.group_id, organization_id)?;
        self.repo
            .get_by_id_for_organization(body.notification_id, organization_id)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?
            .ok_or((StatusCode::NOT_FOUND, "notification not found".into()))?;
        let id = self
            .delivery
            .create_binding(
                body.notification_id,
                organization_id,
                body.resource_type.as_str(),
                body.resource_id,
            )
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
    }

    #[get("/bindings/organization/{organization_id}")]
    async fn bindings(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<NotificationResourceBindingDto>>, ApiError> {
        verify_scope(claims.user.group_id, organization_id)?;
        self.delivery
            .bindings(organization_id)
            .await
            .map(|rows| Json(rows.into_iter().map(Into::into).collect()))
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))
    }

    #[delete("/bindings/{id}/organization/{organization_id}")]
    async fn delete_binding(
        &self,
        RequirePermission(claims, _): RequirePermission<Server, CanDelete>,
        Path((id, organization_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        verify_scope(claims.user.group_id, organization_id)?;
        if self
            .delivery
            .delete_binding(id, organization_id)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?
        {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((StatusCode::NOT_FOUND, "binding not found".into()))
        }
    }

    #[get("/organization/{organization_id}")]
    async fn list_by_organization(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<NotificationResponseDto>>, ApiError> {
        let items = self
            .repo
            .get_by_organization(organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(
            items
                .into_iter()
                .map(NotificationResponseDto::from)
                .collect(),
        ))
    }

    #[get("/{id}/organization/{organization_id}")]
    async fn get_by_id(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanRead>,
        Path((id, organization_id)): Path<(i64, i64)>,
    ) -> Result<Json<NotificationResponseDto>, ApiError> {
        let item = self
            .repo
            .get_by_id_for_organization(id, organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    format!("Notification {id} not found"),
                )
            })?;

        Ok(Json(NotificationResponseDto::from(item)))
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanCreate>,
        ValidatedJson(dto): ValidatedJson<CreateNotificationDto>,
    ) -> Result<(StatusCode, Json<NotificationResponseDto>), ApiError> {
        let now = chrono::Utc::now().timestamp();
        let settings = &dto.settings;
        let provider = match &dto.provider {
            crate::api::dto::notification::create::NotificationConfigDto::Slack(_) => {
                NotificationProvider::Slack
            }
            crate::api::dto::notification::create::NotificationConfigDto::Telegram(_) => {
                NotificationProvider::Telegram
            }
            crate::api::dto::notification::create::NotificationConfigDto::Discord(_) => {
                NotificationProvider::Discord
            }
            crate::api::dto::notification::create::NotificationConfigDto::Email(_) => {
                NotificationProvider::Email
            }
            crate::api::dto::notification::create::NotificationConfigDto::Resend(_) => {
                NotificationProvider::Resend
            }
            crate::api::dto::notification::create::NotificationConfigDto::Gotify(_) => {
                NotificationProvider::Gotify
            }
            crate::api::dto::notification::create::NotificationConfigDto::Ntfy(_) => {
                NotificationProvider::Ntfy
            }
            crate::api::dto::notification::create::NotificationConfigDto::Mattermost(_) => {
                NotificationProvider::Mattermost
            }
            crate::api::dto::notification::create::NotificationConfigDto::Custom(_) => {
                NotificationProvider::Custom
            }
            crate::api::dto::notification::create::NotificationConfigDto::Lark(_) => {
                NotificationProvider::Lark
            }
            crate::api::dto::notification::create::NotificationConfigDto::Pushover(_) => {
                NotificationProvider::Pushover
            }
            crate::api::dto::notification::create::NotificationConfigDto::Teams(_) => {
                NotificationProvider::Teams
            }
        };
        let mut notif = Notification {
            id: None,
            name: settings.name.clone(),
            notification_type: provider.as_str().to_string(),
            on_app_deploy: settings.on_app_deploy as i64,
            on_app_build_error: settings.on_app_build_error as i64,
            on_database_backup: settings.on_database_backup as i64,
            on_volume_backup: settings.on_volume_backup as i64,
            on_panel_restart: settings.on_panel_restart as i64,
            on_docker_cleanup: settings.on_docker_cleanup as i64,
            on_server_threshold: settings.on_server_threshold as i64,
            on_panel_backup: settings.on_panel_backup as i64,
            on_schedule_success: settings.on_schedule_success as i64,
            on_schedule_failure: settings.on_schedule_failure as i64,
            slack_id: None,
            telegram_id: None,
            discord_id: None,
            email_id: None,
            resend_id: None,
            gotify_id: None,
            ntfy_id: None,
            mattermost_id: None,
            custom_id: None,
            lark_id: None,
            pushover_id: None,
            teams_id: None,
            organization_id: settings.organization_id,
            created_at: now,
            updated_at: now,
        };

        self.builder.bind_provider(&mut notif, &dto).await?;

        let created_id = self
            .repo
            .create(&notif)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        notif.id = Some(created_id);
        Ok((
            StatusCode::CREATED,
            Json(NotificationResponseDto::from(notif)),
        ))
    }

    #[patch("/{id}/organization/{organization_id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanCreate>,
        Path((id, organization_id)): Path<(i64, i64)>,
        ValidatedJson(dto): ValidatedJson<PatchNotificationDto>,
    ) -> Result<Json<NotificationResponseDto>, ApiError> {
        let mut notif = self
            .repo
            .get_by_id_for_organization(id, organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    format!("Notification {id} not found"),
                )
            })?;

        if let Some(name) = dto.name {
            notif.name = name;
        }
        if let Some(val) = dto.on_app_deploy {
            notif.on_app_deploy = val as i64;
        }
        if let Some(val) = dto.on_app_build_error {
            notif.on_app_build_error = val as i64;
        }
        if let Some(val) = dto.on_database_backup {
            notif.on_database_backup = val as i64;
        }
        if let Some(val) = dto.on_volume_backup {
            notif.on_volume_backup = val as i64;
        }
        if let Some(val) = dto.on_panel_restart {
            notif.on_panel_restart = val as i64;
        }
        if let Some(val) = dto.on_docker_cleanup {
            notif.on_docker_cleanup = val as i64;
        }
        if let Some(val) = dto.on_server_threshold {
            notif.on_server_threshold = val as i64;
        }
        if let Some(val) = dto.on_panel_backup {
            notif.on_panel_backup = val as i64;
        }
        if let Some(val) = dto.on_schedule_success {
            notif.on_schedule_success = val as i64;
        }
        if let Some(val) = dto.on_schedule_failure {
            notif.on_schedule_failure = val as i64;
        }

        notif.updated_at = chrono::Utc::now().timestamp();

        self.repo
            .update(id, &notif)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(NotificationResponseDto::from(notif)))
    }

    /// Replaces both event settings and provider configuration. The tagged
    /// provider union prevents fields for different providers being mixed.
    #[put("/{id}/organization/{organization_id}")]
    async fn replace(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanCreate>,
        Path((id, organization_id)): Path<(i64, i64)>,
        ValidatedJson(dto): ValidatedJson<CreateNotificationDto>,
    ) -> Result<Json<NotificationResponseDto>, ApiError> {
        if dto.settings.organization_id != organization_id {
            return Err((
                StatusCode::BAD_REQUEST,
                "organization_id must match route".into(),
            ));
        }
        let existing = self
            .repo
            .get_by_id_for_organization(id, organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::NOT_FOUND, "notification not found".into()))?;
        let mut replacement = Notification {
            id: Some(id),
            name: dto.settings.name.clone(),
            notification_type: String::new(),
            on_app_deploy: dto.settings.on_app_deploy as i64,
            on_app_build_error: dto.settings.on_app_build_error as i64,
            on_database_backup: dto.settings.on_database_backup as i64,
            on_volume_backup: dto.settings.on_volume_backup as i64,
            on_panel_restart: dto.settings.on_panel_restart as i64,
            on_docker_cleanup: dto.settings.on_docker_cleanup as i64,
            on_server_threshold: dto.settings.on_server_threshold as i64,
            on_panel_backup: dto.settings.on_panel_backup as i64,
            on_schedule_success: dto.settings.on_schedule_success as i64,
            on_schedule_failure: dto.settings.on_schedule_failure as i64,
            slack_id: None,
            telegram_id: None,
            discord_id: None,
            email_id: None,
            resend_id: None,
            gotify_id: None,
            ntfy_id: None,
            mattermost_id: None,
            custom_id: None,
            lark_id: None,
            pushover_id: None,
            teams_id: None,
            organization_id,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now().timestamp(),
        };
        replacement.notification_type = provider_of(&dto).as_str().to_owned();
        self.builder.bind_provider(&mut replacement, &dto).await?;
        self.repo
            .update(id, &replacement)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(Json(NotificationResponseDto::from(replacement)))
    }

    #[delete("/{id}/organization/{organization_id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanDelete>,
        Path((id, organization_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        let removed = self
            .repo
            .delete_for_organization(id, organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        if removed {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((StatusCode::NOT_FOUND, "notification not found".into()))
        }
    }

    #[post("/{id}/organization/{organization_id}/test")]
    async fn send_test(
        &self,
        RequirePermission(_claims, _): RequirePermission<Server, CanCreate>,
        Path((id, organization_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        self.repo
            .get_by_id_for_organization(id, organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::NOT_FOUND, "notification not found".into()))?;
        let msg = NotificationMessage::new(
            "Rustploy Test Notification",
            "Test dispatch from Rustploy Notification Manager",
        )
        .level(NotificationLevel::Info)
        .field("Status", "Operational")
        .field("Test", "Successful");

        self.service
            .send_test(id, &msg)
            .await
            .map_err(|err_msg| (StatusCode::BAD_REQUEST, err_msg))?;

        Ok(StatusCode::OK)
    }
}

fn provider_of(dto: &CreateNotificationDto) -> NotificationProvider {
    use crate::api::dto::notification::create::NotificationConfigDto::*;
    match &dto.provider {
        Slack(_) => NotificationProvider::Slack,
        Telegram(_) => NotificationProvider::Telegram,
        Discord(_) => NotificationProvider::Discord,
        Email(_) => NotificationProvider::Email,
        Resend(_) => NotificationProvider::Resend,
        Gotify(_) => NotificationProvider::Gotify,
        Ntfy(_) => NotificationProvider::Ntfy,
        Mattermost(_) => NotificationProvider::Mattermost,
        Custom(_) => NotificationProvider::Custom,
        Lark(_) => NotificationProvider::Lark,
        Pushover(_) => NotificationProvider::Pushover,
        Teams(_) => NotificationProvider::Teams,
    }
}

fn verify_scope(authenticated: i64, requested: i64) -> Result<(), ApiError> {
    if authenticated == requested {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            "organization does not match authenticated scope".into(),
        ))
    }
}
