pub mod job;
pub mod queue;
pub mod types;
pub mod worker;

pub use job::TaskJob;
pub use queue::TaskQueue;
pub use types::{Priority, QueueStats, TaskEvent, TaskState};
pub use worker::WorkerPool;
pub use tokio_util::sync::CancellationToken;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct BuildJob {
        pub build_id: String,
        pub repo_url: String,
    }

    impl TaskJob for BuildJob {
        type Output = String;
        type Error = String;

        fn id(&self) -> String {
            self.build_id.clone()
        }

        async fn execute(self, _cancel_token: CancellationToken) -> Result<Self::Output, Self::Error> {
            Ok(format!("Successfully built {}", self.repo_url))
        }
    }

    #[tokio::test]
    async fn test_generic_queue_execution() {
        let queue = TaskQueue::<BuildJob>::new(2, 10);
        let mut rx = queue.subscribe();

        let job = BuildJob {
            build_id: "build_101".into(),
            repo_url: "https://github.com/rustploy/rustploy".into(),
        };

        queue.push(job).await.unwrap();

        let event = rx.recv().await.unwrap();
        assert_eq!(event.task_id, "build_101");
        assert_eq!(event.state, TaskState::Queued);

        let event = rx.recv().await.unwrap();
        assert_eq!(event.task_id, "build_101");
        assert_eq!(event.state, TaskState::Running);

        let event = rx.recv().await.unwrap();
        assert_eq!(event.task_id, "build_101");
        assert_eq!(event.state, TaskState::Completed);
        assert_eq!(
            event.result.unwrap().unwrap(),
            "Successfully built https://github.com/rustploy/rustploy"
        );
    }
}
