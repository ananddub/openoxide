use crate::api::dto::application::{PatchBuildConfigDto, PatchResourceConfigDto};

use super::{ApplicationRecord, ApplicationService};

impl ApplicationService {
    pub async fn patch_build_config(
        &self,
        id: i64,
        input: PatchBuildConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .patch_build_config(
                id,
                input.build_args,
                input.build_secrets,
                input.dockerfile,
                input.docker_context_path,
                input.docker_build_stage,
                input.publish_directory,
                input.is_static_spa,
                input.create_env_file,
                input.railpack_version,
                input.heroku_version,
                input.command,
                input.args,
                input.build_path,
                input.clean_cache,
                input.enable_submodules,
                input.watch_paths,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn patch_resource_config(
        &self,
        id: i64,
        input: PatchResourceConfigDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .patch_resource_config(
                id,
                input.memory_reservation,
                input.memory_limit,
                input.cpu_reservation,
                input.cpu_limit,
                input.replicas,
            )
            .await?;
        self.get_by_id(id).await
    }
}
