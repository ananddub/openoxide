use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::compute::application::ApplicationOperation,
    utils::{
        builder::errors::BuilderError,
        builder::{
            adapter::ApplicationSpecAdapter, application::ApplicationBuilder, custom_type::IdType,
            hash_state::ApplicationState, queue::queue::BuilderQueue,
        },
        exec::CommandExecutor,
    },
};

impl BuilderQueue {
    pub(crate) async fn execute_operation_app(
        &self,
        application_id: i64,
        deployment_id: i64,
        _operation: ApplicationOperation,
    ) -> Result<(), BuilderError> {
        let spec_adapter = resolve::<ApplicationSpecAdapter>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve ApplicationSpecAdapter: {e}"))
        })?;
        let spec = spec_adapter.load(application_id).await.map_err(|e| {
            BuilderError::Execution(format!("could not load deployment config: {e}"))
        })?;

        let q = resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve FlashQueries: {e}"))
            })?;
        let ctx = q
            .services_get_deployment_context(application_id)
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve deployment context: {e}"))
            })?;
        let (environment_id, project_id, server_id) =
            (ctx.environment_id, ctx.project_id, ctx.server_id);

        let app_key = IdType::AppId(application_id);
        let state = resolve::<ApplicationState>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application state: {e}"))
        })?;
        state.reset_default(app_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(app_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let executor = self
            .executor_for_deployment(server_id, deployment_id)
            .await?;

        let (events_tx, events_rx) = mpsc::channel(6);
        tokio::spawn(super::deployment_log::record_builder_events(
            deployment_id,
            events_rx,
            "app",
        ));

        let spec_snapshot = spec.clone();

        let events_tx_clone = events_tx.clone();
        let executor_clone = executor.clone();
        let build_future = async move {
            let builder = ApplicationBuilder::new(executor_clone)
                .with_state(state, app_key)
                .with_events(events_tx);

            builder
                .deploy(&spec, &cancel)
                .await
                .map_err(|e| BuilderError::Execution(e.to_string()))
        };

        let deploy_result = super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await?;

        self.create_rollback_snapshot(
            application_id,
            deployment_id,
            &deploy_result.image,
            &spec_snapshot,
            &executor,
        )
        .await;

        Ok(())
    }

    async fn create_rollback_snapshot(
        &self,
        application_id: i64,
        deployment_id: i64,
        image: &str,
        spec: &crate::utils::builder::spec::ApplicationSpec,
        executor: &CommandExecutor,
    ) {
        use crate::flash_gen::rollbacks::RollbacksCreateParams;

        let q = match resolve::<crate::db::queries::FlashQueries>().await {
            Ok(q) => q,
            Err(e) => {
                tracing::warn!(error = %e, "rollback: could not resolve FlashQueries, skipping snapshot");
                return;
            }
        };

        let version = match q
            .rollbacks_get_next_version_for_application(application_id)
            .await
        {
            Ok(v) => v.unwrap_or(0) + 1,
            Err(e) => {
                tracing::warn!(error = %e, "rollback: could not determine next version, skipping snapshot");
                return;
            }
        };

        // Tag image: myapp:latest → myapp:v{version}
        let versioned_image = format_versioned_image(image, version);
        let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
        if let Err(e) = docker.images().tag(image, &versioned_image).run().await {
            tracing::warn!(
                error = %e,
                source_image = image,
                target_image = %versioned_image,
                "rollback: failed to tag image, skipping snapshot"
            );
            return;
        }

        // Keep rollback images available to other swarm nodes when a registry is configured.
        if let Some(auth) = &spec.image_registry {
            let remote_image = if auth.registry.trim().is_empty() {
                versioned_image.clone()
            } else {
                format!(
                    "{}/{}",
                    auth.registry.trim_end_matches('/'),
                    versioned_image
                )
            };
            let distribution = async {
                docker
                    .system()
                    .login()
                    .registry(&auth.registry)
                    .username(&auth.username)
                    .password(&auth.password)
                    .run()
                    .await?;
                if remote_image != versioned_image {
                    docker
                        .images()
                        .tag(&versioned_image, &remote_image)
                        .run()
                        .await?;
                }
                docker.images().push(&remote_image).run().await
            }
            .await;
            let _ = docker
                .system()
                .logout()
                .registry(&auth.registry)
                .run()
                .await;
            if let Err(error) = distribution {
                tracing::warn!(%error, image = %remote_image, "rollback image push failed; local rollback retained");
            }
        }

        tracing::info!(
            source_image = image,
            versioned_image = %versioned_image,
            version,
            deployment_id,
            "rollback: tagged image for rollback"
        );

        // Serialize the full ApplicationSpec as the rollback context
        let full_context = match serde_json::to_string(spec) {
            Ok(json) => Some(json),
            Err(e) => {
                tracing::warn!(error = %e, "rollback: could not serialize spec, saving without context");
                None
            }
        };

        match q
            .rollbacks_create(&RollbacksCreateParams {
                deployment_id,
                version,
                image: versioned_image,
                full_context: full_context.unwrap_or_default(),
                created_at: chrono::Utc::now().timestamp(),
            })
            .await
        {
            Ok(id) => {
                tracing::info!(
                    rollback_id = id,
                    version,
                    deployment_id,
                    "rollback: snapshot saved"
                );
            }
            Err(e) => {
                tracing::warn!(error = %e, deployment_id, "rollback: failed to save snapshot");
            }
        }
    }
}

fn format_versioned_image(image: &str, version: i64) -> String {
    // Strip the tag only when the last `:` is a tag separator, not a registry
    // port (`registry:5000/img`).
    let base = image
        .rsplit_once(':')
        .filter(|(_, tag)| !tag.contains('/'))
        .map(|(base, _)| base)
        .unwrap_or(image);
    format!("{base}:v{version}")
}
