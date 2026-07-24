use crate::db::models::{domains::Domain, mounts::Mount};
use crate::utils::builder::errors::AdapterError;
use crate::utils::builder::spec::{DomainSpec, MountKind, MountSpec, MountType};

pub fn domain(d: Domain) -> Result<DomainSpec, AdapterError> {
    Ok(DomainSpec {
        key: d.id.unwrap_or_default().to_string(),
        host: d.host,
        https: d.https != 0,
        port: u16::try_from(d.port.unwrap_or(3000)).map_err(|_| AdapterError::InvalidField {
            field: "port",
            message: "invalid domain port".into(),
        })?,
        service_name: d.service_name,
        path: d.path.unwrap_or_else(|| "/".into()),
        internal_path: d.internal_path.unwrap_or_else(|| "/".into()),
        strip_path: d.strip_path != 0,
        entrypoint: d.custom_entrypoint,
        certificate_type: d.certificate_type,
        custom_cert_resolver: d.custom_cert_resolver,
        middlewares: serde_json::from_str(&d.middlewares).unwrap_or_default(),
    })
}

pub fn mount_spec(m: Mount, base_path: &str) -> Result<MountSpec, AdapterError> {
    let mount_type = MountType::from(m.mount_type.as_str());
    let is_file = mount_type == MountType::File;
    let kind = match mount_type {
        MountType::Volume => MountKind::Volume,
        MountType::Bind => MountKind::Bind,
        MountType::File => MountKind::File,
    };
    let source = match kind {
        MountKind::Volume => m
            .volume_name
            .ok_or(AdapterError::MissingField("volume_name"))?,
        MountKind::Bind => m.host_path.ok_or(AdapterError::MissingField("host_path"))?,
        MountKind::File => format!("{}/{}", base_path, m.id.unwrap_or_default()),
    };
    Ok(MountSpec {
        kind,
        source,
        target: m.mount_path,
        read_only: is_file,
        content: m.content,
    })
}
