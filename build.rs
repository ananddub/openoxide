fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=db/migrations");
    println!("cargo:rerun-if-changed=proto/openoxide/api/v1");

    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    tonic_build::configure().compile_protos_with_config(
        config,
        &["proto/agent.proto"],
        &["proto"],
    )?;

    let mut buffa_config = connectrpc_build::CodeGenConfig::default();
    buffa_config.lazy_views = true;
    buffa_config.preserve_unknown_fields = false;
    buffa_config.generate_json = true;
    buffa_config.emit_register_fn = true;
    buffa_config
        .bytes_fields
        .push((".".into(), buffa_codegen::BytesRepr::Bytes));

    connectrpc_build::Config::new()
        .buffa_config(buffa_config)
        .files(&[
            "proto/openoxide/api/v1/common.proto",
            "proto/openoxide/api/v1/public.proto",
            "proto/openoxide/api/v1/auth.proto",
            "proto/openoxide/api/v1/organization.proto",
            "proto/openoxide/api/v1/project.proto",
            "proto/openoxide/api/v1/environment.proto",
            "proto/openoxide/api/v1/application.proto",
            "proto/openoxide/api/v1/compose.proto",
            "proto/openoxide/api/v1/deployment.proto",
            "proto/openoxide/api/v1/preview.proto",
            "proto/openoxide/api/v1/database.proto",
            "proto/openoxide/api/v1/platform.proto",
            "proto/openoxide/api/v1/backup.proto",
            "proto/openoxide/api/v1/schedule.proto",
            "proto/openoxide/api/v1/notification.proto",
            "proto/openoxide/api/v1/networking.proto",
            "proto/openoxide/api/v1/alert.proto",
            "proto/openoxide/api/v1/server_management.proto",
            "proto/openoxide/api/v1/certificate.proto",
            "proto/openoxide/api/v1/domain.proto",
            "proto/openoxide/api/v1/monitoring.proto",
            "proto/openoxide/api/v1/global.proto",
            "proto/openoxide/api/v1/overview.proto",
            "proto/openoxide/api/v1/traefik.proto",
            "proto/openoxide/api/v1/swarm.proto",
            "proto/openoxide/api/v1/incoming_webhook.proto",
        ])
        .includes(&["proto"])
        .include_file("_connectrpc.rs")
        .file_per_package(true)
        .compile()?;

    Ok(())
}
