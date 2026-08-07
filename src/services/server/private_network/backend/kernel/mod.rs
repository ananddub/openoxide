mod backend;
mod rotation;
mod types;

pub(in crate::services::server::private_network) use backend::KernelWireGuardBackend;
pub(in crate::services::server::private_network) use types::{
    KernelWireGuardHealth, WireGuardInstallPlan,
};

#[cfg(test)]
mod tests;
