use crate::utils::builder::spec::{ApplicationSpec, HealthSpec, MountKind};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub(super) struct StackFile {
    version: &'static str,
    services: BTreeMap<String, StackService>,
    networks: BTreeMap<String, ExternalNetwork>,
}

#[derive(Serialize)]
struct StackService {
    image: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    environment: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    args: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    volumes: Vec<StackMount>,
    networks: Vec<String>,
    deploy: DeploySpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthcheck: Option<HealthSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_grace_period: Option<String>,
}

#[derive(Serialize)]
struct DeploySpec {
    replicas: u32,
    resources: DeployResources,
    restart_policy: RestartPolicy,
    update_config: UpdateConfig,
    rollback_config: UpdateConfig,
    #[serde(skip_serializing_if = "Placement::is_empty")]
    placement: Placement,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    labels: Vec<String>,
}

#[derive(Serialize)]
struct DeployResources {
    #[serde(skip_serializing_if = "Limits::is_empty")]
    limits: Limits,
    #[serde(skip_serializing_if = "Limits::is_empty")]
    reservations: Limits,
}

#[derive(Serialize, Default)]
struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<String>,
}

impl Limits {
    fn is_empty(&self) -> bool {
        self.cpus.is_none() && self.memory.is_none()
    }
}

#[derive(Serialize)]
struct RestartPolicy {
    condition: &'static str,
    delay: &'static str,
    max_attempts: u8,
    window: &'static str,
}

#[derive(Serialize)]
struct UpdateConfig {
    parallelism: u8,
    delay: &'static str,
    order: &'static str,
    failure_action: &'static str,
}

#[derive(Serialize, Default)]
struct Placement {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    constraints: Vec<String>,
}

impl Placement {
    fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}

#[derive(Serialize)]
struct ExternalNetwork {
    external: bool,
    name: String,
}

#[derive(Serialize)]
struct StackMount {
    #[serde(rename = "type")]
    kind: &'static str,
    source: String,
    target: String,
    #[serde(skip_serializing_if = "is_false")]
    read_only: bool,
}

