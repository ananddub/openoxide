use serde_yaml::{Mapping, Value};
use tokio_util::sync::CancellationToken;

use super::{compose::ComposeBuilder, spec::ComposeSpec};
use crate::utils::exec::{ExecError, ExecResult};

impl ComposeBuilder {
    pub(super) async fn transform_compose_file(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if !spec.randomize && !spec.isolated_deployment {
            return Ok(());
        }
        let path = spec.compose_file_path();
        let output = self
            .ctx
            .executor
            .run_cancelled("cat", [path.as_str()], cancel)
            .await?;
        let transformed = transform_compose(
            &output.stdout,
            spec.randomize.then_some(spec.suffix.as_str()),
            spec.isolated_deployment.then_some(spec.app_name.as_str()),
            spec.isolated_deployments_volume,
        )?;
        self.ctx
            .write_file_cancelled(&path, transformed.as_bytes(), cancel)
            .await
    }
}

pub fn transform_compose(
    source: &str,
    random_suffix: Option<&str>,
    isolated_network: Option<&str>,
    isolate_volumes: bool,
) -> ExecResult<String> {
    let mut document: Value = serde_yaml::from_str(source).map_err(yaml_error)?;
    let root = document
        .as_mapping_mut()
        .ok_or_else(|| command_error("compose root must be an object"))?;

    if let Some(suffix) = random_suffix {
        if suffix.trim().is_empty() {
            return Err(command_error(
                "randomized compose requires a non-empty suffix",
            ));
        }
        randomize_root(root, suffix);
    }
    if let Some(network) = isolated_network {
        if network.trim().is_empty() {
            return Err(command_error("isolated compose requires a network name"));
        }
        attach_isolated_network(root, network, isolate_volumes);
    }

    serde_yaml::to_string(&document).map_err(yaml_error)
}

pub fn remove_service(source: &str, service_name: &str) -> ExecResult<String> {
    let mut document: Value = serde_yaml::from_str(source).map_err(yaml_error)?;
    let root = document
        .as_mapping_mut()
        .ok_or_else(|| command_error("compose root must be an object"))?;
    let services = mapping_at_mut(root, "services")
        .ok_or_else(|| command_error("compose file has no services"))?;
    if services
        .remove(Value::String(service_name.into()))
        .is_none()
    {
        return Err(command_error(format!(
            "compose service {service_name} not found"
        )));
    }
    for service in services.values_mut().filter_map(Value::as_mapping_mut) {
        remove_service_reference(service, "depends_on", service_name);
        remove_service_reference(service, "links", service_name);
        remove_service_reference(service, "volumes_from", service_name);
        if let Some(extends) = service.get_mut(Value::String("extends".into())) {
            match extends {
                Value::String(value) if value == service_name => *extends = Value::Null,
                Value::Mapping(map)
                    if map
                        .get(Value::String("service".into()))
                        .and_then(Value::as_str)
                        == Some(service_name) =>
                {
                    *extends = Value::Null
                }
                _ => {}
            }
        }
    }
    serde_yaml::to_string(&document).map_err(yaml_error)
}

pub fn list_services(source: &str) -> ExecResult<Vec<String>> {
    let document: Value = serde_yaml::from_str(source).map_err(yaml_error)?;
    let services = document
        .as_mapping()
        .and_then(|root| root.get(Value::String("services".into())))
        .and_then(Value::as_mapping)
        .ok_or_else(|| command_error("compose file has no services"))?;
    Ok(services
        .keys()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect())
}

