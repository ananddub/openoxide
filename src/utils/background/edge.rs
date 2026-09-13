use std::sync::Arc;
use std::time::Duration;

use crate::services::networking::traefik::TraefikService;

const RETRY_DELAY: Duration = Duration::from_secs(30);

/// Retries the local edge repair until Traefik is configured and running.
pub fn start(traefik: Arc<TraefikService>) {
    tokio::spawn(async move {
        loop {
            match traefik.ensure_local_edge().await {
                Ok(()) => {
                    tracing::info!("local Traefik edge ready");
                    return;
                }
                Err(error) => {
                    tracing::warn!(
                        %error,
                        "local Traefik edge repair failed; retrying in {RETRY_DELAY:?}"
                    );
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    });
}
