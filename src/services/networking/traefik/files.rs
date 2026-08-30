use std::path::{Path, PathBuf};

use crate::utils::exec::CommandExecutor;
use crate::utils::os::OsCli;

use super::service::TraefikService;
use super::types::{
    TraefikFileContentDto, TraefikFileNodeDto, TraefikFileTreeNodeDto, TraefikWriteFileDto,
};

impl TraefikService {
    pub(crate) fn get_traefik_base_path() -> String {
        std::env::var("TRAEFIK_BASE_PATH")
            .ok()
            .map(|v| v.trim().trim_end_matches('/').to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| format!("{}/traefik", crate::utils::paths::openoxide_paths().base))
    }

    pub(crate) async fn get_executor(&self, server_id: Option<i64>) -> Result<CommandExecutor, String> {
        crate::services::compute::compose::remote::agent_executor_for(server_id).await
    }

    pub fn sanitize_read_path(relative_path: &str) -> Result<PathBuf, String> {
        Self::sanitize_traefik_path(relative_path, true)
    }

    pub fn sanitize_write_path(relative_path: &str) -> Result<PathBuf, String> {
        Self::sanitize_traefik_path(relative_path, false)
    }

    fn sanitize_traefik_path(
        relative_path: &str,
        allow_etc_traefik: bool,
    ) -> Result<PathBuf, String> {
        let clean_path = relative_path.trim();
        if clean_path.is_empty() {
            return Err("Security Error: File path is required".to_string());
        }
        if clean_path.contains('\0') || clean_path.contains('\\') {
            return Err("Security Error: Invalid file path".to_string());
        }

        let base_dir = Self::get_traefik_base_path();
        let base_path = Path::new(&base_dir);
        let requested_path = Path::new(clean_path);
        let full_path = if requested_path.is_absolute() {
            requested_path.to_path_buf()
        } else {
            base_path.join(requested_path)
        };

        if full_path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err("Security Error: Path traversal detected in file path".to_string());
        }

        let inside_base = full_path.starts_with(base_path);
        let inside_etc_traefik =
            allow_etc_traefik && full_path.starts_with(Path::new("/etc/traefik"));
        if !inside_base && !inside_etc_traefik {
            return Err(format!(
                "Security Error: File path must remain inside {}",
                base_dir
            ));
        }

        let file_name = full_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let is_yml = full_path
            .extension()
            .map_or(false, |ext| ext == "yml" || ext == "yaml" || ext == "json");
        let is_acme = file_name == "acme.json";

        if !is_yml && !is_acme {
            return Err(
                "Security Restriction: Only .yml, .yaml, and .json files are accessible"
                    .to_string(),
            );
        }

