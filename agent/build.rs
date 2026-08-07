fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    let proto_path = if std::path::Path::new("../../proto/monitoring.proto").exists() {
        "../../proto/monitoring.proto"
    } else if std::path::Path::new("../proto/monitoring.proto").exists() {
        "../proto/monitoring.proto"
    } else {
        "proto/monitoring.proto"
    };

    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    tonic_build::configure().compile_protos_with_config(
        config,
        &[proto_path],
        &["../../proto", "../proto", "proto", "../../", "../", "."],
    )?;

    Ok(())
}
