use buffa::EnumValue;

use super::{assert_protocol, create_application};
use crate::api::connect::generated::openoxide::api::v1::{
    ApplicationBuildType, ApplicationSourceType, CreateApplicationRequest,
    SetBitbucketSourceRequest, SetCustomGitSourceRequest, SetDockerSourceRequest,
    SetDropSourceRequest, SetGiteaSourceRequest, SetGithubSourceRequest, SetGitlabSourceRequest,
    UpdateApplicationRequest, UpdateBuildConfigRequest, UpdatePreviewConfigRequest,
    UpdateResourceConfigRequest,
};
use crate::services::compute::application::import_export::ImportApplicationDto;
use crate::services::compute::application::middleware::{
    ApplicationMiddlewareType, MiddlewareHeaderDto, UpsertApplicationMiddlewareDto,
};
use crate::services::compute::application::mount::UpsertApplicationMountDto;
use crate::services::compute::application::network::UpdateApplicationNetworksDto;
use crate::services::compute::application::patch::{
    ApplicationPatchType, UpsertApplicationPatchDto,
};
use crate::services::compute::application::port::{
    PortProtocol, PortPublishMode, UpsertApplicationPortDto,
};
use crate::services::compute::application::redirect::UpsertApplicationRedirectDto;
use crate::services::compute::application::security::UpsertApplicationSecurityDto;
use crate::services::compute::engine::CancelDeploymentResult;
use crate::services::compute::test_support::*;
use crate::utils::builder::custom_type::IdType;
use crate::utils::builder::hash_state::ApplicationState;
use crate::utils::builder::spec::MountType;

#[tokio::test]
async fn create_generates_slug_defaults_and_lists_by_environment() {
    let db = TestDb::new().await;
    let app = create_application(&db, "My App 1", ENV_ID, Some(SERVER_ID)).await;

    assert_eq!(app.name, "My App 1");
    assert!(app.app_name.starts_with("my-app-1-"), "{}", app.app_name);
    assert_eq!(app.app_name.len(), "my-app-1-".len() + 8);
    assert_eq!(app.description, None);
    assert_eq!(app.source_type, "GITHUB");
    assert_eq!(app.build_type, "NIXPACKS");
    assert_eq!(app.app_status, "IDLE");
    assert_eq!(app.environment_id, ENV_ID);
    assert_eq!(app.server_id, Some(SERVER_ID));
    assert_eq!(app.trigger_type, "PUSH");
    assert!(app.auto_deploy);
    assert_eq!(app.build_path, None);
    assert_eq!(app.network_ids, "[]");
    assert_eq!(app.detach_rustploy_network, 0);
    assert_eq!(app.replicas, Some(1));
    assert!(!app.clean_cache);
    assert!(!app.is_static_spa);
    assert_eq!(app.preview_port, Some(3000));
    assert_eq!(app.preview_path.as_deref(), Some("/"));
    assert_eq!(app.preview_limit, Some(3));
    assert!(!app.preview_https);
    assert!(!app.previews_active);
    assert_eq!(app.preview_certificate_type, "NONE");
    assert!(app.preview_require_collaborator_permissions);

    let listed = db.application().list_by_environment(ENV_ID).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, app.id);
}

#[tokio::test]
async fn create_without_server_maps_source_and_build_types() {
    let db = TestDb::new().await;
    let app = db
        .application()
        .create(CreateApplicationRequest {
            name: "Dockerized".into(),
            description: Some("runs a prebuilt image".into()),
            environment_id: STAGING_ENV_ID,
            server_id: None,
            source_type: EnumValue::Known(ApplicationSourceType::Docker),
            build_type: EnumValue::Known(ApplicationBuildType::Dockerfile),
        })
        .await
        .unwrap();

    assert_eq!(app.server_id, None);
    assert_eq!(app.description.as_deref(), Some("runs a prebuilt image"));
    assert_eq!(app.source_type, "DOCKER");
    assert_eq!(app.build_type, "DOCKERFILE");
    assert_eq!(app.environment_id, STAGING_ENV_ID);
}