        Ok(full_path)
    }

    pub async fn list_files(
        &self,
        server_id: Option<i64>,
    ) -> Result<Vec<TraefikFileNodeDto>, String> {
        let executor = self.get_executor(server_id).await?;
        let os = OsCli::new(&executor);
        let base_dir = Self::get_traefik_base_path();

        let output = os
            .dir(&base_dir)
            .walk()
            .also("/etc/traefik")
            .max_depth(3)
            .type_file()
            .names(["*.yml", "*.yaml", "*.json"])
            .output(crate::utils::os::dir::DirWalkOutput::PathSizeModifiedEpoch)
            .ignore_errors()
            .run()
            .await
            .map_err(|e| format!("Failed to list Traefik configuration files: {}", e))?;

        let stdout = output.stdout.as_str();
        let mut nodes = Vec::new();

        for line in stdout.lines() {
            let mut parts = line.splitn(3, '\t');
            let path_str = parts.next().unwrap_or_default().trim();
            if path_str.is_empty() {
                continue;
            }
            let size = parts
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);
            let modified_at = parts
                .next()
                .and_then(|value| value.parse::<f64>().ok())
                .map(|value| value.max(0.0) as u64)
                .unwrap_or(0);
            let path_obj = Path::new(path_str);

            let file_name = path_obj
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path_str.to_string());

            let is_valid_ext = file_name.ends_with(".yml")
                || file_name.ends_with(".yaml")
                || file_name.ends_with(".json");
            if !is_valid_ext {
                continue;
            }

            let relative = if path_str.starts_with(&base_dir) {
                path_str
                    .strip_prefix(&base_dir)
                    .unwrap_or(path_str)
                    .trim_start_matches('/')
                    .to_string()
            } else if path_str.starts_with("/etc/traefik") {
                path_str
                    .strip_prefix("/etc/traefik")
                    .unwrap_or(path_str)
                    .trim_start_matches('/')
                    .to_string()
            } else {
                file_name.clone()
            };

            let is_readonly = file_name == "acme.json";
            nodes.push(TraefikFileNodeDto {
                name: file_name,
                relative_path: relative,
                size,
                is_readonly,
                modified_at,
            });
        }

        nodes.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        nodes.dedup_by(|a, b| a.relative_path == b.relative_path);

        Ok(nodes)
    }

    pub async fn list_file_tree(
        &self,
        server_id: Option<i64>,
    ) -> Result<Vec<TraefikFileTreeNodeDto>, String> {
        let files = self.list_files(server_id).await?;
        Ok(build_file_tree(&files))
    }

    pub async fn read_file(
        &self,
        server_id: Option<i64>,
        relative_path: &str,
    ) -> Result<TraefikFileContentDto, String> {
        let full_path = Self::sanitize_read_path(relative_path)?;
        let path_str = full_path.to_string_lossy().to_string();
        let executor = self.get_executor(server_id).await?;
        let os = OsCli::new(&executor);

        let output = os
            .file(&path_str)
            .read()
            .execute()
            .await
            .map_err(|e| format!("Failed to read file content: {}", e))?;

        if !output.status.success() {
            let err_msg = output.stderr.clone();
            return Err(format!("Could not read file '{}': {}", path_str, err_msg));
        }

        let content = output.stdout.clone();
        let is_readonly = path_str.ends_with("acme.json");

        Ok(TraefikFileContentDto {
            path: relative_path.to_string(),
            content,
            is_readonly,
        })
    }

    pub async fn write_file(&self, payload: TraefikWriteFileDto) -> Result<(), String> {
        let full_path = Self::sanitize_write_path(&payload.path)?;
        let path_str = full_path.to_string_lossy().to_string();

        if path_str.ends_with("acme.json") {
            return Err(
                "Security Violation: acme.json is read-only and cannot be modified".to_string(),
            );
        }

        if path_str.ends_with(".yml") || path_str.ends_with(".yaml") {
            let _parsed: serde_yaml::Value = serde_yaml::from_str(&payload.content)
                .map_err(|e| format!("YAML Syntax Validation Error: {}", e))?;
        }

        let executor = self.get_executor(payload.server_id).await?;
        let os = OsCli::new(&executor);

        if let Some(parent) = full_path.parent() {
            let parent_str = parent.to_string_lossy().to_string();
            let _ = os.dir(&parent_str).create().run().await;
        }

        let backup_path = format!("{}.bak", path_str);
        let _ = os.file(&path_str).copy(&backup_path).execute().await;

        let tmp_path = format!("{}.tmp", path_str);
        let write_out = os
            .file(&tmp_path)
            .write(&payload.content)
            .execute()
            .await
            .map_err(|e| format!("Failed to write temporary file: {}", e))?;

        if !write_out.status.success() {
            let err = write_out.stderr.clone();
            return Err(format!("Failed to write content: {}", err));
        }

        let rename_out = os
            .file(&tmp_path)
            .move_to(&path_str)
            .execute()
            .await
            .map_err(|e| format!("Failed to apply file updates: {}", e))?;

        if !rename_out.status.success() {
            let err = rename_out.stderr.clone();
            return Err(format!("Failed to finalize file update: {}", err));
        }

        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
        let health = self.check_health(payload.server_id).await?;
        if !health.is_healthy {
            let rollback = os.file(&backup_path).copy(&path_str).execute().await;
            return Err(format!(
                "Traefik rejected the configuration; previous file restored: {}{}",
                health.configuration_errors.join("; "),
                rollback
                    .err()
                    .map(|error| format!("; rollback error: {error}"))
                    .unwrap_or_default()
            ));
        }

        Ok(())
    }

    pub async fn delete_directory(
        &self,
        server_id: Option<i64>,
        relative_path: &str,
    ) -> Result<(), String> {
        let full_path = Self::sanitize_write_path(relative_path)?;
        let executor = self.get_executor(server_id).await?;
        OsCli::new(&executor)
            .dir(full_path.to_string_lossy().as_ref())
            .delete()
            .run()
            .await
            .map(|_| ())
            .map_err(|error| format!("Failed to delete certificate directory: {error}"))
    }
}

fn build_file_tree(files: &[TraefikFileNodeDto]) -> Vec<TraefikFileTreeNodeDto> {
    let mut roots = Vec::new();

    for file in files {
        let parts = file
            .relative_path
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }
        insert_file_tree_node(&mut roots, &parts, file);
    }

    sort_file_tree(&mut roots);
    roots
}

