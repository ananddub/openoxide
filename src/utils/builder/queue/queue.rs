use crate::utils::builder::hash_state::ApplicationState;
use auto_di::singleton;
use dashmap::DashMap;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Notify, Semaphore};

type ServerKey = Option<i64>;

pub struct BuilderQueue {
    pub(super) db: Arc<SqlitePool>,
    pub(super) application_state: Arc<ApplicationState>,
    slots: DashMap<ServerKey, Arc<Semaphore>>,
    per_server_limit: usize,
    notify: Notify,
    started: AtomicBool,
}

#[singleton]
impl BuilderQueue {
    pub fn new(db: Arc<SqlitePool>, application_state: Arc<ApplicationState>) -> Self {
        let per_server_limit = std::env::var("DEPLOYMENT_PER_SERVER_CONCURRENCY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1usize)
            .max(1);

        Self {
            db,
            application_state,
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
            let mut interval = tokio::time::interval(Duration::from_mins(30 ));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                interval.tick().await;
                tracing::debug!("builder queue: 30-min reconciler tick");
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

        let repo = auto_di::resolve::<crate::repository::DeploymentRepository>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let queued = repo.get_queued_count().await?;

        if queued >= max_size {
            return Err(sqlx::Error::Protocol(format!(
                "deployment queue is full ({queued}/{max_size}); try again later"
            )));
        }
        Ok(())
    }

    pub async fn cancel_queued_application(&self, application_id: i64) -> sqlx::Result<bool> {
        let repo = auto_di::resolve::<crate::repository::DeploymentRepository>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let ids = repo.get_queued_ids_for_application(application_id).await?;
        let cancelled = repo.cancel_queued_for_application(application_id).await?;

        if cancelled {
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
        let repo = auto_di::resolve::<crate::repository::DeploymentRepository>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let ids = repo.get_queued_ids_for_compose(compose_id).await?;
        let cancelled = repo.cancel_queued_for_compose(compose_id).await?;

        if cancelled {
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
        let repo = auto_di::resolve::<crate::repository::DeploymentRepository>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let ids = repo.get_queued_ids_for_database(database_id).await?;
        let cancelled = repo.cancel_queued_for_database(database_id).await?;

        if cancelled {
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

        let repo = match auto_di::resolve::<crate::repository::DeploymentRepository>().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve DeploymentRepository");
                return;
            }
        };

        if let Err(e) = repo.mark_running_as_recovered().await {
            tracing::error!(error = %e, "builder queue: could not recover stale deployments");
        }

        if let Err(e) = repo.recover_stale_application_statuses().await {
            tracing::error!(error = %e, "builder queue: could not recover stale application statuses");
        }

        if let Err(e) = repo.recover_stale_compose_statuses().await {
            tracing::error!(error = %e, "builder queue: could not recover stale compose statuses");
        }

        if let Err(e) = repo.recover_stale_database_statuses().await {
            tracing::error!(error = %e, "builder queue: could not recover stale database status");
        }
    }

    async fn cleanup_stale_remote_jobs(&self) {
        let repo = match auto_di::resolve::<crate::repository::DeploymentRepository>().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve DeploymentRepository");
                return;
            }
        };
        let rows = match repo.get_running_remote_jobs().await {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not load stale remote jobs");
                return;
            }
        };

        for (deployment_id, server_id, pid_file) in rows {
            self.kill_remote_pid(deployment_id, server_id, &pid_file, "startup")
                .await;
        }
    }

    async fn retry_stale_remote_cleanup(self: Arc<Self>) {
        for attempt in 1..=20 {
            let repo = match auto_di::resolve::<crate::repository::DeploymentRepository>().await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(error = %e, "builder queue: could not resolve DeploymentRepository");
                    return;
                }
            };
            let rows = match repo.get_recovered_remote_jobs().await {
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
            for (deployment_id, server_id, pid_file) in rows {
                let killed = self
                    .kill_remote_pid(
                        deployment_id,
                        server_id,
                        &pid_file,
                        &format!("retry-{attempt}"),
                    )
                    .await;
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
        use crate::services::application::remote::remote_executor;
        match remote_executor(self.db.as_ref(), server_id).await {
            Ok(executor) => match executor.kill_pid_file(pid_file).await {
                Ok(()) => {
                    if let Ok(repo) =
                        auto_di::resolve::<crate::repository::DeploymentRepository>().await
                    {
                        repo.set_pid_null(deployment_id).await.ok();
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
        let repo = match auto_di::resolve::<crate::repository::DeploymentRepository>().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not resolve DeploymentRepository");
                return;
            }
        };
        let queued = match repo.get_queued_deployments_grouped().await {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not load queued deployments");
                return;
            }
        };

        for (deployment_id, server_id) in queued {
            let sem = self.semaphore_for(server_id);

            // Non-blocking — skip if this server is at capacity.
            let Ok(permit) = sem.try_acquire_owned() else {
                continue;
            };

            // Atomically claim this specific deployment.
            let row = repo.claim_queued_deployment(deployment_id).await;

            match row {
                Ok(Some(claim)) => {
                    let dep_id: i64 = claim.id;
                    let application_id: Option<i64> = claim.application_id;
                    let compose_id: Option<i64> = claim.compose_id;
                    let database_id: Option<i64> = claim.database_id;
                    let database_kind: Option<String> = claim.database_kind;
                    let operation: String = claim.operation.unwrap_or_else(|| "deploy".into());

                    let q = Arc::clone(self);

                    tokio::spawn(async move {
                        q.process(
                            dep_id,
                            application_id,
                            compose_id,
                            database_id,
                            database_kind,
                            operation,
                        )
                        .await;
                        // Free the per-server slot, then wake dispatcher.
                        drop(permit);
                        q.notify();
                    });
                }
                Ok(None) => {
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
