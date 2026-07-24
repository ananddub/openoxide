use crate::db::models::projects::Project;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ProjectRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ProjectRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", env_var AS "env_var: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM projects"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", env_var AS "env_var: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM projects WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Project) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO projects (name, description, env_var, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.description,
            &item.env_var,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Project) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE projects SET name = ?, description = ?, env_var = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.description,
            &item.env_var,
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
        sqlx::query!(r#"DELETE FROM projects WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_by_organization(
        &self,
        organization_id: i64,
    ) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            r#"SELECT id AS "id?", name, description, env_var, organization_id, created_at, updated_at
               FROM projects WHERE organization_id = ? ORDER BY created_at DESC, id DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        name: String,
        description: Option<String>,
        env_var: String,
        organization_id: i64,
    ) -> Result<Project, sqlx::Error> {
        sqlx::query_as!(
            Project,
            r#"INSERT INTO projects (name, description, env_var, organization_id) VALUES (?, ?, ?, ?)
               RETURNING id AS "id?", name, description, env_var, organization_id, created_at, updated_at"#,
            name,
            description,
            env_var,
            organization_id
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn update_and_return(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        env_var: String,
    ) -> Result<Project, sqlx::Error> {
        sqlx::query_as!(
            Project,
            r#"UPDATE projects SET name = ?, description = ?, env_var = ? WHERE id = ?
               RETURNING id AS "id?", name, description, env_var, organization_id, created_at, updated_at"#,
            name,
            description,
            env_var,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}
