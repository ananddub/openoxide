use std::sync::Arc;

use auto_di::singleton;
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PermissionGroupRow {
    pub id: i64,
    pub name: String,
    pub is_system: bool,
    pub organization_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PermissionPolicyRow {
    pub id: i64,
    pub action: String,
}

pub struct PermissionGroupRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PermissionGroupRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn seed_organization_defaults(&self, organization_id: i64) -> sqlx::Result<i64> {
        let mut tx = self.pool.begin().await?;
        let existing: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM groups WHERE organization_id = ? AND name = 'Organization Admin'",
        )
        .bind(organization_id)
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(id) = existing {
            tx.commit().await?;
            return Ok(id);
        }
        let admin = sqlx::query("INSERT INTO groups (name, is_system, organization_id) VALUES ('Organization Admin', 1, ?)")
            .bind(organization_id).execute(&mut *tx).await?.last_insert_rowid();
        let policies: Vec<i64> = sqlx::query_scalar("SELECT id FROM policy WHERE action LIKE '%:read' OR action LIKE '%:create' OR action LIKE '%:update' OR action LIKE '%:delete'")
            .fetch_all(&mut *tx).await?;
        for policy in policies {
            sqlx::query("INSERT OR IGNORE INTO group_policy (group_id, policy_id) VALUES (?, ?)")
                .bind(admin)
                .bind(policy)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(admin)
    }

    pub async fn list_groups(&self, organization_id: i64) -> sqlx::Result<Vec<PermissionGroupRow>> {
        sqlx::query_as(
            "SELECT id, name, is_system != 0 AS is_system, organization_id, created_at, updated_at FROM groups WHERE organization_id IS NULL OR organization_id = ? ORDER BY is_system DESC, name",
        )
        .bind(organization_id)
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn group_actions(&self, group_id: i64) -> sqlx::Result<Vec<String>> {
        sqlx::query_scalar(
            "SELECT p.action FROM group_policy gp JOIN policy p ON p.id = gp.policy_id WHERE gp.group_id = ? ORDER BY p.action",
        )
        .bind(group_id)
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn all_policies(&self) -> sqlx::Result<Vec<PermissionPolicyRow>> {
        sqlx::query_as("SELECT id, action FROM policy ORDER BY action")
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn effective_actions(
        &self,
        user_id: i64,
        organization_id: i64,
    ) -> sqlx::Result<Vec<String>> {
        sqlx::query_scalar(
            r#"
            WITH overrides AS (
                SELECT p.action, up.effect
                FROM user_policy up
                JOIN policy p ON p.id = up.policy_id
                WHERE up.user_id = ? AND up.org_id = ?
            )
            SELECT DISTINCT action FROM (
                SELECT p.action
                FROM organization_members om
                JOIN group_policy gp ON gp.group_id = om.group_id
                JOIN policy p ON p.id = gp.policy_id
                WHERE om.user_id = ? AND om.organization_id = ?
                UNION ALL
                SELECT action FROM overrides WHERE effect = 'GRANT'
            )
            WHERE action NOT IN (SELECT action FROM overrides WHERE effect = 'DENY')
            ORDER BY action
            "#,
        )
        .bind(user_id)
        .bind(organization_id)
        .bind(user_id)
        .bind(organization_id)
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_group(
        &self,
        organization_id: i64,
        name: &str,
        policy_ids: &[i64],
    ) -> sqlx::Result<i64> {
        let mut tx = self.pool.begin().await?;
        let result =
            sqlx::query("INSERT INTO groups (name, is_system, organization_id) VALUES (?, 0, ?)")
                .bind(name)
                .bind(organization_id)
                .execute(&mut *tx)
                .await?;
        let group_id = result.last_insert_rowid();
        for policy_id in policy_ids {
            sqlx::query("INSERT INTO group_policy (group_id, policy_id) VALUES (?, ?)")
                .bind(group_id)
                .bind(policy_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(group_id)
    }

    pub async fn update_group(
        &self,
        organization_id: i64,
        group_id: i64,
        name: &str,
        policy_ids: &[i64],
    ) -> sqlx::Result<bool> {
        let mut tx = self.pool.begin().await?;
        let updated = sqlx::query(
            "UPDATE groups SET name = ? WHERE id = ? AND organization_id = ? AND is_system = 0",
        )
        .bind(name)
        .bind(group_id)
        .bind(organization_id)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if updated == 0 {
            tx.rollback().await?;
            return Ok(false);
        }
        sqlx::query("DELETE FROM group_policy WHERE group_id = ?")
            .bind(group_id)
            .execute(&mut *tx)
            .await?;
        for policy_id in policy_ids {
            sqlx::query("INSERT INTO group_policy (group_id, policy_id) VALUES (?, ?)")
                .bind(group_id)
                .bind(policy_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(true)
    }

    pub async fn delete_group(&self, organization_id: i64, group_id: i64) -> sqlx::Result<bool> {
        let assigned: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id = ? AND group_id = ?)",
        )
        .bind(organization_id)
        .bind(group_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        if assigned {
            return Ok(false);
        }
        Ok(
            sqlx::query(
                "DELETE FROM groups WHERE id = ? AND organization_id = ? AND is_system = 0",
            )
            .bind(group_id)
            .bind(organization_id)
            .execute(self.pool.as_ref())
            .await?
            .rows_affected()
                > 0,
        )
    }

    pub async fn assign_group(
        &self,
        organization_id: i64,
        user_id: i64,
        group_id: i64,
    ) -> sqlx::Result<bool> {
        let valid_group: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM groups WHERE id = ? AND (organization_id IS NULL OR organization_id = ?))",
        )
        .bind(group_id)
        .bind(organization_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        if !valid_group {
            return Ok(false);
        }
        Ok(sqlx::query(
            "UPDATE organization_members SET group_id = ? WHERE user_id = ? AND organization_id = ?",
        )
        .bind(group_id)
        .bind(user_id)
        .bind(organization_id)
        .execute(self.pool.as_ref())
        .await?
        .rows_affected()
            > 0)
    }

    pub async fn replace_user_policies(
        &self,
        organization_id: i64,
        user_id: i64,
        policies: &[(i64, &str)],
    ) -> sqlx::Result<bool> {
        let member: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM organization_members WHERE user_id = ? AND organization_id = ?)",
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        if !member {
            return Ok(false);
        }
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM user_policy WHERE user_id = ? AND org_id = ?")
            .bind(user_id)
            .bind(organization_id)
            .execute(&mut *tx)
            .await?;
        for (policy_id, effect) in policies {
            sqlx::query(
                "INSERT INTO user_policy (user_id, org_id, policy_id, effect) VALUES (?, ?, ?, ?)",
            )
            .bind(user_id)
            .bind(organization_id)
            .bind(policy_id)
            .bind(effect)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(true)
    }
}
