use auto_di::resolve;
use axum::Router;
use rustploy::{
    core::config::Config,
    core::logs::init_logs,
    services::{
        monitoring::alert_service::AlertService,
        notification::{NotificationLevel, NotificationMessage, NotificationService, NotificationTrigger},
        schedule::ScheduleRunner,
    },
    utils::builder::queue::BuilderQueue,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
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
    rustploy::utils::ssh::agent::sweep_orphaned_agent_sockets();

    let service: Arc<Router> = resolve::<Router<()>>().await.unwrap();
    resolve::<ScheduleRunner>()
        .await
        .unwrap()
        .start()
        .await
        .expect("failed to start schedule runner");
    resolve::<BuilderQueue>()
        .await
        .unwrap()
        .start()
        .await
        .expect("failed to start builder queue");
    resolve::<AlertService>()
        .await
        .expect("failed to resolve alert service")
        .start();

    // 3. Dispatch PanelRestart notification on boot
    if let Ok(notif_svc) = resolve::<NotificationService>().await {
        let msg = NotificationMessage::new(
            "Rustploy Instance Started",
            "Rustploy control panel server has successfully initialized and is operational.",
        )
        .level(NotificationLevel::Info);
        tokio::spawn(async move {
            notif_svc.notify(NotificationTrigger::PanelRestart, &msg).await;
        });
    }

    let port = resolve::<Config>().await.unwrap().port.clone();
    let host = resolve::<Config>().await.unwrap().host.clone();

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    let svc = service.as_ref().to_owned();

    tracing::info!("Listening on {}:{}", host, port);
    axum::serve(listener, svc).await.unwrap();
}
