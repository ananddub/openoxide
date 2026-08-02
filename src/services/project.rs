use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::project::{CreateProjectDto, PatchProjectDto},
    db::models::projects::Project,
    repository::{EnvironmentRepository, ProjectRepository},
};

pub struct ProjectService {
    db: Arc<SqlitePool>,
    repo_project: Arc<ProjectRepository>,
    repo_env: Arc<EnvironmentRepository>,
}

#[singleton]
impl ProjectService {
    fn new(
        db: Arc<SqlitePool>,
        repo_project: Arc<ProjectRepository>,
        repo_env: Arc<EnvironmentRepository>,
    ) -> Self {
        Self {
            db,
            repo_project,
            repo_env,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Project> {
        self.repo_project
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list_by_organization(&self, organization_id: i64) -> sqlx::Result<Vec<Project>> {
        self.repo_project
            .list_by_organization(organization_id)
            .await
    }

    pub async fn create(&self, input: CreateProjectDto) -> sqlx::Result<Project> {
        let mut tx = self.db.begin().await?;
        let project = self
            .repo_project
            .create_in_transaction(
                &mut tx,
                input.name,
                input.description,
                input.env_var,
                input.organization_id,
            )
            .await?;

        let project_id = project
            .id
            .ok_or_else(|| sqlx::Error::Protocol("missing project id".into()))?;
        self.repo_env
            .create_in_transaction(
                &mut tx,
                "production".to_string(),
                Some("Production environment".to_string()),
                "".to_string(),
                1, // is_default
                project_id,
            )
            .await?;

        tx.commit().await?;
        Ok(project)
    }

    pub async fn update(&self, id: i64, input: PatchProjectDto) -> sqlx::Result<Project> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let env_var = input.env_var.unwrap_or(current.env_var);

        self.repo_project.update_and_return(id, name, description, env_var).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.repo_project.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    #[tokio::test]
    async fn create_also_creates_production_default_environment() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE organization (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE projects (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, description TEXT, env_var TEXT NOT NULL DEFAULT '', organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE environments (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, description TEXT, env_var TEXT NOT NULL DEFAULT '', is_default INTEGER NOT NULL DEFAULT 0, project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO organization (id) VALUES (1)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(pool);
        let service = ProjectService {
            db: db.clone(),
            repo_project: Arc::new(ProjectRepository::new(db.clone())),
            repo_env: Arc::new(EnvironmentRepository::new(db.clone())),
        };

        let project = service
            .create(CreateProjectDto {
                name: "website".into(),
                description: None,
                env_var: String::new(),
                organization_id: 1,
            })
            .await
            .unwrap();

        let environment: (String, i64) =
            sqlx::query_as("SELECT name, is_default FROM environments WHERE project_id = ?")
                .bind(project.id)
                .fetch_one(service.db.as_ref())
                .await
                .unwrap();
        assert_eq!(environment, ("production".into(), 1));
    }
}
