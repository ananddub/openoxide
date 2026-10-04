use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    SwarmNode as SwarmNodeProto, SwarmNodeAvailability as ProtoSwarmNodeAvailability,
    SwarmNodeRole as ProtoSwarmNodeRole, SwarmRole as ProtoSwarmRole,
};
use crate::utils::docker::NodeInspect;
use crate::utils::docker::core::types::{NodeRole, SwarmRole};
use crate::utils::exec::ExecError;

pub fn map_exec(error: ExecError) -> ConnectError {
    tracing::error!(error = %error, "swarm command failed");
    match error {
        ExecError::CommandFailed { .. } => {
            ConnectError::new(ErrorCode::FailedPrecondition, error.to_string())
        }
        ExecError::Ssh(_) => ConnectError::new(ErrorCode::FailedPrecondition, error.to_string()),
        _ => ConnectError::new(ErrorCode::Internal, error.to_string()),
    }
}

fn node_role_to_proto(role: NodeRole) -> ProtoSwarmNodeRole {
    match role {
        NodeRole::Worker => ProtoSwarmNodeRole::Worker,
        NodeRole::Manager => ProtoSwarmNodeRole::Manager,
    }
}

pub fn proto_role_to_docker(
    value: ::buffa::EnumValue<ProtoSwarmRole>,
) -> Result<SwarmRole, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoSwarmRole::Worker) => Ok(SwarmRole::Worker),
        ::buffa::EnumValue::Known(ProtoSwarmRole::Manager) => Ok(SwarmRole::Manager),
        _ => Err(ConnectError::invalid_argument("invalid swarm role")),
    }
}

pub fn availability_to_docker_str(
    value: ::buffa::EnumValue<ProtoSwarmNodeAvailability>,
) -> Result<&'static str, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoSwarmNodeAvailability::Active) => Ok("active"),
        ::buffa::EnumValue::Known(ProtoSwarmNodeAvailability::Pause) => Ok("pause"),
        ::buffa::EnumValue::Known(ProtoSwarmNodeAvailability::Drain) => Ok("drain"),
        _ => Err(ConnectError::invalid_argument("invalid availability value")),
    }
}

pub fn node_to_proto(n: NodeInspect) -> SwarmNodeProto {
    let (is_leader, reachability) = n
        .manager_status
        .map(|m| (m.leader, m.reachability))
        .unwrap_or_default();
    SwarmNodeProto {
        id: n.id,
        hostname: n.description.hostname,
        status: n.status.state,
        availability: n.spec.availability,
        role: ::buffa::EnumValue::Known(node_role_to_proto(n.spec.role)),
        is_leader,
        reachability,
        ip_address: n.status.addr,
        engine_version: n.description.engine.engine_version,
    }
}