pub fn upsert_resource(
    source: &str,
    kind: &str,
    name: &str,
    file: Option<&str>,
    external: bool,
    services: &[String],
) -> ExecResult<String> {
    let field = resource_field(kind)?;
    let mut document: Value = serde_yaml::from_str(source).map_err(yaml_error)?;
    let root = document
        .as_mapping_mut()
        .ok_or_else(|| command_error("compose root must be an object"))?;
    let mut definition = Mapping::new();
    if let Some(file) = file {
        definition.insert(Value::String("file".into()), Value::String(file.into()));
    }
    if external {
        definition.insert(Value::String("external".into()), Value::Bool(true));
    }
    ensure_mapping(root, field).insert(Value::String(name.into()), Value::Mapping(definition));

    let compose_services = mapping_at_mut(root, "services")
        .ok_or_else(|| command_error("compose file has no services"))?;
    for service_name in services {
        let service = compose_services
            .get_mut(Value::String(service_name.clone()))
            .and_then(Value::as_mapping_mut)
            .ok_or_else(|| command_error(format!("compose service {service_name} not found")))?;
        let values = service
            .entry(Value::String(field.into()))
            .or_insert_with(|| Value::Sequence(Vec::new()))
            .as_sequence_mut()
            .ok_or_else(|| {
                command_error(format!("service {service_name} {field} must be a list"))
            })?;
        if !values.iter().any(|value| value.as_str() == Some(name)) {
            values.push(Value::String(name.into()));
        }
    }
    serde_yaml::to_string(&document).map_err(yaml_error)
}

pub fn remove_resource(source: &str, kind: &str, name: &str) -> ExecResult<String> {
    let field = resource_field(kind)?;
    let mut document: Value = serde_yaml::from_str(source).map_err(yaml_error)?;
    let root = document
        .as_mapping_mut()
        .ok_or_else(|| command_error("compose root must be an object"))?;
    if let Some(resources) = mapping_at_mut(root, field) {
        resources.remove(Value::String(name.into()));
    }
    if let Some(services) = mapping_at_mut(root, "services") {
        for service in services.values_mut().filter_map(Value::as_mapping_mut) {
            if let Some(Value::Sequence(values)) = service.get_mut(Value::String(field.into())) {
                values.retain(|value| match value {
                    Value::String(value) => value != name,
                    Value::Mapping(value) => {
                        value
                            .get(Value::String("source".into()))
                            .and_then(Value::as_str)
                            != Some(name)
                    }
                    _ => true,
                });
            }
        }
    }
    serde_yaml::to_string(&document).map_err(yaml_error)
}

fn resource_field(kind: &str) -> ExecResult<&'static str> {
    match kind.trim().to_ascii_uppercase().as_str() {
        "CONFIG" => Ok("configs"),
        "SECRET" => Ok("secrets"),
        _ => Err(command_error("resource kind must be CONFIG or SECRET")),
    }
}

fn randomize_root(root: &mut Mapping, suffix: &str) {
    if let Some(services) = mapping_at_mut(root, "services") {
        rename_services(services, suffix);
        for service in services.values_mut().filter_map(Value::as_mapping_mut) {
            suffix_service_refs(service, suffix);
            suffix_named_volumes(service, suffix);
            suffix_resource_refs(service, "networks", suffix, Some("openoxide-network"));
            suffix_resource_refs(service, "configs", suffix, None);
            suffix_resource_refs(service, "secrets", suffix, None);
        }
    }
    rename_root_mapping(root, "volumes", suffix, None);
    rename_root_mapping(root, "networks", suffix, Some("openoxide-network"));
    rename_root_mapping(root, "configs", suffix, None);
    rename_root_mapping(root, "secrets", suffix, None);
}

fn attach_isolated_network(root: &mut Mapping, network: &str, isolate_volumes: bool) {
    let networks = ensure_mapping(root, "networks");
    networks.insert(
        Value::String(network.into()),
        Value::Mapping(Mapping::from_iter([
            (Value::String("name".into()), Value::String(network.into())),
            (Value::String("external".into()), Value::Bool(true)),
        ])),
    );
    if let Some(services) = mapping_at_mut(root, "services") {
        for service in services.values_mut().filter_map(Value::as_mapping_mut) {
            attach_network(service, network);
            if isolate_volumes {
                suffix_named_volumes(service, network);
            }
        }
    }
    if isolate_volumes {
        rename_root_mapping(root, "volumes", network, None);
    }
}

