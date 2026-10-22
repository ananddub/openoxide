//! End-to-end boot test for the runtime crate.
//!
//! Runs the same `BackgroundManager::start_all()` the panel binary runs, against
//! a real migrated SQLite database, a temp mTLS directory, and an unreachable
//! Docker socket. It then asserts the state an operator can observe after a
//! start: the local server row, its per-server mTLS material in the database
//! and on disk, the agent credential, and the startup notification delivery.

use auto_di::resolve;
use openoxide_runtime::db::queries::FlashQueries;
use openoxide_runtime::utils::background::BackgroundManager;
use std::path::Path;
use std::time::Duration;

// Link anchors: inventory/auto-di registrations live in these crates.
use openoxide_compute as _;
use openoxide_db as _;
use openoxide_domain as _;

const METRICS_TOKEN: &str = "e2e-metrics-token";
const NOTIFICATION_TITLE: &str = "OpenOxide Instance Started";

fn set_env(key: &str, value: impl AsRef<std::ffi::OsStr>) {
    // SAFETY: this integration test binary is a fresh process; every variable is
    // set before the first DI singleton resolves and no background task runs yet.
    unsafe { std::env::set_var(key, value) }
}

#[cfg(unix)]
fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .unwrap_or_else(|e| panic!("stat {}: {e}", path.display()))
        .permissions()
        .mode()
        & 0o777
}

