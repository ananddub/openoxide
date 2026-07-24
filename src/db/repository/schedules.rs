use crate::db::models::schedules::Schedule;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ScheduleRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ScheduleRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", cron_expression AS "cron_expression: String", app_name AS "app_name: String", service_name AS "service_name?: String", shell_type AS "shell_type: String", schedule_type AS "schedule_type: String", command AS "command: String", script AS "script?: String", timezone AS "timezone?: String", enabled AS "enabled: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", organization_id AS "organization_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", schedule_action AS "schedule_action: String" FROM schedules"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", cron_expression AS "cron_expression: String", app_name AS "app_name: String", service_name AS "service_name?: String", shell_type AS "shell_type: String", schedule_type AS "schedule_type: String", command AS "command: String", script AS "script?: String", timezone AS "timezone?: String", enabled AS "enabled: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", organization_id AS "organization_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64", schedule_action AS "schedule_action: String" FROM schedules WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Schedule) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO schedules (name, description, cron_expression, app_name, service_name, shell_type, schedule_type, command, script, timezone, enabled, application_id, compose_id, server_id, organization_id, created_at, updated_at, schedule_action) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.description,
            &item.cron_expression,
            &item.app_name,
            &item.service_name,
            &item.shell_type,
            &item.schedule_type,
            &item.command,
            &item.script,
            &item.timezone,
            item.enabled,
            item.application_id,
            item.compose_id,
            item.server_id,
            item.organization_id,
            item.created_at,
            item.updated_at,
            &item.schedule_action
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Schedule) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE schedules SET name = ?, description = ?, cron_expression = ?, app_name = ?, service_name = ?, shell_type = ?, schedule_type = ?, command = ?, script = ?, timezone = ?, enabled = ?, application_id = ?, compose_id = ?, server_id = ?, organization_id = ?, created_at = ?, updated_at = ?, schedule_action = ? WHERE id = ?"#,
            &item.name,
            &item.description,
            &item.cron_expression,
            &item.app_name,
            &item.service_name,
            &item.shell_type,
            &item.schedule_type,
            &item.command,
            &item.script,
            &item.timezone,
            item.enabled,
            item.application_id,
            item.compose_id,
            item.server_id,
            item.organization_id,
            item.created_at,
            item.updated_at,
            &item.schedule_action,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM schedules WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> Result<Vec<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE application_id = ?
               ORDER BY created_at DESC, id DESC"#,
            application_id,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> Result<Vec<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE compose_id = ?
               ORDER BY created_at DESC, id DESC"#,
            compose_id,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_server(&self, server_id: i64) -> Result<Vec<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE server_id = ?
               ORDER BY created_at DESC, id DESC"#,
            server_id,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_organization(
        &self,
        organization_id: i64,
    ) -> Result<Vec<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE organization_id = ?
               ORDER BY created_at DESC, id DESC"#,
            organization_id,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_enabled(&self) -> Result<Vec<Schedule>, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE enabled = 1
               ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_and_return(
        &self,
        name: String,
        description: Option<String>,
        cron_expression: String,
        app_name: String,
        service_name: Option<String>,
        shell_type: String,
        schedule_type: String,
        schedule_action: String,
        command: String,
        script: Option<String>,
        timezone: Option<String>,
        enabled: i64,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        server_id: Option<i64>,
        organization_id: Option<i64>,
    ) -> Result<Schedule, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"INSERT INTO schedules
               (name, description, cron_expression, app_name, service_name, shell_type,
                schedule_type, schedule_action, command, script, timezone, enabled, application_id, compose_id,
                server_id, organization_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            name,
            description,
            cron_expression,
            app_name,
            service_name,
            shell_type,
            schedule_type,
            schedule_action,
            command,
            script,
            timezone,
            enabled,
            application_id,
            compose_id,
            server_id,
            organization_id,
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn update_and_return(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        cron_expression: String,
        app_name: String,
        service_name: Option<String>,
        shell_type: String,
        schedule_type: String,
        schedule_action: String,
        command: String,
        script: Option<String>,
        timezone: Option<String>,
        enabled: i64,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        server_id: Option<i64>,
        organization_id: Option<i64>,
    ) -> Result<Schedule, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"UPDATE schedules SET
               name = ?, description = ?, cron_expression = ?, app_name = ?, service_name = ?,
               shell_type = ?, schedule_type = ?, schedule_action = ?, command = ?, script = ?, timezone = ?,
               enabled = ?, application_id = ?, compose_id = ?, server_id = ?, organization_id = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            name,
            description,
            cron_expression,
            app_name,
            service_name,
            shell_type,
            schedule_type,
            schedule_action,
            command,
            script,
            timezone,
            enabled,
            application_id,
            compose_id,
            server_id,
            organization_id,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn set_enabled(&self, id: i64, enabled: bool) -> Result<Schedule, sqlx::Error> {
        sqlx::query_as!(
            Schedule,
            r#"UPDATE schedules SET enabled = ? WHERE id = ?
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            if enabled { 1 } else { 0 },
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}