fn rename_services(services: &mut Mapping, suffix: &str) {
    let original = std::mem::take(services);
    for (key, value) in original {
        let key = key
            .as_str()
            .map(|name| Value::String(format!("{name}-{suffix}")))
            .unwrap_or(key);
        services.insert(key, value);
    }
}

fn suffix_service_refs(service: &mut Mapping, suffix: &str) {
    suffix_reference_field(service, "depends_on", suffix);
    suffix_reference_field(service, "links", suffix);
    suffix_reference_field(service, "volumes_from", suffix);
    if let Some(Value::String(name)) = service.get_mut(Value::String("container_name".into())) {
        name.push('-');
        name.push_str(suffix);
    }
    if let Some(extends) = service.get_mut(Value::String("extends".into())) {
        match extends {
            Value::String(name) => suffix_reference(name, suffix),
            Value::Mapping(map) => {
                if let Some(Value::String(name)) = map.get_mut(Value::String("service".into())) {
                    suffix_reference(name, suffix);
                }
            }
            _ => {}
        }
    }
}

fn suffix_reference_field(service: &mut Mapping, field: &str, suffix: &str) {
    let Some(value) = service.get_mut(Value::String(field.into())) else {
        return;
    };
    match value {
        Value::Sequence(items) => {
            for item in items {
                if let Value::String(value) = item {
                    suffix_reference(value, suffix);
                }
            }
        }
        Value::Mapping(mapping) => rename_mapping(mapping, suffix, None),
        _ => {}
    }
}

fn suffix_reference(value: &mut String, suffix: &str) {
    let (name, remainder) = value.split_once(':').unwrap_or((value.as_str(), ""));
    *value = if remainder.is_empty() {
        format!("{name}-{suffix}")
    } else {
        format!("{name}-{suffix}:{remainder}")
    };
}

fn suffix_named_volumes(service: &mut Mapping, suffix: &str) {
    let Some(Value::Sequence(volumes)) = service.get_mut(Value::String("volumes".into())) else {
        return;
    };
    for volume in volumes {
        match volume {
            Value::String(value) => {
                let Some((source, remainder)) = value.split_once(':') else {
                    continue;
                };
                if source.starts_with('.') || source.starts_with('/') || source.starts_with('$') {
                    continue;
                }
                *value = format!("{source}-{suffix}:{remainder}");
            }
            Value::Mapping(map)
                if map
                    .get(Value::String("type".into()))
                    .and_then(Value::as_str)
                    == Some("volume") =>
            {
                if let Some(Value::String(source)) = map.get_mut(Value::String("source".into())) {
                    source.push('-');
                    source.push_str(suffix);
                }
            }
            _ => {}
        }
    }
}

fn suffix_resource_refs(service: &mut Mapping, field: &str, suffix: &str, preserve: Option<&str>) {
    let Some(value) = service.get_mut(Value::String(field.into())) else {
        return;
    };
    match value {
        Value::Sequence(items) => {
            for item in items {
                match item {
                    Value::String(name) if preserve != Some(name.as_str()) => {
                        name.push('-');
                        name.push_str(suffix);
                    }
                    Value::Mapping(map) => {
                        if let Some(Value::String(source)) =
                            map.get_mut(Value::String("source".into()))
                            && preserve != Some(source.as_str())
                        {
                            source.push('-');
                            source.push_str(suffix);
                        }
                    }
                    _ => {}
                }
            }
        }
        Value::Mapping(mapping) => rename_mapping(mapping, suffix, preserve),
        _ => {}
    }
}

fn rename_root_mapping(root: &mut Mapping, field: &str, suffix: &str, preserve: Option<&str>) {
    if let Some(mapping) = mapping_at_mut(root, field) {
        rename_mapping(mapping, suffix, preserve);
    }
}

