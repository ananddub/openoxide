use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Debug, Clone)]
pub struct WorkerPool {
    semaphore: Arc<Semaphore>,
    active_count: Arc<AtomicUsize>,
    max_concurrency: usize,
}

impl WorkerPool {
    pub fn new(max_concurrency: usize) -> Self {
        let max_concurrency = max_concurrency.max(1);
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
            active_count: Arc::new(AtomicUsize::new(0)),
            max_concurrency,
        }
    }

    pub fn max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    pub fn active_workers(&self) -> usize {
        self.active_count.load(Ordering::Relaxed)
    }

    pub fn available_slots(&self) -> usize {
        self.semaphore.available_permits()
    }

    pub async fn acquire(&self) -> WorkerPermit {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("Worker pool semaphore closed unexpectedly");

        self.active_count.fetch_add(1, Ordering::SeqCst);
        WorkerPermit {
            _permit: permit,
            active_count: self.active_count.clone(),
        }
    }
}

pub struct WorkerPermit {
    _permit: tokio::sync::OwnedSemaphorePermit,
    active_count: Arc<AtomicUsize>,
}

impl Drop for WorkerPermit {
    fn drop(&mut self) {
        self.active_count.fetch_sub(1, Ordering::SeqCst);
    }
}