fn insert_file_tree_node(
    siblings: &mut Vec<TraefikFileTreeNodeDto>,
    parts: &[&str],
    file: &TraefikFileNodeDto,
) {
    let name = parts[0];
    let is_file = parts.len() == 1;
    let relative_path = if is_file {
        file.relative_path.clone()
    } else {
        let depth = file.relative_path.split('/').count() - parts.len() + 1;
        file.relative_path
            .split('/')
            .take(depth)
            .collect::<Vec<_>>()
            .join("/")
    };

    let node_index = siblings
        .iter()
        .position(|node| node.name == name && node.relative_path == relative_path)
        .unwrap_or_else(|| {
            siblings.push(TraefikFileTreeNodeDto {
                name: name.to_string(),
                relative_path: relative_path.clone(),
                node_type: if is_file { "file" } else { "directory" }.to_string(),
                size: if is_file { file.size } else { 0 },
                is_readonly: is_file && file.is_readonly,
                modified_at: if is_file { file.modified_at } else { 0 },
                children: Vec::new(),
            });
            siblings.len() - 1
        });

    if is_file {
        let node = &mut siblings[node_index];
        node.node_type = "file".to_string();
        node.size = file.size;
        node.is_readonly = file.is_readonly;
        node.modified_at = file.modified_at;
    } else {
        insert_file_tree_node(&mut siblings[node_index].children, &parts[1..], file);
    }
}

fn sort_file_tree(nodes: &mut [TraefikFileTreeNodeDto]) {
    nodes.sort_by(|a, b| {
        match (
            a.node_type.as_str() == "directory",
            b.node_type.as_str() == "directory",
        ) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    for node in nodes {
        sort_file_tree(&mut node.children);
    }
}

#[cfg(test)]
mod tests {
    use super::{TraefikFileNodeDto, TraefikService, build_file_tree};
    use std::path::Path;

    #[test]
    fn sanitize_write_path_keeps_relative_paths_inside_openoxide_traefik_base() {
        let path = TraefikService::sanitize_write_path("dynamic/middlewares.yml").unwrap();
        assert!(path.starts_with(TraefikService::get_traefik_base_path()));
        assert!(path.ends_with(Path::new("dynamic/middlewares.yml")));
    }

    #[test]
    fn sanitize_path_rejects_traversal_and_backslashes() {
        assert!(TraefikService::sanitize_read_path("../secret.yml").is_err());
        assert!(TraefikService::sanitize_read_path("dynamic/../secret.yml").is_err());
        assert!(TraefikService::sanitize_read_path("dynamic\\secret.yml").is_err());
    }

    #[test]
    fn sanitize_path_allows_etc_traefik_reads_but_not_writes() {
        assert!(TraefikService::sanitize_read_path("/etc/traefik/traefik.yml").is_ok());
        assert!(TraefikService::sanitize_write_path("/etc/traefik/traefik.yml").is_err());
    }

    #[test]
    fn sanitize_path_restricts_file_types() {
        assert!(TraefikService::sanitize_read_path("dynamic/middlewares.yml").is_ok());
        assert!(TraefikService::sanitize_read_path("dynamic/acme.json").is_ok());
        assert!(TraefikService::sanitize_read_path("dynamic/access.log").is_err());
    }

    #[test]
    fn builds_tree_from_flat_file_nodes() {
        let tree = build_file_tree(&[
            TraefikFileNodeDto {
                name: "traefik.yml".into(),
                relative_path: "traefik.yml".into(),
                size: 620,
                is_readonly: false,
                modified_at: 10,
            },
            TraefikFileNodeDto {
                name: "middlewares.yml".into(),
                relative_path: "dynamic/middlewares.yml".into(),
                size: 112,
                is_readonly: false,
                modified_at: 11,
            },
            TraefikFileNodeDto {
                name: "acme.json".into(),
                relative_path: "dynamic/acme.json".into(),
                size: 2,
                is_readonly: true,
                modified_at: 12,
            },
        ]);

        assert_eq!(tree.len(), 2);
        assert_eq!(tree[0].name, "dynamic");
        assert_eq!(tree[0].node_type, "directory");
        assert_eq!(tree[0].children.len(), 2);
        assert_eq!(tree[0].children[0].name, "acme.json");
        assert!(tree[0].children[0].is_readonly);
        assert_eq!(tree[1].name, "traefik.yml");
        assert_eq!(tree[1].node_type, "file");
    }
}
