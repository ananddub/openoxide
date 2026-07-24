use crate::api::dto::application::{CreateApplicationDto, PatchApplicationDto};

use super::{ApplicationRecord, ApplicationService, queries::generate_app_name};

impl ApplicationService {
    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ApplicationRecord> {
        let app = self
            .repo_app
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        Ok(ApplicationRecord::from(app))
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<ApplicationRecord>> {
        let list = self.repo_app.list_by_environment(environment_id).await?;
        Ok(list.into_iter().map(ApplicationRecord::from).collect())
    }

    pub async fn create(&self, input: CreateApplicationDto) -> sqlx::Result<ApplicationRecord> {
        let app_name = generate_app_name(&input.name);
        let app = self
            .repo_app
            .create_simple(
                input.name,
                app_name,
                input.description,
                input.source_type,
                input.build_type,
                input.environment_id,
                input.server_id,
            )
            .await?;
        Ok(ApplicationRecord::from(app))
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchApplicationDto,
    ) -> sqlx::Result<ApplicationRecord> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let build_type = input.build_type.unwrap_or(current.build_type);
        let trigger_type = input.trigger_type.unwrap_or(current.trigger_type);
        let env_var = input.env_var.or(current.env_var);
        let icon = input.icon.or(current.icon);
        let server_id = input.server_id.or(current.server_id);
        let build_server_id = input.build_server_id.or(current.build_server_id);
        let registry_id = input.registry_id.or(current.registry_id);

        let app = self
            .repo_app
            .patch(
                id,
                name,
                description,
                build_type,
                trigger_type,
                env_var,
                icon,
                server_id,
                build_server_id,
                registry_id,
            )
            .await?;
        Ok(ApplicationRecord::from(app))
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.repo_app.delete(id).await?;
        Ok(())
    }
}
