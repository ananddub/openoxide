use auto_di::resolve;
use axum::Router;
use openoxide::{
    core::config::Config, core::logs::init_logs, utils::background::BackgroundManager,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // reqwest is built with rustls-no-provider so the binary can use the
    // smaller ring backend instead of pulling in AWS-LC. Install it before
    // constructing any HTTP client.
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("rustls crypto provider should only be installed once");
    init_logs();

    // 1. Core dump suppression (RLIMIT_CORE = 0) so crashes never dump key memory to disk
    #[cfg(unix)]
    unsafe {
        let rlim = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        let _ = libc::setrlimit(libc::RLIMIT_CORE, &rlim);
    }

    // 2. Orphaned agent socket cleanup on server boot
    openoxide::utils::ssh::agent::sweep_orphaned_agent_sockets();

    let service: Arc<Router> = resolve::<Router<()>>().await.unwrap();
    BackgroundManager::start_all()
        .await
        .expect("failed to start background systems");

    let port = resolve::<Config>().await.unwrap().port.clone();
    let host = resolve::<Config>().await.unwrap().host.clone();

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    let svc = service.as_ref().to_owned();

    tracing::info!("Listening on {}:{}", host, port);
    axum::serve(listener, svc).await.unwrap();
}
