use super::{
    compose::ComposeBuilder,
    spec::{ComposeRuntime, ComposeSpec},
};
use crate::utils::{
    builder::spec::DomainSpec,
    exec::{ExecError, ExecResult},
};
use serde_yaml::{Mapping, Value};
use tokio_util::sync::CancellationToken;

const TRAEFIK_NETWORK: &str = "rustploy-network";

pub(super) async fn write_labeled_compose(
    builder: &ComposeBuilder,
    spec: &ComposeSpec,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    if spec.domains.is_empty() && spec.service_networks.is_empty() {
        return Ok(());
    }

    let compose_path = spec.compose_file_path();
    let output = builder
        .ctx
        .executor
        .run_cancelled("cat", [compose_path.as_str()], cancel)
        .await?;
    let mut document = serde_yaml::from_str::<Value>(&output.stdout)
        .map_err(|error| command_error(format!("invalid compose yaml: {error}")))?;

    inject_domain_labels(&mut document, spec)?;

    let content = serde_yaml::to_string(&document)
        .map_err(|error| ExecError::Json(serde_json::Error::io(std::io::Error::other(error))))?;
    builder
        .ctx
        .write_file_cancelled(&compose_path, content.as_bytes(), cancel)
        .await
}

pub fn inject_compose_yaml_labels(yaml_str: &str, spec: &ComposeSpec) -> Result<String, String> {
    let mut document = serde_yaml::from_str::<Value>(yaml_str)
        .map_err(|e| format!("invalid compose yaml: {e}"))?;

    inject_domain_labels(&mut document, spec)
        .map_err(|e| format!("failed to inject domain labels: {e}"))?;

    serde_yaml::to_string(&document).map_err(|e| format!("failed to serialize yaml: {e}"))
}

