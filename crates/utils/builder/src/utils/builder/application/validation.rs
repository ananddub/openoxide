use crate::utils::{
    builder::{
        shared::{validate_domain_host, validate_name},
        spec::ApplicationSpec,
    },
    exec::{ExecError, ExecResult},
};

pub(super) fn validate_spec(spec: &ApplicationSpec) -> ExecResult<()> {
    validate_name("application", &spec.app_name)?;
    validate_name("stack", &spec.stack_name)?;
    if spec.networks.is_empty() {
        return Err(ExecError::CommandFailed {
            code: None,
            stderr: "application must attach to at least one network".into(),
        });
    }
    for network in &spec.networks {
        validate_name("network", network)?;
    }

    for domain in &spec.domains {
        validate_domain_host(&domain.host)?;
        if !domain.path.starts_with('/') || !domain.internal_path.starts_with('/') {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "domain paths must start with /".into(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::builder::application::stack::tests::spec;

    #[test]
    fn unsafe_app_name_is_rejected() {
        let mut value = spec();
        value.app_name = "../../root".into();
        assert!(validate_spec(&value).is_err());
    }
}
