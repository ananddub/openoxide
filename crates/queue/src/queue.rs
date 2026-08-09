use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

use crate::job::TaskJob;
use crate::types::{QueueStats, TaskEvent, TaskState};
use crate::worker::WorkerPool;

#[derive(Clone)]
pub struct TaskQueue<J>
where
    J: TaskJob,
{
    worker_pool: WorkerPool,
    active_tasks: Arc<DashMap<String, (TaskState, CancellationToken)>>,
    tx_queue: mpsc::Sender<J>,
    tx_events: broadcast::Sender<TaskEvent<J::Output, J::Error>>,
    total_completed: Arc<AtomicU64>,
    total_failed: Arc<AtomicU64>,
}

impl<J> TaskQueue<J>
where
    J: TaskJob,
{
    pub fn new(max_concurrency: usize, capacity: usize) -> Self {
        let worker_pool = WorkerPool::new(max_concurrency);
        let active_tasks = Arc::new(DashMap::new());
        let (tx_queue, mut rx_queue) = mpsc::channel::<J>(capacity.max(100));
        let (tx_events, _) = broadcast::channel(1000);

        let total_completed = Arc::new(AtomicU64::new(0));
        let total_failed = Arc::new(AtomicU64::new(0));

        let pool_clone = worker_pool.clone();
        let tasks_clone = active_tasks.clone();
        let events_clone = tx_events.clone();
        let completed_clone = total_completed.clone();
        let failed_clone = total_failed.clone();

        tokio::spawn(async move {
            while let Some(job) = rx_queue.recv().await {
                let permit = pool_clone.acquire().await;
                let task_id = job.id();
                let cancel_token = CancellationToken::new();

                tasks_clone.insert(task_id.clone(), (TaskState::Running, cancel_token.clone()));
                let _ = events_clone.send(TaskEvent {
                    task_id: task_id.clone(),
                    state: TaskState::Running,
                    result: None,
                });

                let events_tx = events_clone.clone();
                let tasks_map = tasks_clone.clone();
                let completed_counter = completed_clone.clone();
                let failed_counter = failed_clone.clone();

                tokio::spawn(async move {
                    let _permit_guard = permit;
                    tokio::select! {
                        _ = cancel_token.cancelled() => {
                            tasks_map.insert(task_id.clone(), (TaskState::Cancelled, cancel_token));
                            let _ = events_tx.send(TaskEvent {
                                task_id,
                                state: TaskState::Cancelled,
                                result: None,
                            });
                        }
                        res = job.execute(cancel_token.clone()) => {
                            match res {
                                Ok(output) => {
                                    tasks_map.insert(task_id.clone(), (TaskState::Completed, cancel_token));
                                    completed_counter.fetch_add(1, Ordering::SeqCst);
                                    let _ = events_tx.send(TaskEvent {
                                        task_id,
                                        state: TaskState::Completed,
                                        result: Some(Ok(output)),
                                    });
                                }
                                Err(err) => {
                                    tasks_map.insert(task_id.clone(), (TaskState::Failed, cancel_token));
                                    failed_counter.fetch_add(1, Ordering::SeqCst);
                                    let _ = events_tx.send(TaskEvent {
                                        task_id,
                                        state: TaskState::Failed,
                                        result: Some(Err(err)),
                                    });
                                }
                            }
                        }
                    }
                });
            }
        });

        Self {
            worker_pool,
            active_tasks,
            tx_queue,
            tx_events,
            total_completed,
            total_failed,
        }
    }

    pub async fn push(&self, job: J) -> Result<CancellationToken, String> {
        let task_id = job.id();
        let cancel_token = CancellationToken::new();

        self.active_tasks
            .insert(task_id.clone(), (TaskState::Queued, cancel_token.clone()));
        let _ = self.tx_events.send(TaskEvent {
            task_id: task_id.clone(),
            state: TaskState::Queued,
            result: None,
        });

        self.tx_queue
            .send(job)
            .await
            .map_err(|_| "Queue worker channel closed".to_string())?;

        Ok(cancel_token)
    }

    pub fn cancel(&self, task_id: &str) -> bool {
        if let Some(entry) = self.active_tasks.get(task_id) {
            let (state, token) = entry.value();
            if *state == TaskState::Queued || *state == TaskState::Running {
                token.cancel();
                return true;
            }
        }
        false
    }

    pub fn subscribe(&self) -> broadcast::Receiver<TaskEvent<J::Output, J::Error>> {
        self.tx_events.subscribe()
    }

    pub fn stats(&self) -> QueueStats {
        let mut queued = 0;
        let mut running = 0;

        for entry in self.active_tasks.iter() {
            match entry.value().0 {
                TaskState::Queued => queued += 1,
                TaskState::Running => running += 1,
                _ => {}
            }
        }

        QueueStats {
            total_queued: queued,
            total_running: running,
            total_completed: self.total_completed.load(Ordering::Relaxed),
            total_failed: self.total_failed.load(Ordering::Relaxed),
            active_workers: self.worker_pool.active_workers(),
            max_concurrency: self.worker_pool.max_concurrency(),
        }
    }
}
