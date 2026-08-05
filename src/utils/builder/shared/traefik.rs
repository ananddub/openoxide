use crate::utils::traefik::{
    entrypoint::Entrypoint,
    middleware::Middleware,
    rule::Rule,
    tls::{CertResolver, TlsConfig},
    traefik::TraefikBuilder,
    types::CertificateType,
};
use std::collections::{BTreeMap, HashMap};

const TRAEFIK_NETWORK: &str = "rustploy-network";

pub struct SharedDomain {
    pub key: String,
    pub host: String,
    pub https: bool,
    pub port: u16,
    pub service_name: Option<String>,
    pub path: String,
    pub internal_path: String,
    pub strip_path: bool,
    pub entrypoint: Option<String>,
    pub certificate_type: String,
    pub custom_cert_resolver: Option<String>,
    pub middlewares: Vec<String>,
}

/// Build traefik labels grouped by Swarm service name.
/// Returns HashMap<service_name, Vec<label>>
pub fn build_traefik_labels(
    app_name: &str,
    domains: &[SharedDomain],
) -> HashMap<String, Vec<String>> {
    build_traefik_labels_for_network(app_name, domains, TRAEFIK_NETWORK)
}

pub fn build_traefik_labels_for_network(
    app_name: &str,
    domains: &[SharedDomain],
    network: &str,
) -> HashMap<String, Vec<String>> {
    let mut builders: HashMap<String, TraefikBuilder> = HashMap::new();

    for domain in domains {
        let service_name = match &domain.service_name {
            Some(s) if !s.is_empty() => {
                if s == app_name || s.starts_with(&format!("{app_name}_")) {
                    s.clone()
                } else {
                    format!("{app_name}_{s}")
                }
            }
            _ => app_name.to_string(), // fallback to app_name for non-compose (Application)
        };

        let mut traefik = TraefikBuilder::new().enable().network(network);

        let entrypoint =
            domain
                .entrypoint
                .clone()
                .map(Entrypoint::custom)
                .unwrap_or(if domain.https {
                    Entrypoint::WebSecure
                } else {
                    Entrypoint::Web
                });

        let router_name = format!("{app_name}-{}", domain.key);

        let rule = {
            let base = Rule::host(&domain.host);
            if domain.path != "/" {
                base.and(Rule::path_prefix(&domain.path))
            } else {
                base
            }
        };

        // --- Middlewares ---
        let mut middleware_names: Vec<String> = Vec::new();

        if domain.strip_path && domain.path != "/" {
            let name = format!("stripprefix-{router_name}");
            traefik = traefik.middleware(Middleware::StripPrefix {
                name: name.clone(),
                prefixes: vec![domain.path.clone()],
            });
            middleware_names.push(name);
        }

        if domain.internal_path != "/" && domain.internal_path != domain.path {
            let name = format!("addprefix-{router_name}");
            traefik = traefik.middleware(Middleware::AddPrefix {
                name: name.clone(),
                prefix: domain.internal_path.clone(),
            });
            middleware_names.push(name);
        }

        middleware_names.extend(domain.middlewares.clone());

        // --- Main router ---
        let mut r = traefik
            .router(&router_name)
            .rule(&rule)
            .entrypoint(&entrypoint);

        if !middleware_names.is_empty() {
            r = r.middlewares(&middleware_names);
        }

        if domain.https {
            let cert_type = CertificateType::from(domain.certificate_type.as_str());
            let tls = match cert_type {
                CertificateType::LetsEncrypt => CertResolver::LetsEncrypt.into(),
                CertificateType::Custom => domain
                    .custom_cert_resolver
                    .clone()
                    .map(CertResolver::Custom)
                    .map(TlsConfig::from)
                    .unwrap_or_else(TlsConfig::enabled),
                CertificateType::None => TlsConfig::enabled(),
            };
            r = r.tls_config(tls);
        }

        traefik = r
            .service(&router_name)
            .service(&router_name)
            .port(domain.port)
            .finish();

        // --- HTTP → HTTPS redirect router ---
        if domain.https && domain.entrypoint.is_none() {
            let redirect_name = format!("{router_name}-redirect");
            traefik = traefik
                .router(&redirect_name)
                .rule(&rule)
                .entrypoint(Entrypoint::Web)
                .middleware("redirect-to-https@file")
                .finish();
        }

        builders
            .entry(service_name)
            .or_insert_with(TraefikBuilder::new)
            .append(traefik);
    }

    builders
        .into_iter()
        .map(|(service, builder)| (service, deduplicate_labels_by_key(builder.build())))
        .collect()
}

/// Docker Compose validates label sequences as a set, while Traefik's common
/// labels (`traefik.enable`, `traefik.docker.network`) are emitted once per
/// domain. Multiple domains on one service must therefore be reconciled by
/// label key before serializing the stack file.
fn deduplicate_labels_by_key(labels: Vec<String>) -> Vec<String> {
    let mut unique = BTreeMap::<String, String>::new();
    for label in labels {
        let key = label.split_once('=').map_or(label.as_str(), |(key, _)| key);
        unique.insert(key.to_owned(), label);
    }
    unique.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain(key: &str, host: &str) -> SharedDomain {
        SharedDomain {
            key: key.into(),
            host: host.into(),
            https: false,
            port: 3000,
            service_name: None,
            path: "/".into(),
            internal_path: "/".into(),
            strip_path: false,
            entrypoint: None,
            certificate_type: "NONE".into(),
            custom_cert_resolver: None,
            middlewares: vec![],
        }
    }

    #[test]
    fn multiple_domains_emit_common_labels_once() {
        let labels =
            build_traefik_labels("api", &[domain("1", "one.test"), domain("2", "two.test")])
                .remove("api")
                .unwrap();
        assert_eq!(
            labels
                .iter()
                .filter(|v| v.starts_with("traefik.enable="))
                .count(),
            1
        );
        assert_eq!(
            labels
                .iter()
                .filter(|v| v.starts_with("traefik.docker.network="))
                .count(),
            1
        );
        let keys: std::collections::HashSet<_> = labels
            .iter()
            .map(|v| v.split_once('=').map_or(v.as_str(), |(k, _)| k))
            .collect();
        assert_eq!(keys.len(), labels.len());
    }
}
