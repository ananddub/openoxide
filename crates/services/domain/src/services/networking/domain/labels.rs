/// Build traefik labels grouped by Swarm service name.
/// Used for zero-redeploy label updates via `docker service update`.
/// Returns HashMap<service_name, Vec<label>>
pub fn build_compose_service_labels(
    app_name: &str,
    domains: &[crate::services::networking::domain::DomainRecord],
) -> std::collections::HashMap<String, Vec<String>> {
    let shared_domains: Vec<crate::utils::builder::shared::traefik::SharedDomain> = domains
        .iter()
        .filter_map(|d| {
            // skip domains without service_name for compose stack
            if d.service_name
                .as_ref()
                .map(|s| s.is_empty())
                .unwrap_or(true)
            {
                return None;
            }

            Some(crate::utils::builder::shared::traefik::SharedDomain {
                key: d.id.to_string(),
                host: d.host.clone(),
                https: d.https != 0,
                port: d.port.unwrap_or(3000) as u16,
                service_name: d.service_name.clone(),
                path: d.path.clone().unwrap_or_else(|| "/".to_string()),
                internal_path: d.internal_path.clone().unwrap_or_else(|| "/".to_string()),
                strip_path: d.strip_path != 0,
                entrypoint: d.custom_entrypoint.clone(),
                certificate_type: d.certificate_type.clone(),
                custom_cert_resolver: d.custom_cert_resolver.clone(),
                middlewares: vec![], // Compose currently doesn't map middlewares field
            })
        })
        .collect();

    crate::utils::builder::shared::traefik::build_traefik_labels(app_name, &shared_domains)
}
