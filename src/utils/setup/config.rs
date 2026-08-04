#[derive(Clone, Debug)]
pub struct SetupPaths {
    pub base: String,
    pub traefik: String,
    pub traefik_dynamic: String,
    pub logs: String,
    pub applications: String,
    pub compose: String,
    pub ssh: String,
    pub certificates: String,
    pub monitoring: String,
    pub registry: String,
    pub schedules: String,
    pub volume_backups: String,
    pub volume_backup_lock: String,
    pub patch_repos: String,
}

impl SetupPaths {
    pub fn new(base: impl Into<String>) -> Self {
        let base = base.into();
        let traefik = format!("{base}/traefik");
        let traefik_dynamic = format!("{traefik}/dynamic");
        Self {
            logs: format!("{base}/logs"),
            applications: format!("{base}/applications"),
            compose: format!("{base}/compose"),
            ssh: format!("{base}/ssh"),
            certificates: format!("{traefik_dynamic}/certificates"),
            monitoring: format!("{base}/monitoring"),
            registry: format!("{base}/registry"),
            schedules: format!("{base}/schedules"),
            volume_backups: format!("{base}/volume-backups"),
            volume_backup_lock: format!("{base}/volume-backup-lock"),
            patch_repos: format!("{base}/patch-repos"),
            base,
            traefik,
            traefik_dynamic,
        }
    }
    pub fn all(&self) -> [&str; 14] {
        [
            &self.base,
            &self.traefik,
            &self.traefik_dynamic,
            &self.logs,
            &self.applications,
            &self.compose,
            &self.ssh,
            &self.certificates,
            &self.monitoring,
            &self.registry,
            &self.schedules,
            &self.volume_backups,
            &self.volume_backup_lock,
            &self.patch_repos,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct SetupConfig {
    pub paths: SetupPaths,
    pub network_name: String,
    pub traefik_name: String,
    pub traefik_version: String,
    pub http_port: u16,
    pub https_port: u16,
    pub http3_port: u16,
    pub dashboard_port: u16,
    pub acme_email: String,
    pub advertise_addr: Option<String>,
    pub monitoring_server_id: Option<i64>,
    pub monitoring_panel_url: Option<String>,
    pub monitoring_token: Option<String>,
}

impl Default for SetupConfig {
    fn default() -> Self {
        Self {
            paths: SetupPaths::new(crate::utils::paths::rustploy_paths().base),
            network_name: "rustploy-network".into(),
            traefik_name: "rustploy-traefik".into(),
            traefik_version: "3.6.7".into(),
            http_port: 80,
            https_port: 443,
            http3_port: 443,
            dashboard_port: 8080,
            acme_email: "admin@localhost".into(),
            advertise_addr: None,
            monitoring_server_id: None,
            monitoring_panel_url: None,
            monitoring_token: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_paths_are_namespaced_and_complete() {
        let config = SetupConfig::default();
        let expected_base = crate::utils::paths::rustploy_paths().base;
        assert_eq!(config.paths.base, expected_base);
        assert_eq!(config.paths.all().len(), 14);
        assert!(
            config
                .paths
                .certificates
                .starts_with(&format!("{expected_base}/traefik/dynamic"))
        );
        assert_eq!(config.network_name, "rustploy-network");
    }
}