fn rename_mapping(mapping: &mut Mapping, suffix: &str, preserve: Option<&str>) {
    let original = std::mem::take(mapping);
    for (key, value) in original {
        let key = key
            .as_str()
            .filter(|name| preserve != Some(*name))
            .map(|name| Value::String(format!("{name}-{suffix}")))
            .unwrap_or(key);
        mapping.insert(key, value);
    }
}

fn attach_network(service: &mut Mapping, network: &str) {
    let key = Value::String("networks".into());
    match service.get_mut(&key) {
        Some(Value::Sequence(items)) => {
            if !items.iter().any(|item| item.as_str() == Some(network)) {
                items.push(Value::String(network.into()));
            }
        }
        Some(Value::Mapping(mapping)) => {
            mapping
                .entry(Value::String(network.into()))
                .or_insert(Value::Mapping(Mapping::new()));
        }
        Some(_) => {}
        None => {
            service.insert(key, Value::Sequence(vec![Value::String(network.into())]));
        }
    }
}

fn remove_service_reference(service: &mut Mapping, field: &str, name: &str) {
    let Some(value) = service.get_mut(Value::String(field.into())) else {
        return;
    };
    match value {
        Value::Sequence(items) => items
            .retain(|item| item.as_str().and_then(|value| value.split(':').next()) != Some(name)),
        Value::Mapping(mapping) => {
            mapping.remove(Value::String(name.into()));
        }
        _ => {}
    }
}

fn mapping_at_mut<'a>(root: &'a mut Mapping, key: &str) -> Option<&'a mut Mapping> {
    root.get_mut(Value::String(key.into()))?.as_mapping_mut()
}

fn ensure_mapping<'a>(root: &'a mut Mapping, key: &str) -> &'a mut Mapping {
    root.entry(Value::String(key.into()))
        .or_insert_with(|| Value::Mapping(Mapping::new()))
        .as_mapping_mut()
        .expect("compose mapping field must be an object")
}

fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

fn yaml_error(error: serde_yaml::Error) -> ExecError {
    ExecError::Json(serde_json::Error::io(std::io::Error::other(error)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r#"
services:
  api:
    depends_on: [db]
    volumes: [data:/data]
    networks: [backend, openoxide-network]
    configs: [app-config]
    secrets: [db-password]
  db:
    volumes: [data:/var/lib/data]
volumes:
  data: {}
networks:
  backend: {}
  openoxide-network:
    external: true
configs:
  app-config: {file: ./app.conf}
secrets:
  db-password: {file: ./password}
"#;

    #[test]
    fn randomize_updates_roots_and_service_references() {
        let output = transform_compose(SOURCE, Some("abc123"), None, false).unwrap();
        assert!(output.contains("api-abc123:"));
        assert!(output.contains("db-abc123"));
        assert!(output.contains("data-abc123:/data"));
        assert!(output.contains("backend-abc123"));
        assert!(output.contains("openoxide-network"));
        assert!(output.contains("app-config-abc123"));
        assert!(output.contains("db-password-abc123"));
    }

    #[test]
    fn isolation_attaches_external_network_and_optionally_namespaces_volumes() {
        let output = transform_compose(SOURCE, None, Some("project-one"), true).unwrap();
        assert!(output.contains("name: project-one"));
        assert!(output.contains("external: true"));
        assert!(output.contains("data-project-one:/data"));
    }

    #[test]
    fn service_removal_cleans_dependency_references() {
        let output = remove_service(SOURCE, "db").unwrap();
        let value: Value = serde_yaml::from_str(&output).unwrap();
        let services = value["services"].as_mapping().unwrap();
        assert!(!services.contains_key(Value::String("db".into())));
        assert_eq!(
            value["services"]["api"]["depends_on"]
                .as_sequence()
                .unwrap()
                .len(),
            0
        );
    }
}
