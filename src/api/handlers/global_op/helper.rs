use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    GlobalContainer as ProtoGlobalContainer, GlobalContainerAction as ProtoGlobalContainerAction,
    GlobalDockerOutput as ProtoGlobalDockerOutput, GlobalImage as ProtoGlobalImage,
    GlobalNetwork as ProtoGlobalNetwork, GlobalVolume as ProtoGlobalVolume,
};
use crate::services::platform::global_ops::ContainerActionDto;
use crate::utils::docker::{ContainerSummary, ImageSummary, NetworkSummary, VolumeSummary};

pub fn api_error(error: String) -> ConnectError {
    if error.starts_with("invalid") {
        ConnectError::invalid_argument(error)
    } else {
        ConnectError::new(ErrorCode::FailedPrecondition, error)
    }
}

/// Raw docker CLI JSON documents travel to the wire as serialized strings.
pub fn json_data(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// Lifecycle action accepted by `container_action`; unspecified/unknown codes
/// are rejected rather than mapped onto a default action.
pub fn to_container_action(
    value: ::buffa::EnumValue<ProtoGlobalContainerAction>,
) -> Result<ContainerActionDto, ConnectError> {
    Ok(match value {
        ::buffa::EnumValue::Known(ProtoGlobalContainerAction::Start) => ContainerActionDto::Start,
        ::buffa::EnumValue::Known(ProtoGlobalContainerAction::Stop) => ContainerActionDto::Stop,
        ::buffa::EnumValue::Known(ProtoGlobalContainerAction::Restart) => {
            ContainerActionDto::Restart
        }
        ::buffa::EnumValue::Known(ProtoGlobalContainerAction::Kill) => ContainerActionDto::Kill,
        ::buffa::EnumValue::Known(ProtoGlobalContainerAction::Pause) => ContainerActionDto::Pause,
        ::buffa::EnumValue::Known(ProtoGlobalContainerAction::Unpause) => {
            ContainerActionDto::Unpause
        }
        _ => {
            return Err(ConnectError::invalid_argument(
                "action must be a defined container lifecycle action",
            ))
        }
    })
}

// `docker ps`/`images`/`network ls`/`volume ls` projections: every column is
// already the CLI-formatted string on both sides of the mapping.

pub fn container_to_proto(value: ContainerSummary) -> ProtoGlobalContainer {
    ProtoGlobalContainer {
        id: value.id,
        names: value.names,
        image: value.image,
        command: value.command,
        created_at: value.created_at,
        running_for: value.running_for,
        ports: value.ports,
        status: value.status,
        state: value.state,
        size: value.size,
        labels: value.labels,
        mounts: value.mounts,
        networks: value.networks,
        ..Default::default()
    }
}

pub fn image_to_proto(value: ImageSummary) -> ProtoGlobalImage {
    ProtoGlobalImage {
        id: value.id,
        repository: value.repository,
        tag: value.tag,
        digest: value.digest,
        created_at: value.created_at,
        created_since: value.created_since,
        size: value.size,
        shared_size: value.shared_size,
        unique_size: value.unique_size,
        containers: value.containers,
        ..Default::default()
    }
}

pub fn network_to_proto(value: NetworkSummary) -> ProtoGlobalNetwork {
    ProtoGlobalNetwork {
        id: value.id,
        name: value.name,
        driver: value.driver,
        scope: value.scope,
        ipv6: value.ipv6,
        internal: value.internal,
        labels: value.labels,
        created_at: value.created_at,
        ..Default::default()
    }
}

pub fn volume_to_proto(value: VolumeSummary) -> ProtoGlobalVolume {
    ProtoGlobalVolume {
        name: value.name,
        driver: value.driver,
        scope: value.scope,
        mountpoint: value.mountpoint,
        labels: value.labels,
        availability: value.availability,
        group: value.group,
        links: value.links,
        size: value.size,
        ..Default::default()
    }
}

pub fn output_to_proto(value: crate::services::platform::global_ops::DockerActionResponseDto) -> ProtoGlobalDockerOutput {
    ProtoGlobalDockerOutput {
        stdout: value.stdout,
        stderr: value.stderr,
        ..Default::default()
    }
}
