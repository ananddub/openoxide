use crate::api::dto::application::{CreateApplicationDto, PatchApplicationDto};
use crate::core::cache::{CacheEnum, CacheKey};

use super::{ApplicationRecord, ApplicationService, queries::generate_app_name};

impl ApplicationService {
    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<ApplicationRecord> {
        let key = CacheKey::Application(id);
        let res = self
            .cache
            .try_get_with(key, async {
                let app = self
                    .repo_app
                    .get_by_id(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                Ok::<_, sqlx::Error>(CacheEnum::Application(ApplicationRecord::from(app)))
            })
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if let CacheEnum::Application(record) = res {
            Ok(record)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
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
        let network_ids =
            crate::api::dto::database::serialize_json_string_vec(input.network_ids.as_ref())?
                .unwrap_or(current.network_ids);
        let detach_rustploy_network = input
            .detach_rustploy_network
            .unwrap_or(current.detach_rustploy_network);

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
                network_ids,
                detach_rustploy_network,
            )
            .await?;
        let record = ApplicationRecord::from(app);
        self.cache
            .insert(
                CacheKey::Application(id),
                CacheEnum::Application(record.clone()),
            )
            .await;
        Ok(record)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let previews =
            auto_di::resolve::<crate::services::preview_deployment::PreviewDeploymentService>()
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        previews
            .ensure_application_can_be_deleted(id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        previews
            .remove_for_base_application(id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        self.repo_app.delete(id).await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        Ok(())
    }
}
