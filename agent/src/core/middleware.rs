use std::sync::Arc;

use auto_grpc::middleware;

#[derive(Clone)]
pub struct MonitoringMiddleware {
    token: Arc<str>,
}

#[middleware(order = 1)]
impl MonitoringMiddleware {
    pub fn new(config: Arc<crate::core::config::Config>) -> Self {
        Self {
            token: Arc::from(config.metrics_token.clone()),
        }
    }

    pub(crate) fn authenticate(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let supplied = request
            .metadata()
            .get("x-metrics-token")
            .and_then(|value| value.to_str().ok());
        if supplied == Some(self.token.as_ref()) {
            Ok(request)
        } else {
            Err(tonic::Status::unauthenticated("invalid monitoring token"))
        }
    }
}
