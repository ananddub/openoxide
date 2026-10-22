pub fn compose_generation(user_request: &str) -> String {
    format!(
        r#"You are an expert DevOps engineer. Return only one JSON object with a `suggestions` array containing 1 to 3 production-ready Docker Compose deployment options.

Each suggestion must contain exactly these snake_case fields:
- id, name, short_description, description, docker_compose
- env_variables: array of {{"name":"...","value":"..."}}
- domains: array of {{"host":"...","port":3000,"service_name":"..."}}
- config_files: array of {{"file_path":"...","content":"..."}}

Rules:
- Use public `image:` references with explicit tags; never use `build:`, `container_name`, or top-level `version`.
- Use `${{VARIABLE-default}}` in Compose and include every referenced variable in env_variables with an actual value.
- Do not publish host ports. A short syntax port may only contain the container port (for example `"3000"`).
- Include every dependency such as PostgreSQL or Redis as a service.
- Generate config files only when required. File paths must be safe relative paths without `..`.
- Every domain service_name must exist in Compose and port must match the internal service port.
- Secrets must be strong example values, not words such as changeme or password.

User request: {user_request}"#
    )
}

pub fn log_analysis(context: &str, logs: &str) -> String {
    format!(
        r#"Analyze these {context} logs as a senior DevOps engineer. Return concise plain text with: Summary, Issues Found, Root Cause, Suggested Fix. If healthy, clearly say so. Never invent events absent from the logs.

Logs:
{logs}"#
    )
}
