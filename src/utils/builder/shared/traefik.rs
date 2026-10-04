use crate::utils::traefik::{
    builder::TraefikBuilder,
    entrypoint::Entrypoint,
    middleware::Middleware,
    rule::Rule,
    tls::{CertResolver, TlsConfig},
    types::CertificateType,
};
use std::collections::{BTreeMap, HashMap};

const TRAEFIK_NETWORK: &str = "openoxide-network";

/// Converts an internationalized domain name (IDN) to ASCII punycode. Traefik
/// requires ASCII hostnames, so `тест.рф` becomes `xn--e1aybc.xn--p1ai`.
/// Falls back to the original host if parsing fails.
pub fn to_punycode(host: &str) -> String {
    url::Url::parse(&format!("http://{host}"))
        .ok()
        .and_then(|u| u.host_str().map(str::to_owned))
        .unwrap_or_else(|| host.to_owned())
}

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

        // A stored empty/whitespace entrypoint (older rows default to "") is
        // "unset": emitting `entrypoints=""` makes Traefik reject the router.
        let custom_entrypoint = domain
            .entrypoint
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let entrypoint = custom_entrypoint
            .map(Entrypoint::custom)
            .unwrap_or(if domain.https {
                Entrypoint::WebSecure
            } else {
                Entrypoint::Web
            });

        let router_name = format!("{app_name}-{}", domain.key);

        let rule = {
            let punycode_host = to_punycode(&domain.host);
            let base = Rule::host(&punycode_host);
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
        // The service must be explicit: with several watched services on one
        // container, Traefik's name-prefix auto-link refuses the router as
        // ambiguous ("cannot be linked automatically with multiple Services").
        if domain.https && custom_entrypoint.is_none() {
            let redirect_name = format!("{router_name}-redirect");
            traefik = traefik
                .router(&redirect_name)
                .rule(&rule)
                .entrypoint(Entrypoint::Web)
                .middleware("redirect-to-https@file")
                .service(&router_name);
        }

        builders.entry(service_name).or_default().append(traefik);
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
    fn idn_host_is_converted_to_punycode() {
        assert_eq!(to_punycode("тест.рф"), "xn--e1aybc.xn--p1ai");
        assert_eq!(to_punycode("example.com"), "example.com");
        let mut d = domain("1", "münchen.de");
        d.https = false;
        let labels = build_traefik_labels("api", &[d]).remove("api").unwrap();
        assert!(
            labels.iter().any(|l| l.contains("xn--mnchen-3ya.de")),
            "router rule must use the punycode host"
        );
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
    #[test]
    fn empty_entrypoint_falls_back_to_default_entry_points() {
        // Older domain rows store "" for custom_entrypoint; emitting
        // `entrypoints=""` makes Traefik reject the whole router.
        let mut blank = domain("1", "blank.test");
        blank.https = true;
        blank.entrypoint = Some("".into());
        let labels = build_traefik_labels("api", &[blank]).remove("api").unwrap();

        let main = labels
            .iter()
            .find(|label| label.starts_with("traefik.http.routers.api-1.entrypoints="))
            .expect("main router entrypoint label");
        assert_eq!(main.split_once('=').unwrap().1, "websecure");
        // An empty entrypoint counts as unset, so the HTTPS redirect router
        // must still be emitted on `web`.
        let redirect = labels
            .iter()
            .find(|label| label.starts_with("traefik.http.routers.api-1-redirect.entrypoints="))
            .expect("redirect router entrypoint label");
        assert_eq!(redirect.split_once('=').unwrap().1, "web");
    }

    #[test]
    fn custom_entrypoint_is_used_verbatim() {
        let mut custom = domain("1", "custom.test");
        custom.entrypoint = Some("edge".into());
        let labels = build_traefik_labels("api", &[custom])
            .remove("api")
            .unwrap();
        assert!(
            labels
                .iter()
                .any(|label| label == "traefik.http.routers.api-1.entrypoints=edge")
        );
        // A named entrypoint suppresses the automatic redirect router.
        assert!(!labels.iter().any(|label| label.contains("api-1-redirect")));
    }
}
