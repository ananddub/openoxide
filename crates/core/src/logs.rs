pub fn init_logs() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .init();
    tracing::info!("Logs initialized successfully");
}
