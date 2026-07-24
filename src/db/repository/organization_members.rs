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
}
