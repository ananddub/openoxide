use crate::utils::traefik::{
    middleware::Middleware, rule::Rule, traefik::TraefikBuilder, types::CertificateType,
};
use std::collections::HashMap;

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

        let mut traefik = TraefikBuilder::new().enable().network(TRAEFIK_NETWORK);

        let entrypoint = domain.entrypoint.clone().unwrap_or_else(|| {
            if domain.https {
                "websecure".into()
            } else {
                "web".into()
            }
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
            r = r.tls(true);
            let cert_type = CertificateType::from(domain.certificate_type.as_str());
            match cert_type {
                CertificateType::LetsEncrypt => {
                    r = r.cert_resolver("letsencrypt");
                }
                CertificateType::Custom => {
                    if let Some(resolver) = &domain.custom_cert_resolver {
                        r = r.cert_resolver(resolver);
                    }
                }
                CertificateType::None => {}
            }
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
                .entrypoint("web")
                .middleware("redirect-to-https@file")
                .finish();
        }

        builders
            .entry(service_name)
            .or_insert_with(TraefikBuilder::new)
            .labels
            .extend(traefik.labels);
    }

    builders.into_iter().map(|(k, v)| (k, v.build())).collect()
}