fn strip_traefik_labels_from_services(services: &mut Mapping) {
    for (_name, service) in services.iter_mut() {
        if let Ok(service_map) = mapping_mut(service, "") {
            if let Some(labels) = service_map.get_mut(Value::String("labels".into())) {
                match labels {
                    Value::Sequence(items) => {
                        items.retain(|item| {
                            if let Value::String(s) = item {
                                !s.starts_with("traefik.")
                            } else {
                                true
                            }
                        });
                    }
                    Value::Mapping(map) => {
                        map.retain(|k, _| {
                            if let Value::String(s) = k {
                                !s.starts_with("traefik.")
                            } else {
                                true
                            }
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn inject_domain_labels(document: &mut Value, spec: &ComposeSpec) -> ExecResult<()> {
    let root = mapping_mut(document, "compose root must be a yaml object")?;
    let services = root
        .get_mut(Value::String("services".into()))
        .ok_or_else(|| command_error("compose file has no services"))?;
    let services = mapping_mut(services, "compose services must be a yaml object")?;

    if !spec.domains.is_empty() {
        strip_traefik_labels_from_services(services);
    }

    for domain in &spec.domains {
        let service_name = domain.service_name.as_deref().ok_or_else(|| {
            command_error(format!("domain {} is missing service_name", domain.host))
        })?;
        let service = services
            .get_mut(Value::String(service_name.into()))
            .ok_or_else(|| {
                command_error(format!(
                    "domain {} points to missing compose service {}",
                    domain.host, service_name
                ))
            })?;
        inject_labels_for_service(service, spec, domain)?;
        add_named_network_to_service(service, TRAEFIK_NETWORK)?;
    }

    for service_network in &spec.service_networks {
        let service = services
            .get_mut(Value::String(service_network.service_name.clone()))
            .ok_or_else(|| {
                command_error(format!(
                    "network config points to missing compose service {}",
                    service_network.service_name
                ))
            })?;
        for network in &service_network.networks {
            add_named_network_to_service(service, network)?;
        }
    }

    if !spec.domains.is_empty() {
        add_network_to_root(root, TRAEFIK_NETWORK);
    }
    for service_network in &spec.service_networks {
        for network in &service_network.networks {
            add_network_to_root(root, network);
        }
    }
    Ok(())
}

fn inject_labels_for_service(
    service: &mut Value,
    spec: &ComposeSpec,
    domain: &DomainSpec,
) -> ExecResult<()> {
    let service = mapping_mut(service, "compose service must be a yaml object")?;
    let labels_value = match spec.runtime {
        ComposeRuntime::Compose => service
            .entry(Value::String("labels".into()))
            .or_insert_with(|| Value::Sequence(vec![])),
        ComposeRuntime::Stack => {
            let deploy = service
                .entry(Value::String("deploy".into()))
                .or_insert_with(|| Value::Mapping(Mapping::new()));
            mapping_mut(deploy, "compose deploy must be a yaml object")?
                .entry(Value::String("labels".into()))
                .or_insert_with(|| Value::Sequence(vec![]))
        }
    };

    for label in create_domain_labels(spec, domain) {
        insert_label(labels_value, label)?;
    }
    Ok(())
}

fn create_domain_labels(spec: &ComposeSpec, domain: &DomainSpec) -> Vec<String> {
    let shared_domain = crate::utils::builder::shared::traefik::SharedDomain {
        key: domain.key.clone(),
        host: domain.host.clone(),
        https: domain.https,
        port: domain.port,
        service_name: domain.service_name.clone(),
        path: domain.path.clone(),
        internal_path: domain.internal_path.clone(),
        strip_path: domain.strip_path,
        entrypoint: domain.entrypoint.clone(),
        certificate_type: domain.certificate_type.clone(),
        custom_cert_resolver: domain.custom_cert_resolver.clone(),
        middlewares: domain.middlewares.clone(),
    };

    let map = crate::utils::builder::shared::traefik::build_traefik_labels(
        &spec.app_name,
        &[shared_domain],
    );
    // The key in the map will be domain.service_name (prefixed) or app_name fallback.
    // Since there's only 1 domain passed, we just take the first/only value.
    map.into_values().next().unwrap_or_default()
}

fn add_named_network_to_service(service: &mut Value, network_name: &str) -> ExecResult<()> {
    let service = mapping_mut(service, "compose service must be a yaml object")?;
    let networks = service
        .entry(Value::String("networks".into()))
        .or_insert_with(|| Value::Sequence(vec![]));
    match networks {
        Value::Sequence(items) => {
            push_unique(items, Value::String(network_name.into()));
            Ok(())
        }
        Value::Mapping(map) => {
            map.entry(Value::String(network_name.into()))
                .or_insert(Value::Null);
            Ok(())
        }
        _ => Err(command_error("service networks must be a list or object")),
    }
}

fn add_network_to_root(root: &mut Mapping, network_name: &str) {
    let networks = root
        .entry(Value::String("networks".into()))
        .or_insert_with(|| Value::Mapping(Mapping::new()));
    if let Ok(map) = mapping_mut(networks, "root networks must be a yaml object") {
        let mut network = Mapping::new();
        network.insert(Value::String("external".into()), Value::Bool(true));
        network.insert(
            Value::String("name".into()),
            Value::String(network_name.into()),
        );
        map.entry(Value::String(network_name.into()))
            .or_insert(Value::Mapping(network));
    }
}

fn insert_label(labels: &mut Value, label: impl Into<String>) -> ExecResult<()> {
    let label = label.into();
    match labels {
        Value::Sequence(items) => {
            push_unique(items, Value::String(label));
            Ok(())
        }
        Value::Mapping(map) => {
            let Some((key, value)) = label.split_once('=') else {
                return Err(command_error("invalid traefik label"));
            };
            map.insert(Value::String(key.into()), Value::String(value.into()));
            Ok(())
        }
        _ => Err(command_error("labels must be a list or object")),
    }
}

fn push_unique(items: &mut Vec<Value>, value: Value) {
    if !items.iter().any(|item| item == &value) {
        items.insert(0, value);
    }
}

fn mapping_mut<'a>(value: &'a mut Value, message: &str) -> ExecResult<&'a mut Mapping> {
    match value {
        Value::Mapping(map) => Ok(map),
        _ => Err(command_error(message)),
    }
}

/// Build traefik labels grouped by Swarm service name.
/// Used for zero-redeploy label updates via `docker service update`.
/// Returns HashMap<service_name, Vec<label>>
pub fn build_compose_service_labels(
    app_name: &str,
    domains: &[crate::services::domain::DomainRecord],
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

fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::builder::{
        compose::spec::{ComposeSource, ComposeSpec},
        spec::DomainSpec,
    };
    use std::collections::BTreeMap;

    #[test]
    fn injects_stack_labels_on_service_deploy() {
        let mut doc: Value = serde_yaml::from_str(
            r#"
services:
  web:
    image: nginx
"#,
        )
        .unwrap();
        inject_domain_labels(&mut doc, &spec(ComposeRuntime::Stack)).unwrap();
        let out = serde_yaml::to_string(&doc).unwrap();
        assert!(out.contains("traefik.enable=true"));
        assert!(out.contains("traefik.docker.network=rustploy-network"));
        assert!(out.contains("traefik.http.routers.site-1.tls.certresolver=letsencrypt"));
        assert!(out.contains("rustploy-network"));
    }

    #[test]
    fn injects_compose_labels_on_service_labels() {
        let mut doc: Value = serde_yaml::from_str(
            r#"
services:
  web:
    image: nginx
"#,
        )
        .unwrap();
        inject_domain_labels(&mut doc, &spec(ComposeRuntime::Compose)).unwrap();
        let out = serde_yaml::to_string(&doc).unwrap();
        assert!(out.contains("traefik.docker.network=rustploy-network"));
        assert!(out.contains("traefik.http.services.site-1.loadbalancer.server.port=3000"));
    }

    #[test]
    fn injects_service_networks_without_domains() {
        let mut spec = spec(ComposeRuntime::Compose);
        spec.domains.clear();
        spec.service_networks = vec![
            crate::utils::builder::compose::spec::ComposeServiceNetworkSpec {
                service_name: "web".into(),
                networks: vec!["private-net".into()],
            },
        ];

        let out = inject_compose_yaml_labels(
            r#"
services:
  web:
    image: nginx
"#,
            &spec,
        )
        .unwrap();

        assert!(out.contains("private-net"));
        assert!(out.contains("external: true"));
        assert!(!out.contains("traefik.enable=true"));
    }

    fn spec(runtime: ComposeRuntime) -> ComposeSpec {
        ComposeSpec {
            app_name: "site".into(),
            stack_name: "site".into(),
            source: ComposeSource::Raw {
                content: String::new(),
            },
            runtime,
            work_directory: "/tmp/site".into(),
            compose_path: "/tmp/site/compose.yml".into(),
            rendered_stack_file: "/tmp/site/rendered.yml".into(),
            env_file: "/tmp/site/.env".into(),
            environment: BTreeMap::new(),
            mounts: vec![],
            patches: vec![],
            domains: vec![DomainSpec {
                key: "1".into(),
                host: "site.example.com".into(),
                https: true,
                port: 3000,
                service_name: Some("web".into()),
                path: "/".into(),
                internal_path: "/".into(),
                strip_path: false,
                entrypoint: None,
                certificate_type: "LETSENCRYPT".into(),
                custom_cert_resolver: None,
                middlewares: vec![],
            }],
            service_networks: vec![],
            randomize: false,
            suffix: String::new(),
            isolated_deployment: false,
            isolated_deployments_volume: false,
        }
    }
}
