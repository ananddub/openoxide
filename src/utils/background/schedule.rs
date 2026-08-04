use std::sync::Arc;

use crate::services::schedule::ScheduleRunner;

pub async fn start(runner: Arc<ScheduleRunner>) -> Result<(), String> {
    runner.start().await
}
