use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::environment::{CreateEnvironmentDto, PatchEnvironmentDto},
    db::models::environments::Environment,
    repository::EnvironmentRepository,
};

pub struct EnvironmentService {
    db: Arc<SqlitePool>,
    repo_env: Arc<EnvironmentRepository>,
}

#[singleton]
impl EnvironmentService {
    fn new(db: Arc<SqlitePool>, repo_env: Arc<EnvironmentRepository>) -> Self {
        Self { db, repo_env }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Environment> {
        self.repo_env
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list_by_project(&self, project_id: i64) -> sqlx::Result<Vec<Environment>> {
        self.repo_env.list_by_project(project_id).await
    }

    pub async fn create(&self, input: CreateEnvironmentDto) -> sqlx::Result<Environment> {
        let mut tx = self.db.begin().await?;
        let count = self
            .repo_env
            .count_by_project(&mut tx, input.project_id)
            .await?;
        let make_default = input.is_default || count == 0;

        if make_default {
            self.repo_env
                .clear_default(&mut tx, input.project_id)
                .await?;
        }

        let is_default = i64::from(make_default);
        let environment = self
            .repo_env
            .create_in_transaction(
                &mut tx,
                input.name,
                input.description,
                input.env_var,
                is_default,
                input.project_id,
            )
            .await?;

        tx.commit().await?;
        Ok(environment)
    }

    pub async fn update(&self, id: i64, input: PatchEnvironmentDto) -> sqlx::Result<Environment> {
        let mut tx = self.db.begin().await?;
        let current = self.repo_env.get_in_transaction(&mut tx, id).await?;

        if input.is_default == Some(true) {
            self.repo_env
                .clear_default(&mut tx, current.project_id)
                .await?;
        }

        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let env_var = input.env_var.unwrap_or(current.env_var);
        let is_default = match input.is_default {
            Some(false) if current.is_default != 0 => {
                return Err(sqlx::Error::Protocol(
                    "the default environment cannot be unset; set another environment as default instead".into(),
                ));
            }
            Some(value) => i64::from(value),
            None => current.is_default,
        };

        let environment = self
            .repo_env
            .update_in_transaction(&mut tx, id, name, description, env_var, is_default)
            .await?;

        tx.commit().await?;
        Ok(environment)
    }

    pub async fn set_default(&self, id: i64) -> sqlx::Result<Environment> {
        let mut tx = self.db.begin().await?;
        let current = self.repo_env.get_in_transaction(&mut tx, id).await?;
        self.repo_env
            .clear_default(&mut tx, current.project_id)
            .await?;
        let environment = self
            .repo_env
            .update_in_transaction(
                &mut tx,
                id,
                current.name,
                current.description,
                current.env_var,
                1,
            )
            .await?;
        tx.commit().await?;
        Ok(environment)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        let mut tx = self.db.begin().await?;
        let current = self.repo_env.get_in_transaction(&mut tx, id).await?;
        self.repo_env.delete_in_transaction(&mut tx, id).await?;

        if current.is_default != 0 {
            self.repo_env
                .promote_oldest_to_default(&mut tx, current.project_id)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    async fn service() -> EnvironmentService {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE projects (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE environments (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, description TEXT, env_var TEXT NOT NULL DEFAULT '', is_default INTEGER NOT NULL DEFAULT 0, project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO projects (id) VALUES (1)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(pool);
        EnvironmentService {
            db: db.clone(),
            repo_env: Arc::new(EnvironmentRepository::new(db.clone())),
        }
    }

    fn create_input(name: &str, is_default: bool) -> CreateEnvironmentDto {
        CreateEnvironmentDto {
            name: name.into(),
            description: None,
            env_var: String::new(),
            is_default,
            project_id: 1,
        }
    }

    #[tokio::test]
    async fn first_environment_is_default_and_only_one_default_is_kept() {
        let service = service().await;
        let first = service
            .create(create_input("production", false))
            .await
            .unwrap();
        let second = service.create(create_input("preview", true)).await.unwrap();

        assert_eq!(first.is_default, 1);
        assert_eq!(second.is_default, 1);
        let environments = service.list_by_project(1).await.unwrap();
        assert_eq!(
            environments
                .iter()
                .filter(|item| item.is_default == 1)
                .count(),
            1
        );
        assert_eq!(environments[0].name, "preview");
    }

    #[tokio::test]
    async fn deleting_default_promotes_oldest_remaining_environment() {
        let service = service().await;
        let first = service
            .create(create_input("production", false))
            .await
            .unwrap();
        let second = service
            .create(create_input("preview", false))
            .await
            .unwrap();

        service.delete(first.id.unwrap()).await.unwrap();

        let remaining = service.get_by_id(second.id.unwrap()).await.unwrap();
        assert_eq!(remaining.is_default, 1);
    }

    #[tokio::test]
    async fn patch_changes_only_supplied_fields() {
        let service = service().await;
        let environment = service
            .create(CreateEnvironmentDto {
                name: "production".into(),
                description: Some("Primary".into()),
                env_var: "PORT=3000".into(),
                is_default: false,
                project_id: 1,
            })
            .await
            .unwrap();

        let patched = service
            .update(
                environment.id.unwrap(),
                PatchEnvironmentDto {
                    name: Some("prod".into()),
                    description: None,
                    env_var: None,
                    is_default: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(patched.name, "prod");
        assert_eq!(patched.description.as_deref(), Some("Primary"));
        assert_eq!(patched.env_var, "PORT=3000");
        assert_eq!(patched.is_default, 1);
    }
}
