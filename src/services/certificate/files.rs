use crate::{
    api::dto::traefik::TraefikWriteFileDto,
    db::models::certificates::Certificate,
    services::traefik::TraefikService,
    utils::{
        exec::{CommandExecutor, LocalExecutor},
        os::OsCli,
    },
};

pub(super) async fn certificate_expiry(certificate: &str) -> Result<i64, String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    OsCli::new(&executor)
        .crypto()
        .certificate(certificate)
        .expiry()
        .run()
        .await
        .map_err(|error| error.to_string())
}

pub(super) async fn validate_certificate_pair(
    certificate: &str,
    private_key: &str,
) -> Result<i64, String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    OsCli::new(&executor)
        .crypto()
        .certificate(certificate)
        .validate_with_key(private_key)
        .run()
        .await
        .map(|result| result.expires_at)
        .map_err(|error| error.to_string())
}

pub(super) async fn write(certificate: &Certificate) -> Result<(), String> {
    validate_path(&certificate.certificate_path)?;

    let directory = format!("dynamic/certificates/{}", certificate.certificate_path);
    let cert_path = format!("{directory}/chain.crt");
    let key_path = format!("{directory}/privkey.key");
    let config_path = format!("{directory}/certificate.yml");
    let config = format!(
        "tls:\n  certificates:\n    - certFile: /etc/openoxide/traefik/{cert_path}\n      keyFile: /etc/openoxide/traefik/{key_path}\n"
    );

    let traefik = resolve_traefik().await?;
    for (path, content) in [
        (cert_path, certificate.certificate_data.clone()),
        (key_path, certificate.private_key.clone()),
        (config_path, config),
    ] {
        traefik
            .write_file(TraefikWriteFileDto {
                server_id: certificate.server_id,
                path,
                content,
            })
            .await?;
    }
    Ok(())
}

pub(super) async fn delete(certificate: &Certificate) -> Result<(), String> {
    validate_path(&certificate.certificate_path)?;
    resolve_traefik()
        .await?
        .delete_directory(
            certificate.server_id,
            &format!("dynamic/certificates/{}", certificate.certificate_path),
        )
        .await
}

fn validate_path(path: &str) -> Result<(), String> {
    if path.trim().is_empty() || path.contains("..") || path.contains('/') || path.contains('\\') {
        return Err("certificate path must be a simple directory name".into());
    }
    Ok(())
}

async fn resolve_traefik() -> Result<std::sync::Arc<TraefikService>, String> {
    auto_di::resolve::<TraefikService>()
        .await
        .map_err(|error| error.to_string())
}
