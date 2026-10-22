use std::sync::Arc;

use auto_di::singleton;

use crate::services::networking::diagnostics::{
    CdnPurgeService, NetworkDiagnosticService, RootNetworkService,
};

pub struct NetworkingController {
    pub(super) diagnostics: Arc<NetworkDiagnosticService>,
    pub(super) root_network: Arc<RootNetworkService>,
    pub(super) cdn: Arc<CdnPurgeService>,
}

#[singleton]
impl NetworkingController {
    fn new(
        diagnostics: Arc<NetworkDiagnosticService>,
        root_network: Arc<RootNetworkService>,
        cdn: Arc<CdnPurgeService>,
    ) -> Self {
        Self {
            diagnostics,
            root_network,
            cdn,
        }
    }
}
