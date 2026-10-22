use std::sync::Arc;

use crate::services::automation::schedule::ScheduleRunner;

pub async fn start(runner: Arc<ScheduleRunner>) -> Result<(), String> {
    runner.start().await
}
