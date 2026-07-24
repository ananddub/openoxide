use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use auto_di::singleton;
use dashmap::{DashMap, DashSet};
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use super::schedule::ScheduleService;

const REFRESH_CRON: &str = "0/30 * * * * *";

pub struct ScheduleRunner {
    service: Arc<ScheduleService>,
    scheduler: Mutex<Option<Arc<JobScheduler>>>,
    jobs: DashMap<String, RegisteredScheduleJob>,
    in_flight: DashSet<String>,
    started: AtomicBool,
}

#[derive(Debug, Clone)]
struct RegisteredScheduleJob {
    job_id: Uuid,
    cron_expression: String,
    enabled: i64,
}

#[singleton]
impl ScheduleRunner {
    fn new(service: Arc<ScheduleService>) -> Self {
        Self {
            service,
            scheduler: Mutex::new(None),
            jobs: DashMap::new(),
            in_flight: DashSet::new(),
            started: AtomicBool::new(false),
        }
    }

    pub async fn start(self: &Arc<Self>) -> Result<(), String> {
        if self.started.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let scheduler = Arc::new(
            JobScheduler::new()
                .await
                .map_err(|error| format!("could not create schedule runner: {error}"))?,
        );

        let runner = Arc::clone(self);
        scheduler
            .add(
                Job::new_async(REFRESH_CRON, move |_job_id, _lock| {
                    let runner = Arc::clone(&runner);
                    Box::pin(async move {
                        if let Err(error) = runner.refresh_jobs().await {
                            tracing::error!(error = %error, "schedule refresh failed");
                        }
                    })
                })
                .map_err(|error| format!("could not create schedule refresh job: {error}"))?,
            )
            .await
            .map_err(|error| format!("could not register schedule refresh job: {error}"))?;

        {
            let mut guard = self.scheduler.lock().await;
            *guard = Some(Arc::clone(&scheduler));
        }

        self.refresh_jobs().await?;
        scheduler
            .start()
            .await
            .map_err(|error| format!("could not start schedule runner: {error}"))?;
        tracing::info!("schedule runner started");
        Ok(())
    }

