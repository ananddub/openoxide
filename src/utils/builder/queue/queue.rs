use crate::core::cache::{AppStateCache, CacheKey};
use crate::utils::builder::hash_state::ApplicationState;
use auto_di::singleton;
use dashmap::DashMap;
use futures::FutureExt;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Notify, Semaphore};

type ServerKey = Option<i64>;

pub struct BuilderQueue {
    pub(super) db: Arc<SqlitePool>,
    pub(super) application_state: Arc<ApplicationState>,
    pub(super) cache: Arc<AppStateCache>,
    slots: DashMap<ServerKey, Arc<Semaphore>>,
    per_server_limit: usize,
    notify: Notify,
    started: AtomicBool,
}

#[singleton]
impl BuilderQueue {
    pub(crate) async fn is_local_server_id(&self, server_id: Option<i64>) -> Result<bool, String> {
        let Some(server_id) = server_id else {
            return Ok(true);
        };
        let ip = sqlx::query_scalar::<_, String>("SELECT ip_address FROM servers WHERE id = ?")
            .bind(server_id)
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(|error| error.to_string())?;
        Ok(ip
            .as_deref()
            .is_some_and(|value| matches!(value, "127.0.0.1" | "localhost" | "openoxide-agent")))
    }

    pub fn new(
        db: Arc<SqlitePool>,
        application_state: Arc<ApplicationState>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        let per_server_limit = std::env::var("DEPLOYMENT_PER_SERVER_CONCURRENCY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1usize)
            .max(1);

        Self {
            db,
            application_state,
            cache,
            slots: DashMap::new(),
            per_server_limit,
            notify: Notify::new(),
            started: AtomicBool::new(false),
        }
    }

    pub async fn start(self: &Arc<Self>) -> Result<(), String> {
        if self.started.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        self.recover_stale_deployments().await;
        let q = Arc::clone(self);
        tokio::spawn(async move {
            q.retry_stale_remote_cleanup().await;
        });

        let q = Arc::clone(self);
        tokio::spawn(async move {
            q.dispatch_loop().await;
        });

        let q = Arc::clone(self);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                interval.tick().await;
                q.notify();
            }
        });

        tracing::info!(
            per_server_limit = self.per_server_limit,
            "builder queue started"
        );
        self.notify();
        Ok(())
    }

    pub fn notify(&self) {
        self.notify.notify_one();
    }

    pub async fn ensure_capacity(&self) -> sqlx::Result<()> {
        let max_size: i64 = std::env::var("DEPLOYMENT_QUEUE_MAX_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100)
            .max(1);

        let q = auto_di::resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let queued = q.deployments_get_queued_count().await?;

        if queued >= max_size {
            return Err(sqlx::Error::Protocol(format!(
                "deployment queue is full ({queued}/{max_size}); try again later"
            )));
        }
        Ok(())
    }

    pub async fn cancel_queued_application(&self, application_id: i64) -> sqlx::Result<bool> {
        let q = auto_di::resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let ids = q
            .deployments_get_queued_ids_for_application(application_id)
            .await?;
        q.deployments_cancel_queued_for_application(application_id)
            .await?;
        let cancelled = !ids.is_empty();

        if cancelled {
            self.cache
                .invalidate(&CacheKey::Application(application_id))
                .await;
            for id in ids {
                if let Ok(mut log) = super::deployment_log::DeploymentLog::open(id).await {
                    let _ = log
                        .write_line("[CANCELLED] deployment cancelled before worker started")
                        .await;
                }
            }
        }
        Ok(cancelled)
    }

    pub async fn cancel_queued_compose(&self, compose_id: i64) -> sqlx::Result<bool> {
        let q = auto_di::resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let ids = q.deployments_get_queued_ids_for_compose(compose_id).await?;
        q.deployments_cancel_queued_for_compose(compose_id).await?;
        let cancelled = !ids.is_empty();

        if cancelled {
            self.cache.invalidate(&CacheKey::Compose(compose_id)).await;
            for id in ids {
                if let Ok(mut log) = super::deployment_log::DeploymentLog::open(id).await {
                    let _ = log
                        .write_line("[CANCELLED] deployment cancelled before worker started")
                        .await;
                }
            }
        }
        Ok(cancelled)
    }

    pub async fn cancel_queued_database(&self, database_id: i64) -> sqlx::Result<bool> {
        let q = auto_di::resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let ids = q
            .deployments_get_queued_ids_for_database(database_id)
            .await?;
        q.deployments_cancel_queued_for_database(database_id)
            .await?;
        let cancelled = !ids.is_empty();

        if cancelled {
            self.cache
                .invalidate(&CacheKey::Database(database_id))
                .await;
            for id in ids {
                if let Ok(mut log) = super::deployment_log::DeploymentLog::open(id).await {
                    let _ = log
                        .write_line("[CANCELLED] deployment cancelled before worker started")
                        .await;
                }
            }
        }
        Ok(cancelled)
    }

    fn semaphore_for(&self, server_id: ServerKey) -> Arc<Semaphore> {
        if let Some(entry) = self.slots.get(&server_id) {
            return Arc::clone(entry.value());
        }
        let sem = Arc::new(Semaphore::new(self.per_server_limit));
        self.slots.insert(server_id, Arc::clone(&sem));
        sem
    }

    async fn recover_stale_deployments(&self) {
        self.cleanup_stale_remote_jobs().await;

        let q = match auto_di::resolve::<crate::db::queries::FlashQueries>().await {
            Ok(q) => q,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve FlashQueries");
                return;
            }
        };

        if let Err(e) = q.deployments_mark_interrupted_as_recovered().await {
            tracing::error!(error = %e, "builder queue: could not recover stale deployments");
        }

        // One pass over `services` covers applications, compose and databases.
        if let Err(e) = q.deployments_recover_stale_application_statuses().await {
            tracing::error!(error = %e, "builder queue: could not recover stale application statuses");
        }
    }

    async fn cleanup_stale_remote_jobs(&self) {
        let q = match auto_di::resolve::<crate::db::queries::FlashQueries>().await {
            Ok(q) => q,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve FlashQueries");
                return;
            }
        };
        let rows = match q.deployments_get_running_remote_jobs().await {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not load stale remote jobs");
                return;
            }
        };

        for row in rows {
            if let (Some(server_id), Some(pid_file)) = (row.server_id, row.pid.clone()) {
                self.kill_remote_pid(row.id, server_id, &pid_file, "startup")
                    .await;
            }
        }
    }

    async fn retry_stale_remote_cleanup(self: Arc<Self>) {
        for attempt in 1..=20 {
            let q = match auto_di::resolve::<crate::db::queries::FlashQueries>().await {
                Ok(q) => q,
                Err(e) => {
                    tracing::error!(error = %e, "builder queue: could not resolve FlashQueries");
                    return;
                }
            };
            let rows = match q.deployments_get_recovered_remote_jobs().await {
                Ok(rows) => rows,
                Err(e) => {
                    tracing::error!(error = %e, "builder queue: could not load recovered remote jobs");
                    return;
                }
            };

            if rows.is_empty() {
                return;
            }

            let mut pending = 0usize;
            for row in rows {
                let killed = match (row.server_id, row.pid.clone()) {
                    (Some(server_id), Some(pid_file)) => {
                        self.kill_remote_pid(
                            row.id,
                            server_id,
                            &pid_file,
                            &format!("retry-{attempt}"),
                        )
                        .await
                    }
                    _ => false,
                };
                if !killed {
                    pending += 1;
                }
            }

            if pending == 0 {
                return;
            }
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }

    async fn kill_remote_pid(
        &self,
        deployment_id: i64,
        server_id: i64,
        pid_file: &str,
        stage: &str,
    ) -> bool {
        match crate::services::compute::compose::remote::agent_executor(server_id).await {
            Ok(executor) => match executor.kill_pid_file(pid_file).await {
                Ok(()) => {
                    if let Ok(q) = auto_di::resolve::<crate::db::queries::FlashQueries>().await {
                        q.deployments_set_pid_null(deployment_id).await.ok();
                    }
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file,
                        stage,
                        "builder queue: killed stale remote deployment"
                    );
                    true
                }
                Err(e) => {
                    tracing::warn!(deployment_id, server_id, pid_file, stage,
                        error = %e, "builder queue: failed to kill stale remote deployment");
                    false
                }
            },
            Err(e) => {
                tracing::warn!(deployment_id, server_id, pid_file, stage,
                    error = %e, "builder queue: could not create remote executor for stale cleanup");
                false
            }
        }
    }

    // ------------------------------------------------------------------ //
    //  Internal dispatch                                                   //
    // ------------------------------------------------------------------ //

    async fn dispatch_loop(self: Arc<Self>) {
        loop {
            self.notify.notified().await;
            self.dispatch_available().await;
        }
    }

    async fn dispatch_available(self: &Arc<Self>) {
        let q = match auto_di::resolve::<crate::db::queries::FlashQueries>().await {
            Ok(q) => q,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve FlashQueries");
                return;
            }
        };
        let queued = match q.deployments_get_queued_deployments_grouped().await {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not load queued deployments");
                return;
            }
        };

        for row in queued {
            let deployment_id = row.id;
            let sem = self.semaphore_for(row.server_id);

            // Non-blocking — skip if this server is at capacity.
            let Ok(permit) = sem.try_acquire_owned() else {
                continue;
            };

            // Atomically claim this specific deployment.
            let claim = q.deployments_claim_queued_deployment(deployment_id).await;

            match claim {
                Ok(claim) => {
                    let dep_id: i64 = claim.id;
                    let service_id: Option<i64> = claim.service_id;
                    let database_kind: Option<String> = claim.database_kind;
                    let operation: String = claim.operation.unwrap_or_else(|| "deploy".into());

                    let service_type: Option<String> = match service_id {
                        Some(sid) => q.deployments_claim_service_type(sid).await.ok(),
                        None => None,
                    };
                    let (application_id, compose_id, database_id) = match service_type.as_deref()
                    {
                        Some("COMPOSE") => (None, service_id, None),
                        Some("DATABASE") => (None, None, service_id),
                        _ => (service_id, None, None),
                    };

                    let q = Arc::clone(self);

                    tokio::spawn(async move {
                        let outcome = std::panic::AssertUnwindSafe(q.process(
                            dep_id,
                            application_id,
                            compose_id,
                            database_id,
                            database_kind.clone(),
                            operation.clone(),
                        ))
                        .catch_unwind()
                        .await;
                        if outcome.is_err() {
                            tracing::error!(
                                deployment_id = dep_id,
                                "builder queue worker panicked"
                            );
                            let service_id = application_id.or(compose_id).or(database_id);
                            if let Ok(fq) =
                                auto_di::resolve::<crate::db::queries::FlashQueries>().await
                            {
                                let _ = fq
                                    .deployments_finalize_with_resource1(
                                        &crate::flash_gen::deployments::DeploymentsFinalizeWithResource1Params {
                                            status: "ERROR".into(),
                                            state: "ERROR".into(),
                                            error_message: "builder worker panicked".into(),
                                            id: dep_id,
                                        },
                                    )
                                    .await;
                                if let Some(sid) = service_id {
                                    let _ = fq
                                        .deployments_finalize_with_resource3(
                                            &crate::flash_gen::deployments::DeploymentsFinalizeWithResource3Params {
                                                app_status: "ERROR".into(),
                                                last_error: "builder worker panicked".into(),
                                                id: sid,
                                            },
                                        )
                                        .await;
                                }
                                let _ = fq.deployments_finalize_with_resource6(dep_id).await;
                            }
                            if let Some(id) = application_id {
                                q.cache.invalidate(&CacheKey::Application(id)).await;
                            }
                            if let Some(id) = compose_id {
                                q.cache.invalidate(&CacheKey::Compose(id)).await;
                            }
                            if let Some(id) = database_id {
                                q.cache.invalidate(&CacheKey::Database(id)).await;
                            }
                        }
                        // Free the per-server slot, then wake dispatcher.
                        drop(permit);
                        q.notify();
                    });
                }
                Err(sqlx::Error::RowNotFound) => {
                    // Already claimed by someone else — release permit.
                    drop(permit);
                }
                Err(e) => {
                    tracing::error!(deployment_id, error = %e,
                        "builder queue: failed to claim deployment");
                    drop(permit);
                }
            }
        }
    }
}
