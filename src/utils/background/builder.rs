use std::sync::Arc;

use crate::utils::builder::queue::BuilderQueue;

pub async fn start(queue: Arc<BuilderQueue>) -> Result<(), String> {
    queue.start().await
}
