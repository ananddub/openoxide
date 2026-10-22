use std::time::Duration;

use crate::services::compute::engine::{
    CancelDeploymentResult, DeploymentListFilter, LogSearchOptions,
};
use crate::services::compute::test_support::*;
use crate::utils::builder::custom_type::{DeployState, IdType};
use crate::utils::builder::hash_state::ApplicationState;

use super::assert_protocol;

fn temp_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("openoxide-engine-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_log(dir: &std::path::Path, name: &str, contents: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, contents).unwrap();
    path.to_string_lossy().into_owned()
}

async fn deployment_state(db: &TestDb, id: i64) -> (String, String, Option<i64>) {
    sqlx::query_as("SELECT status, state, finished_at FROM deployments WHERE id = ?")
        .bind(id)
        .fetch_one(&*db.pool)
        .await
        .unwrap()
}

async fn list_ids(
    deployments: &crate::services::compute::engine::DeploymentService,
    filter: DeploymentListFilter,
) -> Vec<i64> {
    deployments
        .list(filter)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.id)
        .collect()
}

async fn list_org_ids(
    deployments: &crate::services::compute::engine::DeploymentService,
    organization_id: i64,
    filter: DeploymentListFilter,
) -> Vec<i64> {
    deployments
        .list_for_organization(organization_id, filter)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.id)
        .collect()
}

#[tokio::test]
async fn get_by_id_and_actor_email_lookup() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let id = db
        .insert_deployment(
            Some(service),
            Some(SERVER_ID),
            "SUCCESS",
            "DONE",
            100,
            "",
            USER_ID,
        )
        .await;

    let row = deployments.get_by_id(id).await.unwrap();
    assert_eq!(row.id, id);
    assert_eq!(row.status, "SUCCESS");
    assert_eq!(row.service_id, Some(service));
    assert!(matches!(
        deployments.get_by_id(999).await,
        Err(sqlx::Error::RowNotFound)
    ));

    assert_eq!(deployments.actor_email(0).await, None);
    assert_eq!(
        deployments.actor_email(USER_ID).await.as_deref(),
        Some("dev@acme.test")
    );
    assert_eq!(
        deployments.actor_email(3).await.as_deref(),
        Some("ops@acme.test")
    );
    assert_eq!(deployments.actor_email(999).await, None);
}

