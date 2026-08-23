pub(crate) mod build;
pub(crate) mod exec;
mod mapping;
pub(crate) mod mutations;
mod transport;

pub use mapping::{ResponseShape, encode_query_public, map_cli_args};
pub use transport::DockerApi;

pub const API_VERSION: &str = "v1.52";

/// Returns whether a CLI command has a concrete Engine API v1.52 mapping.
///
/// This is intentionally kept beside the dispatcher so callers can reject
/// unsupported `.api()` requests before dispatching to the local Engine API.
pub fn supports_command(args: &[String]) -> bool {
    let is_exec = args.first().map(String::as_str) == Some("container")
        && args.get(1).map(String::as_str) == Some("exec");
    let is_build = args.first().map(String::as_str) == Some("image")
        && args.get(1).map(String::as_str) == Some("build");
    is_exec || is_build || mutations::handles(args) || mapping::map_cli_args(args).is_some()
}

#[cfg(test)]
mod tests {
    use super::{API_VERSION, ResponseShape, map_cli_args};

    #[test]
    fn uses_engine_api_v152() {
        assert_eq!(API_VERSION, "v1.52");
    }

    #[test]
    fn maps_lifecycle_and_resources() {
        let restart = ["container".into(), "restart".into(), "web".into()];
        let mapped = map_cli_args(&restart).expect("restart mapping");
        assert_eq!(mapped.method, "POST");
        assert_eq!(mapped.path, "/containers/web/restart");
        assert_eq!(mapped.response_shape, ResponseShape::Raw);
        let secrets = ["secret".into(), "ls".into()];
        let mapped = map_cli_args(&secrets).expect("secret mapping");
        assert_eq!(mapped.method, "GET");
        assert_eq!(mapped.path, "/secrets");
        assert_eq!(mapped.response_shape, ResponseShape::Raw);
    }

    #[test]
    fn unsupported_operations_are_not_guessed() {
        let compose = ["compose".into(), "up".into(), "web".into()];
        assert!(map_cli_args(&compose).is_none());
        assert!(!super::supports_command(&compose));
    }

    #[test]
    fn container_run_maps_create_and_start() {
        let args = [
            "container".into(),
            "run".into(),
            "--name".into(),
            "web api".into(),
            "--memory".into(),
            "512m".into(),
            "nginx".into(),
        ];
        let request = map_cli_args(&args).expect("run mapping");
        assert!(request.start_created);
        assert_eq!(request.path, "/containers/create?name=web%20api");
        assert_eq!(
            request.body.as_ref().unwrap()["HostConfig"]["Memory"],
            512 * 1024 * 1024
        );
    }

    #[test]
    fn maps_resource_create_endpoints() {
        let volume = [
            "volume".into(),
            "create".into(),
            "--driver".into(),
            "local".into(),
            "cache".into(),
        ];
        assert_eq!(map_cli_args(&volume).unwrap().path, "/volumes/create");
        let network = [
            "network".into(),
            "create".into(),
            "--internal".into(),
            "private".into(),
        ];
        assert_eq!(map_cli_args(&network).unwrap().path, "/networks/create");
    }

    #[test]
    fn maps_stream_and_mutation_endpoints() {
        let logs = [
            "container".into(),
            "logs".into(),
            "web".into(),
            "--follow".into(),
        ];
        assert!(
            map_cli_args(&logs)
                .unwrap()
                .path
                .contains("/containers/web/logs")
        );
        let auth = [
            "login".into(),
            "--username".into(),
            "u".into(),
            "registry.local".into(),
        ];
        assert_eq!(map_cli_args(&auth).unwrap().path, "/auth");
    }
}
