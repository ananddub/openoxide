use super::{DockerCli, DockerDiskUsage, DockerInfo, DockerOutput, DockerResult, DockerVersion};

impl DockerCli {
    pub async fn info(&self) -> DockerResult<DockerInfo> {
        self.json(&["info", "--format", "{{json .}}"]).await
    }
    pub async fn version(&self) -> DockerResult<DockerVersion> {
        self.json(&["version", "--format", "{{json .}}"]).await
    }
    pub async fn system_df(&self) -> DockerResult<DockerDiskUsage> {
        self.json(&["system", "df", "--format", "{{json .}}"]).await
    }
    pub async fn system_prune(
        &self,
        all: bool,
        volumes: bool,
        filters: &[&str],
    ) -> DockerResult<DockerOutput> {
        let mut args = vec!["system", "prune", "--force"];
        if all {
            args.push("--all");
        }
        if volumes {
            args.push("--volumes");
        }
        for filter in filters {
            args.extend(["--filter", filter]);
        }
        self.run(args).await
    }
    pub async fn login(
        &self,
        registry: Option<&str>,
        username: &str,
        password: &str,
    ) -> DockerResult<DockerOutput> {
        let mut args = vec!["login", "--username", username, "--password-stdin"];
        if let Some(registry) = registry {
            args.push(registry);
        }
        self.run_with_stdin(args, password).await
    }
    pub async fn logout(&self, registry: Option<&str>) -> DockerResult<DockerOutput> {
        let mut args = vec!["logout"];
        if let Some(registry) = registry {
            args.push(registry);
        }
        self.run(args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::docker::{ContainerSummary, DockerError};
    use std::os::unix::fs::PermissionsExt;

    fn fake_docker(body: &str) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("docker");
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        let mut permissions = std::fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).unwrap();
        dir
    }
    #[tokio::test]
    async fn container_rows_are_typed() {
        let dir = fake_docker("printf '{\"ID\":\"one\",\"State\":\"running\"}\\n'");
        let docker = DockerCli::with_executable(dir.path().join("docker"));
        let rows: Vec<ContainerSummary> = docker
            .containers_raw(false, &["status=running"])
            .await
            .unwrap();
        assert_eq!(rows[0].id, "one");
        assert_eq!(rows[0].state, "running");
    }
    #[tokio::test]
    async fn command_failure_is_typed() {
        let dir = fake_docker("echo unavailable >&2; exit 17");
        let error = DockerCli::with_executable(dir.path().join("docker"))
            .container_start(&["app"])
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            DockerError::CommandFailed { code: Some(17), .. }
        ));
    }

    #[tokio::test]
    async fn streams_stdout_and_stderr_without_utf8_assumptions() {
        let dir = fake_docker("printf 'out'; printf '\\377err' >&2");
        let docker = DockerCli::with_executable(dir.path().join("docker"));
        let (sender, mut receiver) = tokio::sync::mpsc::channel(4);
        let task = tokio::spawn(async move { docker.run_stream(["logs"], sender).await });
        let mut events = Vec::new();
        while let Some(event) = receiver.recv().await {
            events.push(event);
        }
        assert!(task.await.unwrap().unwrap().success());
        assert!(events.iter().any(|event| matches!(event, crate::utils::docker::DockerStreamEvent::Stdout(bytes) if bytes == b"out")));
        assert!(events.iter().any(|event| matches!(event, crate::utils::docker::DockerStreamEvent::Stderr(bytes) if bytes.ends_with(b"err"))));
    }

    #[tokio::test]
    #[ignore = "requires an explicitly configured SSH Docker test host"]
    async fn remote_docker_version_over_password_ssh() {
        let host = std::env::var("RUSTPLOY_TEST_SSH_HOST").expect("SSH host");
        let user = std::env::var("RUSTPLOY_TEST_SSH_USER").expect("SSH user");
        let password = std::env::var("RUSTPLOY_TEST_SSH_PASSWORD").expect("SSH password");
        let docker = DockerCli::new_remote(
            host,
            22,
            user,
            crate::utils::docker::SshAuth::password(password),
            crate::utils::docker::RemoteHostKey::InsecureAcceptAny,
        )
        .with_remote_sudo();
        let version = tokio::time::timeout(std::time::Duration::from_secs(10), docker.version())
            .await
            .unwrap()
            .unwrap();
        assert!(!version.client.version.is_empty() || !version.server.version.is_empty());
        let second = tokio::time::timeout(std::time::Duration::from_secs(10), docker.version())
            .await
            .unwrap()
            .unwrap();
        assert!(!second.client.version.is_empty() || !second.server.version.is_empty());
    }
}
