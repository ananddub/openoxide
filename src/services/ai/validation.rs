use serde_yaml::{Mapping, Value};
use std::collections::HashSet;

use super::types::{AiComposeSuggestion, AiGenerationOutput};

pub fn validate_output(output: &AiGenerationOutput) -> Result<(), String> {
    if output.suggestions.is_empty() || output.suggestions.len() > 3 {
        return Err("AI output must contain between one and three suggestions".into());
    }

    let mut ids = HashSet::new();
    for suggestion in &output.suggestions {
        if suggestion.id.trim().is_empty() || !ids.insert(suggestion.id.to_ascii_lowercase()) {
            return Err("suggestion IDs must be non-empty and unique".into());
        }
        validate_suggestion(suggestion)?;
    }
    Ok(())
}

pub fn validate_suggestion(suggestion: &AiComposeSuggestion) -> Result<(), String> {
    if suggestion.name.trim().is_empty() || suggestion.description.trim().is_empty() {
        return Err("suggestion name and description are required".into());
    }

    let root: Value = serde_yaml::from_str(&suggestion.docker_compose)
        .map_err(|error| format!("invalid Docker Compose YAML: {error}"))?;
    let root = root
        .as_mapping()
        .ok_or_else(|| "Docker Compose root must be a mapping".to_string())?;
    if root.contains_key(Value::String("version".into())) {
        return Err("Docker Compose must not contain a top-level version".into());
    }
    let services = mapping(root, "services")?;
    if services.is_empty() {
        return Err("Docker Compose must define at least one service".into());
    }

    for (name, service) in services {
        let name = name
            .as_str()
            .ok_or_else(|| "Compose service names must be strings".to_string())?;
        let service = service
            .as_mapping()
            .ok_or_else(|| format!("Compose service `{name}` must be a mapping"))?;
        if !service.contains_key(Value::String("image".into())) {
            return Err(format!("Compose service `{name}` must define image"));
        }
        if service.contains_key(Value::String("build".into())) {
            return Err(format!("Compose service `{name}` must not use build"));
        }
        if service.contains_key(Value::String("container_name".into())) {
            return Err(format!(
                "Compose service `{name}` must not set container_name"
            ));
        }
        validate_ports(name, service)?;
    }

    let env_names = suggestion
        .env_variables
        .iter()
        .map(|variable| variable.name.as_str())
        .collect::<HashSet<_>>();
    if env_names.len() != suggestion.env_variables.len() {
        return Err("environment variable names must be unique".into());
    }
    for variable in &suggestion.env_variables {
        if variable.name.trim().is_empty()
            || !variable
                .name
                .chars()
                .all(|c| c == '_' || c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            return Err(format!(
                "invalid environment variable name `{}`",
                variable.name
            ));
        }
        if variable.value.trim().is_empty() {
            return Err(format!(
                "environment variable `{}` has an empty value",
                variable.name
            ));
        }
    }
    for referenced in referenced_environment_variables(&suggestion.docker_compose) {
        if !env_names.contains(referenced.as_str()) {
            return Err(format!(
                "Compose references `{referenced}` but env_variables does not define it"
            ));
        }
    }

    for domain in &suggestion.domains {
        if domain.host.trim().is_empty()
            || !(1..=65_535).contains(&domain.port)
            || !services.contains_key(Value::String(domain.service_name.clone()))
        {
            return Err(format!("invalid domain definition for `{}`", domain.host));
        }
    }
    for file in &suggestion.config_files {
        let path = file.file_path.trim();
        if path.is_empty()
            || path.starts_with('/')
            || path.split('/').any(|component| component == "..")
            || file.content.is_empty()
        {
            return Err(format!("unsafe or empty generated file path `{path}`"));
        }
    }
    Ok(())
}

fn mapping<'a>(root: &'a Mapping, key: &str) -> Result<&'a Mapping, String> {
    root.get(Value::String(key.into()))
        .and_then(Value::as_mapping)
        .ok_or_else(|| format!("Docker Compose `{key}` must be a mapping"))
}

fn validate_ports(name: &str, service: &Mapping) -> Result<(), String> {
    let Some(ports) = service.get(Value::String("ports".into())) else {
        return Ok(());
    };
    let ports = ports
        .as_sequence()
        .ok_or_else(|| format!("Compose service `{name}` ports must be an array"))?;
    for port in ports {
        if let Some(value) = port.as_str()
            && value.matches(':').count() > 0
        {
            return Err(format!(
                "Compose service `{name}` must not publish a host port `{value}`"
            ));
        }
        if port
            .as_mapping()
            .is_some_and(|mapping| mapping.contains_key(Value::String("published".into())))
        {
            return Err(format!(
                "Compose service `{name}` must not define a published host port"
            ));
        }
    }
    Ok(())
}

fn referenced_environment_variables(input: &str) -> HashSet<String> {
    let bytes = input.as_bytes();
    let mut found = HashSet::new();
    let mut index = 0;
    while index + 2 < bytes.len() {
        if bytes[index] == b'$' && bytes[index + 1] == b'{' {
            let start = index + 2;
            let mut end = start;
            while end < bytes.len()
                && (bytes[end] == b'_'
                    || bytes[end].is_ascii_uppercase()
                    || bytes[end].is_ascii_digit())
            {
                end += 1;
            }
            if end > start {
                found.insert(input[start..end].to_string());
            }
            index = end;
        } else {
            index += 1;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::ai::types::{AiEnvironmentVariable, AiGenerationOutput};

    #[test]
    fn validates_safe_compose_output() {
        let output = AiGenerationOutput {
            suggestions: vec![AiComposeSuggestion {
                id: "web".into(),
                name: "Web".into(),
                short_description: "web".into(),
                description: "web service".into(),
                docker_compose: "services:\n  web:\n    image: nginx:1.27-alpine\n    ports: [\"80\"]\n    environment:\n      TOKEN: ${TOKEN-value}\n".into(),
                env_variables: vec![AiEnvironmentVariable { name: "TOKEN".into(), value: "safe-value".into() }],
                domains: vec![],
                config_files: vec![],
            }],
        };
        assert!(validate_output(&output).is_ok());
    }

    #[test]
    fn rejects_build_and_host_port_publication() {
        let suggestion = AiComposeSuggestion {
            id: "unsafe".into(),
            name: "Unsafe".into(),
            short_description: "unsafe".into(),
            description: "unsafe service".into(),
            docker_compose:
                "services:\n  web:\n    build: .\n    image: web:1\n    ports: [\"8080:80\"]\n"
                    .into(),
            env_variables: vec![],
            domains: vec![],
            config_files: vec![],
        };
        assert!(validate_suggestion(&suggestion).is_err());
    }

    #[test]
    fn rejects_undefined_environment_variable() {
        let suggestion = AiComposeSuggestion {
            id: "missing-env".into(),
            name: "Missing env".into(),
            short_description: "missing env".into(),
            description: "missing env service".into(),
            docker_compose: "services:\n  web:\n    image: nginx:1.27-alpine\n    environment:\n      TOKEN: ${TOKEN}\n".into(),
            env_variables: vec![],
            domains: vec![],
            config_files: vec![],
        };
        assert!(validate_suggestion(&suggestion).is_err());
    }
}
