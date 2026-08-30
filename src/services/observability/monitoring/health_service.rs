use crate::db::queries::FlashQueries;
use crate::flash_gen::{health_report::HealthReportCreateParams, models::HealthReports};
use auto_di::singleton;
use std::sync::Arc;

pub struct HealthService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl HealthService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn list_reports(&self, limit: i64) -> Result<Vec<HealthReports>, String> {
        self.queries
            .health_report_list(limit)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn record_report(&self, report: &HealthReports) -> Result<(), String> {
        self.queries
            .health_report_create(&HealthReportCreateParams {
                target_id: report.target_id,
                target_type: report.target_type.clone(),
                status: report.status.clone(),
                response_time_ms: report.response_time_ms,
                error_message: report.error_message.clone().unwrap_or_default(),
                created_at: chrono::Utc::now().timestamp(),
            })
            .await
            .map_err(|e| e.to_string())
    }
}