/// Polls until the startup notification has been through the delivery
/// pipeline. The test webhook points at loopback, which the SSRF guard rejects,
/// so a persisted FAILED row proves dispatch → config load → delivery record →
/// sender attempt all ran.
async fn wait_for_startup_delivery(queries: &FlashQueries, notification_id: i64) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(12);
    loop {
        let failed: Option<(String, String, Option<String>)> = sqlx::query_as(
            "SELECT status, title, error FROM notification_delivery_attempts
             WHERE notification_id = ? AND trigger_name = 'PANEL_RESTART' AND status = 'FAILED'
             ORDER BY id LIMIT 1",
        )
        .bind(notification_id)
        .fetch_optional(&queries.pool)
        .await
        .expect("query notification delivery attempts");

        if let Some((status, title, error)) = failed {
            assert_eq!(status, "FAILED");
            assert_eq!(title, NOTIFICATION_TITLE);
            let error = error.expect("a failed delivery attempt records its error");
            assert!(
                error.contains("blocked address range"),
                "unexpected dispatch error: {error}"
            );
            return;
        }

        assert!(
            tokio::time::Instant::now() < deadline,
            "startup notification was not dispatched within 12s"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn panel_boot_starts_background_systems_end_to_end() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    let temp = tempfile::tempdir().expect("create temp dir");
    let db_path = temp.path().join("panel.db");
    let mtls_dir = temp.path().join("mtls");

    set_env("DATABASE_URL", format!("sqlite://{}", db_path.display()));
    set_env("OPENOXIDE_MTLS_DIR", &mtls_dir);
    set_env("OPENOXIDE_BASE_PATH", temp.path().join("base"));
    // Never touch a real Docker daemon: the boot's agent-restart and edge
    // repair both dial this socket, fail fast on ENOENT, and degrade to logs.
    set_env("SOCKET_PATH", temp.path().join("no-docker.sock"));
    set_env("METRICS_TOKEN", METRICS_TOKEN);
    set_env("SECRET_KEY", "e2e-secret");

    // Resolving the query layer opens the database and applies every migration.
    let queries = resolve::<FlashQueries>().await.expect("database layer");

    let org_id =
        sqlx::query("INSERT INTO organization (name, slug, owner_id) VALUES ('E2E Org', 'e2e', 1)")
            .execute(&queries.pool)
            .await
            .expect("seed organization")
            .last_insert_rowid();

    // A channel subscribed to PANEL_RESTART so the startup notification is
    // actually dispatched. Loopback is rejected by design, which makes the
    // attempt observable without any network dependency.
    let notification_id = sqlx::query(
        "INSERT INTO notifications (
            name, notification_type, config_json,
            on_app_deploy, on_app_build_error, on_database_backup,
            on_volume_backup, on_panel_backup, on_panel_restart,
            on_docker_cleanup, on_server_threshold, on_server_migration,
            on_security_alert, on_schedule_success, on_schedule_failure,
            organization_id
         ) VALUES (
            'E2E channel', 'CUSTOM', '{\"url\":\"http://127.0.0.1:9/e2e\"}',
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, ?
         )",
    )
    .bind(org_id)
    .execute(&queries.pool)
    .await
    .expect("seed startup notification channel")
    .last_insert_rowid();

    BackgroundManager::start_all()
        .await
        .expect("background systems start");

    // --- Local server registration -----------------------------------------
    let servers: Vec<(i64, String, String)> =
        sqlx::query_as("SELECT id, ip_address, app_name FROM servers ORDER BY id")
            .fetch_all(&queries.pool)
            .await
            .expect("list servers");
    assert_eq!(
        servers.len(),
        1,
        "boot must register exactly one local host"
    );
    let (server_id, ip_address, app_name) = servers.into_iter().next().unwrap();
    assert_eq!(ip_address, "openoxide-agent");
    assert_eq!(app_name, "localhost");

    // --- mTLS material persisted on the server row -------------------------
    type MtlsRow = (
        Option<Vec<u8>>,
        Option<Vec<u8>>,
        Option<Vec<u8>>,
        Option<Vec<u8>>,
        Option<Vec<u8>>,
    );
    let mtls: MtlsRow = sqlx::query_as(
        "SELECT mtls_ca_cert, mtls_agent_cert, mtls_agent_key, mtls_panel_cert, mtls_panel_key
         FROM servers WHERE id = ?",
    )
    .bind(server_id)
    .fetch_one(&queries.pool)
    .await
    .expect("read server mTLS material");
    let (ca_cert, agent_cert, agent_key, panel_cert, panel_key) = mtls;
    let ca_cert = ca_cert.expect("CA cert persisted");
    let agent_cert = agent_cert.expect("agent cert persisted");
    let agent_key = agent_key.expect("agent key persisted");
    let panel_cert = panel_cert.expect("panel cert persisted");
    let panel_key = panel_key.expect("panel key persisted");
    for (name, pem) in [
        ("ca", &ca_cert),
        ("agent", &agent_cert),
        ("panel", &panel_cert),
    ] {
        assert!(
            pem.starts_with(b"-----BEGIN CERTIFICATE-----"),
            "{name} cert is not PEM"
        );
    }
    for (name, pem) in [("agent", &agent_key), ("panel", &panel_key)] {
        assert!(
            pem.starts_with(b"-----BEGIN PRIVATE KEY-----"),
            "{name} key is not PEM"
        );
    }

    // --- mTLS files written for mounts and agent distribution --------------
    let server_dir = mtls_dir.join(format!("server-{server_id}"));
    for (name, expected) in [
        ("ca.crt", ca_cert.as_slice()),
        ("agent.crt", agent_cert.as_slice()),
        ("agent.key", agent_key.as_slice()),
        ("panel-client.crt", panel_cert.as_slice()),
        ("panel-client.key", panel_key.as_slice()),
    ] {
        let path = server_dir.join(name);
        assert_eq!(
            std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display())),
            expected,
            "{} does not match the DB",
            path.display()
        );
        #[cfg(unix)]
        assert_eq!(mode(&path), 0o600, "{} must be 0600", path.display());
    }
    // The bundled agent mounts the mTLS root read-only and reads this copy.
    for (name, expected) in [
        ("ca.crt", ca_cert.as_slice()),
        ("agent.crt", agent_cert.as_slice()),
        ("agent.key", agent_key.as_slice()),
    ] {
        let path = mtls_dir.join(name);
        assert_eq!(
            std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display())),
            expected,
            "{} does not match the DB",
            path.display()
        );
        #[cfg(unix)]
        assert_eq!(mode(&path), 0o600, "{} must be 0600", path.display());
    }

    // --- Local agent credential registered for METRICS_TOKEN --------------
    let agent: (i64, Option<String>) = sqlx::query_as(
        "SELECT organization_id, query_token FROM monitoring_agents WHERE server_id = ?",
    )
    .bind(server_id)
    .fetch_one(&queries.pool)
    .await
    .expect("local agent credential registered");
    assert_eq!(agent.0, org_id);
    assert_eq!(agent.1.as_deref(), Some(METRICS_TOKEN));

    // --- Startup notification delivered through the real pipeline ---------
    wait_for_startup_delivery(&queries, notification_id).await;

    // --- A second boot reuses the provisioned identity --------------------
    let before: Vec<(String, Vec<u8>)> = [
        "ca.crt",
        "agent.crt",
        "agent.key",
        "panel-client.crt",
        "panel-client.key",
    ]
    .into_iter()
    .map(|name| {
        let path = server_dir.join(name);
        (
            name.to_string(),
            std::fs::read(&path).expect("read mtls file"),
        )
    })
    .collect();

    BackgroundManager::start_all()
        .await
        .expect("second boot starts background systems");

    let servers_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM servers")
        .fetch_one(&queries.pool)
        .await
        .expect("count servers");
    assert_eq!(servers_after, 1, "reboot must not duplicate the local host");
    let agents_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM monitoring_agents")
        .fetch_one(&queries.pool)
        .await
        .expect("count agents");
    assert_eq!(
        agents_after, 1,
        "reboot must not duplicate agent credentials"
    );
    for (name, bytes) in before {
        let path = server_dir.join(&name);
        assert_eq!(
            std::fs::read(&path).expect("read mtls file after reboot"),
            bytes,
            "{name} was regenerated on reboot"
        );
    }
    // The DB copy must still be the same material mounted on disk.
    let ca_after: Vec<u8> = sqlx::query_scalar("SELECT mtls_ca_cert FROM servers WHERE id = ?")
        .bind(server_id)
        .fetch_one(&queries.pool)
        .await
        .expect("read CA after reboot");
    assert_eq!(ca_after, ca_cert, "CA was rotated on reboot");
}