#[tokio::test]
async fn create_unknown_enums_fall_back_to_github_and_nixpacks() {
    let db = TestDb::new().await;
    let app = db
        .application()
        .create(CreateApplicationRequest {
            name: "Fallback".into(),
            environment_id: ENV_ID,
            source_type: EnumValue::Unknown(99),
            build_type: EnumValue::Unknown(99),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(app.source_type, "GITHUB");
    assert_eq!(app.build_type, "NIXPACKS");
}

#[tokio::test]
async fn load_record_and_get_by_id_reject_missing_or_non_application() {
    let db = TestDb::new().await;
    let services = db.application();
    assert!(services.load_record(999).await.unwrap().is_none());
    // get_by_id wraps cache misses in a Protocol error instead of RowNotFound.
    assert!(matches!(
        services.get_by_id(999).await,
        Err(sqlx::Error::Protocol(_))
    ));

    let compose_id = db
        .insert_service("COMPOSE", "IDLE", ENV_ID, Some(SERVER_ID))
        .await;
    assert!(services.load_record(compose_id).await.unwrap().is_none());
}

#[tokio::test]
async fn ensure_organization_access_checks_project_scope() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Scoped", ENV_ID, None).await;

    services
        .ensure_organization_access(app.id, ORG_ID)
        .await
        .unwrap();
    assert!(matches!(
        services
            .ensure_organization_access(app.id, OTHER_ORG_ID)
            .await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn patch_updates_identity_network_and_build_fields() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Patch Me", ENV_ID, Some(SERVER_ID)).await;

    let patched = services
        .patch(
            app.id,
            UpdateApplicationRequest {
                id: app.id,
                name: Some("Renamed".into()),
                description: Some("updated description".into()),
                build_type: Some(EnumValue::Known(ApplicationBuildType::Dockerfile)),
                trigger_type: Some("TAG".into()),
                env_var: Some("KEY=1".into()),
                server_id: Some(SERVER_ID_2),
                build_server_id: Some(SERVER_ID_2),
                registry_id: Some(REGISTRY_ID),
                network_ids: vec!["7".into(), "8".into()],
                detach_rustploy_network: Some(true),
            },
        )
        .await
        .unwrap();

    assert_eq!(patched.name, "Renamed");
    assert_eq!(patched.app_name, app.app_name);
    assert_eq!(patched.description.as_deref(), Some("updated description"));
    assert_eq!(patched.build_type, "DOCKERFILE");
    assert_eq!(patched.trigger_type, "TAG");
    assert_eq!(patched.env_var.as_deref(), Some("KEY=1"));
    assert_eq!(patched.server_id, Some(SERVER_ID_2));
    assert_eq!(patched.build_server_id, Some(SERVER_ID_2));
    assert_eq!(patched.registry_id, Some(REGISTRY_ID));
    assert_eq!(patched.network_ids, r#"["7","8"]"#);
    assert_eq!(patched.detach_rustploy_network, 1);

    let cached = services.get_by_id(app.id).await.unwrap();
    assert_eq!(cached.name, "Renamed");
    assert_eq!(cached.network_ids, r#"["7","8"]"#);
}

#[tokio::test]
async fn patch_keeps_networks_when_list_is_empty() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Keep Networks", ENV_ID, None).await;

    services
        .patch(
            app.id,
            UpdateApplicationRequest {
                id: app.id,
                network_ids: vec!["1".into(), "2".into()],
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let patched = services
        .patch(
            app.id,
            UpdateApplicationRequest {
                id: app.id,
                name: Some("Still Here".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(patched.network_ids, r#"["1","2"]"#);
    assert_eq!(patched.name, "Still Here");
    assert!(matches!(
        services
            .patch(999, UpdateApplicationRequest::default())
            .await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn delete_is_blocked_by_active_deployments() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Busy", ENV_ID, None).await;
    db.insert_deployment(Some(app.id), None, "RUNNING", "RUNNING", 1, "", USER_ID)
        .await;

    assert_protocol(services.delete(app.id).await, "active dependencies");
    assert!(services.get_by_id(app.id).await.is_ok());
}

#[tokio::test]
async fn delete_is_blocked_by_enabled_volume_backups() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Backed Up", ENV_ID, None).await;
    db.insert_volume_backup(app.id, true).await;

    assert_protocol(services.delete(app.id).await, "enabled_backups=1");
    db.insert_volume_backup(app.id, false).await;
    let counts = services.dependencies(app.id).await.unwrap();
    assert_eq!(counts.backups, 2);
    assert_eq!(counts.enabled_backups, 1);
}

#[tokio::test]
async fn dependency_counts_reflect_relations() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Counted", ENV_ID, None).await;
    db.port()
        .create(
            app.id,
            UpsertApplicationPortDto {
                published_port: 8080,
                target_port: 80,
                protocol: PortProtocol::Tcp,
                publish_mode: PortPublishMode::Ingress,
            },
        )
        .await
        .unwrap();
    db.mount()
        .create(
            app.id,
            UpsertApplicationMountDto {
                mount_type: MountType::Volume,
                host_path: None,
                volume_name: Some("data".into()),
                file_path: None,
                content: None,
                mount_path: "/data".into(),
            },
        )
        .await
        .unwrap();
    db.patch()
        .create(
            app.id,
            UpsertApplicationPatchDto {
                patch_type: ApplicationPatchType::Create,
                file_path: "docker/entrypoint.sh".into(),
                enabled: true,
                content: "echo hi".into(),
            },
        )
        .await
        .unwrap();
    db.middleware()
        .create(
            app.id,
            UpsertApplicationMiddlewareDto {
                name: "compress".into(),
                middleware_type: ApplicationMiddlewareType::Compress,
                enabled: true,
                headers: None,
                average: None,
                burst: None,
                source_ranges: None,
            },
        )
        .await
        .unwrap();
    db.redirect()
        .create(
            app.id,
            UpsertApplicationRedirectDto {
                regex: "^/old$".into(),
                replacement: "/new".into(),
                permanent: 1,
                unique_config_key: None,
            },
        )
        .await
        .unwrap();
    db.security()
        .create(
            app.id,
            UpsertApplicationSecurityDto {
                username: "admin".into(),
                password: "secret".into(),
            },
        )
        .await
        .unwrap();
    db.insert_deployment(Some(app.id), None, "RUNNING", "RUNNING", 1, "", USER_ID)
        .await;
    db.insert_deployment(Some(app.id), None, "DONE", "DONE", 2, "", 0)
        .await;

    let counts = services.dependencies(app.id).await.unwrap();
    assert_eq!(counts.active_deployments, 1);
    assert_eq!(counts.deployments, 2);
    assert_eq!(counts.ports, 1);
    assert_eq!(counts.mounts, 1);
    assert_eq!(counts.patches, 1);
    assert_eq!(counts.middleware, 1);
    assert_eq!(counts.redirects, 1);
    assert_eq!(counts.security_rules, 1);
}

#[tokio::test]
async fn rotate_webhook_token_persists_a_hex_value() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Webhook", ENV_ID, None).await;

    let token = services.rotate_webhook_token(app.id).await.unwrap();
    assert_eq!(token.len(), 64);
    assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    let stored: String = sqlx::query_scalar("SELECT refresh_token FROM services WHERE id = ?")
        .bind(app.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(stored, token);
    assert!(matches!(
        services.rotate_webhook_token(999).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn move_to_environment_moves_within_organization() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Mover", ENV_ID, None).await;

    let moved = services
        .move_to_environment(app.id, STAGING_ENV_ID)
        .await
        .unwrap();
    assert_eq!(moved.environment_id, STAGING_ENV_ID);
}

#[tokio::test]
async fn move_to_environment_rejects_other_organization() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Stay", ENV_ID, None).await;

    assert_protocol(
        services.move_to_environment(app.id, OTHER_ENV_ID).await,
        "target environment not found or belongs to another organization",
    );
    assert_eq!(
        services.get_by_id(app.id).await.unwrap().environment_id,
        ENV_ID
    );
}

#[tokio::test]
async fn move_to_environment_is_blocked_by_queued_deployment() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Deploying", ENV_ID, None).await;
    db.insert_deployment(Some(app.id), None, "QUEUED", "QUEUED", 1, "", 0)
        .await;

    assert_protocol(
        services.move_to_environment(app.id, STAGING_ENV_ID).await,
        "already queued or running",
    );
}

#[tokio::test]
async fn clear_deployment_history_removes_finished_rows_only() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "History", ENV_ID, None).await;
    db.insert_deployment(Some(app.id), None, "DONE", "DONE", 1, "", 0)
        .await;
    db.insert_deployment(Some(app.id), None, "FAILED", "FAILED", 2, "", 0)
        .await;
    db.insert_deployment(Some(app.id), None, "RUNNING", "RUNNING", 3, "", USER_ID)
        .await;

    let result = services.clear_deployment_history(app.id).await.unwrap();
    assert_eq!(result.affected, 2);
    let remaining: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM deployments WHERE service_id = ?")
            .bind(app.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(remaining, 1);
}

#[tokio::test]
async fn cleanup_deployment_queue_requires_an_existing_application() {
    // The success path reaches BuilderQueue::cancel_queued_application, which
    // opens the process-global database pool; only the guard is unit-testable.
    let db = TestDb::new().await;
    let services = db.application();
    assert!(matches!(
        services.cleanup_deployment_queue(999).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn force_kill_requires_a_running_deployment() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Idle", ENV_ID, None).await;
    db.insert_deployment(Some(app.id), None, "DONE", "DONE", 1, "", 0)
        .await;

    assert_protocol(
        services.force_kill_deployment(app.id).await,
        "application has no running deployment",
    );
}

#[tokio::test]
async fn force_kill_running_deployment_not_owned_by_process() {
    let _guard = state_guard().await;
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Kill", ENV_ID, None).await;
    let deployment_id = db
        .insert_deployment(Some(app.id), None, "RUNNING", "RUNNING", 1, "", USER_ID)
        .await;
    let state = auto_di::resolve::<ApplicationState>().await.unwrap();
    state.remove_state(IdType::AppId(app.id));

    let result = services.force_kill_deployment(app.id).await.unwrap();
    assert_eq!(result.deployment_id, deployment_id);
    assert!(matches!(
        result.result,
        CancelDeploymentResult::NotActiveInThisProcess
    ));
    let status: String = sqlx::query_scalar("SELECT status FROM deployments WHERE id = ?")
        .bind(deployment_id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(status, "RUNNING");
    state.remove_state(IdType::AppId(app.id));
}

#[tokio::test]
async fn patch_preview_config_validates_wildcard_and_persists() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Preview", ENV_ID, None).await;

    assert_protocol(
        services
            .patch_preview_config(
                app.id,
                UpdatePreviewConfigRequest {
                    id: app.id,
                    wildcard: None,
                    active: true,
                    path: "/".into(),
                    certificate_type: "NONE".into(),
                    ..Default::default()
                },
            )
            .await,
        "preview_wildcard is required",
    );
    assert!(!services.get_by_id(app.id).await.unwrap().previews_active);

    services
        .patch_preview_config(
            app.id,
            UpdatePreviewConfigRequest {
                id: app.id,
                preview_env: Some("PREVIEW=1".into()),
                build_args: Some("A=1".into()),
                build_secrets: Some("B=2".into()),
                labels: Some("team=platform".into()),
                wildcard: Some("*.preview.test".into()),
                port: Some(8080),
                https: true,
                path: "/app".into(),
                certificate_type: "LETSENCRYPT".into(),
                custom_cert_resolver: Some("resolver".into()),
                limit: 5,
                active: true,
                require_collaborator_permissions: false,
            },
        )
        .await
        .unwrap();

    let record = services.get_by_id(app.id).await.unwrap();
    assert_eq!(record.preview_env.as_deref(), Some("PREVIEW=1"));
    assert_eq!(record.preview_build_args.as_deref(), Some("A=1"));
    assert_eq!(record.preview_build_secrets.as_deref(), Some("B=2"));
    assert_eq!(record.preview_labels.as_deref(), Some("team=platform"));
    assert_eq!(record.preview_wildcard.as_deref(), Some("*.preview.test"));
    assert_eq!(record.preview_port, Some(8080));
    assert!(record.preview_https);
    assert_eq!(record.preview_path.as_deref(), Some("/app"));
    assert_eq!(record.preview_certificate_type, "LETSENCRYPT");
    assert_eq!(
        record.preview_custom_cert_resolver.as_deref(),
        Some("resolver")
    );
    assert_eq!(record.preview_limit, Some(5));
    assert!(record.previews_active);
    assert!(!record.preview_require_collaborator_permissions);
}

#[tokio::test]
async fn patch_build_config_updates_build_and_git_columns() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Build", ENV_ID, None).await;

    let record = services
        .patch_build_config(
            app.id,
            UpdateBuildConfigRequest {
                id: app.id,
                build_args: Some("ARG=1".into()),
                build_secrets: Some("SECRET=2".into()),
                dockerfile: Some("Dockerfile.dev".into()),
                context_path: Some(".".into()),
                build_stage: Some("builder".into()),
                publish_directory: Some("dist".into()),
                static_spa: Some(true),
                create_env_file: Some(false),
                command: Some("npm start".into()),
                args: Some("--host 0.0.0.0".into()),
                build_path: Some("/app".into()),
                clean_cache: Some(true),
                enable_submodules: Some(true),
                watch_paths: Some("src/**".into()),
                railpack_version: Some("1.2.3".into()),
                heroku_version: Some("22".into()),
                auto_deploy: Some(false),
            },
        )
        .await
        .unwrap();

    assert_eq!(record.dockerfile.as_deref(), Some("Dockerfile.dev"));
    assert_eq!(record.docker_context_path.as_deref(), Some("."));
    assert_eq!(record.docker_build_stage.as_deref(), Some("builder"));
    assert_eq!(record.publish_directory.as_deref(), Some("dist"));
    assert!(record.is_static_spa);
    assert!(record.clean_cache);
    assert_eq!(record.build_path.as_deref(), Some("/app"));
    assert_eq!(record.railpack_version.as_deref(), Some("1.2.3"));
    assert_eq!(record.heroku_version.as_deref(), Some("22"));
    assert_eq!(record.command.as_deref(), Some("npm start"));
    assert_eq!(record.args.as_deref(), Some("--host 0.0.0.0"));
    assert_eq!(record.watch_paths.as_deref(), Some("src/**"));
    assert!(record.enable_submodules);
    assert!(!record.auto_deploy);

    let counts: i64 =
        sqlx::query_scalar("SELECT create_env_file FROM service_application WHERE service_id = ?")
            .bind(app.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(counts, 1);
}

#[tokio::test]
async fn patch_resource_config_updates_columns() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Resources", ENV_ID, Some(SERVER_ID)).await;

    let record = services
        .patch_resource_config(
            app.id,
            UpdateResourceConfigRequest {
                id: app.id,
                memory_reservation: Some("128m".into()),
                memory_limit: Some("256m".into()),
                cpu_reservation: Some("0.25".into()),
                cpu_limit: Some("0.5".into()),
                replicas: Some(3),
            },
        )
        .await
        .unwrap();

    assert_eq!(record.memory_reservation.as_deref(), Some("128m"));
    assert_eq!(record.memory_limit.as_deref(), Some("256m"));
    assert_eq!(record.cpu_reservation.as_deref(), Some("0.25"));
    assert_eq!(record.cpu_limit.as_deref(), Some("0.5"));
    assert_eq!(record.replicas, Some(3));

    let cleared = services
        .patch_resource_config(
            app.id,
            UpdateResourceConfigRequest {
                id: app.id,
                replicas: None,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(cleared.memory_limit, None);
    assert_eq!(cleared.replicas, Some(3));
}

#[tokio::test]
async fn port_crud_is_scoped_to_applications() {
    let db = TestDb::new().await;
    let ports = db.port();
    let app = create_application(&db, "Ports", ENV_ID, None).await;
    let other = create_application(&db, "Other Ports", ENV_ID, None).await;

    assert!(ports.list(app.id).await.unwrap().is_empty());
    let created = ports
        .create(
            app.id,
            UpsertApplicationPortDto {
                published_port: 8080,
                target_port: 80,
                protocol: PortProtocol::Tcp,
                publish_mode: PortPublishMode::Ingress,
            },
        )
        .await
        .unwrap();
    assert_eq!(created.published_port, 8080);
    assert_eq!(created.target_port, 80);
    assert_eq!(created.protocol, "TCP");
    assert_eq!(created.publish_mode, "INGRESS");
    assert_eq!(created.service_id, app.id);
    assert_eq!(ports.list(other.id).await.unwrap().len(), 0);

    let updated = ports
        .update(
            app.id,
            created.id,
            UpsertApplicationPortDto {
                published_port: 9090,
                target_port: 90,
                protocol: PortProtocol::Udp,
                publish_mode: PortPublishMode::Host,
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.published_port, 9090);
    assert_eq!(updated.protocol, "UDP");
    assert_eq!(updated.publish_mode, "HOST");

    assert!(ports.delete(app.id, created.id).await.unwrap());
    assert!(!ports.delete(app.id, created.id).await.unwrap());

    let compose_id = db.insert_service("COMPOSE", "IDLE", ENV_ID, None).await;
    assert!(matches!(
        ports.list(compose_id).await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn mount_normalization_and_crud() {
    let db = TestDb::new().await;
    let mounts = db.mount();
    let app = create_application(&db, "Mounts", ENV_ID, None).await;

    let volume = mounts
        .create(
            app.id,
            UpsertApplicationMountDto {
                mount_type: MountType::Volume,
                host_path: Some("/ignored".into()),
                volume_name: Some("  data  ".into()),
                file_path: Some("/ignored".into()),
                content: Some("ignored".into()),
                mount_path: "/data".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(volume.mount_type, "VOLUME");
    assert_eq!(volume.volume_name.as_deref(), Some("data"));
    assert_eq!(volume.host_path, None);
    assert_eq!(volume.file_path, None);
    assert_eq!(volume.content, None);

    let bind = mounts
        .create(
            app.id,
            UpsertApplicationMountDto {
                mount_type: MountType::Bind,
                host_path: Some("  /var/lib/app  ".into()),
                volume_name: None,
                file_path: None,
                content: None,
                mount_path: "  /config  ".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(bind.host_path.as_deref(), Some("/var/lib/app"));
    assert_eq!(bind.mount_path, "/config");

    assert_protocol(
        mounts
            .create(
                app.id,
                UpsertApplicationMountDto {
                    mount_type: MountType::Bind,
                    host_path: Some("/var/lib/app".into()),
                    volume_name: None,
                    file_path: None,
                    content: None,
                    mount_path: "config".into(),
                },
            )
            .await,
        "mount_path must be an absolute container path",
    );
    assert_protocol(
        mounts
            .create(
                app.id,
                UpsertApplicationMountDto {
                    mount_type: MountType::Bind,
                    host_path: Some("relative".into()),
                    volume_name: None,
                    file_path: None,
                    content: None,
                    mount_path: "/config".into(),
                },
            )
            .await,
        "host_path must be absolute for BIND mounts",
    );
    assert_protocol(
        mounts
            .create(
                app.id,
                UpsertApplicationMountDto {
                    mount_type: MountType::File,
                    host_path: None,
                    volume_name: None,
                    file_path: Some("/app/config.yml".into()),
                    content: None,
                    mount_path: "/app/config.yml".into(),
                },
            )
            .await,
        "content is required for FILE mounts",
    );

    let file = mounts
        .update(
            app.id,
            bind.id,
            UpsertApplicationMountDto {
                mount_type: MountType::File,
                host_path: None,
                volume_name: None,
                file_path: Some("/app/config.yml".into()),
                content: Some("key: value".into()),
                mount_path: "/app/config.yml".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(file.mount_type, "FILE");
    assert_eq!(file.file_path.as_deref(), Some("/app/config.yml"));
    assert_eq!(mounts.list(app.id).await.unwrap().len(), 2);
    assert!(mounts.delete(app.id, file.id).await.unwrap());
    assert!(!mounts.delete(app.id, file.id).await.unwrap());
}

#[tokio::test]
async fn patch_normalization_and_crud() {
    let db = TestDb::new().await;
    let patches = db.patch();
    let app = create_application(&db, "Patches", ENV_ID, None).await;

    let created = patches
        .create(
            app.id,
            UpsertApplicationPatchDto {
                patch_type: ApplicationPatchType::Create,
                file_path: "  src\\config.yml  ".into(),
                enabled: true,
                content: "key: value".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(created.file_path, "src/config.yml");
    assert_eq!(created.content, "key: value\n");
    assert_eq!(created.enabled, 1);
    assert_eq!(created.patch_type, "CREATE");

    let deleted = patches
        .update(
            app.id,
            created.id,
            UpsertApplicationPatchDto {
                patch_type: ApplicationPatchType::Delete,
                file_path: "src/config.yml".into(),
                enabled: false,
                content: "ignored".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(deleted.patch_type, "DELETE");
    assert_eq!(deleted.content, "");
    assert_eq!(deleted.enabled, 0);

    assert_protocol(
        patches
            .create(
                app.id,
                UpsertApplicationPatchDto {
                    patch_type: ApplicationPatchType::Create,
                    file_path: "../../etc/passwd".into(),
                    enabled: true,
                    content: "x".into(),
                },
            )
            .await,
        "file_path must be a safe relative repository path",
    );
    assert_protocol(
        patches
            .create(
                app.id,
                UpsertApplicationPatchDto {
                    patch_type: ApplicationPatchType::Create,
                    file_path: "src/new.yml".into(),
                    enabled: true,
                    content: String::new(),
                },
            )
            .await,
        "content is required",
    );
    assert!(patches.delete(app.id, created.id).await.unwrap());
    assert!(!patches.delete(app.id, created.id).await.unwrap());
}

#[tokio::test]
async fn middleware_validation_and_crud() {
    let db = TestDb::new().await;
    let middlewares = db.middleware();
    let app = create_application(&db, "Middleware", ENV_ID, None).await;

    let headers = middlewares
        .create(
            app.id,
            UpsertApplicationMiddlewareDto {
                name: "  My Middleware  ".into(),
                middleware_type: ApplicationMiddlewareType::Headers,
                enabled: true,
                headers: Some(vec![
                    MiddlewareHeaderDto {
                        name: " x-test ".into(),
                        value: "1".into(),
                    },
                    MiddlewareHeaderDto {
                        name: "X-Frame-Options".into(),
                        value: "DENY".into(),
                    },
                ]),
                average: None,
                burst: None,
                source_ranges: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(headers.name, "my-middleware");
    assert_eq!(headers.middleware_type, "HEADERS");
    assert!(headers.enabled);
    assert_eq!(headers.headers.len(), 2);
    assert_eq!(headers.headers[0].name, "X-Frame-Options");
    assert_eq!(headers.headers[1].name, "x-test");

    assert_protocol(
        middlewares
            .create(
                app.id,
                UpsertApplicationMiddlewareDto {
                    name: "bad!name".into(),
                    middleware_type: ApplicationMiddlewareType::Compress,
                    enabled: true,
                    headers: None,
                    average: None,
                    burst: None,
                    source_ranges: None,
                },
            )
            .await,
        "middleware name may contain only letters, numbers and hyphens",
    );
    assert_protocol(
        middlewares
            .create(
                app.id,
                UpsertApplicationMiddlewareDto {
                    name: "headers".into(),
                    middleware_type: ApplicationMiddlewareType::Headers,
                    enabled: true,
                    headers: None,
                    average: None,
                    burst: None,
                    source_ranges: None,
                },
            )
            .await,
        "HEADERS middleware requires at least one header",
    );
    assert_protocol(
        middlewares
            .create(
                app.id,
                UpsertApplicationMiddlewareDto {
                    name: "rate".into(),
                    middleware_type: ApplicationMiddlewareType::RateLimit,
                    enabled: true,
                    headers: None,
                    average: Some(0),
                    burst: Some(1),
                    source_ranges: None,
                },
            )
            .await,
        "RATE_LIMIT average must be positive",
    );
    assert_protocol(
        middlewares
            .create(
                app.id,
                UpsertApplicationMiddlewareDto {
                    name: "allow".into(),
                    middleware_type: ApplicationMiddlewareType::IpAllowlist,
                    enabled: true,
                    headers: None,
                    average: None,
                    burst: None,
                    source_ranges: Some(vec!["10.0.0.0/33".into()]),
                },
            )
            .await,
        "invalid CIDR",
    );

    let rate = middlewares
        .create(
            app.id,
            UpsertApplicationMiddlewareDto {
                name: "rate".into(),
                middleware_type: ApplicationMiddlewareType::RateLimit,
                enabled: false,
                headers: None,
                average: Some(10),
                burst: Some(20),
                source_ranges: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(rate.average, Some(10));
    assert_eq!(rate.burst, Some(20));
    assert!(!rate.enabled);

    let allow = middlewares
        .update(
            app.id,
            rate.id,
            UpsertApplicationMiddlewareDto {
                name: "allow".into(),
                middleware_type: ApplicationMiddlewareType::IpAllowlist,
                enabled: true,
                headers: None,
                average: None,
                burst: None,
                source_ranges: Some(vec!["10.0.0.0/8".into(), "2001:db8::/32".into()]),
            },
        )
        .await
        .unwrap();
    assert_eq!(allow.middleware_type, "IP_ALLOWLIST");
    assert_eq!(allow.source_ranges.len(), 2);
    assert_eq!(middlewares.list(app.id).await.unwrap().len(), 2);
    assert!(middlewares.delete(app.id, allow.id).await.unwrap());
    assert!(!middlewares.delete(app.id, allow.id).await.unwrap());
}

#[tokio::test]
async fn redirect_validation_and_crud() {
    let db = TestDb::new().await;
    let redirects = db.redirect();
    let app = create_application(&db, "Redirects", ENV_ID, None).await;

    let created = redirects
        .create(
            app.id,
            UpsertApplicationRedirectDto {
                regex: "^/old$".into(),
                replacement: "/new".into(),
                permanent: 1,
                unique_config_key: Some(7),
            },
        )
        .await
        .unwrap();
    assert_eq!(created.permanent, 1);
    assert_eq!(created.unique_config_key, Some(7));
    assert_eq!(created.service_id, app.id);

    let updated = redirects
        .update(
            app.id,
            created.id,
            UpsertApplicationRedirectDto {
                regex: "^/legacy$".into(),
                replacement: "/modern".into(),
                permanent: 0,
                unique_config_key: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.regex, "^/legacy$");
    assert_eq!(updated.permanent, 0);
    assert_eq!(redirects.list(app.id).await.unwrap().len(), 1);

    assert_protocol(
        redirects
            .create(
                app.id,
                UpsertApplicationRedirectDto {
                    regex: "^/x$".into(),
                    replacement: "/y".into(),
                    permanent: 2,
                    unique_config_key: None,
                },
            )
            .await,
        "permanent must be 0 or 1",
    );
    assert!(redirects.delete(app.id, created.id).await.unwrap());
    assert!(!redirects.delete(app.id, created.id).await.unwrap());
}

#[tokio::test]
async fn security_hashes_password_and_crud() {
    let db = TestDb::new().await;
    let security = db.security();
    let app = create_application(&db, "Security", ENV_ID, None).await;

    let created = security
        .create(
            app.id,
            UpsertApplicationSecurityDto {
                username: "  admin  ".into(),
                password: "hunter2".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(created.username, "admin");
    assert_ne!(created.password, "hunter2");
    assert!(bcrypt::verify("hunter2", &created.password).unwrap());

    let listed = security.list(app.id).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert!(!listed[0].password.is_empty());

    let updated = security
        .update(
            app.id,
            created.id,
            UpsertApplicationSecurityDto {
                username: "operator".into(),
                password: "new-password".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.username, "operator");
    assert!(bcrypt::verify("new-password", &updated.password).unwrap());

    assert_protocol(
        security
            .create(
                app.id,
                UpsertApplicationSecurityDto {
                    username: "bad:user".into(),
                    password: "x".into(),
                },
            )
            .await,
        "username cannot be empty or contain ':' or ','",
    );
    assert_protocol(
        security
            .create(
                app.id,
                UpsertApplicationSecurityDto {
                    username: String::new(),
                    password: "x".into(),
                },
            )
            .await,
        "username cannot be empty or contain ':' or ','",
    );
    assert!(security.delete(app.id, created.id).await.unwrap());
    assert!(!security.delete(app.id, created.id).await.unwrap());
}

#[tokio::test]
async fn network_state_parsing_and_update_validation() {
    let db = TestDb::new().await;
    let networks = db.application_network();
    let services = db.application();
    let app = create_application(&db, "Networks", ENV_ID, Some(SERVER_ID)).await;

    let (detach, resolved) = networks.get(app.id).await.unwrap();
    assert!(!detach);
    assert!(resolved.is_empty());

    let (detach, resolved) = networks
        .update(
            app.id,
            UpdateApplicationNetworksDto {
                network_ids: vec![2, 1, 2],
                detach_rustploy_network: false,
            },
        )
        .await
        .unwrap();
    assert!(!detach);
    assert_eq!(
        resolved.iter().map(|n| n.id).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(
        services.get_by_id(app.id).await.unwrap().network_ids,
        r#"["1","2"]"#
    );

    let (detach, resolved) = networks
        .update(
            app.id,
            UpdateApplicationNetworksDto {
                network_ids: vec![1],
                detach_rustploy_network: true,
            },
        )
        .await
        .unwrap();
    assert!(detach);
    assert_eq!(resolved.len(), 1);
    assert_eq!(
        services.get_by_id(app.id).await.unwrap().network_ids,
        r#"["1"]"#
    );
}

#[tokio::test]
async fn network_update_rejects_foreign_and_missing_networks() {
    let db = TestDb::new().await;
    let networks = db.application_network();
    let app = create_application(&db, "Bad Networks", ENV_ID, Some(SERVER_ID)).await;

    assert_protocol(
        networks
            .update(
                app.id,
                UpdateApplicationNetworksDto {
                    network_ids: vec![OTHER_SERVER_NETWORK_ID],
                    detach_rustploy_network: false,
                },
            )
            .await,
        "belongs to a different server",
    );
    assert_protocol(
        networks
            .update(
                app.id,
                UpdateApplicationNetworksDto {
                    network_ids: vec![999],
                    detach_rustploy_network: false,
                },
            )
            .await,
        "network 999 not found",
    );
    assert_protocol(
        networks
            .update(
                app.id,
                UpdateApplicationNetworksDto {
                    network_ids: Vec::new(),
                    detach_rustploy_network: true,
                },
            )
            .await,
        "at least one network is required when the OpenOxide network is detached",
    );

    sqlx::query("UPDATE services SET network_ids = 'not-json' WHERE id = ?")
        .bind(app.id)
        .execute(&*db.pool)
        .await
        .unwrap();
    assert_protocol(
        networks.get(app.id).await,
        "invalid application network state",
    );
    sqlx::query(r#"UPDATE services SET network_ids = '["abc"]' WHERE id = ?"#)
        .bind(app.id)
        .execute(&*db.pool)
        .await
        .unwrap();
    assert_protocol(
        networks.get(app.id).await,
        "application network state contains a non-numeric id",
    );
}

#[tokio::test]
async fn source_switching_between_providers() {
    let db = TestDb::new().await;
    let services = db.application();
    let app = create_application(&db, "Sources", ENV_ID, None).await;
    services
        .patch(
            app.id,
            UpdateApplicationRequest {
                id: app.id,
                trigger_type: Some("TAG".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let github = services
        .set_github_source(
            app.id,
            SetGithubSourceRequest {
                id: app.id,
                repository: "web".into(),
                owner: "acme".into(),
                branch: Some("dev".into()),
                build_path: Some("/app".into()),
                provider_id: Some(GIT_PROVIDER_ID),
                auto_deploy: Some(false),
            },
        )
        .await
        .unwrap();
    assert_eq!(github.source_type, "GITHUB");
    assert_eq!(github.repository.as_deref(), Some("web"));
    assert_eq!(github.owner.as_deref(), Some("acme"));
    assert_eq!(github.branch.as_deref(), Some("dev"));
    assert_eq!(github.build_path.as_deref(), Some("/app"));
    assert_eq!(github.trigger_type, "TAG");
    assert!(!github.auto_deploy);

    let gitlab = services
        .set_gitlab_source(
            app.id,
            SetGitlabSourceRequest {
                id: app.id,
                repository: "gitlab-repo".into(),
                owner: "gitlab-owner".into(),
                branch: Some("main".into()),
                build_path: Some("/src".into()),
                path_namespace: Some("group".into()),
                provider_id: Some(GIT_PROVIDER_ID),
                project_id: Some(42),
            },
        )
        .await
        .unwrap();
    assert_eq!(gitlab.source_type, "GITLAB");
    assert_eq!(gitlab.repository, None);
    assert_eq!(gitlab.gitlab_repository.as_deref(), Some("gitlab-repo"));
    assert_eq!(gitlab.gitlab_owner.as_deref(), Some("gitlab-owner"));
    assert_eq!(gitlab.gitlab_branch.as_deref(), Some("main"));

    let gitea = services
        .set_gitea_source(
            app.id,
            SetGiteaSourceRequest {
                id: app.id,
                repository: "gitea-repo".into(),
                owner: "gitea-owner".into(),
                branch: Some("dev".into()),
                build_path: None,
                provider_id: Some(GIT_PROVIDER_ID),
            },
        )
        .await
        .unwrap();
    assert_eq!(gitea.source_type, "GITEA");
    assert_eq!(gitea.gitea_repository.as_deref(), Some("gitea-repo"));

    let bitbucket = services
        .set_bitbucket_source(
            app.id,
            SetBitbucketSourceRequest {
                id: app.id,
                repository: "bb-repo".into(),
                owner: "bb-owner".into(),
                branch: Some("main".into()),
                build_path: None,
                repository_slug: Some("bb-slug".into()),
                provider_id: Some(GIT_PROVIDER_ID),
            },
        )
        .await
        .unwrap();
    assert_eq!(bitbucket.source_type, "BITBUCKET");
    assert_eq!(bitbucket.bitbucket_repository.as_deref(), Some("bb-repo"));

    let custom = services
        .set_custom_git_source(
            app.id,
            SetCustomGitSourceRequest {
                id: app.id,
                url: "ssh://git.example.com/repo.git".into(),
                branch: Some("main".into()),
                build_path: None,
                ssh_key_id: Some(SSH_KEY_ID),
            },
        )
        .await
        .unwrap();
    assert_eq!(custom.source_type, "CUSTOM");
    assert_eq!(
        custom.custom_git_url.as_deref(),
        Some("ssh://git.example.com/repo.git")
    );

    let docker = services
        .set_docker_source(
            app.id,
            SetDockerSourceRequest {
                id: app.id,
                image: "nginx:latest".into(),
                username: Some("user".into()),
                password: Some("pass".into()),
                registry_url: Some("registry.test".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(docker.source_type, "DOCKER");
    assert_eq!(docker.docker_image.as_deref(), Some("nginx:latest"));
    assert_eq!(docker.registry_url.as_deref(), Some("registry.test"));
    assert_eq!(docker.repository, None);

    let drop = services
        .set_drop_source(
            app.id,
            SetDropSourceRequest {
                id: app.id,
                build_path: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(drop.source_type, "DROP");
    assert_eq!(drop.drop_build_path.as_deref(), Some("/"));
    assert_eq!(drop.docker_image, None);

    assert!(matches!(
        services
            .set_drop_source(
                999,
                SetDropSourceRequest {
                    id: 999,
                    build_path: None,
                },
            )
            .await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn export_redacts_secrets_and_import_restores_relations() {
    let db = TestDb::new().await;
    let services = db.application();
    let transfer = db.application_transfer();
    let app = create_application(&db, "Transferable", ENV_ID, Some(SERVER_ID)).await;
    services
        .patch(
            app.id,
            UpdateApplicationRequest {
                id: app.id,
                env_var: Some("SECRET=env".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    services
        .patch_build_config(
            app.id,
            UpdateBuildConfigRequest {
                id: app.id,
                build_secrets: Some("TOKEN=abc".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    db.port()
        .create(
            app.id,
            UpsertApplicationPortDto {
                published_port: 8080,
                target_port: 80,
                protocol: PortProtocol::Tcp,
                publish_mode: PortPublishMode::Ingress,
            },
        )
        .await
        .unwrap();
    db.mount()
        .create(
            app.id,
            UpsertApplicationMountDto {
                mount_type: MountType::Volume,
                host_path: None,
                volume_name: Some("data".into()),
                file_path: None,
                content: None,
                mount_path: "/data".into(),
            },
        )
        .await
        .unwrap();
    db.patch()
        .create(
            app.id,
            UpsertApplicationPatchDto {
                patch_type: ApplicationPatchType::Create,
                file_path: "src/config.yml".into(),
                enabled: true,
                content: "a: b".into(),
            },
        )
        .await
        .unwrap();
    db.middleware()
        .create(
            app.id,
            UpsertApplicationMiddlewareDto {
                name: "compress".into(),
                middleware_type: ApplicationMiddlewareType::Compress,
                enabled: true,
                headers: None,
                average: None,
                burst: None,
                source_ranges: None,
            },
        )
        .await
        .unwrap();
    db.redirect()
        .create(
            app.id,
            UpsertApplicationRedirectDto {
                regex: "^/a$".into(),
                replacement: "/b".into(),
                permanent: 1,
                unique_config_key: Some(3),
            },
        )
        .await
        .unwrap();
    db.security()
        .create(
            app.id,
            UpsertApplicationSecurityDto {
                username: "admin".into(),
                password: "hunter2".into(),
            },
        )
        .await
        .unwrap();

    let redacted = transfer.export(app.id, false).await.unwrap();
    assert!(!redacted.secrets_included);
    assert_eq!(redacted.application.env_var, None);
    assert_eq!(redacted.application.refresh_token, None);
    assert_eq!(redacted.service_application.build_secrets, None);
    assert!(
        redacted
            .security
            .iter()
            .all(|item| item.password.is_empty())
    );

    let full = transfer.export(app.id, true).await.unwrap();
    assert!(full.secrets_included);
    assert_eq!(full.application.env_var.as_deref(), Some("SECRET=env"));
    assert_eq!(
        full.service_application.build_secrets.as_deref(),
        Some("TOKEN=abc")
    );
    assert!(!full.security[0].password.is_empty());

    let imported = transfer
        .import(ImportApplicationDto {
            target_environment_id: STAGING_ENV_ID,
            target_server_id: Some(SERVER_ID_2),
            name: Some("Imported".into()),
            archive: serde_json::to_string(&full).unwrap(),
        })
        .await
        .unwrap();
    assert_eq!(imported.name, "Imported");
    assert_ne!(imported.app_name, app.app_name);
    assert_eq!(imported.environment_id, STAGING_ENV_ID);
    assert_eq!(imported.server_id, Some(SERVER_ID_2));
    assert_eq!(imported.app_status, "IDLE");
    assert_eq!(imported.env_var.as_deref(), Some("SECRET=env"));

    let counts = services.dependencies(imported.id).await.unwrap();
    assert_eq!(counts.ports, 1);
    assert_eq!(counts.mounts, 1);
    assert_eq!(counts.patches, 1);
    assert_eq!(counts.middleware, 1);
    assert_eq!(counts.redirects, 1);
    assert_eq!(counts.security_rules, 1);

    let redacted_import = transfer
        .import(ImportApplicationDto {
            target_environment_id: ENV_ID,
            target_server_id: None,
            name: None,
            archive: serde_json::to_string(&redacted).unwrap(),
        })
        .await
        .unwrap();
    let counts = services.dependencies(redacted_import.id).await.unwrap();
    assert_eq!(counts.security_rules, 0);
}

#[tokio::test]
async fn import_rejects_invalid_archives_and_targets() {
    let db = TestDb::new().await;
    let transfer = db.application_transfer();
    let app = create_application(&db, "Source App", ENV_ID, None).await;
    let bundle = transfer.export(app.id, true).await.unwrap();

    assert_protocol(
        transfer
            .import(ImportApplicationDto {
                target_environment_id: ENV_ID,
                target_server_id: None,
                name: None,
                archive: "not json".into(),
            })
            .await,
        "invalid application archive",
    );
    assert_protocol(
        transfer
            .import(ImportApplicationDto {
                target_environment_id: 999,
                target_server_id: None,
                name: None,
                archive: serde_json::to_string(&bundle).unwrap(),
            })
            .await,
        "target environment not found",
    );

    let mut wrong_schema = bundle.clone();
    wrong_schema.schema_version = 2;
    assert_protocol(
        transfer
            .import(ImportApplicationDto {
                target_environment_id: ENV_ID,
                target_server_id: None,
                name: None,
                archive: serde_json::to_string(&wrong_schema).unwrap(),
            })
            .await,
        "unsupported application export schema version 2",
    );
}
