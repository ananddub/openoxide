use std::collections::BTreeMap;

use super::assert_protocol;
use crate::api::connect::generated::openoxide::api::v1::{CreateMountRequest, UpdateMountRequest};
use crate::core::cache::CacheKey;
use crate::services::compute::database::crud::{
    CreateDatabaseDto, PatchDatabaseDto, parse_json_string_vec,
};
use crate::services::compute::database::management::{
    ImportDatabaseDto, PostgresAdvancedConfigDto, PostgresReplicationRole,
};
use crate::services::compute::database::swarm_settings::DatabaseSwarmSettingsDto;
use crate::services::compute::database::{DatabaseKind, DatabaseOperation};
use crate::services::compute::test_support::*;

fn create_dto(name: &str, environment_id: i64, server_id: Option<i64>) -> CreateDatabaseDto {
    CreateDatabaseDto {
        name: name.into(),
        description: None,
        environment_id,
        server_id,
        docker_image: None,
        database_name: None,
        database_user: None,
        database_password: None,
        database_root_password: None,
        external_port: None,
        external_grpc_port: None,
        external_admin_port: None,
        command: None,
        args: None,
        env_var: None,
        replica_sets: None,
        sqld_node: None,
        sqld_primary_url: None,
        enable_namespaces: None,
        network_ids: None,
        detach_rustploy_network: None,
    }
}

fn patch_dto() -> PatchDatabaseDto {
    PatchDatabaseDto {
        name: None,
        description: None,
        docker_image: None,
        database_name: None,
        database_user: None,
        database_password: None,
        database_root_password: None,
        external_port: None,
        external_grpc_port: None,
        external_admin_port: None,
        command: None,
        args: None,
        env_var: None,
        memory_reservation: None,
        memory_limit: None,
        cpu_reservation: None,
        cpu_limit: None,
        replicas: None,
        server_id: None,
        replica_sets: None,
        sqld_node: None,
        sqld_primary_url: None,
        enable_namespaces: None,
        network_ids: None,
        detach_rustploy_network: None,
    }
}

fn swarm_settings() -> DatabaseSwarmSettingsDto {
    DatabaseSwarmSettingsDto {
        health_check_swarm: None,
        restart_policy_swarm: None,
        placement_swarm: None,
        update_config_swarm: None,
        rollback_config_swarm: None,
        mode_swarm: None,
        labels_swarm: None,
        network_swarm: None,
        endpoint_spec_swarm: None,
        ulimits_swarm: None,
        stop_grace_period_swarm: None,
    }
}

fn mount_request(mount_type: &str) -> CreateMountRequest {
    CreateMountRequest {
        mount_type: mount_type.into(),
        mount_path: "/data".into(),
        ..Default::default()
    }
}

fn assert_random_secret(value: &str) {
    assert_eq!(value.len(), 32, "unexpected secret length: {value}");
    assert!(
        value.chars().all(|ch| ch.is_ascii_hexdigit()),
        "unexpected secret: {value}"
    );
}

async fn engine_config(db: &TestDb, id: i64) -> serde_json::Value {
    let raw: String =
        sqlx::query_scalar("SELECT engine_config FROM service_database WHERE service_id = ?")
            .bind(id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    serde_json::from_str(&raw).unwrap()
}

#[tokio::test]
async fn create_postgres_defaults_and_lists_by_environment() {
    let db = TestDb::new().await;
    let databases = db.database();

    let created = databases
        .create(DatabaseKind::Postgres, create_dto("My DB", ENV_ID, None))
        .await
        .unwrap();
    assert_eq!(created.kind, DatabaseKind::Postgres);
    assert_eq!(created.name, "My DB");
    assert!(created.app_name.starts_with("postgres-my-db-"));
    assert_eq!(created.app_name.len(), "postgres-my-db-".len() + 6);
    assert_eq!(created.description.as_deref(), Some(""));
    assert_eq!(created.docker_image, "postgres:18");
    assert_eq!(created.database_name.as_deref(), Some("my-db"));
    assert_eq!(created.database_user.as_deref(), Some("openoxide"));
    assert_random_secret(created.database_password.as_deref().unwrap());
    assert_eq!(created.external_port, None);
    assert_eq!(created.command, None);
    assert_eq!(created.args.as_deref(), Some(""));
    assert_eq!(created.env_var.as_deref(), Some(""));
    assert_eq!(created.network_ids, "[]");
    assert_eq!(created.detach_rustploy_network, 0);
    assert_eq!(created.replicas, 1);
    assert_eq!(created.app_status, "IDLE");
    assert_eq!(created.last_error, None);
    assert_eq!(created.environment_id, ENV_ID);
    assert_eq!(created.server_id, None);
    assert_eq!(created.memory_limit, None);
    assert!(created.created_at > 0);
    assert_eq!(created.created_at, created.updated_at);

    let db_type: String =
        sqlx::query_scalar("SELECT db_type FROM service_database WHERE service_id = ?")
            .bind(created.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(db_type, "POSTGRES");
    let engine = engine_config(&db, created.id).await;
    assert_eq!(engine["postgres_config"], serde_json::json!({}));
    assert_eq!(engine["replication_role"], "STANDALONE");

    let listed = databases.list_by_environment(ENV_ID).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.id);
    assert_eq!(listed[0].kind, DatabaseKind::Postgres);
    assert!(
        databases
            .list_by_environment(STAGING_ENV_ID)
            .await
            .unwrap()
            .is_empty()
    );

    let odd = databases
        .create(DatabaseKind::Postgres, create_dto("!!!", ENV_ID, None))
        .await
        .unwrap();
    assert!(odd.app_name.starts_with("postgres-database-"));
    assert_eq!(odd.database_name.as_deref(), Some("database"));
}

#[tokio::test]
async fn create_all_kinds_apply_defaults() {
    let db = TestDb::new().await;
    let databases = db.database();

    for (kind, has_db_name, has_user) in [
        (DatabaseKind::Mysql, true, true),
        (DatabaseKind::Mariadb, true, true),
        (DatabaseKind::Mongo, false, true),
        (DatabaseKind::Redis, false, false),
        (DatabaseKind::Libsql, false, true),
    ] {
        let name = format!("Svc {}", kind.as_str());
        let record = databases
            .create(kind, create_dto(&name, ENV_ID, Some(SERVER_ID)))
            .await
            .unwrap();
        assert_eq!(record.kind, kind);
        assert_eq!(record.docker_image, kind.default_image());
        assert_eq!(record.server_id, Some(SERVER_ID));
        assert_eq!(record.replicas, 1);
        let slug = format!("svc-{}", kind.as_str());
        if has_db_name {
            assert_eq!(record.database_name.as_deref(), Some(slug.as_str()));
        } else {
            // Cleared columns are persisted as empty strings, not NULL.
            assert_eq!(record.database_name.as_deref(), Some(""));
        }
        if has_user {
            assert_eq!(record.database_user.as_deref(), Some("openoxide"));
        } else {
            assert_eq!(record.database_user.as_deref(), Some(""));
        }

        let engine = engine_config(&db, record.id).await;
        match kind {
            DatabaseKind::Mysql | DatabaseKind::Mariadb | DatabaseKind::Mongo => {
                assert_random_secret(engine["database_root_password"].as_str().unwrap());
            }
            DatabaseKind::Redis | DatabaseKind::Libsql => {
                assert_eq!(engine, serde_json::json!({}));
            }
            DatabaseKind::Postgres => unreachable!(),
        }
    }
}

#[tokio::test]
async fn create_persists_custom_configuration() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Analytics", ENV_ID, Some(SERVER_ID));
    input.description = Some("warehouse".into());
    input.docker_image = Some("postgres:16".into());
    input.database_name = Some("analytics".into());
    input.database_user = Some("analyst".into());
    input.database_password = Some("custom-password".into());
    input.database_root_password = Some("root-password".into());
    input.external_port = Some(15432);
    input.command = Some("postgres -c shared_buffers=1GB".into());
    input.args = Some(vec!["-c".into(), "max_connections=100".into()]);
    input.env_var = Some("POSTGRES_EXTRA=1".into());
    input.network_ids = Some(vec!["2".into(), "1".into()]);
    input.detach_rustploy_network = Some(1);
    let created = databases
        .create(DatabaseKind::Postgres, input)
        .await
        .unwrap();

    assert_eq!(created.description.as_deref(), Some("warehouse"));
    assert_eq!(created.docker_image, "postgres:16");
    assert_eq!(created.database_name.as_deref(), Some("analytics"));
    assert_eq!(created.database_user.as_deref(), Some("analyst"));
    assert_eq!(
        created.database_password.as_deref(),
        Some("custom-password")
    );
    assert_eq!(created.external_port, Some(15432));
    assert_eq!(
        created.command.as_deref(),
        Some("postgres -c shared_buffers=1GB")
    );
    assert_eq!(
        parse_json_string_vec(created.args.as_deref().unwrap()),
        vec!["-c".to_string(), "max_connections=100".into()]
    );
    assert_eq!(created.env_var.as_deref(), Some("POSTGRES_EXTRA=1"));
    assert_eq!(created.network_ids, "[\"2\",\"1\"]");
    assert_eq!(created.detach_rustploy_network, 1);
    assert_eq!(created.server_id, Some(SERVER_ID));

    let engine = engine_config(&db, created.id).await;
    assert_eq!(engine["postgres_config"], serde_json::json!({}));
    assert_eq!(engine["replication_role"], "STANDALONE");
    assert!(engine.get("database_root_password").is_none());
}