    pub async fn refresh_jobs(&self) -> Result<(), String> {
        let scheduler = self
            .scheduler
            .lock()
            .await
            .clone()
            .ok_or("schedule runner is not started")?;

        let mut enabled_keys = HashSet::new();

        let schedules = self
            .service
            .repo_schedule
            .list_enabled()
            .await
            .map_err(|error| format!("could not load enabled schedules: {error}"))?;
        for s in &schedules {
            if let Some(id) = s.id {
                enabled_keys.insert(format!("schedule:{id}"));
            }
        }

        let db_backups = self
            .service
            .repo_backup
            .get_all()
            .await
            .map_err(|error| format!("could not load database backups: {error}"))?;
        let db_backups: Vec<_> = db_backups.into_iter().filter(|b| b.enabled == 1).collect();
        for b in &db_backups {
            if let Some(id) = b.id {
                enabled_keys.insert(format!("db_backup:{id}"));
            }
        }

        let vol_backups = self
            .service
            .repo_volume_backup
            .get_all()
            .await
            .map_err(|error| format!("could not load volume backups: {error}"))?;
        let vol_backups: Vec<_> = vol_backups.into_iter().filter(|b| b.enabled == 1).collect();
        for b in &vol_backups {
            if let Some(id) = b.id {
                enabled_keys.insert(format!("vol_backup:{id}"));
            }
        }

        let stale_jobs = self
            .jobs
            .iter()
            .filter_map(|entry| {
                let key = entry.key().clone();
                if enabled_keys.contains(&key) {
                    None
                } else {
                    Some((key, entry.value().job_id))
                }
            })
            .collect::<Vec<_>>();
        for (key, job_id) in stale_jobs {
            self.remove_job(&scheduler, &key, job_id).await;
        }

        let current = self
            .jobs
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect::<HashMap<_, _>>();

        for schedule in schedules {
            let Some(schedule_id) = schedule.id else {
                continue;
            };
            let key = format!("schedule:{schedule_id}");
            let cron_expression = normalize_cron_expression(&schedule.cron_expression);
            let should_replace = current.get(&key).is_none_or(|registered| {
                registered.cron_expression != cron_expression
                    || registered.enabled != schedule.enabled
            });
            if !should_replace {
                continue;
            }
            if let Some(registered) = current.get(&key) {
                self.remove_job(&scheduler, &key, registered.job_id).await;
            }

            let service = Arc::clone(&self.service);
            let in_flight = self.in_flight.clone();
            let key_clone = key.clone();
            let job = Job::new_async(cron_expression.as_str(), move |_job_id, _lock| {
                let service = Arc::clone(&service);
                let in_flight = in_flight.clone();
                let key_str = key_clone.clone();
                Box::pin(async move {
                    if !in_flight.insert(key_str.clone()) {
                        tracing::warn!(key_str, "schedule skipped because previous run is still active");
                        return;
                    }
                    let result = service.run_now(schedule_id).await;
                    in_flight.remove(&key_str);
                    match result {
                        Ok(result) => tracing::info!(
                            schedule_id,
                            action = %result.action,
                            deployment_id = ?result.deployment_id,
                            "scheduled job executed"
                        ),
                        Err(error) => tracing::error!(
                            schedule_id,
                            error = %error,
                            "scheduled job failed"
                        ),
                    }
                })
            })
            .map_err(|error| {
                format!(
                    "invalid cron expression for schedule {schedule_id} ({cron_expression}): {error}"
                )
            })?;
            let job_id = job.guid();
            scheduler
                .add(job)
                .await
                .map_err(|error| format!("could not register schedule {schedule_id}: {error}"))?;
            self.jobs.insert(
                key,
                RegisteredScheduleJob {
                    job_id,
                    cron_expression,
                    enabled: schedule.enabled,
                },
            );
            tracing::info!(schedule_id, "schedule job registered");
        }

        for backup in db_backups {
            let Some(backup_id) = backup.id else {
                continue;
            };
            let key = format!("db_backup:{backup_id}");
            let cron_expression = normalize_cron_expression(&backup.schedule);
            let should_replace = current.get(&key).is_none_or(|registered| {
                registered.cron_expression != cron_expression
                    || registered.enabled != backup.enabled
            });
            if !should_replace {
                continue;
            }
            if let Some(registered) = current.get(&key) {
                self.remove_job(&scheduler, &key, registered.job_id).await;
            }

            let service = Arc::clone(&self.service);
            let in_flight = self.in_flight.clone();
            let key_clone = key.clone();
            let job = Job::new_async(cron_expression.as_str(), move |_job_id, _lock| {
                let service = Arc::clone(&service);
                let in_flight = in_flight.clone();
                let key_str = key_clone.clone();
                Box::pin(async move {
                    if !in_flight.insert(key_str.clone()) {
                        tracing::warn!(key_str, "database backup skipped because previous run is still active");
                        return;
                    }
                    let result = service.run_database_backup(backup_id).await;
                    in_flight.remove(&key_str);
                    match result {
                        Ok(()) => tracing::info!(
                            backup_id,
                            "database backup executed successfully"
                        ),
                        Err(error) => tracing::error!(
                            backup_id,
                            error = %error,
                            "database backup failed"
                        ),
                    }
                })
            })
            .map_err(|error| {
                format!(
                    "invalid cron expression for database backup {backup_id} ({cron_expression}): {error}"
                )
            })?;
            let job_id = job.guid();
            scheduler.add(job).await.map_err(|error| {
                format!("could not register database backup {backup_id}: {error}")
            })?;
            self.jobs.insert(
                key,
                RegisteredScheduleJob {
                    job_id,
                    cron_expression,
                    enabled: backup.enabled,
                },
            );
            tracing::info!(backup_id, "database backup job registered");
        }

        for backup in vol_backups {
            let Some(backup_id) = backup.id else {
                continue;
            };
            let key = format!("vol_backup:{backup_id}");
            let cron_expression = normalize_cron_expression(&backup.cron_expression);
            let should_replace = current.get(&key).is_none_or(|registered| {
                registered.cron_expression != cron_expression
                    || registered.enabled != backup.enabled
            });
            if !should_replace {
                continue;
            }
            if let Some(registered) = current.get(&key) {
                self.remove_job(&scheduler, &key, registered.job_id).await;
            }

            let service = Arc::clone(&self.service);
            let in_flight = self.in_flight.clone();
            let key_clone = key.clone();
            let job = Job::new_async(cron_expression.as_str(), move |_job_id, _lock| {
                let service = Arc::clone(&service);
                let in_flight = in_flight.clone();
                let key_str = key_clone.clone();
                Box::pin(async move {
                    if !in_flight.insert(key_str.clone()) {
                        tracing::warn!(key_str, "volume backup skipped because previous run is still active");
                        return;
                    }
                    let result = service.run_volume_backup(backup_id).await;
                    in_flight.remove(&key_str);
                    match result {
                        Ok(()) => tracing::info!(
                            backup_id,
                            "volume backup executed successfully"
                        ),
                        Err(error) => tracing::error!(
                            backup_id,
                            error = %error,
                            "volume backup failed"
                        ),
                    }
                })
            })
            .map_err(|error| {
                format!(
                    "invalid cron expression for volume backup {backup_id} ({cron_expression}): {error}"
                )
            })?;
            let job_id = job.guid();
            scheduler.add(job).await.map_err(|error| {
                format!("could not register volume backup {backup_id}: {error}")
            })?;
            self.jobs.insert(
                key,
                RegisteredScheduleJob {
                    job_id,
                    cron_expression,
                    enabled: backup.enabled,
                },
            );
            tracing::info!(backup_id, "volume backup job registered");
        }

        Ok(())
    }

    async fn remove_job(&self, scheduler: &JobScheduler, key: &str, job_id: Uuid) {
        if let Err(error) = scheduler.remove(&job_id).await {
            tracing::warn!(key, job_id = %job_id, error = %error, "could not remove schedule job");
        }
        self.jobs.remove(key);
        self.in_flight.remove(key);
    }
}

fn normalize_cron_expression(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.split_whitespace().count() == 5 {
        format!("0 {trimmed}")
    } else {
        trimmed.into()
    }
}
