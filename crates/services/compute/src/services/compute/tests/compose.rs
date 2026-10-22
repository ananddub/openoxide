use super::assert_protocol;
use crate::api::connect::generated::openoxide::api::v1::ComposeMountRequest;
use crate::services::compute::compose::crud::{
    ComposeResponseDto, ComposeServiceNetworkDto, CreateComposeDto, PatchComposeDto,
};
use crate::services::compute::compose::management::{
    ComposePreviewDto, ComposeResourceKind, DeleteComposeResourceDto, ImportComposeDto,
    InstallComposeTemplateDto, UpsertComposePatchDto, UpsertComposeResourceDto,
};
use crate::services::compute::compose::source::{
    PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto, PatchComposeGiteaSourceDto,
    PatchComposeGithubSourceDto, PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};
use crate::services::compute::compose::{ComposeOperation, ComposeType};
use crate::services::compute::test_support::*;
use crate::utils::builder::custom_type::IdType;
use crate::utils::builder::hash_state::ApplicationState;

const COMPOSE_FILE: &str = "\
services:
  app:
    image: nginx:alpine
  worker:
    image: busybox
    depends_on: [app]
";

fn create_dto(
    name: &str,
    environment_id: i64,
    server_id: Option<i64>,
    source_type: &str,
    compose_file: &str,
) -> CreateComposeDto {
    CreateComposeDto {
        name: name.into(),
        description: None,
        environment_id,
        server_id,
        source_type: source_type.into(),
        compose_type: ComposeType::DockerCompose,
        compose_file: compose_file.into(),
    }
}

fn patch_dto() -> PatchComposeDto {
    PatchComposeDto {
        name: None,
        description: None,
        env_var: None,
        compose_file: None,
        compose_type: None,
        trigger_type: None,
        command: None,
        enable_submodules: None,
        compose_path: None,
        suffix: None,
        randomize: None,
        isolated_deployment: None,
        isolated_deployments_volume: None,
        watch_paths: None,
        service_networks: None,
        server_id: None,
    }
}

fn patch_dto_json(
    patch_type: &str,
    file_path: &str,
    enabled: bool,
    content: &str,
) -> UpsertComposePatchDto {
    serde_json::from_value(serde_json::json!({
        "patch_type": patch_type,
        "file_path": file_path,
        "enabled": enabled,
        "content": content,
    }))
    .unwrap()
}

fn compose_mount(mount_type: &str) -> ComposeMountRequest {
    ComposeMountRequest {
        mount_type: mount_type.into(),
        mount_path: "/data".into(),
        ..Default::default()
    }
}

