use std::sync::Arc;

use auto_di::singleton;
use connectrpc::ConnectError;

use crate::db::queries::FlashQueries;
use crate::services::server::{
    RemoteServerService, RemoteServerService as AppRemoteServerService, ServerCleanupService,
    ServerLifecycleService, ServerManagementService as AppServerManagementService,
    ServerPrivateNetworkService as AppServerPrivateNetworkService,
};
use crate::utils::exec::{RemoteExecutor, SshAuth};

use super::helper::{host_key_policy, map_db_error};

pub struct ServerController {
    pub(super) service: Arc<RemoteServerService>,
    pub(super) queries: Arc<FlashQueries>,
}

#[singleton]
impl ServerController {
    fn new(service: Arc<RemoteServerService>, queries: Arc<FlashQueries>) -> Self {
        Self { service, queries }
    }
}

pub struct ServerManagementController {
    pub(super) management: Arc<AppServerManagementService>,
    pub(super) cleanup: Arc<ServerCleanupService>,
    pub(super) lifecycle: Arc<ServerLifecycleService>,
}

#[singleton]
impl ServerManagementController {
    fn new(
        management: Arc<AppServerManagementService>,
        cleanup: Arc<ServerCleanupService>,
        lifecycle: Arc<ServerLifecycleService>,
    ) -> Self {
        Self {
            management,
            cleanup,
            lifecycle,
        }
    }
}

pub struct ServerPrivateNetworkController {
    pub(super) service: Arc<AppServerPrivateNetworkService>,
}

#[singleton]
impl ServerPrivateNetworkController {
    fn new(service: Arc<AppServerPrivateNetworkService>) -> Self {
        Self { service }
    }
}

pub struct RemoteServerController {
    pub(super) service: Arc<AppRemoteServerService>,
}

#[singleton]
impl RemoteServerController {
    fn new(service: Arc<AppRemoteServerService>) -> Self {
        Self { service }
    }
}

// Controller extensions — non-gRPC helper methods live here, not in controller.rs.
impl ServerController {
    pub(super) async fn executor(
        &self,
        id: i64,
        host_key: &Option<String>,
        sudo_password: Option<&str>,
        _pool_size: Option<usize>,
        require_sudo: bool,
    ) -> Result<RemoteExecutor, ConnectError> {
        let (server, key) = self
            .service
            .connection_details(id)
            .await
            .map_err(map_db_error)?;
        if key.private_key.trim().is_empty() {
            return Err(ConnectError::invalid_argument(
                "selected SSH key has no private key",
            ));
        }
        let port = u16::try_from(server.port).map_err(|_| {
            ConnectError::invalid_argument("SSH port must be between 0 and 65535")
        })?;
        let host_policy = host_key_policy(id, host_key);
        let auth = SshAuth::key_pair(key.private_key, key.public_key);
        let is_root = server.username == "root";
        let mut executor =
            RemoteExecutor::new(server.ip_address, port, server.username, auth, host_policy);
        executor = if is_root {
            executor
        } else if let Some(password) = sudo_password {
            executor.with_sudo_password(password)
        } else if require_sudo {
            executor.with_sudo()
        } else {
            executor
        };
        Ok(executor)
    }
}
