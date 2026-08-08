use crate::db::models::organization_invites::OrganizationInvite;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct OrganizationInviteRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl OrganizationInviteRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<OrganizationInvite>, sqlx::Error> {
        sqlx::query_as!(
            OrganizationInvite,
            r#"SELECT id AS "id?: i64", email AS "email: String", role AS "role?: String", status AS "status?: String", token AS "token: String", group_id AS "group_id: i64", organization_id AS "organization_id: i64", invited_by AS "invited_by: i64", expired_at AS "expired_at: i64", created_at AS "created_at: i64" FROM organization_invites"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<OrganizationInvite>, sqlx::Error> {
        sqlx::query_as!(
            OrganizationInvite,
            r#"SELECT id AS "id?: i64", email AS "email: String", role AS "role?: String", status AS "status?: String", token AS "token: String", group_id AS "group_id: i64", organization_id AS "organization_id: i64", invited_by AS "invited_by: i64", expired_at AS "expired_at: i64", created_at AS "created_at: i64" FROM organization_invites WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &OrganizationInvite) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO organization_invites (email, role, status, token, group_id, organization_id, invited_by, expired_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.email,
            &item.role,
            &item.status,
            &item.token,
            item.group_id,
            item.organization_id,
            item.invited_by,
            item.expired_at,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &OrganizationInvite) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE organization_invites SET email = ?, role = ?, status = ?, token = ?, group_id = ?, organization_id = ?, invited_by = ?, expired_at = ?, created_at = ? WHERE id = ?"#,
            &item.email,
            &item.role,
            &item.status,
            &item.token,
            item.group_id,
            item.organization_id,
            item.invited_by,
            item.expired_at,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM organization_invites WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_pending_for_organization(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OrganizationInvite>> {
        sqlx::query_as!(
            OrganizationInvite,
            r#"SELECT id AS "id?: i64", email AS "email: String", role AS "role?: String", status AS "status?: String", token AS "token: String", group_id AS "group_id: i64", organization_id AS "organization_id: i64", invited_by AS "invited_by: i64", expired_at AS "expired_at: i64", created_at AS "created_at: i64" FROM organization_invites WHERE organization_id = ? AND status = 'PENDING' AND expired_at > unixepoch() ORDER BY created_at DESC"#,
            organization_id
        ).fetch_all(self.pool.as_ref()).await
    }

    pub async fn find_pending_by_token(
        &self,
        token_hash: &str,
    ) -> sqlx::Result<Option<OrganizationInvite>> {
        sqlx::query_as!(
            OrganizationInvite,
            r#"SELECT id AS "id?: i64", email AS "email: String", role AS "role?: String", status AS "status?: String", token AS "token: String", group_id AS "group_id: i64", organization_id AS "organization_id: i64", invited_by AS "invited_by: i64", expired_at AS "expired_at: i64", created_at AS "created_at: i64" FROM organization_invites WHERE token = ? AND status = 'PENDING' AND expired_at > unixepoch()"#,
            token_hash
        ).fetch_optional(self.pool.as_ref()).await
    }

    pub async fn set_status(&self, id: i64, status: &str) -> sqlx::Result<bool> {
        Ok(sqlx::query(
            "UPDATE organization_invites SET status = ? WHERE id = ? AND status = 'PENDING'",
        )
        .bind(status)
        .bind(id)
        .execute(self.pool.as_ref())
        .await?
        .rows_affected()
            > 0)
    }

    pub async fn purge_expired(&self) -> sqlx::Result<u64> {
        Ok(sqlx::query("UPDATE organization_invites SET status = 'REJECTED' WHERE status = 'PENDING' AND expired_at <= unixepoch()")
            .execute(self.pool.as_ref()).await?.rows_affected())
    }
}
