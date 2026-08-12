use super::SetupConfig;

pub fn static_config(config: &SetupConfig) -> String {
    format!(
        r#"global:
  sendAnonymousUsage: false
providers:
  swarm:
    exposedByDefault: false
    watch: true
  docker:
    exposedByDefault: false
    watch: true
    network: "{}"
  file:
    directory: /etc/openoxide/traefik/dynamic
    watch: true
entryPoints:
  web:
    address: ":{}"
  websecure:
    address: ":{}"
    http3:
      advertisedPort: {}
    http:
      tls:
        certResolver: letsencrypt
api:
  insecure: true
certificatesResolvers:
  letsencrypt:
    acme:
      email: "{}"
      storage: /etc/openoxide/traefik/dynamic/acme.json
      httpChallenge:
        entryPoint: web
"#,
        config.network_name,
        config.http_port,
        config.https_port,
        config.http3_port,
        config.acme_email
    )
}

pub fn default_middlewares() -> &'static str {
    r#"http:
  middlewares:
    redirect-to-https:
      redirectScheme:
        scheme: https
        permanent: true
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_uses_selected_network_ports_and_acme() {
        let mut config = SetupConfig::default();
        config.http_port = 8080;
        config.acme_email = "ops@example.com".into();
        let yaml = static_config(&config);
        assert!(yaml.contains("network: \"openoxide-network\""));
        assert!(yaml.contains("address: \":8080\""));
        assert!(yaml.contains("email: \"ops@example.com\""));
        assert!(default_middlewares().contains("redirect-to-https"));
    }
}