pub(super) fn stack_spec(app: &ApplicationSpec) -> StackFile {
    let shared_domains: Vec<crate::utils::builder::shared::traefik::SharedDomain> = app
        .domains
        .iter()
        .map(|d| crate::utils::builder::shared::traefik::SharedDomain {
            key: d.key.clone(),
            host: d.host.clone(),
            https: d.https,
            port: d.port,
            service_name: d.service_name.clone(),
            path: d.path.clone(),
            internal_path: d.internal_path.clone(),
            strip_path: d.strip_path,
            entrypoint: d.entrypoint.clone(),
            certificate_type: d.certificate_type.clone(),
            custom_cert_resolver: d.custom_cert_resolver.clone(),
            middlewares: d.middlewares.clone(),
        })
        .collect();

    let traefik_network = app
        .networks
        .iter()
        .find(|network| network.as_str() == crate::utils::builder::swarm::RUSTPLOY_NETWORK)
        .or_else(|| app.networks.first())
        .map(String::as_str)
        .unwrap_or(crate::utils::builder::swarm::RUSTPLOY_NETWORK);
    let traefik_map = crate::utils::builder::shared::traefik::build_traefik_labels_for_network(
        &app.app_name,
        &shared_domains,
        traefik_network,
    );
    let traefik_labels = reconcile_labels(traefik_map.into_values().flatten());
    let mut services = BTreeMap::new();
    services.insert(
        app.app_name.clone(),
        StackService {
            image: app.image.clone(),
            environment: app.environment.clone(),
            command: app.command.clone(),
            args: app.args.clone(),
            volumes: app
                .mounts
                .iter()
                .map(|mount| StackMount {
                    kind: match mount.kind {
                        MountKind::Volume => "volume",
                        MountKind::Bind | MountKind::File => "bind",
                    },
                    source: mount.source.clone(),
                    target: mount.target.clone(),
                    read_only: mount.read_only || matches!(mount.kind, MountKind::File),
                })
                .collect(),
            networks: app.networks.clone(),
            deploy: DeploySpec {
                replicas: app.replicas,
                resources: DeployResources {
                    limits: Limits {
                        cpus: app.resources.cpu_limit.clone(),
                        memory: app.resources.memory_limit.clone(),
                    },
                    reservations: Limits {
                        cpus: app.resources.cpu_reservation.clone(),
                        memory: app.resources.memory_reservation.clone(),
                    },
                },
                restart_policy: RestartPolicy {
                    condition: "on-failure",
                    delay: "5s",
                    max_attempts: 3,
                    window: "120s",
                },
                update_config: UpdateConfig {
                    parallelism: 1,
                    delay: "5s",
                    order: "start-first",
                    failure_action: "rollback",
                },
                rollback_config: UpdateConfig {
                    parallelism: 1,
                    delay: "5s",
                    order: "stop-first",
                    failure_action: "pause",
                },
                placement: Placement {
                    constraints: app.placement_constraints.clone(),
                },
                labels: traefik_labels,
            },
            healthcheck: app.healthcheck.clone(),
            stop_grace_period: app.stop_grace_period.clone(),
        },
    );

    let mut networks = BTreeMap::new();
    for network in &app.networks {
        networks.insert(
            network.clone(),
            ExternalNetwork {
                external: true,
                name: network.clone(),
            },
        );
    }

    StackFile {
        version: "3.9",
        services,
        networks,
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn reconcile_labels(labels: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut unique = BTreeMap::<String, String>::new();
    for label in labels {
        let key = label.split_once('=').map_or(label.as_str(), |(key, _)| key);
        unique.insert(key.to_owned(), label);
    }
    unique.into_values().collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::utils::builder::spec::{ApplicationSpec, MountSpec, ResourceSpec, SourceSpec};

    pub(crate) fn spec() -> ApplicationSpec {
        ApplicationSpec {
            app_name: "api".into(),
            stack_name: "prod".into(),
            source: SourceSpec::Docker {
                image: "api:1".into(),
                registry: None,
            },
            build: None,
            work_directory: "/tmp/api".into(),
            image: "api:1".into(),
            environment: BTreeMap::from([("PORT".into(), "3000".into())]),
            build_args: BTreeMap::new(),
            build_secrets: BTreeMap::new(),
            command: None,
            args: vec![],
            replicas: 2,
            networks: vec!["rustploy-network".into()],
            mounts: vec![MountSpec {
                kind: MountKind::Volume,
                source: "api-data".into(),
                target: "/data".into(),
                read_only: false,
                content: None,
            }],
            domains: vec![],
            resources: ResourceSpec {
                memory_limit: Some("512M".into()),
                ..Default::default()
            },
            healthcheck: None,
            placement_constraints: vec!["node.role==worker".into()],
            stop_grace_period: Some("15s".into()),
        }
    }

    #[test]
    fn stack_yaml_contains_reconciled_deployment_config() {
        let yaml = serde_yaml::to_string(&stack_spec(&spec())).unwrap();
        assert!(yaml.contains("replicas: 2"));
        assert!(yaml.contains("rustploy-network"));
        assert!(yaml.contains("node.role==worker"));
        assert!(yaml.contains("failure_action: rollback"));
        assert!(yaml.contains("source: api-data"));
    }

    #[test]
    fn final_application_labels_are_unique_by_key() {
        let labels = reconcile_labels([
            "traefik.enable=true".into(),
            "traefik.http.routers.one.rule=Host(`one.test`)".into(),
            "traefik.enable=true".into(),
            "traefik.http.routers.two.rule=Host(`two.test`)".into(),
        ]);
        assert_eq!(
            labels
                .iter()
                .filter(|label| label.starts_with("traefik.enable="))
                .count(),
            1
        );
        assert_eq!(labels.len(), 3);
    }
}
