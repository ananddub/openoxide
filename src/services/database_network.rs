use crate::{
    api::dto::database_network::{CreateDatabaseNetworkDto, PatchDatabaseNetworkDto},
    db::models::database_networks::DatabaseNetwork,
    repository::DatabaseNetworkRepository,
};
use auto_di::singleton;
use std::sync::Arc;

pub struct DatabaseNetworkService {
    repo: Arc<DatabaseNetworkRepository>,
}

#[singleton]
impl DatabaseNetworkService {
    fn new(repo: Arc<DatabaseNetworkRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> sqlx::Result<Vec<DatabaseNetwork>> {
        self.repo.list().await
    }

    pub async fn list_by_server(
        &self,
        server_id: Option<i64>,
    ) -> sqlx::Result<Vec<DatabaseNetwork>> {
        self.repo.list_by_server(server_id).await
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<DatabaseNetwork> {
        self.repo
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn create(&self, input: CreateDatabaseNetworkDto) -> sqlx::Result<DatabaseNetwork> {
        let id = self
            .repo
            .create(
                &input.name,
                &input.docker_network_name,
                input.description.as_deref(),
                input.external.unwrap_or(1),
                input.server_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchDatabaseNetworkDto,
    ) -> sqlx::Result<DatabaseNetwork> {
        self.get_by_id(id).await?;
        self.repo
            .update(
                id,
                input.name.as_deref(),
                input.docker_network_name.as_deref(),
                input.description.as_deref(),
                input.external,
                input.server_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.repo.delete(id).await
    }
}