#[tokio::test]
async fn create_libsql_stores_engine_config() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Edge", ENV_ID, None);
    input.sqld_node = Some("primary-1".into());
    input.sqld_primary_url = Some("http://sqld:8080".into());
    input.enable_namespaces = Some(1);
    input.external_grpc_port = Some(5001);
    input.external_admin_port = Some(5002);
    let created = databases.create(DatabaseKind::Libsql, input).await.unwrap();

    assert_eq!(created.external_port, None);
    assert_eq!(
        engine_config(&db, created.id).await,
        serde_json::json!({
            "sqld_node": "primary-1",
            "sqld_primary_url": "http://sqld:8080",
            "enable_namespaces": 1,
            "external_grpc_port": 5001,
            "external_admin_port": 5002,
        })
    );
}

#[tokio::test]
async fn get_by_id_enforces_kind_and_presence() {
    let db = TestDb::new().await;
    let databases = db.database();
    let created = databases
        .create(DatabaseKind::Postgres, create_dto("Lookup", ENV_ID, None))
        .await
        .unwrap();

    let found = databases
        .get_by_id(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(found.id, created.id);
    assert_protocol(
        databases.get_by_id(DatabaseKind::Mysql, created.id).await,
        "no rows",
    );
    assert_protocol(
        databases.get_by_id(DatabaseKind::Postgres, 999).await,
        "no rows",
    );
}

#[tokio::test]
async fn patch_updates_configuration_and_keeps_omitted_fields() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Initial DB", ENV_ID, Some(SERVER_ID));
    input.database_name = Some("appdb".into());
    input.database_user = Some("appuser".into());
    input.database_password = Some("initial-password-x".into());
    input.external_port = Some(15432);
    input.env_var = Some("A=1".into());
    let created = databases
        .create(DatabaseKind::Postgres, input)
        .await
        .unwrap();

    let mut patch = patch_dto();
    patch.name = Some("Renamed DB".into());
    patch.description = Some("renamed description".into());
    patch.docker_image = Some("postgres:17".into());
    patch.database_name = Some("renamed".into());
    patch.database_user = Some("renamed_user".into());
    patch.database_password = Some("rotated-password-1".into());
    patch.external_port = Some(25432);
    patch.command = Some("postgres -c x=1".into());
    patch.args = Some(vec!["-c".into(), "x=1".into()]);
    patch.env_var = Some("B=2".into());
    patch.memory_reservation = Some("128M".into());
    patch.memory_limit = Some("1G".into());
    patch.cpu_reservation = Some("0.25".into());
    patch.cpu_limit = Some("1.5".into());
    patch.replicas = Some(3);
    patch.server_id = Some(SERVER_ID_2);
    patch.network_ids = Some(vec!["3".into()]);
    patch.detach_rustploy_network = Some(1);
    let updated = databases
        .patch(DatabaseKind::Postgres, created.id, patch)
        .await
        .unwrap();

    assert_eq!(updated.name, "Renamed DB");
    assert_eq!(updated.description.as_deref(), Some("renamed description"));
    assert_eq!(updated.docker_image, "postgres:17");
    assert_eq!(updated.database_name.as_deref(), Some("renamed"));
    assert_eq!(updated.database_user.as_deref(), Some("renamed_user"));
    assert_eq!(
        updated.database_password.as_deref(),
        Some("rotated-password-1")
    );
    assert_eq!(updated.external_port, Some(25432));
    assert_eq!(updated.command.as_deref(), Some("postgres -c x=1"));
    assert_eq!(
        parse_json_string_vec(updated.args.as_deref().unwrap()),
        vec!["-c".to_string(), "x=1".into()]
    );
    assert_eq!(updated.env_var.as_deref(), Some("B=2"));
    assert_eq!(updated.memory_reservation.as_deref(), Some("128M"));
    assert_eq!(updated.memory_limit.as_deref(), Some("1G"));
    assert_eq!(updated.cpu_reservation.as_deref(), Some("0.25"));
    assert_eq!(updated.cpu_limit.as_deref(), Some("1.5"));
    assert_eq!(updated.replicas, 3);
    assert_eq!(updated.server_id, Some(SERVER_ID_2));
    assert_eq!(updated.network_ids, "[\"3\"]");
    assert_eq!(updated.detach_rustploy_network, 1);
    assert_eq!(
        engine_config(&db, created.id).await["replication_role"],
        "STANDALONE"
    );

    let kept = databases
        .patch(DatabaseKind::Postgres, created.id, patch_dto())
        .await
        .unwrap();
    assert_eq!(kept.name, "Renamed DB");
    assert_eq!(kept.docker_image, "postgres:17");
    assert_eq!(kept.database_name.as_deref(), Some("renamed"));
    assert_eq!(
        kept.database_password.as_deref(),
        Some("rotated-password-1")
    );
    assert_eq!(kept.external_port, Some(25432));
    assert_eq!(kept.memory_limit.as_deref(), Some("1G"));
    assert_eq!(kept.replicas, 3);
    assert_eq!(kept.server_id, Some(SERVER_ID_2));

    assert_protocol(
        databases
            .patch(DatabaseKind::Postgres, 999, patch_dto())
            .await,
        "no rows",
    );
    assert_protocol(
        databases
            .patch(DatabaseKind::Mysql, created.id, patch_dto())
            .await,
        "no rows",
    );
}

