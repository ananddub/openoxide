use super::builder::ComposeBuilder;
use crate::utils::builder::shared::mounts::prepare_file_mounts;
use crate::utils::builder::spec::{MountKind, MountSpec};
use crate::utils::{builder::compose::spec::ComposeSpec, exec::ExecResult};
use serde_yaml::{Mapping, Value};
use tokio_util::sync::CancellationToken;

impl ComposeBuilder {
    pub(super) async fn prepare_runtime_files(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if let Some(parent) = std::path::Path::new(&spec.env_file).parent() {
            self.ctx
                .executor
                .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                .await?;
        }
        self.ctx
            .write_file_cancelled(&spec.env_file, env_file(spec).as_bytes(), cancel)
            .await?;
        prepare_file_mounts(&self.ctx, &spec.mounts, cancel).await?;
        self.inject_bind_volume_mounts(spec, cancel).await?;
        Ok(())
    }

    /// Adds BIND/VOLUME mounts to the compose file. Docker never infers these
    /// from the mounts table, so they are injected into the service's
    /// `volumes:` list (and named volumes declared at the top level). When the
    /// file has more than one service the target service is ambiguous, so the
    /// injection is skipped with a warning rather than guessed.
    async fn inject_bind_volume_mounts(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let entries: Vec<&MountSpec> = spec
            .mounts
            .iter()
            .filter(|m| matches!(m.kind, MountKind::Bind | MountKind::Volume))
            .collect();
        if entries.is_empty() {
            return Ok(());
        }

        let path = spec.compose_file_path();
        let output = self
            .ctx
            .executor
            .run_cancelled("cat", [path.as_str()], cancel)
            .await?;
        let Ok(mut document) = serde_yaml::from_str::<Value>(&output.stdout) else {
            return Ok(());
        };
        let Some(root) = document.as_mapping_mut() else {
            return Ok(());
        };
        let Some(services) = root
            .get_mut(Value::String("services".into()))
            .and_then(Value::as_mapping_mut)
        else {
            return Ok(());
        };
        if services.len() != 1 {
            self.ctx
                .emit(crate::utils::builder::spec::BuilderEvent::Message(format!(
                    "skipping {} bind/volume mount(s): compose has {} services, target is ambiguous",
                    entries.len(),
                    services.len()
                )))
                .await;
            return Ok(());
        }

        let Some(service) = services.values_mut().next().and_then(Value::as_mapping_mut) else {
            return Ok(());
        };
        let volumes = service
            .entry(Value::String("volumes".into()))
            .or_insert_with(|| Value::Sequence(Vec::new()));
        let Some(list) = volumes.as_sequence_mut() else {
            return Ok(());
        };
        let mut named_volumes: Vec<String> = Vec::new();
        for mount in &entries {
            let line = format!("{}:{}", mount.source, mount.target);
            if !list.iter().any(|v| v.as_str() == Some(line.as_str())) {
                list.push(Value::String(line));
            }
            if matches!(mount.kind, MountKind::Volume) {
                named_volumes.push(mount.source.clone());
            }
        }

        // Declare named volumes at the top level so compose creates them.
        if !named_volumes.is_empty() {
            let root = document.as_mapping_mut().unwrap();
            let top = root
                .entry(Value::String("volumes".into()))
                .or_insert_with(|| Value::Mapping(Mapping::new()));
            if let Some(top) = top.as_mapping_mut() {
                for name in named_volumes {
                    top.entry(Value::String(name))
                        .or_insert(Value::Mapping(Mapping::new()));
                }
            }
        }

        let rendered = serde_yaml::to_string(&document).map_err(|error| {
            crate::utils::exec::ExecError::CommandFailed {
                code: None,
                stderr: format!("failed to serialize compose with mounts: {error}"),
            }
        })?;
        self.ctx
            .write_file_cancelled(&path, rendered.as_bytes(), cancel)
            .await?;
        Ok(())
    }
}

fn env_file(spec: &ComposeSpec) -> String {
    spec.environment
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect()
}
