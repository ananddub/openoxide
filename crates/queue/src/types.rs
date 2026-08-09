use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "QUEUED"),
            Self::Running => write!(f, "RUNNING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskEvent<T, E> {
    pub task_id: String,
    pub state: TaskState,
    pub result: Option<Result<T, E>>,
}

#[derive(Debug, Clone)]
pub struct QueueStats {
    pub total_queued: usize,
    pub total_running: usize,
    pub total_completed: u64,
    pub total_failed: u64,
    pub active_workers: usize,
    pub max_concurrency: usize,
}