#[tokio::test]
async fn patch_engine_config_merges_per_kind() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut mysql_input = create_dto("Shop", ENV_ID, None);
    mysql_input.database_root_password = Some("root-one".into());
    let mysql = databases
        .create(DatabaseKind::Mysql, mysql_input)
        .await
        .unwrap();
    assert_eq!(
        engine_config(&db, mysql.id).await["database_root_password"],
        "root-one"
    );
    let mut patch = patch_dto();
    patch.database_root_password = Some("root-two".into());
    patch.replica_sets = Some(9);
    databases
        .patch(DatabaseKind::Mysql, mysql.id, patch)
        .await
        .unwrap();
    let engine = engine_config(&db, mysql.id).await;
    assert_eq!(engine["database_root_password"], "root-two");
    assert!(engine.get("replica_sets").is_none());

    let mut mongo_input = create_dto("Cache", ENV_ID, None);
    mongo_input.database_root_password = Some("mongo-root".into());
    mongo_input.replica_sets = Some(3);
    let mongo = databases
        .create(DatabaseKind::Mongo, mongo_input)
        .await
        .unwrap();
    assert_eq!(engine_config(&db, mongo.id).await["replica_sets"], 3);
    let mut patch = patch_dto();
    patch.replica_sets = Some(5);
    patch.database_root_password = Some("mongo-root-2".into());
    databases
        .patch(DatabaseKind::Mongo, mongo.id, patch)
        .await
        .unwrap();
    let engine = engine_config(&db, mongo.id).await;
    assert_eq!(engine["replica_sets"], 5);
    assert_eq!(engine["database_root_password"], "mongo-root-2");

    let mut libsql_input = create_dto("Edge", ENV_ID, None);
    libsql_input.sqld_node = Some("node-a".into());
    libsql_input.sqld_primary_url = Some("http://a".into());
    let libsql = databases
        .create(DatabaseKind::Libsql, libsql_input)
        .await
        .unwrap();
    let mut patch = patch_dto();
    patch.sqld_node = Some("node-b".into());
    patch.enable_namespaces = Some(0);
    patch.external_grpc_port = Some(6001);
    patch.external_admin_port = Some(6002);
    databases
        .patch(DatabaseKind::Libsql, libsql.id, patch)
        .await
        .unwrap();
    let engine = engine_config(&db, libsql.id).await;
    assert_eq!(engine["sqld_node"], "node-b");
    assert_eq!(engine["sqld_primary_url"], "http://a");
    assert_eq!(engine["enable_namespaces"], 0);
    assert_eq!(engine["external_grpc_port"], 6001);
    assert_eq!(engine["external_admin_port"], 6002);

    let postgres = databases
        .create(DatabaseKind::Postgres, create_dto("PG", ENV_ID, None))
        .await
        .unwrap();
    let mut patch = patch_dto();
    patch.database_root_password = Some("ignored".into());
    databases
        .patch(DatabaseKind::Postgres, postgres.id, patch)
        .await
        .unwrap();
    let engine = engine_config(&db, postgres.id).await;
    assert!(engine.get("database_root_password").is_none());
    assert_eq!(engine["replication_role"], "STANDALONE");
}

