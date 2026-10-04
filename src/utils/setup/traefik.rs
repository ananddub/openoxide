use super::SetupConfig;
use serde::Serialize;

/// Traefik static configuration as typed structures, serialized with
/// `serde_yaml` so values (network names, emails, ports) can never produce
/// malformed or injectable YAML through string interpolation.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaticConfig {
    global: Global,
    providers: Providers,
    entry_points: EntryPoints,
    api: Api,
    metrics: Metrics,
    certificates_resolvers: CertResolvers,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Global {
    send_anonymous_usage: bool,
}

#[derive(Serialize)]
struct Providers {
    swarm: SwarmProvider,
    docker: DockerProvider,
    file: FileProvider,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SwarmProvider {
    exposed_by_default: bool,
    watch: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DockerProvider {
    exposed_by_default: bool,
    watch: bool,
    network: String,
}

#[derive(Serialize)]
struct FileProvider {
    directory: String,
    watch: bool,
}

#[derive(Serialize)]
struct EntryPoints {
    web: EntryPoint,
    websecure: WebSecureEntryPoint,
}

#[derive(Serialize)]
struct EntryPoint {
    address: String,
}

#[derive(Serialize)]
struct WebSecureEntryPoint {
    address: String,
    http3: Http3,
    http: HttpOptions,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Http3 {
    advertised_port: u16,
}

#[derive(Serialize)]
struct HttpOptions {
    tls: TlsOptions,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TlsOptions {
    cert_resolver: String,
}

#[derive(Serialize)]
struct Api {
    insecure: bool,
}

/// Per-service request counters for the Traffic view. Scraped by the panel on
/// the same insecure API port (:8080/metrics); no request payloads are stored.
#[derive(Serialize)]
struct Metrics {
    prometheus: PrometheusMetrics,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PrometheusMetrics {
    add_routers_labels: bool,
}

#[derive(Serialize)]
struct CertResolvers {
    letsencrypt: Letsencrypt,
}

#[derive(Serialize)]
struct Letsencrypt {
    acme: Acme,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Acme {
    email: String,
    storage: String,
    http_challenge: HttpChallenge,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HttpChallenge {
    entry_point: String,
}

fn to_yaml<T: Serialize>(value: &T) -> String {
    serde_yaml::to_string(value).expect("traefik config structures always serialize")
}

pub fn static_config(config: &SetupConfig) -> String {
    let model = StaticConfig {
        global: Global {
            send_anonymous_usage: false,
        },
        providers: Providers {
            swarm: SwarmProvider {
                exposed_by_default: false,
                watch: true,
            },
            docker: DockerProvider {
                exposed_by_default: false,
                watch: true,
                network: config.network_name.clone(),
            },
            file: FileProvider {
                directory: "/etc/openoxide/traefik/dynamic".into(),
                watch: true,
            },
        },
        entry_points: EntryPoints {
            web: EntryPoint {
                address: format!(":{}", config.http_port),
            },
            websecure: WebSecureEntryPoint {
                address: format!(":{}", config.https_port),
                http3: Http3 {
                    advertised_port: config.http3_port,
                },
                http: HttpOptions {
                    tls: TlsOptions {
                        cert_resolver: "letsencrypt".into(),
                    },
                },
            },
        },
        api: Api { insecure: true },
        metrics: Metrics {
            prometheus: PrometheusMetrics {
                add_routers_labels: true,
            },
        },
        certificates_resolvers: CertResolvers {
            letsencrypt: Letsencrypt {
                acme: Acme {
                    email: config.acme_email.clone(),
                    storage: "/etc/openoxide/traefik/dynamic/acme.json".into(),
                    http_challenge: HttpChallenge {
                        entry_point: "web".into(),
                    },
                },
            },
        },
    };
    to_yaml(&model)
}

#[derive(Serialize)]
struct MiddlewaresFile {
    http: HttpRoot,
}

#[derive(Serialize)]
struct HttpRoot {
    middlewares: Middlewares,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct Middlewares {
    redirect_to_https: RedirectToHttps,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RedirectToHttps {
    redirect_scheme: RedirectScheme,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RedirectScheme {
    scheme: String,
    permanent: bool,
}

pub fn default_middlewares() -> String {
    let model = MiddlewaresFile {
        http: HttpRoot {
            middlewares: Middlewares {
                redirect_to_https: RedirectToHttps {
                    redirect_scheme: RedirectScheme {
                        scheme: "https".into(),
                        permanent: true,
                    },
                },
            },
        },
    };
    to_yaml(&model)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_uses_selected_network_ports_and_acme() {
        let config = SetupConfig {
            http_port: 8080,
            acme_email: "ops@example.com".into(),
            ..Default::default()
        };
        let yaml = static_config(&config);
        let parsed: serde_yaml::Value =
            serde_yaml::from_str(&yaml).expect("static config is valid yaml");
        assert_eq!(
            parsed["providers"]["docker"]["network"].as_str(),
            Some("openoxide-network")
        );
        assert_eq!(
            parsed["entryPoints"]["web"]["address"].as_str(),
            Some(":8080")
        );
        assert_eq!(
            parsed["certificatesResolvers"]["letsencrypt"]["acme"]["email"].as_str(),
            Some("ops@example.com")
        );
        assert!(parsed["metrics"]["prometheus"].is_mapping());
        assert_eq!(
            parsed["global"]["sendAnonymousUsage"].as_bool(),
            Some(false)
        );
        assert_eq!(
            parsed["entryPoints"]["websecure"]["http"]["tls"]["certResolver"].as_str(),
            Some("letsencrypt")
        );
        let middlewares: serde_yaml::Value =
            serde_yaml::from_str(&default_middlewares()).expect("middlewares are valid yaml");
        assert_eq!(
            middlewares["http"]["middlewares"]["redirect-to-https"]["redirectScheme"]["scheme"]
                .as_str(),
            Some("https")
        );
    }

    #[test]
    fn hostile_values_cannot_forge_yaml_structure() {
        // A network name containing YAML syntax must serialize as a scalar,
        // never as extra keys or nested structure.
        let config = SetupConfig {
            network_name: "net:\n  fake: true".into(),
            ..Default::default()
        };
        let yaml = static_config(&config);
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("valid yaml");
        assert_eq!(
            parsed["providers"]["docker"]["network"].as_str().unwrap(),
            "net:\n  fake: true"
        );
    }
}
