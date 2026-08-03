pub mod builder;

use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use builder::NotificationProviderBuilder;

use crate::{
    api::dto::notification::{CreateNotificationDto, NotificationResponseDto, PatchNotificationDto},
    core::middleware::validator::ValidatedJson,
    db::{
        models::notifications::Notification,
        repository::{
            NotifCustomRepository, NotifDiscordRepository, NotifEmailRepository, NotifGotifyRepository,
            NotifLarkRepository, NotifMattermostRepository, NotifNtfyRepository,
            NotifPushoverRepository, NotifResendRepository, NotifSlackRepository, NotifTeamRepository,
            NotifTelegramRepository, NotificationRepository,
        },
    },
    services::notification::{
        NotificationLevel, NotificationMessage, NotificationService,
    },
};

type ApiError = (StatusCode, String);

pub struct NotificationController {
    repo: Arc<NotificationRepository>,
    service: Arc<NotificationService>,
    builder: NotificationProviderBuilder,
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
        }
    }

    #[get]
    async fn list_all(&self) -> Result<Json<Vec<NotificationResponseDto>>, ApiError> {
        let items = self
            .repo
            .get_all()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(
            items.into_iter().map(NotificationResponseDto::from).collect(),
        ))
    }

    #[get("/{id}")]
    async fn get_by_id(&self, Path(id): Path<i64>) -> Result<Json<NotificationResponseDto>, ApiError> {
        let item = self
            .repo
            .get_by_id(id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Notification {id} not found")))?;

        Ok(Json(NotificationResponseDto::from(item)))
    }

    #[post]
    async fn create(
        &self,
        ValidatedJson(dto): ValidatedJson<CreateNotificationDto>,
    ) -> Result<(StatusCode, Json<NotificationResponseDto>), ApiError> {
        let now = chrono::Utc::now().timestamp();
        let mut notif = Notification {
            id: None,
            name: dto.name.clone(),
            notification_type: dto.notification_type.as_str().to_string(),
            on_app_deploy: dto.on_app_deploy as i64,
            on_app_build_error: dto.on_app_build_error as i64,
            on_database_backup: dto.on_database_backup as i64,
            on_volume_backup: dto.on_volume_backup as i64,
            on_panel_restart: dto.on_panel_restart as i64,
            on_docker_cleanup: dto.on_docker_cleanup as i64,
            on_server_threshold: dto.on_server_threshold as i64,
            on_panel_backup: dto.on_panel_backup as i64,
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
            organization_id: 1,
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
        Ok((StatusCode::CREATED, Json(NotificationResponseDto::from(notif))))
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        Path(id): Path<i64>,
        ValidatedJson(dto): ValidatedJson<PatchNotificationDto>,
    ) -> Result<Json<NotificationResponseDto>, ApiError> {
        let mut notif = self
            .repo
            .get_by_id(id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Notification {id} not found")))?;

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

        notif.updated_at = chrono::Utc::now().timestamp();

        self.repo
            .update(id, &notif)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(NotificationResponseDto::from(notif)))
    }

    #[delete("/{id}")]
    async fn delete(&self, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        self.repo
            .delete(id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/{id}/test")]
    async fn send_test(&self, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        let msg = NotificationMessage::new("Rustploy Test Notification", "Test dispatch from Rustploy Notification Manager")
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