#[tokio::test]
async fn create_raw_source_defaults_and_lists_by_environment() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto(
            "My Stack",
            ENV_ID,
            Some(SERVER_ID),
            "RAW",
            COMPOSE_FILE,
        ))
        .await
        .unwrap();

    assert_eq!(record.name, "My Stack");
    assert!(
        record.app_name.starts_with("my-stack-"),
        "{}",
        record.app_name
    );
    assert_eq!(record.description, None);
    assert_eq!(record.source_type, "RAW");
    assert_eq!(record.compose_type, ComposeType::DockerCompose);
    assert_eq!(record.compose_status, "IDLE");
    assert_eq!(record.compose_file, COMPOSE_FILE);
    assert_eq!(record.compose_path, "");
    assert_eq!(record.suffix, None);
    assert!(!record.randomize);
    assert_eq!(record.service_networks, "[]");
    assert_eq!(record.trigger_type, "PUSH");
    assert_eq!(record.environment_id, ENV_ID);
    assert_eq!(record.server_id, Some(SERVER_ID));

    let listed = service.list_by_environment(ENV_ID).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, record.id);
    assert!(
        service
            .list_by_environment(STAGING_ENV_ID)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn create_git_source_inserts_git_row_with_auto_deploy_off() {
    let db = TestDb::new().await;
    let record = db
        .compose()
        .create(create_dto(
            "Git Stack",
            ENV_ID,
            None,
            "GITHUB",
            COMPOSE_FILE,
        ))
        .await
        .unwrap();

    assert_eq!(record.source_type, "GITHUB");
    assert_eq!(record.server_id, None);
    let git: (String, i64, Option<String>) = sqlx::query_as(
        "SELECT source_type, auto_deploy, repository FROM service_git WHERE service_id = ?",
    )
    .bind(record.id)
    .fetch_one(&*db.pool)
    .await
    .unwrap();
    assert_eq!(git.0, "GITHUB");
    assert_eq!(git.1, 0);
    assert_eq!(git.2, None);
}

#[tokio::test]
async fn stored_unknown_compose_type_falls_back_to_docker_compose() {
    let db = TestDb::new().await;
    let id = db.insert_service("COMPOSE", "IDLE", ENV_ID, None).await;
    sqlx::query(
        "INSERT INTO service_compose (service_id, compose_type, compose_file, compose_path, \
             suffix, randomize, isolated_deployment, isolated_deployments_volume, service_networks) \
         VALUES (?, 'BOGUS', 'services: {}', '', '', 0, 0, 0, '[]')",
    )
    .bind(id)
    .execute(&*db.pool)
    .await
    .unwrap();

    let record = db.compose().get_by_id(id).await.unwrap();
    assert_eq!(record.compose_type, ComposeType::DockerCompose);
    assert_eq!(record.source_type, "RAW");
}

#[tokio::test]
async fn get_by_id_rejects_missing_and_non_compose_services() {
    let db = TestDb::new().await;
    let service = db.compose();
    assert!(matches!(
        service.get_by_id(999).await,
        Err(sqlx::Error::Protocol(_))
    ));

    let application_id = db.insert_service("APPLICATION", "IDLE", ENV_ID, None).await;
    assert!(matches!(
        service.get_by_id(application_id).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn list_by_environment_skips_services_without_compose_rows() {
    let db = TestDb::new().await;
    let service = db.compose();
    let orphan = db.insert_service("COMPOSE", "IDLE", ENV_ID, None).await;
    let valid = service
        .create(create_dto("Valid", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let listed = service.list_by_environment(ENV_ID).await.unwrap();
    let ids: Vec<i64> = listed.iter().map(|record| record.id).collect();
    assert_eq!(ids, vec![valid.id]);
    assert!(!ids.contains(&orphan));
}

#[tokio::test]
async fn compose_response_parses_services_and_networks() {
    let db = TestDb::new().await;
    let record = db
        .compose()
        .create(create_dto("Response", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let response = ComposeResponseDto::from(record);
    let mut names = response.services.clone();
    names.sort();
    assert_eq!(names, vec!["app".to_string(), "worker".to_string()]);
    assert_eq!(response.compose_type, "DOCKER-COMPOSE");
    assert!(response.service_networks.is_empty());

    assert_eq!(
        crate::services::compute::compose::crud::detect_compose_service_names(
            &db.compose().get_by_id(response.id).await.unwrap()
        ),
        vec!["app".to_string(), "worker".to_string()]
    );
}

#[tokio::test]
async fn patch_updates_identity_and_config_fields() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto(
            "Before",
            ENV_ID,
            Some(SERVER_ID),
            "GITHUB",
            COMPOSE_FILE,
        ))
        .await
        .unwrap();

    let mut input = patch_dto();
    input.name = Some("After".into());
    input.description = Some("updated".into());
    input.env_var = Some("KEY=value".into());
    input.compose_file = Some("services:\n  api:\n    image: httpd\n".into());
    input.compose_type = Some(ComposeType::Stack);
    input.command = Some("docker stack deploy".into());
    input.compose_path = Some("./deploy/compose.yml".into());
    input.server_id = Some(SERVER_ID_2);

    let patched = service.patch(record.id, input).await.unwrap();
    assert_eq!(patched.name, "After");
    assert_eq!(patched.app_name, record.app_name);
    assert_eq!(patched.description.as_deref(), Some("updated"));
    assert_eq!(patched.env_var.as_deref(), Some("KEY=value"));
    assert!(patched.compose_file.contains("httpd"));
    assert_eq!(patched.compose_type, ComposeType::Stack);
    assert_eq!(patched.command, "docker stack deploy");
    assert_eq!(patched.compose_path, "./deploy/compose.yml");
    assert_eq!(patched.server_id, Some(SERVER_ID_2));
    assert_eq!(patched.trigger_type, "PUSH");
}

#[tokio::test]
async fn patch_keeps_omitted_fields_and_rejects_missing_compose() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Stable", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    service
        .patch(
            record.id,
            PatchComposeDto {
                env_var: Some("A=1".into()),
                ..patch_dto()
            },
        )
        .await
        .unwrap();

    let patched = service
        .patch(
            record.id,
            PatchComposeDto {
                command: Some("custom".into()),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    assert_eq!(patched.name, "Stable");
    assert_eq!(patched.env_var.as_deref(), Some("A=1"));
    assert_eq!(patched.compose_file, COMPOSE_FILE);
    assert_eq!(patched.command, "custom");

    assert!(matches!(
        service.patch(999, patch_dto()).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn patch_suffix_randomize_and_isolation_semantics() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Suffixes", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let randomized = service
        .patch(
            record.id,
            PatchComposeDto {
                randomize: Some(1),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    assert!(randomized.randomize);
    let generated = randomized.suffix.clone().expect("generated suffix");
    assert_eq!(generated.len(), 8);
    assert!(generated.chars().all(|c| c.is_ascii_hexdigit()));

    let named = service
        .patch(
            record.id,
            PatchComposeDto {
                suffix: Some("blue".into()),
                randomize: Some(1),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    assert_eq!(named.suffix.as_deref(), Some("blue"));

    let disabled = service
        .patch(
            record.id,
            PatchComposeDto {
                randomize: Some(0),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    assert!(!disabled.randomize);
    assert_eq!(disabled.suffix, None);

    service
        .patch(
            record.id,
            PatchComposeDto {
                isolated_deployment: Some(1),
                isolated_deployments_volume: Some(1),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    let flags: (i64, i64) = sqlx::query_as(
        "SELECT isolated_deployment, isolated_deployments_volume FROM service_compose \
         WHERE service_id = ?",
    )
    .bind(record.id)
    .fetch_one(&*db.pool)
    .await
    .unwrap();
    assert_eq!(flags, (1, 1));
}

#[tokio::test]
async fn patch_serializes_service_networks() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Networks", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let patched = service
        .patch(
            record.id,
            PatchComposeDto {
                service_networks: Some(vec![
                    ComposeServiceNetworkDto {
                        service_name: "app".into(),
                        network_ids: vec!["1".into(), "2".into()],
                        detach_rustploy_network: 1,
                    },
                    ComposeServiceNetworkDto {
                        service_name: "worker".into(),
                        network_ids: vec![],
                        detach_rustploy_network: 0,
                    },
                ]),
                ..patch_dto()
            },
        )
        .await
        .unwrap();

    let parsed: Vec<ComposeServiceNetworkDto> =
        serde_json::from_str(&patched.service_networks).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].service_name, "app");
    assert_eq!(parsed[0].network_ids, vec!["1", "2"]);
    assert_eq!(parsed[0].detach_rustploy_network, 1);
    assert_eq!(parsed[1].service_name, "worker");
}

#[tokio::test]
async fn patch_updates_git_trigger_watch_paths_and_submodules() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto(
            "Git Controls",
            ENV_ID,
            None,
            "GITHUB",
            COMPOSE_FILE,
        ))
        .await
        .unwrap();

    let patched = service
        .patch(
            record.id,
            PatchComposeDto {
                trigger_type: Some("TAG".into()),
                watch_paths: Some("apps/**,charts/**".into()),
                enable_submodules: Some(1),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    assert_eq!(patched.trigger_type, "TAG");

    let git: (String, String, i64, i64) = sqlx::query_as(
        "SELECT trigger_type, watch_paths, enable_submodules, auto_deploy FROM service_git \
         WHERE service_id = ?",
    )
    .bind(record.id)
    .fetch_one(&*db.pool)
    .await
    .unwrap();
    assert_eq!(git.0, "TAG");
    assert_eq!(git.1, "apps/**,charts/**");
    assert_eq!(git.2, 1);
    assert_eq!(git.3, 0);
}

#[tokio::test]
async fn delete_is_blocked_by_active_deployments() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Busy", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "RUNNING", "RUNNING", 1, "", USER_ID)
        .await;

    assert_protocol(service.delete(record.id).await, "active dependencies");
    assert!(service.get_by_id(record.id).await.is_ok());
}

#[tokio::test]
async fn delete_removes_compose_and_cascades_children() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Doomed", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.compose_mount()
        .create(record.id, {
            let mut input = compose_mount("VOLUME");
            input.volume_name = Some("data".into());
            input
        })
        .await
        .unwrap();
    db.compose_patch()
        .create(record.id, patch_dto_json("UPDATE", "app.conf", true, "x=1"))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "DONE", "DONE", 1, "", 0)
        .await;

    service.delete(record.id).await.unwrap();

    let services: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM services WHERE id = ?")
        .bind(record.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    let compose_rows: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM service_compose WHERE service_id = ?")
            .bind(record.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    let mounts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM mounts WHERE service_id = ?")
        .bind(record.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    let patches: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patches WHERE service_id = ?")
        .bind(record.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    let deployments: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM deployments WHERE service_id = ?")
            .bind(record.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(
        (services, compose_rows, mounts, patches, deployments),
        (0, 0, 0, 0, 0)
    );
}

#[tokio::test]
async fn dependencies_count_relations_and_backups() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Counted", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.compose_mount()
        .create(record.id, {
            let mut input = compose_mount("VOLUME");
            input.volume_name = Some("data".into());
            input
        })
        .await
        .unwrap();
    db.compose_patch()
        .create(record.id, patch_dto_json("UPDATE", "app.conf", true, "x=1"))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "QUEUED", "QUEUED", 1, "", 0)
        .await;
    db.insert_volume_backup(record.id, true).await;

    let counts = service.dependencies(record.id).await.unwrap();
    assert_eq!(counts.active_deployments, 1);
    assert_eq!(counts.deployments, 1);
    assert_eq!(counts.mounts, 1);
    assert_eq!(counts.patches, 1);
    assert_eq!(counts.backups, 1);
    assert_eq!(counts.enabled_backups, 1);
    assert!(counts.blocks_delete());

    assert_protocol(service.delete(record.id).await, "enabled_backups=1");
}

#[tokio::test]
async fn preview_transforms_randomized_and_isolated_compose() {
    let db = TestDb::new().await;
    let management = db.compose_management();

    let transformed = management
        .preview(ComposePreviewDto {
            compose_file: COMPOSE_FILE.into(),
            randomize: true,
            suffix: Some("blue".into()),
            isolated_deployment: true,
            isolated_deployments_volume: true,
            app_name: Some("proj".into()),
        })
        .unwrap();
    assert!(transformed.compose_file.contains("app-blue:"));
    assert!(transformed.compose_file.contains("worker-blue"));
    assert!(transformed.compose_file.contains("name: proj"));
    let mut services = transformed.services.clone();
    services.sort();
    assert_eq!(services, vec!["app-blue", "worker-blue"]);

    let generated = management
        .preview(ComposePreviewDto {
            compose_file: COMPOSE_FILE.into(),
            randomize: true,
            suffix: Some("  ".into()),
            isolated_deployment: false,
            isolated_deployments_volume: false,
            app_name: None,
        })
        .unwrap();
    let generated_suffix = generated
        .compose_file
        .lines()
        .find_map(|line| line.strip_prefix("  app-"))
        .and_then(|rest| rest.split(':').next())
        .expect("randomized service name");
    assert_eq!(generated_suffix.len(), 8);

    let fallback_network = management
        .preview(ComposePreviewDto {
            compose_file: COMPOSE_FILE.into(),
            randomize: false,
            suffix: None,
            isolated_deployment: true,
            isolated_deployments_volume: false,
            app_name: Some(" ".into()),
        })
        .unwrap();
    assert!(
        fallback_network
            .compose_file
            .contains("name: compose-preview")
    );
}

#[tokio::test]
async fn preview_rejects_invalid_compose_files() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    assert_protocol(
        management.preview(ComposePreviewDto {
            compose_file: "[]".into(),
            randomize: true,
            suffix: Some("x".into()),
            isolated_deployment: false,
            isolated_deployments_volume: false,
            app_name: None,
        }),
        "compose root must be an object",
    );
    assert!(
        management
            .preview(ComposePreviewDto {
                compose_file: "not: [valid".into(),
                randomize: false,
                suffix: None,
                isolated_deployment: false,
                isolated_deployments_volume: false,
                app_name: None,
            })
            .is_err()
    );
}

#[tokio::test]
async fn install_template_resolves_env_domains_and_mounts() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let template = r#"
[variables]
main_domain = "${domain}"
db_password = "${password:16}"
[config.env]
DOMAIN = "${main_domain}"
DB_PASSWORD = "${db_password}"
[[config.domains]]
serviceName = "app"
port = 8_080
host = "${main_domain}"
path = "/"
[[config.mounts]]
filePath = "/config/db.sql"
content = "SELECT 1;"
"#;

    let record = management
        .install_template(InstallComposeTemplateDto {
            name: "Template Stack".into(),
            description: Some("from template".into()),
            environment_id: ENV_ID,
            server_id: Some(SERVER_ID),
            compose_file: COMPOSE_FILE.into(),
            env_var: Some(template.into()),
        })
        .await
        .unwrap();

    assert_eq!(record.source_type, "RAW");
    assert_eq!(record.description.as_deref(), Some("from template"));
    let env = record.env_var.clone().expect("resolved env");
    assert!(env.contains("DOMAIN="));
    let password = env
        .lines()
        .find_map(|line| line.strip_prefix("DB_PASSWORD="))
        .expect("DB_PASSWORD line");
    assert_eq!(password.len(), 16);
    assert!(!env.contains("${"));

    let domains = db
        .queries
        .domains_list_by_compose_raw(record.id)
        .await
        .unwrap();
    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0].service_name.as_deref(), Some("app"));
    assert_eq!(domains[0].port, Some(8080));
    assert_eq!(domains[0].path.as_deref(), Some("/"));
    assert_eq!(domains[0].domain_type, "compose");
    assert_eq!(domains[0].organization_id, ORG_ID);
    assert!(
        domains[0].host.ends_with("-10-0-0-10.sslip.io"),
        "host={}",
        domains[0].host
    );

    let mounts = db.queries.mount_fetch_for_compose(record.id).await.unwrap();
    assert_eq!(mounts.len(), 1);
    assert_eq!(mounts[0].mount_type, "FILE");
    assert_eq!(mounts[0].file_path.as_deref(), Some("/config/db.sql"));
    assert_eq!(mounts[0].content.as_deref(), Some("SELECT 1;"));
}

#[tokio::test]
async fn install_template_expands_plain_env_blocks() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = management
        .install_template(InstallComposeTemplateDto {
            name: "Bare Env".into(),
            description: None,
            environment_id: ENV_ID,
            server_id: None,
            compose_file: COMPOSE_FILE.into(),
            env_var: Some("APP_KEY=${password:24}\nRAW=${UNKNOWN_REF}".into()),
        })
        .await
        .unwrap();

    let env = record.env_var.clone().expect("env resolved");
    let app_key = env
        .lines()
        .find_map(|line| line.strip_prefix("APP_KEY="))
        .unwrap();
    assert_eq!(app_key.len(), 24);
    assert!(env.contains("RAW=${UNKNOWN_REF}"));

    let untouched = management
        .install_template(InstallComposeTemplateDto {
            name: "No Env".into(),
            description: None,
            environment_id: ENV_ID,
            server_id: None,
            compose_file: COMPOSE_FILE.into(),
            env_var: None,
        })
        .await
        .unwrap();
    assert_eq!(untouched.env_var, None);
}

#[tokio::test]
async fn remove_service_requires_raw_source_and_cleans_references() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let raw = db
        .compose()
        .create(create_dto("Raw", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let record = management.remove_service(raw.id, "worker").await.unwrap();
    assert!(!record.compose_file.contains("worker"));
    let value: serde_yaml::Value = serde_yaml::from_str(&record.compose_file).unwrap();
    let services = value["services"].as_mapping().unwrap();
    assert!(services.contains_key(serde_yaml::Value::String("app".into())));
    assert_eq!(
        services[serde_yaml::Value::String("app".into())]["depends_on"],
        serde_yaml::Value::Null.clone()
    );
    assert_protocol(
        management.remove_service(raw.id, "missing").await,
        "not found",
    );

    let git = db
        .compose()
        .create(create_dto("Git", ENV_ID, None, "GITHUB", COMPOSE_FILE))
        .await
        .unwrap();
    assert_protocol(
        management.remove_service(git.id, "worker").await,
        "requires a RAW compose source",
    );
}

#[tokio::test]
async fn upsert_and_remove_resource_requires_raw_source() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let raw = db
        .compose()
        .create(create_dto("Resources", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let upserted = management
        .upsert_resource(
            raw.id,
            UpsertComposeResourceDto {
                kind: ComposeResourceKind::Config,
                name: "app-config".into(),
                file: Some("./app.conf".into()),
                external: false,
                services: vec!["app".into()],
            },
        )
        .await
        .unwrap();
    assert!(upserted.compose_file.contains("app-config:"));
    let value: serde_yaml::Value = serde_yaml::from_str(&upserted.compose_file).unwrap();
    assert_eq!(
        value["configs"]["app-config"]["file"].as_str(),
        Some("./app.conf")
    );
    assert_eq!(
        value["services"]["app"]["configs"]
            .as_sequence()
            .unwrap()
            .len(),
        1
    );

    let removed = management
        .remove_resource(
            raw.id,
            DeleteComposeResourceDto {
                kind: ComposeResourceKind::Config,
                name: "app-config".into(),
            },
        )
        .await
        .unwrap();
    let value: serde_yaml::Value = serde_yaml::from_str(&removed.compose_file).unwrap();
    assert!(value["configs"]["app-config"].is_null());
    assert!(
        value["services"]["app"]["configs"]
            .as_sequence()
            .unwrap()
            .is_empty()
    );

    let git = db
        .compose()
        .create(create_dto(
            "Git Resources",
            ENV_ID,
            None,
            "GITHUB",
            COMPOSE_FILE,
        ))
        .await
        .unwrap();
    assert_protocol(
        management
            .upsert_resource(
                git.id,
                UpsertComposeResourceDto {
                    kind: ComposeResourceKind::Secret,
                    name: "token".into(),
                    file: None,
                    external: true,
                    services: vec![],
                },
            )
            .await,
        "requires a RAW compose source",
    );
}

#[tokio::test]
async fn move_to_environment_updates_scope_and_rejects_foreign_targets() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = db
        .compose()
        .create(create_dto("Mover", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let moved = management
        .move_to_environment(record.id, STAGING_ENV_ID)
        .await
        .unwrap();
    assert_eq!(moved.environment_id, STAGING_ENV_ID);
    assert_eq!(
        db.compose()
            .get_by_id(record.id)
            .await
            .unwrap()
            .environment_id,
        STAGING_ENV_ID
    );

    assert_protocol(
        management
            .move_to_environment(record.id, OTHER_ENV_ID)
            .await,
        "target environment not found or belongs to another organization",
    );
}

#[tokio::test]
async fn move_to_environment_is_blocked_by_active_deployments() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = db
        .compose()
        .create(create_dto("Deploying", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "QUEUED", "QUEUED", 1, "", 0)
        .await;

    assert_protocol(
        management
            .move_to_environment(record.id, STAGING_ENV_ID)
            .await,
        "already queued or running",
    );
}

#[tokio::test]
async fn rotate_webhook_token_persists_hex_and_requires_compose() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = db
        .compose()
        .create(create_dto("Webhook", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    let token = management.rotate_webhook_token(record.id).await.unwrap();
    assert_eq!(token.len(), 64);
    assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    let stored: String = sqlx::query_scalar("SELECT refresh_token FROM services WHERE id = ?")
        .bind(record.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(stored, token);

    assert!(matches!(
        management.rotate_webhook_token(999).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn cleanup_queue_requires_an_existing_compose() {
    // The success path calls BuilderQueue::cancel_queued_compose, which opens
    // the process-global database pool; only the guard is unit-testable.
    let db = TestDb::new().await;
    assert!(matches!(
        db.compose_management().cleanup_queue(999).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn clear_history_removes_finished_deployments_only() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = db
        .compose()
        .create(create_dto("History", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "DONE", "DONE", 1, "", 0)
        .await;
    db.insert_deployment(Some(record.id), None, "FAILED", "FAILED", 2, "", 0)
        .await;
    db.insert_deployment(Some(record.id), None, "RUNNING", "RUNNING", 3, "", USER_ID)
        .await;
    db.insert_deployment(Some(record.id), None, "QUEUED", "QUEUED", 4, "", 0)
        .await;

    let removed = management.clear_history(record.id).await.unwrap();
    assert_eq!(removed, 2);
    let remaining: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM deployments WHERE service_id = ?")
            .bind(record.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(remaining, 2);
}

#[tokio::test]
async fn force_kill_requires_a_running_deployment() {
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = db
        .compose()
        .create(create_dto("Force", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "DONE", "DONE", 1, "", 0)
        .await;

    assert_protocol(
        management.force_kill(record.id).await,
        "compose has no running deployment",
    );
}

#[tokio::test]
async fn force_kill_running_deployment_not_owned_by_process() {
    let _guard = state_guard().await;
    let db = TestDb::new().await;
    let management = db.compose_management();
    let record = db
        .compose()
        .create(create_dto("Orphan", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    let deployment = db
        .insert_deployment(Some(record.id), None, "RUNNING", "RUNNING", 1, "", USER_ID)
        .await;
    let state = auto_di::resolve::<ApplicationState>().await.unwrap();
    state.remove_state(IdType::ComposeId(record.id));

    let result = management.force_kill(record.id).await.unwrap();
    assert_eq!(
        result,
        crate::services::compute::engine::CancelDeploymentResult::NotActiveInThisProcess
    );
    let status: String = sqlx::query_scalar("SELECT status FROM deployments WHERE id = ?")
        .bind(deployment)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(status, "RUNNING");
}

#[tokio::test]
async fn run_operation_rejects_when_a_deployment_is_active() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Queued", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    db.insert_deployment(Some(record.id), None, "QUEUED", "QUEUED", 1, "", 0)
        .await;

    assert_protocol(
        service
            .run_operation(record.id, ComposeOperation::Deploy, USER_ID)
            .await,
        "already queued or running",
    );
    assert_protocol(
        service
            .run_operation(record.id, ComposeOperation::Redeploy, USER_ID)
            .await,
        "already queued or running",
    );
}

#[tokio::test]
async fn source_switching_preserves_git_controls() {
    let db = TestDb::new().await;
    let service = db.compose();
    let record = service
        .create(create_dto("Sources", ENV_ID, None, "GITHUB", COMPOSE_FILE))
        .await
        .unwrap();
    service
        .patch(
            record.id,
            PatchComposeDto {
                trigger_type: Some("TAG".into()),
                watch_paths: Some("apps/**".into()),
                enable_submodules: Some(1),
                ..patch_dto()
            },
        )
        .await
        .unwrap();

    let github = service
        .set_github_source(
            record.id,
            PatchComposeGithubSourceDto {
                repository: "compose-repo".into(),
                owner: "acme".into(),
                branch: "main".into(),
                github_provider_id: Some(GIT_PROVIDER_ID),
                auto_deploy: None,
            },
        )
        .await
        .unwrap();
    assert_eq!(github.source_type, "GITHUB");
    assert_eq!(github.repository.as_deref(), Some("compose-repo"));
    assert_eq!(github.trigger_type, "TAG");
    let auto_deploy: i64 =
        sqlx::query_scalar("SELECT auto_deploy FROM service_git WHERE service_id = ?")
            .bind(record.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(auto_deploy, 1);

    let gitlab = service
        .set_gitlab_source(
            record.id,
            PatchComposeGitlabSourceDto {
                gitlab_project_id: Some(42),
                gitlab_repository: "gl-repo".into(),
                gitlab_owner: "gl-owner".into(),
                gitlab_branch: "dev".into(),
                gitlab_path_namespace: Some("group".into()),
                gitlab_provider_id: Some(GIT_PROVIDER_ID),
                gitlab_build_path: Some("/svc".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(gitlab.source_type, "GITLAB");
    assert_eq!(gitlab.repository, None);
    assert_eq!(gitlab.gitlab_repository.as_deref(), Some("gl-repo"));
    assert_eq!(gitlab.gitlab_owner.as_deref(), Some("gl-owner"));
    assert_eq!(gitlab.gitlab_branch.as_deref(), Some("dev"));
    assert_eq!(gitlab.build_path.as_deref(), Some("/svc"));
    let gitlab_auto_deploy: i64 =
        sqlx::query_scalar("SELECT auto_deploy FROM service_git WHERE service_id = ?")
            .bind(record.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(gitlab_auto_deploy, 1);

    let gitea = service
        .set_gitea_source(
            record.id,
            PatchComposeGiteaSourceDto {
                gitea_repository: "gt-repo".into(),
                gitea_owner: "gt-owner".into(),
                gitea_branch: "main".into(),
                gitea_provider_id: Some(GIT_PROVIDER_ID),
                gitea_build_path: Some("/gt".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(gitea.source_type, "GITEA");
    assert_eq!(gitea.gitea_repository.as_deref(), Some("gt-repo"));
    assert_eq!(gitea.build_path.as_deref(), Some("/gt"));

    let bitbucket = service
        .set_bitbucket_source(
            record.id,
            PatchComposeBitbucketSourceDto {
                bitbucket_repository: "bb-repo".into(),
                bitbucket_repository_slug: Some("bb-slug".into()),
                bitbucket_owner: "bb-owner".into(),
                bitbucket_branch: "main".into(),
                bitbucket_provider_id: Some(GIT_PROVIDER_ID),
                bitbucket_build_path: Some("/bb".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(bitbucket.source_type, "BITBUCKET");
    assert_eq!(bitbucket.bitbucket_repository.as_deref(), Some("bb-repo"));
    assert_eq!(bitbucket.build_path.as_deref(), Some("/bb"));

    let custom = service
        .set_custom_git_source(
            record.id,
            PatchComposeCustomGitSourceDto {
                custom_git_url: "ssh://git.example.com/stack.git".into(),
                custom_git_branch: "main".into(),
                custom_git_ssh_key_id: Some(SSH_KEY_ID),
                custom_git_build_path: Some("/custom".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(custom.source_type, "CUSTOM");
    assert_eq!(
        custom.custom_git_url.as_deref(),
        Some("ssh://git.example.com/stack.git")
    );
    assert_eq!(custom.custom_git_ssh_key_id, Some(SSH_KEY_ID));
    assert_eq!(custom.build_path.as_deref(), Some("/custom"));
    assert_eq!(custom.trigger_type, "TAG");

    let raw = service
        .set_raw_source(
            record.id,
            PatchComposeRawSourceDto {
                compose_file: "services:\n  only:\n    image: alpine\n".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(raw.source_type, "RAW");
    assert_eq!(raw.repository, None);
    assert!(raw.compose_file.contains("alpine"));

    assert!(
        service
            .set_github_source(
                999,
                PatchComposeGithubSourceDto {
                    repository: "missing".into(),
                    owner: "missing".into(),
                    branch: "main".into(),
                    github_provider_id: None,
                    auto_deploy: None,
                },
            )
            .await
            .is_err()
    );
    assert!(matches!(
        service
            .set_raw_source(
                999,
                PatchComposeRawSourceDto {
                    compose_file: String::new(),
                },
            )
            .await,
        Err(sqlx::Error::Protocol(_))
    ));
}

#[tokio::test]
async fn mounts_validate_normalization_and_scope() {
    let db = TestDb::new().await;
    let mounts = db.compose_mount();
    let compose = db
        .compose()
        .create(create_dto("Mounts", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    assert_protocol(
        mounts.create(compose.id, compose_mount("BIND")).await,
        "host_path is required for BIND",
    );
    let mut relative_bind = compose_mount("BIND");
    relative_bind.host_path = Some("data".into());
    assert_protocol(
        mounts.create(compose.id, relative_bind).await,
        "host_path must be absolute",
    );
    assert_protocol(
        mounts.create(compose.id, compose_mount("VOLUME")).await,
        "volume_name is required for VOLUME",
    );
    assert_protocol(
        mounts.create(compose.id, compose_mount("FILE")).await,
        "content is required for FILE",
    );
    assert_protocol(
        mounts.create(compose.id, compose_mount("NOPE")).await,
        "invalid mount type",
    );
    let mut relative_mount = compose_mount("VOLUME");
    relative_mount.volume_name = Some("data".into());
    relative_mount.mount_path = "data".into();
    assert_protocol(
        mounts.create(compose.id, relative_mount).await,
        "mount_path must be absolute",
    );

    let created = mounts
        .create(compose.id, {
            let mut input = compose_mount("BIND");
            input.host_path = Some("/srv/data".into());
            input.volume_name = Some("ignored".into());
            input
        })
        .await
        .unwrap();
    assert_eq!(created.mount_type, "BIND");
    assert_eq!(created.service_type, "COMPOSE");
    assert_eq!(created.host_path.as_deref(), Some("/srv/data"));
    // Cleared fields are persisted as empty strings rather than NULL.
    assert_eq!(created.volume_name.as_deref(), Some(""));
    assert_eq!(created.mount_path, "/data");

    let updated = mounts
        .update(compose.id, created.id, {
            let mut input = compose_mount("VOLUME");
            input.volume_name = Some("renamed".into());
            input.mount_path = "/var/lib/data".into();
            input
        })
        .await
        .unwrap();
    assert_eq!(updated.mount_type, "VOLUME");
    assert_eq!(updated.volume_name.as_deref(), Some("renamed"));
    assert_eq!(updated.host_path.as_deref(), Some(""));
    assert_eq!(updated.mount_path, "/var/lib/data");

    let listed = mounts.list(compose.id).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.id);

    assert!(mounts.delete(compose.id, created.id).await.unwrap());
    assert!(!mounts.delete(compose.id, created.id).await.unwrap());
    let mut gone = compose_mount("VOLUME");
    gone.volume_name = Some("gone".into());
    assert!(matches!(
        mounts.update(compose.id, created.id, gone).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let application_id = db.insert_service("APPLICATION", "IDLE", ENV_ID, None).await;
    assert!(matches!(
        mounts.list(application_id).await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn patches_validate_paths_and_crud() {
    let db = TestDb::new().await;
    let patches = db.compose_patch();
    let compose = db
        .compose()
        .create(create_dto("Patches", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();

    for unsafe_path in ["/etc/passwd", "../escape", "a/../b"] {
        assert_protocol(
            patches
                .create(compose.id, patch_dto_json("UPDATE", unsafe_path, true, "x"))
                .await,
            "unsafe patch file path",
        );
    }

    let created = patches
        .create(
            compose.id,
            patch_dto_json("UPDATE", "conf/app.conf", true, "a=1\n"),
        )
        .await
        .unwrap();
    assert_eq!(created.patch_type, "UPDATE");
    assert_eq!(created.enabled, 1);
    assert_eq!(created.file_path, "conf/app.conf");

    let updated = patches
        .update(
            compose.id,
            created.id,
            patch_dto_json("DELETE", "conf/app.conf", false, ""),
        )
        .await
        .unwrap();
    assert_eq!(updated.patch_type, "DELETE");
    assert_eq!(updated.enabled, 0);

    let listed = patches.list(compose.id).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.id);

    assert!(patches.delete(compose.id, created.id).await.unwrap());
    assert!(!patches.delete(compose.id, created.id).await.unwrap());
    assert!(matches!(
        patches
            .update(
                compose.id,
                created.id,
                patch_dto_json("UPDATE", "app.conf", true, "x")
            )
            .await,
        Err(sqlx::Error::RowNotFound)
    ));

    let application_id = db.insert_service("APPLICATION", "IDLE", ENV_ID, None).await;
    assert!(matches!(
        patches.list(application_id).await,
        Err(sqlx::Error::Protocol(_))
    ));
}

async fn transfer_fixture(db: &TestDb) -> (i64, crate::services::compute::compose::ComposeRecord) {
    let service = db.compose();
    let record = service
        .create(create_dto(
            "Transfer",
            ENV_ID,
            Some(SERVER_ID),
            "GITHUB",
            COMPOSE_FILE,
        ))
        .await
        .unwrap();
    service
        .set_github_source(
            record.id,
            PatchComposeGithubSourceDto {
                repository: "compose-repo".into(),
                owner: "acme".into(),
                branch: "main".into(),
                github_provider_id: Some(GIT_PROVIDER_ID),
                auto_deploy: Some(1),
            },
        )
        .await
        .unwrap();
    let record = service
        .patch(
            record.id,
            PatchComposeDto {
                env_var: Some("SECRET=1".into()),
                command: Some("docker compose up".into()),
                trigger_type: Some("TAG".into()),
                compose_file: Some(
                    "services:\n  app:\n    image: nginx\n    environment:\n      DB_PASSWORD: hunter2\n      API_TOKEN: abc\n    secrets:\n      - /run/secrets/db_password\n"
                        .into(),
                ),
                ..patch_dto()
            },
        )
        .await
        .unwrap();
    db.compose_mount()
        .create(record.id, {
            let mut input = compose_mount("VOLUME");
            input.volume_name = Some("data".into());
            input
        })
        .await
        .unwrap();
    db.compose_patch()
        .create(
            record.id,
            patch_dto_json("UPDATE", "app.conf", true, "x=1\n"),
        )
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO domains (host, https, port, path, internal_path, custom_entrypoint, \
             service_name, custom_cert_resolver, strip_path, middlewares, domain_type, \
             certificate_type, service_id, organization_id) \
         VALUES ('app.test', 1, 3000, '/', '', '', 'app', '', 0, '[]', 'COMPOSE', \
             'LETSENCRYPT', ?, 1)",
    )
    .bind(record.id)
    .execute(&*db.pool)
    .await
    .unwrap();
    (record.id, record)
}

#[tokio::test]
async fn export_redacts_secrets_unless_included() {
    let db = TestDb::new().await;
    let transfer = db.compose_transfer();
    let (id, _) = transfer_fixture(&db).await;

    let redacted = transfer.export(id, false).await.unwrap();
    assert!(!redacted.secrets_included);
    assert_eq!(redacted.schema_version, 1);
    assert_eq!(redacted.domains.len(), 1);
    assert_eq!(redacted.mounts.len(), 1);
    assert_eq!(redacted.patches.len(), 1);
    assert_eq!(redacted.compose.env_var, None);
    assert_eq!(redacted.compose.refresh_token, None);
    let file = &redacted.compose.compose_file;
    assert!(file.contains("DB_PASSWORD: REDACTED"), "{file}");
    assert!(file.contains("API_TOKEN: REDACTED"), "{file}");
    assert!(!file.contains("hunter2"));
    assert!(!file.contains("abc"));
    assert!(file.contains("- /run/secrets/db_password"));

    let full = transfer.export(id, true).await.unwrap();
    assert!(full.secrets_included);
    assert_eq!(full.compose.env_var.as_deref(), Some("SECRET=1"));
    assert!(full.compose.compose_file.contains("hunter2"));
    assert_eq!(full.compose.source_type, "GITHUB");
    assert_eq!(full.compose.repository.as_deref(), Some("compose-repo"));
    assert_eq!(full.compose.trigger_type, "TAG");
    assert_eq!(full.compose.auto_deploy, 1);
    assert_eq!(full.compose.command, "docker compose up");

    assert!(matches!(
        transfer.export(999, false).await,
        Err(sqlx::Error::RowNotFound)
    ));
    let application_id = db.insert_service("APPLICATION", "IDLE", ENV_ID, None).await;
    assert!(matches!(
        transfer.export(application_id, false).await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn import_roundtrip_restores_sources_and_relations() {
    let db = TestDb::new().await;
    let transfer = db.compose_transfer();
    let (source_id, source) = transfer_fixture(&db).await;
    let bundle = transfer.export(source_id, true).await.unwrap();
    let archive = serde_json::to_string(&bundle).unwrap();

    let imported = transfer
        .import(ImportComposeDto {
            target_environment_id: STAGING_ENV_ID,
            target_server_id: Some(SERVER_ID_2),
            name: Some("Imported Stack".into()),
            archive,
        })
        .await
        .unwrap();

    assert_ne!(imported.id, source_id);
    assert_eq!(imported.name, "Imported Stack");
    assert!(imported.app_name.starts_with("imported-stack-"));
    assert_eq!(imported.environment_id, STAGING_ENV_ID);
    assert_eq!(imported.server_id, Some(SERVER_ID_2));
    assert_eq!(imported.source_type, "GITHUB");
    assert_eq!(imported.repository.as_deref(), Some("compose-repo"));
    assert_eq!(imported.owner.as_deref(), Some("acme"));
    assert_eq!(imported.trigger_type, "TAG");
    let imported_auto_deploy: i64 =
        sqlx::query_scalar("SELECT auto_deploy FROM service_git WHERE service_id = ?")
            .bind(imported.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(imported_auto_deploy, 1);
    assert!(imported.compose_file.contains("hunter2"));
    assert_eq!(imported.compose_type, ComposeType::DockerCompose);
    // import() returns the record captured before env/command are written back,
    // so verify those two through a fresh read.
    let stored = db.compose().get_by_id(imported.id).await.unwrap();
    assert_eq!(stored.env_var.as_deref(), Some("SECRET=1"));
    assert_eq!(stored.command, "docker compose up");

    let domains = db
        .queries
        .domains_list_by_compose_raw(imported.id)
        .await
        .unwrap();
    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0].host, "app.test");
    assert_eq!(domains[0].service_id, Some(imported.id));
    assert_eq!(domains[0].organization_id, ORG_ID);
    let mounts = db
        .queries
        .mount_fetch_for_compose(imported.id)
        .await
        .unwrap();
    assert_eq!(mounts.len(), 1);
    assert_eq!(mounts[0].volume_name.as_deref(), Some("data"));
    assert_eq!(mounts[0].service_type, "COMPOSE");
    let patches = db
        .queries
        .patches_list_by_compose(imported.id)
        .await
        .unwrap();
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].file_path, "app.conf");

    let defaults = transfer
        .import(ImportComposeDto {
            target_environment_id: ENV_ID,
            target_server_id: None,
            name: None,
            archive: serde_json::to_string(&transfer.export(source.id, false).await.unwrap())
                .unwrap(),
        })
        .await
        .unwrap();
    assert_eq!(defaults.name, source.name);
    assert_eq!(defaults.env_var, None);
    assert_eq!(defaults.server_id, None);
}

#[tokio::test]
async fn import_raw_source_skips_git_row() {
    let db = TestDb::new().await;
    let transfer = db.compose_transfer();
    let raw = db
        .compose()
        .create(create_dto("Raw Export", ENV_ID, None, "RAW", COMPOSE_FILE))
        .await
        .unwrap();
    let archive = serde_json::to_string(&transfer.export(raw.id, true).await.unwrap()).unwrap();

    let imported = transfer
        .import(ImportComposeDto {
            target_environment_id: ENV_ID,
            target_server_id: None,
            name: None,
            archive,
        })
        .await
        .unwrap();
    assert_eq!(imported.source_type, "RAW");
    let git_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM service_git WHERE service_id = ?")
        .bind(imported.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(git_rows, 0);
    assert_eq!(imported.compose_file, COMPOSE_FILE);
}

#[tokio::test]
async fn import_rejects_invalid_archives_schemas_and_environments() {
    let db = TestDb::new().await;
    let transfer = db.compose_transfer();
    let (source_id, _) = transfer_fixture(&db).await;

    assert_protocol(
        transfer
            .import(ImportComposeDto {
                target_environment_id: ENV_ID,
                target_server_id: None,
                name: None,
                archive: "not json".into(),
            })
            .await,
        "invalid compose archive",
    );

    let bundle = transfer.export(source_id, false).await.unwrap();
    let mut value = serde_json::to_value(&bundle).unwrap();
    value["schema_version"] = serde_json::json!(2);
    assert_protocol(
        transfer
            .import(ImportComposeDto {
                target_environment_id: ENV_ID,
                target_server_id: None,
                name: None,
                archive: serde_json::to_string(&value).unwrap(),
            })
            .await,
        "unsupported compose export schema version 2",
    );

    assert_protocol(
        transfer
            .import(ImportComposeDto {
                target_environment_id: 999,
                target_server_id: None,
                name: None,
                archive: serde_json::to_string(&bundle).unwrap(),
            })
            .await,
        "target environment not found",
    );
}
