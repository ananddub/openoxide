use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::connect::generated::openoxide::api::v1::{
        CreateEnvironmentRequest, UpdateEnvironmentRequest,
    },
    db::{
        models::environments::Environment,
        queries::FlashQueries,
    },
    flash_gen::{
        environments::{EnvironmentsCreateInTransactionParams, EnvironmentsUpdateInTransactionParams},
        models::Environments,
    },
};

fn to_model(row: Environments) -> Environment {
    Environment {
        id: row.id,
        name: row.name,
        description: row.description,
        env_var: row.env_var,
        is_default: row.is_default,
        project_id: row.project_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub struct EnvironmentService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl EnvironmentService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Environment> {
        let row = self.queries.environments_get_by_id(id).await?;
        Ok(to_model(row))
    }

    pub async fn list_by_project(&self, project_id: i64) -> sqlx::Result<Vec<Environment>> {
        let rows = self.queries.environments_list_by_project(project_id).await?;
        Ok(rows.into_iter().map(to_model).collect())
    }

    pub async fn create(&self, input: CreateEnvironmentRequest) -> sqlx::Result<Environment> {
        let count = self
            .queries
            .environments_count_by_project(input.project_id)
            .await?;
        let make_default = input.is_default || count == 0;

        if make_default {
            self.clear_default(input.project_id).await?;
        }

        let now = chrono::Utc::now().timestamp();
        let row = self
            .queries
            .environments_create_in_transaction(&EnvironmentsCreateInTransactionParams {
                name: input.name,
                description: input.description.unwrap_or_default(),
                env_var: input.environment_variables,
                is_default: i64::from(make_default),
                project_id: input.project_id,
                created_at: now,
                updated_at: now,
            })
            .await?;
        Ok(to_model(row))
    }

    pub async fn update(&self, id: i64, input: UpdateEnvironmentRequest) -> sqlx::Result<Environment> {
        let current = self.get_by_id(id).await?;

        if input.is_default == Some(true) {
            self.clear_default(current.project_id).await?;
        }

        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description).unwrap_or_default();
        let env_var = input.environment_variables.unwrap_or(current.env_var);
        let is_default = match input.is_default {
            Some(false) if current.is_default != 0 => {
                return Err(sqlx::Error::Protocol(
                    "the default environment cannot be unset; set another environment as default instead".into(),
                ));
            }
            Some(value) => i64::from(value),
            None => current.is_default,
        };

        let row = self
            .queries
            .environments_update_in_transaction(&EnvironmentsUpdateInTransactionParams {
                name,
                description,
                env_var,
                is_default,
                id,
            })
            .await?;
        Ok(to_model(row))
    }

    pub async fn set_default(&self, id: i64) -> sqlx::Result<Environment> {
        let current = self.get_by_id(id).await?;
        self.clear_default(current.project_id).await?;
        let row = self
            .queries
            .environments_update_in_transaction(&EnvironmentsUpdateInTransactionParams {
                name: current.name,
                description: current.description.unwrap_or_default(),
                env_var: current.env_var,
                is_default: 1,
                id,
            })
            .await?;
        Ok(to_model(row))
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        let current = self.get_by_id(id).await?;
        self.queries.environments_delete_in_transaction(id).await?;

        if current.is_default != 0 {
            self.queries
                .environments_promote_oldest_to_default(current.project_id)
                .await?;
        }

        Ok(())
    }

    async fn clear_default(&self, project_id: i64) -> sqlx::Result<()> {
        self.queries.environments_clear_default(project_id).await
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

        EnvironmentService {
            queries: Arc::new(FlashQueries::new(Arc::new(pool))),
        }
    }

    fn create_input(name: &str, is_default: bool) -> CreateEnvironmentRequest {
        CreateEnvironmentRequest {
            name: name.into(),
            description: None,
            environment_variables: String::new(),
            is_default,
            project_id: 1,
            ..Default::default()
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
        let second = service.create(create_input("preview", false)).await.unwrap();

        service.delete(first.id).await.unwrap();

        let remaining = service.get_by_id(second.id).await.unwrap();
        assert_eq!(remaining.is_default, 1);
    }

    #[tokio::test]
    async fn patch_changes_only_supplied_fields() {
        let service = service().await;
        let environment = service
            .create(CreateEnvironmentRequest {
                name: "production".into(),
                description: Some("Primary".into()),
                environment_variables: "PORT=3000".into(),
                is_default: false,
                project_id: 1,
                ..Default::default()
            })
            .await
            .unwrap();

        let patched = service
            .update(
                environment.id,
                UpdateEnvironmentRequest {
                    name: Some("prod".into()),
                    description: None,
                    environment_variables: None,
                    is_default: None,
                    ..Default::default()
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