#[tokio::test]
async fn list_filters_normalizes_and_paginates() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;

    let oldest = db
        .insert_deployment(
            Some(service),
            Some(SERVER_ID),
            "SUCCESS",
            "DONE",
            100,
            "",
            USER_ID,
        )
        .await;
    let second = db
        .insert_deployment(
            Some(service),
            Some(SERVER_ID),
            "RUNNING",
            "RUNNING",
            200,
            "",
            USER_ID,
        )
        .await;
    let third = db
        .insert_deployment(
            Some(service),
            Some(SERVER_ID_2),
            "FAILED",
            "FAILED",
            300,
            "",
            USER_ID,
        )
        .await;
    let newest = db
        .insert_deployment(
            Some(service),
            Some(SERVER_ID_2),
            "RUNNING",
            "RUNNING",
            400,
            "",
            USER_ID,
        )
        .await;

    let all = list_ids(
        &deployments,
        DeploymentListFilter {
            application_id: Some(service),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(all, vec![newest, third, second, oldest]);

    let running = list_ids(
        &deployments,
        DeploymentListFilter {
            status: Some(" running ".into()),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(running, vec![newest, second]);

    let done = list_ids(
        &deployments,
        DeploymentListFilter {
            state: Some(" done ".into()),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(done, vec![oldest]);

    let combined = list_ids(
        &deployments,
        DeploymentListFilter {
            status: Some("running".into()),
            state: Some("RUNNING".into()),
            server_id: Some(SERVER_ID_2),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(combined, vec![newest]);

    let by_server = list_ids(
        &deployments,
        DeploymentListFilter {
            server_id: Some(SERVER_ID_2),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(by_server, vec![newest, third]);

    let page = list_ids(
        &deployments,
        DeploymentListFilter {
            application_id: Some(service),
            limit: 2,
            offset: 1,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(page, vec![third, second]);

    // A zero limit clamps up to one and a negative offset clamps down to zero.
    let clamped_limit = list_ids(
        &deployments,
        DeploymentListFilter {
            application_id: Some(service),
            limit: 0,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(clamped_limit, vec![newest]);

    let clamped_offset = list_ids(
        &deployments,
        DeploymentListFilter {
            application_id: Some(service),
            limit: 2,
            offset: -5,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(clamped_offset, vec![newest, third]);

    let none = list_ids(
        &deployments,
        DeploymentListFilter {
            status: Some("NOPE".into()),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert!(none.is_empty());

    let unknown_service = list_ids(
        &deployments,
        DeploymentListFilter {
            application_id: Some(999),
            limit: 10,
            ..Default::default()
        },
    )
    .await;
    assert!(unknown_service.is_empty());

    let running_rows = deployments.list_running(10, 0).await.unwrap();
    let running_ids = running_rows.iter().map(|row| row.id).collect::<Vec<_>>();
    assert_eq!(running_ids, vec![newest, second]);
}

#[tokio::test]
async fn cancel_handles_statuses_and_process_state() {
    let _guard = state_guard().await;
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let state = auto_di::resolve::<ApplicationState>().await.unwrap();

    let queued = db
        .insert_deployment(None, None, "QUEUED", "QUEUE", 10, "", USER_ID)
        .await;
    assert_eq!(
        deployments.cancel(queued).await.unwrap(),
        CancelDeploymentResult::CancelRequested
    );
    let (status, deploy_state, finished_at) = deployment_state(&db, queued).await;
    assert_eq!(status, "CANCELLED");
    assert_eq!(deploy_state, "CANCELLED");
    assert!(finished_at.is_some());

    assert!(matches!(
        deployments.cancel(9999).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let finished = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 11, "", USER_ID)
        .await;
    assert_eq!(
        deployments.cancel(finished).await.unwrap(),
        CancelDeploymentResult::NotRunning
    );

    let orphan = db
        .insert_deployment(None, None, "RUNNING", "RUNNING", 12, "", USER_ID)
        .await;
    assert_eq!(
        deployments.cancel(orphan).await.unwrap(),
        CancelDeploymentResult::NotCancellable
    );

    // Simulate a dangling service reference for the missing-service branch.
    sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&*db.pool)
        .await
        .unwrap();
    let missing_service = db
        .insert_deployment(Some(9999), None, "RUNNING", "RUNNING", 13, "", USER_ID)
        .await;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&*db.pool)
        .await
        .unwrap();
    assert_eq!(
        deployments.cancel(missing_service).await.unwrap(),
        CancelDeploymentResult::NotCancellable
    );

    let app_service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let app_deployment = db
        .insert_deployment(
            Some(app_service),
            Some(SERVER_ID),
            "RUNNING",
            "RUNNING",
            14,
            "",
            USER_ID,
        )
        .await;
    state.add_default(IdType::AppId(app_service), ENV_ID, PROJECT_ID);
    assert_eq!(
        deployments.cancel(app_deployment).await.unwrap(),
        CancelDeploymentResult::CancelRequested
    );
    assert!(
        state
            .cancellation_token(IdType::AppId(app_service))
            .unwrap()
            .is_cancelled()
    );
    let (status, deploy_state, _) = deployment_state(&db, app_deployment).await;
    assert_eq!(status, "CANCELLING");
    assert_eq!(deploy_state, "CANCEL_REQUESTED");

    // A RUNNING deployment whose component state lives in another process is
    // reported back without touching the row.
    let idle_service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let idle_deployment = db
        .insert_deployment(
            Some(idle_service),
            Some(SERVER_ID),
            "RUNNING",
            "RUNNING",
            15,
            "",
            USER_ID,
        )
        .await;
    assert_eq!(
        deployments.cancel(idle_deployment).await.unwrap(),
        CancelDeploymentResult::NotActiveInThisProcess
    );
    let (status, deploy_state, _) = deployment_state(&db, idle_deployment).await;
    assert_eq!(status, "RUNNING");
    assert_eq!(deploy_state, "RUNNING");

    // Compose and database components map to their own IdType variants.
    let compose_service = db
        .insert_service("COMPOSE", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let compose_deployment = db
        .insert_deployment(
            Some(compose_service),
            Some(SERVER_ID),
            "RUNNING",
            "RUNNING",
            16,
            "",
            USER_ID,
        )
        .await;
    state.add_default(IdType::ComposeId(compose_service), ENV_ID, PROJECT_ID);
    assert_eq!(
        deployments.cancel(compose_deployment).await.unwrap(),
        CancelDeploymentResult::CancelRequested
    );
    assert!(
        state
            .cancellation_token(IdType::ComposeId(compose_service))
            .unwrap()
            .is_cancelled()
    );

    let database_service = db
        .insert_service("DATABASE", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let database_deployment = db
        .insert_deployment(
            Some(database_service),
            Some(SERVER_ID),
            "RUNNING",
            "RUNNING",
            17,
            "",
            USER_ID,
        )
        .await;
    state.add_default(IdType::DatabaseId(database_service), ENV_ID, PROJECT_ID);
    assert_eq!(
        deployments.cancel(database_deployment).await.unwrap(),
        CancelDeploymentResult::CancelRequested
    );
    assert!(
        state
            .cancellation_token(IdType::DatabaseId(database_service))
            .unwrap()
            .is_cancelled()
    );

    state.remove_state(IdType::AppId(app_service));
    state.remove_state(IdType::ComposeId(compose_service));
    state.remove_state(IdType::DatabaseId(database_service));
}

#[tokio::test]
async fn active_components_and_subscriptions_track_state() {
    let _guard = state_guard().await;
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let state = auto_di::resolve::<ApplicationState>().await.unwrap();
    let service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    state.add_default(IdType::AppId(service), ENV_ID, PROJECT_ID);

    let snapshot = deployments.list_active_components().await.unwrap();
    let entry = snapshot
        .iter()
        .find(|entry| entry.id == IdType::AppId(service))
        .expect("active component is exposed");
    assert_eq!(entry.project_id, PROJECT_ID);
    assert_eq!(entry.env_id, ENV_ID);
    assert_eq!(entry.state, DeployState::Queue);

    let mut subscription = deployments
        .subscribe_component(IdType::AppId(service))
        .await
        .unwrap()
        .expect("component has a subscription");
    assert_eq!(*subscription.state.borrow(), DeployState::Queue);

    state
        .send_state(IdType::AppId(service), DeployState::Deployed)
        .unwrap();
    subscription.state.changed().await.unwrap();
    assert_eq!(*subscription.state.borrow(), DeployState::Deployed);
    assert_eq!(
        subscription.events.recv().await.unwrap(),
        crate::utils::builder::custom_type::DeployEvent::StateChanged(DeployState::Deployed)
    );

    assert!(
        deployments
            .subscribe_component(IdType::AppId(9999))
            .await
            .unwrap()
            .is_none()
    );
    state.remove_state(IdType::AppId(service));
}

#[tokio::test]
async fn delete_removes_rows_and_log_files() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let dir = temp_dir();
    let log_path = write_log(&dir, "deploy.log", "boom");

    let id = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 10, &log_path, USER_ID)
        .await;
    assert!(deployments.delete(id).await.unwrap());
    assert!(matches!(
        deployments.get_by_id(id).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert!(!std::path::Path::new(&log_path).exists());
    assert!(!deployments.delete(id).await.unwrap());

    let missing_log = db
        .insert_deployment(
            None,
            None,
            "FAILED",
            "FAILED",
            11,
            "/definitely/missing/deploy.log",
            USER_ID,
        )
        .await;
    assert!(deployments.delete(missing_log).await.unwrap());

    let queued = db
        .insert_deployment(None, None, "QUEUED", "QUEUE", 12, "", USER_ID)
        .await;
    assert!(deployments.delete(queued).await.unwrap());

    let running = db
        .insert_deployment(None, None, "RUNNING", "RUNNING", 13, "", USER_ID)
        .await;
    assert!(deployments.delete(running).await.unwrap());

    assert!(
        deployments
            .list(DeploymentListFilter {
                limit: 200,
                ..Default::default()
            })
            .await
            .unwrap()
            .is_empty()
    );

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn clear_all_scopes_by_service_server_and_global() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let dir = temp_dir();
    let service_one = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let service_two = db
        .insert_service("COMPOSE", "RUNNING", ENV_ID, Some(SERVER_ID_2))
        .await;

    let one_path = write_log(&dir, "one.log", "one");
    let running_path = write_log(&dir, "running.log", "running");
    let one_server_two_path = write_log(&dir, "one-server-two.log", "two");
    let two_path = write_log(&dir, "two.log", "three");
    let global_path = write_log(&dir, "global.log", "four");
    let extra_path = write_log(&dir, "extra.log", "five");

    let one = db
        .insert_deployment(
            Some(service_one),
            Some(SERVER_ID),
            "SUCCESS",
            "DONE",
            100,
            &one_path,
            USER_ID,
        )
        .await;
    let running = db
        .insert_deployment(
            Some(service_one),
            Some(SERVER_ID),
            "RUNNING",
            "RUNNING",
            200,
            &running_path,
            USER_ID,
        )
        .await;
    let one_server_two = db
        .insert_deployment(
            Some(service_one),
            Some(SERVER_ID_2),
            "FAILED",
            "FAILED",
            300,
            &one_server_two_path,
            USER_ID,
        )
        .await;
    let two = db
        .insert_deployment(
            Some(service_two),
            Some(SERVER_ID_2),
            "SUCCESS",
            "DONE",
            400,
            &two_path,
            USER_ID,
        )
        .await;
    let global = db
        .insert_deployment(
            None,
            None,
            "CANCELLED",
            "CANCELLED",
            500,
            &global_path,
            USER_ID,
        )
        .await;

    assert_eq!(
        deployments
            .clear_all(Some(service_one), None, None, Some(SERVER_ID_2))
            .await
            .unwrap(),
        1
    );
    assert!(!std::path::Path::new(&one_server_two_path).exists());
    assert!(std::path::Path::new(&one_path).exists());
    assert!(std::path::Path::new(&two_path).exists());
    assert!(matches!(
        deployments.get_by_id(one_server_two).await,
        Err(sqlx::Error::RowNotFound)
    ));

    assert_eq!(
        deployments
            .clear_all(Some(service_one), None, None, None)
            .await
            .unwrap(),
        1
    );
    assert!(!std::path::Path::new(&one_path).exists());
    assert!(std::path::Path::new(&running_path).exists());
    assert!(matches!(
        deployments.get_by_id(one).await,
        Err(sqlx::Error::RowNotFound)
    ));

    assert_eq!(
        deployments
            .clear_all(None, Some(service_two), None, None)
            .await
            .unwrap(),
        1
    );
    assert!(!std::path::Path::new(&two_path).exists());
    assert!(matches!(
        deployments.get_by_id(two).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let extra = db
        .insert_deployment(
            None,
            Some(SERVER_ID_2),
            "SUCCESS",
            "DONE",
            600,
            &extra_path,
            USER_ID,
        )
        .await;
    assert_eq!(
        deployments
            .clear_all(None, None, None, Some(SERVER_ID_2))
            .await
            .unwrap(),
        1
    );
    assert!(!std::path::Path::new(&extra_path).exists());
    assert!(matches!(
        deployments.get_by_id(extra).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert!(matches!(
        deployments.get_by_id(running).await,
        Ok(row) if row.status == "RUNNING"
    ));

    assert_eq!(
        deployments.clear_all(None, None, None, None).await.unwrap(),
        1
    );
    assert!(!std::path::Path::new(&global_path).exists());
    assert!(matches!(
        deployments.get_by_id(global).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let remaining = deployments
        .list(DeploymentListFilter {
            limit: 200,
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, running);
    assert_eq!(remaining[0].status, "RUNNING");

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn log_read_search_and_cleanup() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let dir = temp_dir();
    let path_a = write_log(&dir, "a.log", "alpha\nError: boom\nomega");
    let path_b = write_log(&dir, "b.log", "beta line\nerror again\n");
    let a = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 100, &path_a, USER_ID)
        .await;
    let b = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 200, &path_b, USER_ID)
        .await;
    for (id, title) in [(a, "App A"), (b, "DB B")] {
        sqlx::query("UPDATE deployments SET title = ? WHERE id = ?")
            .bind(title)
            .bind(id)
            .execute(&*db.pool)
            .await
            .unwrap();
    }

    assert_eq!(
        deployments.read_deployment_log(a).await.unwrap(),
        "alpha\nError: boom\nomega"
    );
    let missing_file = db
        .insert_deployment(
            None,
            None,
            "SUCCESS",
            "DONE",
            300,
            "/definitely/missing/log.txt",
            USER_ID,
        )
        .await;
    assert_protocol(
        deployments.read_deployment_log(missing_file).await,
        "could not read log file",
    );
    assert!(matches!(
        deployments.read_deployment_log(9999).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let matches = deployments
        .search_logs(LogSearchOptions {
            query: Some("ERROR".into()),
            limit: 10,
        })
        .await
        .unwrap();
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].deployment_id, b);
    assert_eq!(matches[0].title, "DB B");
    assert_eq!(matches[0].line_number, 2);
    assert_eq!(matches[0].line, "error again");
    assert_eq!(matches[1].deployment_id, a);
    assert_eq!(matches[1].title, "App A");
    assert_eq!(matches[1].line_number, 2);
    assert_eq!(matches[1].line, "Error: boom");

    let limited = deployments
        .search_logs(LogSearchOptions {
            query: None,
            limit: 2,
        })
        .await
        .unwrap();
    assert_eq!(limited.len(), 2);
    assert_eq!(limited[0].line, "beta line");
    assert_eq!(limited[1].line, "error again");

    let no_matches = deployments
        .search_logs(LogSearchOptions {
            query: Some("zzz".into()),
            limit: 10,
        })
        .await
        .unwrap();
    assert!(no_matches.is_empty());

    sqlx::query("UPDATE deployments SET finished_at = ? WHERE id = ?")
        .bind(50_i64)
        .bind(a)
        .execute(&*db.pool)
        .await
        .unwrap();
    sqlx::query("UPDATE deployments SET finished_at = ? WHERE id = ?")
        .bind(150_i64)
        .bind(b)
        .execute(&*db.pool)
        .await
        .unwrap();
    let path_c = write_log(&dir, "c.log", "gamma");
    let path_d = write_log(&dir, "d.log", "delta");
    let c = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 10, &path_c, USER_ID)
        .await;
    let d = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 200, &path_d, USER_ID)
        .await;

    assert_eq!(deployments.cleanup_logs_before(100).await.unwrap(), 2);
    assert!(!std::path::Path::new(&path_a).exists());
    assert!(!std::path::Path::new(&path_c).exists());
    assert!(std::path::Path::new(&path_b).exists());
    assert!(std::path::Path::new(&path_d).exists());
    assert!(matches!(
        deployments.get_by_id(c).await,
        Ok(row) if row.id == c
    ));
    assert!(matches!(
        deployments.get_by_id(d).await,
        Ok(row) if row.id == d
    ));

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn stream_logs_emit_lines_and_latest_selectors() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let dir = temp_dir();
    let path = write_log(&dir, "deploy.log", "line one\nline two\npart");
    let id = db
        .insert_deployment(None, None, "SUCCESS", "DONE", 100, &path, USER_ID)
        .await;

    let mut receiver = deployments.stream_deployment_log(id).await.unwrap();
    let mut lines = Vec::new();
    loop {
        let item = tokio::time::timeout(Duration::from_secs(5), receiver.recv())
            .await
            .expect("stream produced a line before timing out");
        match item {
            Some(line) => lines.push(line),
            None => break,
        }
    }
    assert_eq!(lines, vec!["line one", "line two", "part"]);

    assert!(matches!(
        deployments.stream_deployment_log(9999).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let missing = db
        .insert_deployment(
            None,
            None,
            "QUEUED",
            "QUEUE",
            101,
            "/definitely/missing/stream.log",
            USER_ID,
        )
        .await;
    let mut receiver = deployments.stream_deployment_log(missing).await.unwrap();
    let message = tokio::time::timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("missing log file is reported")
        .expect("stream reports an error line");
    assert!(message.starts_with("Log file not found:"));
    assert!(
        tokio::time::timeout(Duration::from_secs(5), receiver.recv())
            .await
            .unwrap()
            .is_none()
    );

    let app_service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let older_path = write_log(&dir, "older.log", "old line");
    let newer_path = write_log(&dir, "newer.log", "new line");
    db.insert_deployment(
        Some(app_service),
        None,
        "SUCCESS",
        "DONE",
        200,
        &older_path,
        USER_ID,
    )
    .await;
    db.insert_deployment(
        Some(app_service),
        None,
        "SUCCESS",
        "DONE",
        300,
        &newer_path,
        USER_ID,
    )
    .await;

    let mut receiver = deployments
        .stream_application_latest_log(app_service)
        .await
        .unwrap();
    let first_line = tokio::time::timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("application stream produced a line")
        .expect("application stream has content");
    assert_eq!(first_line, "new line");
    drop(receiver);
    assert!(matches!(
        deployments.stream_application_latest_log(9999).await,
        Err(sqlx::Error::RowNotFound)
    ));

    let compose_service = db
        .insert_service("COMPOSE", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let compose_path = write_log(&dir, "compose.log", "compose line");
    db.insert_deployment(
        Some(compose_service),
        None,
        "SUCCESS",
        "DONE",
        400,
        &compose_path,
        USER_ID,
    )
    .await;
    let mut receiver = deployments
        .stream_compose_latest_log(compose_service)
        .await
        .unwrap();
    let first_line = tokio::time::timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("compose stream produced a line")
        .expect("compose stream has content");
    assert_eq!(first_line, "compose line");
    drop(receiver);

    let database_service = db
        .insert_service("DATABASE", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let database_path = write_log(&dir, "database.log", "database line");
    db.insert_deployment(
        Some(database_service),
        None,
        "SUCCESS",
        "DONE",
        500,
        &database_path,
        USER_ID,
    )
    .await;
    let mut receiver = deployments
        .stream_database_latest_log(database_service)
        .await
        .unwrap();
    let first_line = tokio::time::timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("database stream produced a line")
        .expect("database stream has content");
    assert_eq!(first_line, "database line");
    drop(receiver);
    assert!(matches!(
        deployments.stream_compose_latest_log(9999).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert!(matches!(
        deployments.stream_database_latest_log(9999).await,
        Err(sqlx::Error::RowNotFound)
    ));

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn organization_scoping_filters_deployments_and_services() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let dir = temp_dir();

    let own_service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    let own = db
        .insert_deployment(
            Some(own_service),
            Some(SERVER_ID),
            "SUCCESS",
            "DONE",
            100,
            &write_log(&dir, "own.log", "own\n"),
            USER_ID,
        )
        .await;
    // A foreign tenant's service that happens to run on the same (org 1)
    // server must not become visible to org 1 through the server fallback.
    let foreign_service = db
        .insert_service("APPLICATION", "RUNNING", OTHER_ENV_ID, Some(SERVER_ID))
        .await;
    let foreign = db
        .insert_deployment(
            Some(foreign_service),
            Some(SERVER_ID),
            "SUCCESS",
            "DONE",
            200,
            "",
            USER_ID,
        )
        .await;
    // Standalone server jobs inherit the server's tenant.
    let server_job = db
        .insert_deployment(None, Some(SERVER_ID), "FAILED", "FAILED", 300, "", USER_ID)
        .await;

    assert!(
        deployments
            .ensure_organization_access(own, ORG_ID)
            .await
            .is_ok()
    );
    assert!(matches!(
        deployments.ensure_organization_access(foreign, ORG_ID).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert!(
        deployments
            .ensure_organization_access(server_job, ORG_ID)
            .await
            .is_ok()
    );

    assert!(
        deployments
            .ensure_service_organization_access(own_service, ORG_ID)
            .await
            .is_ok()
    );
    assert!(matches!(
        deployments
            .ensure_service_organization_access(foreign_service, ORG_ID)
            .await,
        Err(sqlx::Error::RowNotFound)
    ));

    let org_ids = list_org_ids(
        &deployments,
        ORG_ID,
        DeploymentListFilter {
            limit: 200,
            ..Default::default()
        },
    )
    .await;
    assert!(org_ids.contains(&own));
    assert!(org_ids.contains(&server_job));
    assert!(!org_ids.contains(&foreign));

    let other_ids = list_org_ids(
        &deployments,
        OTHER_ORG_ID,
        DeploymentListFilter {
            limit: 200,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(other_ids, vec![foreign]);

    // Server filters outside the caller's organization are rejected rather
    // than silently returning another tenant's jobs.
    assert!(matches!(
        deployments
            .list_for_organization(
                OTHER_ORG_ID,
                DeploymentListFilter {
                    server_id: Some(SERVER_ID),
                    ..Default::default()
                },
            )
            .await,
        Err(sqlx::Error::RowNotFound)
    ));

    let running = list_org_ids(
        &deployments,
        ORG_ID,
        DeploymentListFilter {
            status: Some("success".into()),
            limit: 200,
            ..Default::default()
        },
    )
    .await;
    assert_eq!(running, vec![own]);

    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn search_logs_for_organization_scopes_results() {
    let db = TestDb::new().await;
    let deployments = db.deployments();
    let dir = temp_dir();

    let own_service = db
        .insert_service("APPLICATION", "RUNNING", ENV_ID, Some(SERVER_ID))
        .await;
    db.insert_deployment(
        Some(own_service),
        Some(SERVER_ID),
        "SUCCESS",
        "DONE",
        100,
        &write_log(&dir, "own.log", "alpha own\nbeta own\n"),
        USER_ID,
    )
    .await;
    let foreign_service = db
        .insert_service("COMPOSE", "RUNNING", OTHER_ENV_ID, Some(SERVER_ID))
        .await;
    db.insert_deployment(
        Some(foreign_service),
        Some(SERVER_ID),
        "SUCCESS",
        "DONE",
        200,
        &write_log(&dir, "foreign.log", "alpha foreign\n"),
        USER_ID,
    )
    .await;

    let results = deployments
        .search_logs_for_organization(
            ORG_ID,
            LogSearchOptions {
                query: Some("alpha".into()),
                limit: 10,
            },
        )
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].line, "alpha own");

    std::fs::remove_dir_all(&dir).ok();
}