#[tokio::test]
async fn postgres_advanced_config_roundtrip_and_validation() {
    let db = TestDb::new().await;
    let databases = db.database();
    let postgres = databases
        .create(DatabaseKind::Postgres, create_dto("Primary", ENV_ID, None))
        .await
        .unwrap();

    let initial = databases
        .get_postgres_advanced_config(postgres.id)
        .await
        .unwrap();
    assert!(initial.settings.is_empty());
    assert_eq!(
        initial.replication_role,
        PostgresReplicationRole::Standalone
    );
    assert_eq!(initial.primary_host, None);
    assert_eq!(initial.primary_port, None);
    assert!(!initial.replication_password_configured);
    assert!(initial.redeploy_required);

    let mut settings = BTreeMap::new();
    settings.insert("max_connections".into(), "200".into());
    settings.insert("work_mem".into(), "64MB".into());
    let updated = databases
        .update_postgres_advanced_config(
            postgres.id,
            PostgresAdvancedConfigDto {
                settings: settings.clone(),
                replication_role: PostgresReplicationRole::Primary,
                primary_host: Some("db.internal".into()),
                primary_port: Some(5432),
                replication_user: Some("replica".into()),
                replication_password: Some("0123456789abcdef".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.settings, settings);
    assert_eq!(updated.replication_role, PostgresReplicationRole::Primary);
    assert_eq!(updated.primary_host.as_deref(), Some("db.internal"));
    assert_eq!(updated.primary_port, Some(5432));
    assert_eq!(updated.replication_user.as_deref(), Some("replica"));
    assert!(updated.replication_password_configured);

    let engine = engine_config(&db, postgres.id).await;
    assert_eq!(
        engine["postgres_config"],
        serde_json::json!({"max_connections": "200", "work_mem": "64MB"})
    );
    assert_eq!(engine["replication_role"], "PRIMARY");
    assert_eq!(engine["primary_host"], "db.internal");
    assert_eq!(engine["primary_port"], 5432);
    assert_eq!(engine["replication_user"], "replica");
    assert_eq!(engine["replication_password"], "0123456789abcdef");

    let mut invalid = settings.clone();
    invalid.insert("bogus".into(), "1".into());
    assert_protocol(
        databases
            .update_postgres_advanced_config(
                postgres.id,
                PostgresAdvancedConfigDto {
                    settings: invalid,
                    replication_role: PostgresReplicationRole::Standalone,
                    primary_host: None,
                    primary_port: None,
                    replication_user: None,
                    replication_password: None,
                },
            )
            .await,
        "unsupported PostgreSQL setting",
    );
    let mut blank = BTreeMap::new();
    blank.insert("work_mem".into(), "   ".into());
    assert_protocol(
        databases
            .update_postgres_advanced_config(
                postgres.id,
                PostgresAdvancedConfigDto {
                    settings: blank,
                    replication_role: PostgresReplicationRole::Standalone,
                    primary_host: None,
                    primary_port: None,
                    replication_user: None,
                    replication_password: None,
                },
            )
            .await,
        "invalid PostgreSQL setting value",
    );

    assert_protocol(
        databases
            .update_postgres_advanced_config(
                postgres.id,
                PostgresAdvancedConfigDto {
                    settings: BTreeMap::new(),
                    replication_role: PostgresReplicationRole::Replica,
                    primary_host: Some("db.internal".into()),
                    primary_port: Some(5432),
                    replication_user: Some("replica".into()),
                    replication_password: Some("short".into()),
                },
            )
            .await,
        "replica requires",
    );

    let replica = databases
        .update_postgres_advanced_config(
            postgres.id,
            PostgresAdvancedConfigDto {
                settings: BTreeMap::new(),
                replication_role: PostgresReplicationRole::Replica,
                primary_host: Some("db.internal".into()),
                primary_port: Some(5433),
                replication_user: Some("replica".into()),
                replication_password: Some("0123456789abcdef".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(replica.replication_role, PostgresReplicationRole::Replica);
    assert_eq!(
        engine_config(&db, postgres.id).await["replication_role"],
        "REPLICA"
    );

    assert!(matches!(
        databases.get_postgres_advanced_config(999).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert_protocol(
        databases
            .update_postgres_advanced_config(
                999,
                PostgresAdvancedConfigDto {
                    settings: BTreeMap::new(),
                    replication_role: PostgresReplicationRole::Standalone,
                    primary_host: None,
                    primary_port: None,
                    replication_user: None,
                    replication_password: None,
                },
            )
            .await,
        "no rows",
    );

    let mysql = databases
        .create(DatabaseKind::Mysql, create_dto("MySQL", ENV_ID, None))
        .await
        .unwrap();
    assert_protocol(
        databases
            .update_postgres_advanced_config(
                mysql.id,
                PostgresAdvancedConfigDto {
                    settings: BTreeMap::new(),
                    replication_role: PostgresReplicationRole::Standalone,
                    primary_host: None,
                    primary_port: None,
                    replication_user: None,
                    replication_password: None,
                },
            )
            .await,
        "no rows",
    );
}

#[tokio::test]
async fn connection_details_build_encoded_urls() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Conn", ENV_ID, Some(SERVER_ID));
    input.database_name = Some("app_db".into());
    input.database_user = Some("app_user".into());
    input.database_password = Some("p@ss/w:rd".into());
    input.external_port = Some(15432);
    let postgres = databases
        .create(DatabaseKind::Postgres, input)
        .await
        .unwrap();

    let details = databases
        .connection_details(DatabaseKind::Postgres, postgres.id)
        .await
        .unwrap();
    let host = format!("{}_db", postgres.app_name);
    assert_eq!(details.kind, DatabaseKind::Postgres);
    assert_eq!(details.host, host);
    assert_eq!(details.port, 5432);
    assert_eq!(details.database.as_deref(), Some("app_db"));
    assert_eq!(details.username.as_deref(), Some("app_user"));
    assert_eq!(details.password, "p@ss/w:rd");
    assert_eq!(
        details.internal_url,
        format!("postgresql://app_user:p%40ss%2Fw%3Ard@{host}:5432/app_db")
    );
    assert_eq!(
        details.external_url.as_deref(),
        Some("postgresql://app_user:p%40ss%2Fw%3Ard@SERVER_HOST:15432/app_db")
    );
    assert_eq!(details.server_id, Some(SERVER_ID));

    let mut redis_input = create_dto("Sessions", ENV_ID, None);
    redis_input.database_password = Some("redis-password".into());
    let redis = databases
        .create(DatabaseKind::Redis, redis_input)
        .await
        .unwrap();
    let details = databases
        .connection_details(DatabaseKind::Redis, redis.id)
        .await
        .unwrap();
    assert_eq!(details.host, format!("{}_db", redis.app_name));
    assert_eq!(details.port, 6379);
    assert_eq!(details.database.as_deref(), Some(""));
    assert_eq!(details.username.as_deref(), Some(""));
    assert_eq!(
        details.internal_url,
        format!("redis://:redis-password@{}_db:6379/", redis.app_name)
    );
    assert_eq!(details.external_url, None);

    let mut mysql_input = create_dto("MySQL", ENV_ID, None);
    mysql_input.external_port = Some(13306);
    let mysql = databases
        .create(DatabaseKind::Mysql, mysql_input)
        .await
        .unwrap();
    let details = databases
        .connection_details(DatabaseKind::Mysql, mysql.id)
        .await
        .unwrap();
    assert_eq!(details.port, 3306);
    assert!(details.internal_url.starts_with("mysql://openoxide:"));
    assert!(
        details
            .internal_url
            .contains(&format!("{}:3306/mysql", details.host))
    );

    assert!(matches!(
        databases
            .connection_details(DatabaseKind::Postgres, 999)
            .await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn rotate_credentials_updates_password_and_guards() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Rotate", ENV_ID, None);
    input.database_password = Some("initial-password-1".into());
    let postgres = databases
        .create(DatabaseKind::Postgres, input)
        .await
        .unwrap();

    let rotated = databases
        .rotate_credentials(
            DatabaseKind::Postgres,
            postgres.id,
            Some("rotated-password-123".into()),
        )
        .await
        .unwrap();
    assert_eq!(rotated.password, "rotated-password-123");
    assert!(rotated.redeploy_required);
    assert_eq!(
        databases
            .get_by_id(DatabaseKind::Postgres, postgres.id)
            .await
            .unwrap()
            .database_password
            .as_deref(),
        Some("rotated-password-123")
    );
    assert_eq!(
        databases
            .connection_details(DatabaseKind::Postgres, postgres.id)
            .await
            .unwrap()
            .password,
        "rotated-password-123"
    );

    let random = databases
        .rotate_credentials(DatabaseKind::Postgres, postgres.id, None)
        .await
        .unwrap();
    assert_random_secret(&random.password);
    assert!(random.redeploy_required);
    assert_eq!(
        databases
            .get_by_id(DatabaseKind::Postgres, postgres.id)
            .await
            .unwrap()
            .database_password
            .as_deref(),
        Some(random.password.as_str())
    );

    assert_protocol(
        databases
            .rotate_credentials(DatabaseKind::Postgres, postgres.id, Some("short".into()))
            .await,
        "at least 16 characters",
    );
    assert_eq!(
        databases
            .get_by_id(DatabaseKind::Postgres, postgres.id)
            .await
            .unwrap()
            .database_password
            .as_deref(),
        Some(random.password.as_str())
    );
    assert_protocol(
        databases
            .rotate_credentials(DatabaseKind::Postgres, 999, None)
            .await,
        "no rows",
    );

    let mut live_input = create_dto("Live", ENV_ID, None);
    live_input.database_password = Some("lib-initial-password".into());
    let libsql = databases
        .create(DatabaseKind::Libsql, live_input)
        .await
        .unwrap();
    sqlx::query("UPDATE services SET app_status = 'RUNNING' WHERE id = ?")
        .bind(libsql.id)
        .execute(&*db.pool)
        .await
        .unwrap();
    db.cache.invalidate(&CacheKey::Database(libsql.id)).await;
    assert_protocol(
        databases
            .rotate_credentials(
                DatabaseKind::Libsql,
                libsql.id,
                Some("rotated-password-123".into()),
            )
            .await,
        "stop libSQL",
    );
    assert_eq!(
        databases
            .get_by_id(DatabaseKind::Libsql, libsql.id)
            .await
            .unwrap()
            .database_password
            .as_deref(),
        Some("lib-initial-password")
    );
}

#[tokio::test]
async fn validate_configuration_reports_errors_and_warnings() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Valid", ENV_ID, None);
    input.external_port = Some(15432);
    let valid = databases
        .create(DatabaseKind::Postgres, input)
        .await
        .unwrap();
    let report = databases
        .validate_configuration(DatabaseKind::Postgres, valid.id)
        .await
        .unwrap();
    assert!(report.valid);
    assert!(report.errors.is_empty());
    assert_eq!(report.warnings, vec!["database is exposed on a host port"]);

    let plain = databases
        .create(DatabaseKind::Postgres, create_dto("Plain", ENV_ID, None))
        .await
        .unwrap();
    let report = databases
        .validate_configuration(DatabaseKind::Postgres, plain.id)
        .await
        .unwrap();
    assert!(report.valid);
    assert!(report.warnings.is_empty());

    let mut patch = patch_dto();
    patch.external_port = Some(70_000);
    patch.cpu_limit = Some("0".into());
    patch.cpu_reservation = Some("-2".into());
    patch.memory_limit = Some("12XB".into());
    patch.memory_reservation = Some("0M".into());
    patch.replicas = Some(0);
    patch.network_ids = Some(vec![]);
    patch.detach_rustploy_network = Some(1);
    databases
        .patch(DatabaseKind::Postgres, plain.id, patch)
        .await
        .unwrap();
    let report = databases
        .validate_configuration(DatabaseKind::Postgres, plain.id)
        .await
        .unwrap();
    assert!(!report.valid);
    for expected in [
        "external_port must be between 1 and 65535",
        "cpu_limit must be a positive CPU value",
        "cpu_reservation must be a positive CPU value",
        "memory_limit must use a positive Docker memory value",
        "memory_reservation must use a positive Docker memory value",
        "replicas must be at least 1",
    ] {
        assert!(
            report.errors.iter().any(|error| error == expected),
            "missing {expected:?} in {:?}",
            report.errors
        );
    }
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("detached from openoxide-network"))
    );

    let mut patch = patch_dto();
    patch.external_port = Some(15433);
    patch.cpu_limit = Some("1.5".into());
    patch.cpu_reservation = Some("0.5".into());
    patch.memory_limit = Some("1G".into());
    patch.memory_reservation = Some("512MB".into());
    patch.replicas = Some(2);
    patch.network_ids = Some(vec!["1".into()]);
    patch.detach_rustploy_network = Some(0);
    databases
        .patch(DatabaseKind::Postgres, plain.id, patch)
        .await
        .unwrap();
    let report = databases
        .validate_configuration(DatabaseKind::Postgres, plain.id)
        .await
        .unwrap();
    assert!(report.valid, "{:?}", report.errors);
    assert!(report.errors.is_empty());

    assert_protocol(
        databases
            .validate_configuration(DatabaseKind::Postgres, 999)
            .await,
        "no rows",
    );
}

#[tokio::test]
async fn delete_requires_clear_dependencies_and_cascades() {
    let db = TestDb::new().await;
    let databases = db.database();
    let created = databases
        .create(
            DatabaseKind::Postgres,
            create_dto("Disposable", ENV_ID, None),
        )
        .await
        .unwrap();

    let dependencies = databases
        .dependencies(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(dependencies.active_deployments, 0);
    assert_eq!(dependencies.enabled_backups, 0);
    assert!(!dependencies.blocks_delete());

    let running = db
        .insert_deployment(
            Some(created.id),
            None,
            "RUNNING",
            "DEPLOY",
            0,
            "/tmp/d",
            USER_ID,
        )
        .await;
    let dependencies = databases
        .dependencies(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(dependencies.active_deployments, 1);
    assert!(dependencies.blocks_delete());
    assert_protocol(
        databases.delete(DatabaseKind::Postgres, created.id).await,
        "active_deployments=1",
    );

    sqlx::query("UPDATE deployments SET status = 'DONE', state = 'DONE' WHERE id = ?")
        .bind(running)
        .execute(&*db.pool)
        .await
        .unwrap();
    let backup = db.insert_volume_backup(created.id, true).await;
    let dependencies = databases
        .dependencies(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(dependencies.active_deployments, 0);
    assert_eq!(dependencies.deployments, 1);
    assert_eq!(dependencies.backups, 1);
    assert_eq!(dependencies.enabled_backups, 1);
    assert_protocol(
        databases.delete(DatabaseKind::Postgres, created.id).await,
        "enabled_backups=1",
    );

    sqlx::query("UPDATE volume_backups SET enabled = 0 WHERE id = ?")
        .bind(backup)
        .execute(&*db.pool)
        .await
        .unwrap();
    db.database_mount()
        .create(
            DatabaseKind::Postgres,
            created.id,
            CreateMountRequest {
                mount_type: "VOLUME".into(),
                volume_name: Some("data".into()),
                mount_path: "/data".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let dependencies = databases
        .dependencies(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(dependencies.mounts, 1);
    assert!(!dependencies.blocks_delete());

    databases
        .delete(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    let service_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM services WHERE id = ?")
        .bind(created.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(service_rows, 0);
    let database_rows: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM service_database WHERE service_id = ?")
            .bind(created.id)
            .fetch_one(&*db.pool)
            .await
            .unwrap();
    assert_eq!(database_rows, 0);
    let mount_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM mounts WHERE service_id = ?")
        .bind(created.id)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(mount_rows, 0);
    assert_protocol(
        databases
            .get_by_id(DatabaseKind::Postgres, created.id)
            .await,
        "no rows",
    );
    assert_protocol(
        databases.delete(DatabaseKind::Postgres, created.id).await,
        "no rows",
    );
    assert_protocol(
        databases
            .dependencies(DatabaseKind::Postgres, created.id)
            .await,
        "no rows",
    );
}

#[tokio::test]
async fn mounts_validate_crud_and_scope() {
    let db = TestDb::new().await;
    let databases = db.database();
    let mounts = db.database_mount();
    let created = databases
        .create(DatabaseKind::Postgres, create_dto("Mounted", ENV_ID, None))
        .await
        .unwrap();

    assert!(
        mounts
            .list(DatabaseKind::Postgres, created.id)
            .await
            .unwrap()
            .is_empty()
    );

    let bind = mounts
        .create(
            DatabaseKind::Postgres,
            created.id,
            CreateMountRequest {
                mount_type: "BIND".into(),
                host_path: Some(" /srv/pg ".into()),
                mount_path: " /var/lib/pg ".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(bind.mount_type, "BIND");
    assert_eq!(bind.service_type, "DATABASE");
    assert_eq!(bind.host_path.as_deref(), Some("/srv/pg"));
    assert_eq!(bind.volume_name.as_deref(), Some(""));
    assert_eq!(bind.file_path.as_deref(), Some(""));
    assert_eq!(bind.content.as_deref(), Some(""));
    assert_eq!(bind.mount_path, "/var/lib/pg");

    let volume = mounts
        .create(
            DatabaseKind::Postgres,
            created.id,
            CreateMountRequest {
                mount_type: "VOLUME".into(),
                volume_name: Some(" pgdata ".into()),
                mount_path: "/data".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(volume.volume_name.as_deref(), Some("pgdata"));
    assert_eq!(volume.host_path.as_deref(), Some(""));

    let file = mounts
        .create(
            DatabaseKind::Postgres,
            created.id,
            CreateMountRequest {
                mount_type: "FILE".into(),
                file_path: Some(" /etc/pg.conf ".into()),
                content: Some("listen = true".into()),
                mount_path: "/etc/pg.conf".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(file.file_path.as_deref(), Some("/etc/pg.conf"));
    assert_eq!(file.content.as_deref(), Some("listen = true"));
    assert_eq!(file.volume_name.as_deref(), Some(""));

    assert_protocol(
        mounts
            .create(
                DatabaseKind::Postgres,
                created.id,
                CreateMountRequest {
                    mount_type: "BIND".into(),
                    mount_path: "/data".into(),
                    ..Default::default()
                },
            )
            .await,
        "host_path is required",
    );
    assert_protocol(
        mounts
            .create(
                DatabaseKind::Postgres,
                created.id,
                CreateMountRequest {
                    mount_type: "BIND".into(),
                    host_path: Some("relative".into()),
                    mount_path: "/data".into(),
                    ..Default::default()
                },
            )
            .await,
        "host_path must be absolute",
    );
    assert_protocol(
        mounts
            .create(DatabaseKind::Postgres, created.id, mount_request("VOLUME"))
            .await,
        "volume_name is required",
    );
    assert_protocol(
        mounts
            .create(DatabaseKind::Postgres, created.id, mount_request("FILE"))
            .await,
        "file_path is required",
    );
    assert_protocol(
        mounts
            .create(
                DatabaseKind::Postgres,
                created.id,
                CreateMountRequest {
                    mount_type: "FILE".into(),
                    file_path: Some("/etc/x".into()),
                    mount_path: "/etc/x".into(),
                    ..Default::default()
                },
            )
            .await,
        "content is required for FILE mounts",
    );
    assert_protocol(
        mounts
            .create(
                DatabaseKind::Postgres,
                created.id,
                CreateMountRequest {
                    mount_type: "VOLUME".into(),
                    volume_name: Some("data".into()),
                    mount_path: "data".into(),
                    ..Default::default()
                },
            )
            .await,
        "mount_path must be an absolute container path",
    );
    assert_protocol(
        mounts
            .create(DatabaseKind::Postgres, created.id, mount_request("NOPE"))
            .await,
        "invalid mount type",
    );

    let listed = mounts
        .list(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(listed.len(), 3);

    let updated = mounts
        .update(
            DatabaseKind::Postgres,
            created.id,
            file.id,
            UpdateMountRequest {
                mount_type: "VOLUME".into(),
                volume_name: Some("renamed".into()),
                mount_path: "/var/data".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.mount_type, "VOLUME");
    assert_eq!(updated.volume_name.as_deref(), Some("renamed"));
    assert_eq!(updated.file_path.as_deref(), Some(""));
    assert_eq!(updated.content.as_deref(), Some(""));

    assert!(matches!(
        mounts
            .update(
                DatabaseKind::Postgres,
                created.id,
                999,
                UpdateMountRequest {
                    mount_type: "VOLUME".into(),
                    volume_name: Some("gone".into()),
                    mount_path: "/data".into(),
                    ..Default::default()
                },
            )
            .await,
        Err(sqlx::Error::RowNotFound)
    ));

    let other = databases
        .create(DatabaseKind::Postgres, create_dto("Other", ENV_ID, None))
        .await
        .unwrap();
    assert!(matches!(
        mounts
            .update(
                DatabaseKind::Postgres,
                other.id,
                bind.id,
                UpdateMountRequest {
                    mount_type: "VOLUME".into(),
                    volume_name: Some("other".into()),
                    mount_path: "/data".into(),
                    ..Default::default()
                },
            )
            .await,
        Err(sqlx::Error::RowNotFound)
    ));

    assert!(
        mounts
            .delete(DatabaseKind::Postgres, created.id, bind.id)
            .await
            .unwrap()
    );
    assert!(
        !mounts
            .delete(DatabaseKind::Postgres, created.id, bind.id)
            .await
            .unwrap()
    );

    let application = db.insert_service("APPLICATION", "IDLE", ENV_ID, None).await;
    assert_protocol(
        mounts.list(DatabaseKind::Postgres, application).await,
        "no rows",
    );
    assert_protocol(mounts.list(DatabaseKind::Postgres, 999).await, "no rows");
    assert_protocol(
        mounts
            .create(DatabaseKind::Postgres, 999, mount_request("VOLUME"))
            .await,
        "no rows",
    );
}

#[tokio::test]
async fn swarm_settings_roundtrip() {
    let db = TestDb::new().await;
    let databases = db.database();
    let settings = db.swarm_settings();
    let created = databases
        .create(DatabaseKind::Postgres, create_dto("Swarm", ENV_ID, None))
        .await
        .unwrap();

    let empty = settings
        .get(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(empty.health_check_swarm, None);
    assert_eq!(empty.mode_swarm, None);
    assert_eq!(empty.stop_grace_period_swarm, None);

    let value = DatabaseSwarmSettingsDto {
        health_check_swarm: Some(r#"{"test":["CMD","pg_isready"]}"#.into()),
        restart_policy_swarm: Some(r#"{"condition":"on-failure"}"#.into()),
        placement_swarm: Some(r#"{"constraints":["node.role==worker"]}"#.into()),
        update_config_swarm: Some(r#"{"parallelism":2}"#.into()),
        rollback_config_swarm: Some(r#"{"parallelism":1}"#.into()),
        mode_swarm: Some("replicated".into()),
        labels_swarm: Some(r#"{"tier":"db"}"#.into()),
        network_swarm: Some(r#"{"target":"backend"}"#.into()),
        endpoint_spec_swarm: Some(r#"{"mode":"vip"}"#.into()),
        ulimits_swarm: Some(r#"{"nofile":65536}"#.into()),
        stop_grace_period_swarm: Some(30),
    };
    settings
        .update(DatabaseKind::Postgres, created.id, value.clone())
        .await
        .unwrap();
    let stored = settings
        .get(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(stored.health_check_swarm, value.health_check_swarm);
    assert_eq!(stored.restart_policy_swarm, value.restart_policy_swarm);
    assert_eq!(stored.placement_swarm, value.placement_swarm);
    assert_eq!(stored.update_config_swarm, value.update_config_swarm);
    assert_eq!(stored.rollback_config_swarm, value.rollback_config_swarm);
    assert_eq!(stored.mode_swarm, value.mode_swarm);
    assert_eq!(stored.labels_swarm, value.labels_swarm);
    assert_eq!(stored.network_swarm, value.network_swarm);
    assert_eq!(stored.endpoint_spec_swarm, value.endpoint_spec_swarm);
    assert_eq!(stored.ulimits_swarm, value.ulimits_swarm);
    assert_eq!(stored.stop_grace_period_swarm, Some(30));

    settings
        .update(DatabaseKind::Mysql, created.id, swarm_settings())
        .await
        .unwrap();
    let cleared = settings
        .get(DatabaseKind::Postgres, created.id)
        .await
        .unwrap();
    assert_eq!(cleared.health_check_swarm.as_deref(), Some(""));
    assert_eq!(cleared.mode_swarm.as_deref(), Some(""));
    assert_eq!(cleared.stop_grace_period_swarm, Some(0));

    assert!(matches!(
        settings.get(DatabaseKind::Postgres, 999).await,
        Err(sqlx::Error::RowNotFound)
    ));
    settings
        .update(DatabaseKind::Postgres, 999, swarm_settings())
        .await
        .unwrap();
}

#[tokio::test]
async fn export_and_import_roundtrip() {
    let db = TestDb::new().await;
    let databases = db.database();

    let mut input = create_dto("Shop DB", ENV_ID, Some(SERVER_ID));
    input.description = Some("prod".into());
    input.docker_image = Some("mysql:8".into());
    input.database_name = Some("shop".into());
    input.database_user = Some("shopuser".into());
    input.database_password = Some("shop-password".into());
    input.database_root_password = Some("root-password".into());
    input.external_port = Some(13306);
    input.env_var = Some("MYSQL_EXTRA=1".into());
    input.network_ids = Some(vec!["2".into()]);
    let source = databases.create(DatabaseKind::Mysql, input).await.unwrap();
    let mut patch = patch_dto();
    patch.memory_reservation = Some("256M".into());
    patch.memory_limit = Some("1G".into());
    patch.cpu_reservation = Some("0.5".into());
    patch.cpu_limit = Some("2".into());
    patch.replicas = Some(2);
    databases
        .patch(DatabaseKind::Mysql, source.id, patch)
        .await
        .unwrap();

    let secret_bundle = databases
        .export(DatabaseKind::Mysql, source.id, true)
        .await
        .unwrap();
    assert_eq!(secret_bundle.schema_version, 1);
    assert!(secret_bundle.exported_at > 0);
    assert!(secret_bundle.secrets_included);
    assert_eq!(secret_bundle.kind, DatabaseKind::Mysql);
    assert_eq!(secret_bundle.name, "Shop DB");
    assert_eq!(secret_bundle.description.as_deref(), Some("prod"));
    assert_eq!(secret_bundle.docker_image, "mysql:8");
    assert_eq!(secret_bundle.database_name.as_deref(), Some("shop"));
    assert_eq!(secret_bundle.database_user.as_deref(), Some("shopuser"));
    assert_eq!(
        secret_bundle.database_password.as_deref(),
        Some("shop-password")
    );
    assert_eq!(secret_bundle.external_port, Some(13306));
    assert_eq!(secret_bundle.env_var.as_deref(), Some("MYSQL_EXTRA=1"));
    assert_eq!(secret_bundle.memory_reservation.as_deref(), Some("256M"));
    assert_eq!(secret_bundle.memory_limit.as_deref(), Some("1G"));
    assert_eq!(secret_bundle.cpu_reservation.as_deref(), Some("0.5"));
    assert_eq!(secret_bundle.cpu_limit.as_deref(), Some("2"));
    assert_eq!(secret_bundle.replicas, 2);
    assert_eq!(secret_bundle.network_ids, vec!["2".to_string()]);
    assert_eq!(secret_bundle.detach_rustploy_network, 0);

    let public_bundle = databases
        .export(DatabaseKind::Mysql, source.id, false)
        .await
        .unwrap();
    assert!(!public_bundle.secrets_included);
    assert_eq!(public_bundle.database_password, None);
    assert_eq!(public_bundle.env_var, None);
    assert_eq!(public_bundle.database_name.as_deref(), Some("shop"));
    assert_eq!(public_bundle.replicas, 2);

    let imported = databases
        .import(ImportDatabaseDto {
            archive: serde_json::to_string(&secret_bundle).unwrap(),
            environment_id: STAGING_ENV_ID,
            server_id: Some(SERVER_ID_2),
            name: Some("Imported Shop".into()),
        })
        .await
        .unwrap();
    assert_ne!(imported.id, source.id);
    assert_eq!(imported.kind, DatabaseKind::Mysql);
    assert_eq!(imported.name, "Imported Shop");
    assert!(imported.app_name.starts_with("mysql-imported-shop-"));
    assert_eq!(imported.environment_id, STAGING_ENV_ID);
    assert_eq!(imported.server_id, Some(SERVER_ID_2));
    assert_eq!(imported.description.as_deref(), Some("prod"));
    assert_eq!(imported.docker_image, "mysql:8");
    assert_eq!(imported.database_name.as_deref(), Some("shop"));
    assert_eq!(imported.database_user.as_deref(), Some("shopuser"));
    assert_eq!(imported.database_password.as_deref(), Some("shop-password"));
    assert_eq!(imported.external_port, Some(13306));
    assert_eq!(imported.env_var.as_deref(), Some("MYSQL_EXTRA=1"));
    assert_eq!(imported.memory_reservation.as_deref(), Some("256M"));
    assert_eq!(imported.memory_limit.as_deref(), Some("1G"));
    assert_eq!(imported.cpu_reservation.as_deref(), Some("0.5"));
    assert_eq!(imported.cpu_limit.as_deref(), Some("2"));
    assert_eq!(imported.replicas, 2);
    assert_eq!(imported.network_ids, "[\"2\"]");
    let engine = engine_config(&db, imported.id).await;
    assert_random_secret(engine["database_root_password"].as_str().unwrap());

    let public_import = databases
        .import(ImportDatabaseDto {
            archive: serde_json::to_string(&public_bundle).unwrap(),
            environment_id: STAGING_ENV_ID,
            server_id: None,
            name: None,
        })
        .await
        .unwrap();
    assert_eq!(public_import.name, "Shop DB");
    assert_eq!(public_import.server_id, None);
    assert_eq!(public_import.env_var.as_deref(), Some(""));
    let password = public_import.database_password.as_deref().unwrap();
    assert_ne!(password, "shop-password");
    assert_random_secret(password);
}

#[tokio::test]
async fn import_rejects_invalid_archives() {
    let db = TestDb::new().await;
    let databases = db.database();
    let source = databases
        .create(DatabaseKind::Redis, create_dto("Cache", ENV_ID, None))
        .await
        .unwrap();
    let mut bundle = databases
        .export(DatabaseKind::Redis, source.id, true)
        .await
        .unwrap();

    assert_protocol(
        databases
            .import(ImportDatabaseDto {
                archive: "not-json".into(),
                environment_id: ENV_ID,
                server_id: None,
                name: None,
            })
            .await,
        "invalid database archive",
    );

    bundle.schema_version = 2;
    assert_protocol(
        databases
            .import(ImportDatabaseDto {
                archive: serde_json::to_string(&bundle).unwrap(),
                environment_id: ENV_ID,
                server_id: None,
                name: None,
            })
            .await,
        "unsupported database archive schema version: 2",
    );
}

#[tokio::test]
async fn operations_guard_active_deployments() {
    let db = TestDb::new().await;
    let databases = db.database();
    let created = databases
        .create(DatabaseKind::Postgres, create_dto("Ops", ENV_ID, None))
        .await
        .unwrap();

    let queued = db
        .insert_deployment(
            Some(created.id),
            None,
            "QUEUED",
            "QUEUE",
            0,
            "/tmp/q",
            USER_ID,
        )
        .await;
    let running = db
        .insert_deployment(
            Some(created.id),
            None,
            "RUNNING",
            "DEPLOY",
            0,
            "/tmp/r",
            USER_ID,
        )
        .await;

    for operation in [
        DatabaseOperation::Deploy,
        DatabaseOperation::Redeploy,
        DatabaseOperation::Reload,
        DatabaseOperation::Start,
    ] {
        assert_protocol(
            databases
                .run_operation(DatabaseKind::Postgres, created.id, operation, USER_ID)
                .await,
            "already queued or running",
        );
    }

    // The stop/cancel path resolves the process-global BuilderQueue, which
    // opens the real database pool; only the synchronous guard is unit-testable.
    let queued_status: String = sqlx::query_scalar("SELECT status FROM deployments WHERE id = ?")
        .bind(queued)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(queued_status, "QUEUED");
    let running_status: String = sqlx::query_scalar("SELECT status FROM deployments WHERE id = ?")
        .bind(running)
        .fetch_one(&*db.pool)
        .await
        .unwrap();
    assert_eq!(running_status, "RUNNING");
}
