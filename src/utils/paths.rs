const DEFAULT_BASE_PATH: &str = "/etc/openoxide";
const BASE_PATH_ENV: &str = "OPENOXIDE_BASE_PATH";

#[derive(Clone, Debug)]
pub struct OpenOxidePaths {
    pub base: String,
}

impl OpenOxidePaths {
    pub fn from_env() -> Self {
        let base = std::env::var(BASE_PATH_ENV)
            .or_else(|_| std::env::var("RUSTPLOY_BASE_PATH"))
            .ok()
            .map(|value| value.trim().trim_end_matches('/').to_owned())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_BASE_PATH.into());
        Self { base }
    }

    pub fn applications(&self) -> String {
        format!("{}/applications", self.base)
    }

    pub fn application_dir(&self, app_name: &str) -> String {
        format!("{}/{}", self.applications(), app_name)
    }

    pub fn application_code(&self, app_name: &str) -> String {
        format!("{}/code", self.application_dir(app_name))
    }

    pub fn application_files(&self, app_name: &str) -> String {
        format!("{}/files", self.application_dir(app_name))
    }

    pub fn compose(&self) -> String {
        format!("{}/compose", self.base)
    }

    pub fn compose_dir(&self, app_name: &str) -> String {
        format!("{}/{}", self.compose(), app_name)
    }

    pub fn compose_source(&self, app_name: &str) -> String {
        format!("{}/source", self.compose_dir(app_name))
    }

    pub fn compose_files(&self, app_name: &str) -> String {
        format!("{}/files", self.compose_dir(app_name))
    }

    pub fn traefik_dynamic(&self) -> String {
        format!("{}/traefik/dynamic", self.base)
    }

    pub fn traefik_application_file(&self, app_name: &str) -> String {
        format!("{}/{}.yml", self.traefik_dynamic(), app_name)
    }

    pub fn deployment_logs(&self) -> String {
        format!("{}/logs/deployments", self.base)
    }

    pub fn deployment_log_file(&self, deployment_id: i64) -> String {
        format!("{}/{}.log", self.deployment_logs(), deployment_id)
    }

    pub fn schedule_logs(&self) -> String {
        format!("{}/logs/schedules", self.base)
    }

    pub fn schedule_log_file(&self, key: &str) -> String {
        format!("{}/{}.log", self.schedule_logs(), key)
    }
}

pub fn openoxide_paths() -> OpenOxidePaths {
    OpenOxidePaths::from_env()
}
