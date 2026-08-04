use crate::db::models::notifications::Notification;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NotificationRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl NotificationRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Notification>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"SELECT id AS "id?: i64", name AS "name: String", notification_type AS "notification_type: String", on_app_deploy AS "on_app_deploy: i64", on_app_build_error AS "on_app_build_error: i64", on_database_backup AS "on_database_backup: i64", on_volume_backup AS "on_volume_backup: i64", on_panel_restart AS "on_panel_restart: i64", on_docker_cleanup AS "on_docker_cleanup: i64", on_server_threshold AS "on_server_threshold: i64", on_panel_backup AS "on_panel_backup: i64", slack_id AS "slack_id?: i64", telegram_id AS "telegram_id?: i64", discord_id AS "discord_id?: i64", email_id AS "email_id?: i64", resend_id AS "resend_id?: i64", gotify_id AS "gotify_id?: i64", ntfy_id AS "ntfy_id?: i64", mattermost_id AS "mattermost_id?: i64", custom_id AS "custom_id?: i64", lark_id AS "lark_id?: i64", pushover_id AS "pushover_id?: i64", teams_id AS "teams_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM notifications"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    /// Notifications belonging to one organization.
    ///
    /// Dispatch must use this rather than `get_all`: sending one tenant's alerts
    /// to every tenant's channels leaks who runs what.
    pub async fn get_by_organization(
        &self,
        organization_id: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"SELECT id AS "id?: i64", name AS "name: String", notification_type AS "notification_type: String", on_app_deploy AS "on_app_deploy: i64", on_app_build_error AS "on_app_build_error: i64", on_database_backup AS "on_database_backup: i64", on_volume_backup AS "on_volume_backup: i64", on_panel_restart AS "on_panel_restart: i64", on_docker_cleanup AS "on_docker_cleanup: i64", on_server_threshold AS "on_server_threshold: i64", on_panel_backup AS "on_panel_backup: i64", slack_id AS "slack_id?: i64", telegram_id AS "telegram_id?: i64", discord_id AS "discord_id?: i64", email_id AS "email_id?: i64", resend_id AS "resend_id?: i64", gotify_id AS "gotify_id?: i64", ntfy_id AS "ntfy_id?: i64", mattermost_id AS "mattermost_id?: i64", custom_id AS "custom_id?: i64", lark_id AS "lark_id?: i64", pushover_id AS "pushover_id?: i64", teams_id AS "teams_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM notifications WHERE organization_id = ?"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Notification>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"SELECT id AS "id?: i64", name AS "name: String", notification_type AS "notification_type: String", on_app_deploy AS "on_app_deploy: i64", on_app_build_error AS "on_app_build_error: i64", on_database_backup AS "on_database_backup: i64", on_volume_backup AS "on_volume_backup: i64", on_panel_restart AS "on_panel_restart: i64", on_docker_cleanup AS "on_docker_cleanup: i64", on_server_threshold AS "on_server_threshold: i64", on_panel_backup AS "on_panel_backup: i64", slack_id AS "slack_id?: i64", telegram_id AS "telegram_id?: i64", discord_id AS "discord_id?: i64", email_id AS "email_id?: i64", resend_id AS "resend_id?: i64", gotify_id AS "gotify_id?: i64", ntfy_id AS "ntfy_id?: i64", mattermost_id AS "mattermost_id?: i64", custom_id AS "custom_id?: i64", lark_id AS "lark_id?: i64", pushover_id AS "pushover_id?: i64", teams_id AS "teams_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM notifications WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Option<Notification>, sqlx::Error> {
        sqlx::query_as!(
            Notification,
            r#"SELECT id AS "id?: i64", name AS "name: String", notification_type AS "notification_type: String", on_app_deploy AS "on_app_deploy: i64", on_app_build_error AS "on_app_build_error: i64", on_database_backup AS "on_database_backup: i64", on_volume_backup AS "on_volume_backup: i64", on_panel_restart AS "on_panel_restart: i64", on_docker_cleanup AS "on_docker_cleanup: i64", on_server_threshold AS "on_server_threshold: i64", on_panel_backup AS "on_panel_backup: i64", slack_id AS "slack_id?: i64", telegram_id AS "telegram_id?: i64", discord_id AS "discord_id?: i64", email_id AS "email_id?: i64", resend_id AS "resend_id?: i64", gotify_id AS "gotify_id?: i64", ntfy_id AS "ntfy_id?: i64", mattermost_id AS "mattermost_id?: i64", custom_id AS "custom_id?: i64", lark_id AS "lark_id?: i64", pushover_id AS "pushover_id?: i64", teams_id AS "teams_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM notifications WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Notification) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO notifications (name, notification_type, on_app_deploy, on_app_build_error, on_database_backup, on_volume_backup, on_panel_restart, on_docker_cleanup, on_server_threshold, on_panel_backup, slack_id, telegram_id, discord_id, email_id, resend_id, gotify_id, ntfy_id, mattermost_id, custom_id, lark_id, pushover_id, teams_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.notification_type,
            item.on_app_deploy,
            item.on_app_build_error,
            item.on_database_backup,
            item.on_volume_backup,
            item.on_panel_restart,
            item.on_docker_cleanup,
            item.on_server_threshold,
            item.on_panel_backup,
            item.slack_id,
            item.telegram_id,
            item.discord_id,
            item.email_id,
            item.resend_id,
            item.gotify_id,
            item.ntfy_id,
            item.mattermost_id,
            item.custom_id,
            item.lark_id,
            item.pushover_id,
            item.teams_id,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Notification) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE notifications SET name = ?, notification_type = ?, on_app_deploy = ?, on_app_build_error = ?, on_database_backup = ?, on_volume_backup = ?, on_panel_restart = ?, on_docker_cleanup = ?, on_server_threshold = ?, on_panel_backup = ?, slack_id = ?, telegram_id = ?, discord_id = ?, email_id = ?, resend_id = ?, gotify_id = ?, ntfy_id = ?, mattermost_id = ?, custom_id = ?, lark_id = ?, pushover_id = ?, teams_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.notification_type,
            item.on_app_deploy,
            item.on_app_build_error,
            item.on_database_backup,
            item.on_volume_backup,
            item.on_panel_restart,
            item.on_docker_cleanup,
            item.on_server_threshold,
            item.on_panel_backup,
            item.slack_id,
            item.telegram_id,
            item.discord_id,
            item.email_id,
            item.resend_id,
            item.gotify_id,
            item.ntfy_id,
            item.mattermost_id,
            item.custom_id,
            item.lark_id,
            item.pushover_id,
            item.teams_id,
            item.organization_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM notifications WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn delete_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM notifications WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
