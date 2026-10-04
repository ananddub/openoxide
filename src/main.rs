use auto_di::resolve;
use openoxide::{
    core::config::Config, core::logs::init_logs, utils::background::BackgroundManager,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
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

    let connect_router = auto_grpc::connect_routes().await.unwrap();

    let reflector = connectrpc_reflection::Reflector::from_descriptor_set_bytes(include_bytes!(
        concat!(env!("OUT_DIR"), "/openoxide.fds.bin")
    ))
    .expect("embedded openoxide descriptor set should parse");
    let connect_router = connectrpc_reflection::install(connect_router, reflector);
    let connect_service = connectrpc::ConnectRpcService::new(connect_router);
    BackgroundManager::start_all()
        .await
        .expect("failed to start background systems");

    let config = resolve::<Config>().await.unwrap();
    let (host, port) = (config.host.clone(), config.port);

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    let tls = openoxide::api::connect::tls_config(
        &config.tls_cert_path,
        &config.tls_key_path,
    )
    .unwrap_or_else(|error| {
        panic!(
            "TLS setup failed (cert: {}, key: {}): {} — the panel is HTTPS/HTTP-2 only, no cleartext fallback",
            config.tls_cert_path, config.tls_key_path, error
        )
    });

    tracing::info!(
        "Listening on https://{}:{} (HTTP/2 only, ALPN [h2])",
        host,
        port
    );
    openoxide::api::connect::serve(listener, connect_service, tls)
        .await
        .unwrap();
}
