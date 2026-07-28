use crate::utils::{
    builder::{
        compose::spec::ComposeSpec,
        shared::{validate_domain_host, validate_name},
    },
    exec::{ExecError, ExecResult},
};

pub(super) fn validate_spec(spec: &ComposeSpec) -> ExecResult<()> {
    validate_name("compose", &spec.app_name)?;
    validate_name("stack", &spec.stack_name)?;

    if !spec.compose_file_path().ends_with(".yml") && !spec.compose_file_path().ends_with(".yaml") {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: "compose file must end with .yml or .yaml".into(),
        });
    }
    for domain in &spec.domains {
        validate_domain_host(&domain.host)?;
    }
    for service_network in &spec.service_networks {
        validate_name("compose service", &service_network.service_name)?;
        if service_network.networks.is_empty() {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: format!(
                    "compose service {} must attach to at least one network",
                    service_network.service_name
                ),
            });
        }
        for network in &service_network.networks {
            validate_name("network", network)?;
        }
    }
    Ok(())
}
