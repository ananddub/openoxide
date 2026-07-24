use crate::db::models::environments::Environment;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct EnvironmentRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl EnvironmentRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", env_var AS "env_var: String", is_default AS "is_default: i64", project_id AS "project_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM environments"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", env_var AS "env_var: String", is_default AS "is_default: i64", project_id AS "project_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM environments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Environment) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO environments (name, description, env_var, is_default, project_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.description,
            &item.env_var,
            item.is_default,
            item.project_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Environment) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE environments SET name = ?, description = ?, env_var = ?, is_default = ?, project_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.description,
            &item.env_var,
            item.is_default,
            item.project_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM environments WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        id: i64,
    ) -> Result<Environment, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"SELECT id AS "id?", name, description, env_var, is_default, project_id, created_at, updated_at
               FROM environments WHERE id = ?"#,
            id
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn list_by_project(&self, project_id: i64) -> Result<Vec<Environment>, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"SELECT id AS "id?", name, description, env_var, is_default, project_id, created_at, updated_at
               FROM environments WHERE project_id = ?
               ORDER BY is_default DESC, created_at ASC, id ASC"#,
            project_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn count_by_project(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        project_id: i64,
    ) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM environments WHERE project_id = ?",
            project_id
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(count)
    }

    pub async fn clear_default(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        project_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE environments SET is_default = 0 WHERE project_id = ? AND is_default = 1",
            project_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn create_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        name: String,
        description: Option<String>,
        env_var: String,
        is_default: i64,
        project_id: i64,
    ) -> Result<Environment, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"INSERT INTO environments (name, description, env_var, is_default, project_id)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id AS "id?", name, description, env_var, is_default, project_id, created_at, updated_at"#,
            name,
            description,
            env_var,
            is_default,
            project_id
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn update_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        id: i64,
        name: String,
        description: Option<String>,
        env_var: String,
        is_default: i64,
    ) -> Result<Environment, sqlx::Error> {
        sqlx::query_as!(
            Environment,
            r#"UPDATE environments SET name = ?, description = ?, env_var = ?, is_default = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, env_var, is_default, project_id, created_at, updated_at"#,
            name,
            description,
            env_var,
            is_default,
            id
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn delete_in_transaction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM environments WHERE id = ?", id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn promote_oldest_to_default(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        project_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE environments SET is_default = 1 WHERE id = (SELECT id FROM environments WHERE project_id = ? ORDER BY created_at ASC, id ASC LIMIT 1)",
            project_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
