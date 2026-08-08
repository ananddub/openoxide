use crate::db::models::organization_members::OrganizationMember;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct OrganizationMemberRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl OrganizationMemberRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn first_organization_id(&self, user_id: i64) -> sqlx::Result<Option<i64>> {
        sqlx::query_scalar(
            "SELECT organization_id FROM organization_members WHERE user_id=? ORDER BY id LIMIT 1",
        )
        .bind(user_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn role(&self, user_id: i64, organization_id: i64) -> sqlx::Result<Option<String>> {
        sqlx::query_scalar(
            "SELECT role FROM organization_members WHERE user_id=? AND organization_id=?",
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_for_organization(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OrganizationMember>> {
        sqlx::query_as!(
            OrganizationMember,
            r#"SELECT id AS "id?: i64", role AS "role?: String", user_id AS "user_id: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM organization_members WHERE organization_id = ? ORDER BY created_at ASC, id ASC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn update_role(
        &self,
        user_id: i64,
        organization_id: i64,
        role: &str,
    ) -> sqlx::Result<bool> {
        Ok(sqlx::query("UPDATE organization_members SET role = ?, updated_at = unixepoch() WHERE user_id = ? AND organization_id = ?")
            .bind(role)
            .bind(user_id)
            .bind(organization_id)
            .execute(self.pool.as_ref())
            .await?
            .rows_affected() > 0)
    }

    pub async fn remove_member(&self, user_id: i64, organization_id: i64) -> sqlx::Result<bool> {
        Ok(sqlx::query(
            "DELETE FROM organization_members WHERE user_id = ? AND organization_id = ?",
        )
        .bind(user_id)
        .bind(organization_id)
        .execute(self.pool.as_ref())
        .await?
        .rows_affected()
            > 0)
    }

    pub async fn get_all(&self) -> Result<Vec<OrganizationMember>, sqlx::Error> {
        sqlx::query_as!(
            OrganizationMember,
            r#"SELECT id AS "id?: i64", role AS "role?: String", user_id AS "user_id: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM organization_members"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<OrganizationMember>, sqlx::Error> {
        sqlx::query_as!(
            OrganizationMember,
            r#"SELECT id AS "id?: i64", role AS "role?: String", user_id AS "user_id: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM organization_members WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &OrganizationMember) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO organization_members (role, user_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"#,
            &item.role,
            item.user_id,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &OrganizationMember) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE organization_members SET role = ?, user_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.role,
            item.user_id,
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
        sqlx::query!(r#"DELETE FROM organization_members WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn add_member_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        role: &str,
        user_id: i64,
        organization_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO organization_members (role, user_id, organization_id) VALUES (?, ?, ?)",
            role,
            user_id,
            organization_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn add_member_with_group_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        role: &str,
        user_id: i64,
        organization_id: i64,
        group_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO organization_members (role, user_id, organization_id, group_id) VALUES (?, ?, ?, ?)",
        )
        .bind(role)
        .bind(user_id)
        .bind(organization_id)
        .bind(group_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn add_member_with_group(
        &self,
        role: &str,
        user_id: i64,
        organization_id: i64,
        group_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO organization_members (role, user_id, organization_id, group_id) VALUES (?, ?, ?, ?)")
            .bind(role).bind(user_id).bind(organization_id).bind(group_id)
            .execute(self.pool.as_ref()).await?;
        Ok(())
    }
}
