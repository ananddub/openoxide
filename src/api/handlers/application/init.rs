use std::sync::Arc;

use auto_di::singleton;

use crate::services::application::import_export::ApplicationTransferService;
use crate::services::application::ApplicationService as AppApplicationService;

pub struct ApplicationController {
    pub(super) service: Arc<AppApplicationService>,
    pub(super) transfer: Arc<ApplicationTransferService>,
    pub(super) patches: Arc<crate::services::application::patch::PatchService>,
    pub(super) middles: Arc<crate::services::application::middleware::ApplicationMiddlewareService>,
    pub(super) mounts: Arc<crate::services::application::mount::MountService>,
    pub(super) networks: Arc<crate::services::application::network::ApplicationNetworkService>,
    pub(super) ports: Arc<crate::services::application::port::PortService>,
    pub(super) redirects: Arc<crate::services::application::redirect::RedirectService>,
    pub(super) securities: Arc<crate::services::application::security::SecurityService>,
}

#[singleton]
impl ApplicationController {
    fn new(
        service: Arc<AppApplicationService>,
        transfer: Arc<ApplicationTransferService>,
        patches: Arc<crate::services::application::patch::PatchService>,
        middles: Arc<crate::services::application::middleware::ApplicationMiddlewareService>,
        mounts: Arc<crate::services::application::mount::MountService>,
        networks: Arc<crate::services::application::network::ApplicationNetworkService>,
        ports: Arc<crate::services::application::port::PortService>,
        redirects: Arc<crate::services::application::redirect::RedirectService>,
        securities: Arc<crate::services::application::security::SecurityService>,
    ) -> Self {
        Self {
            service,
            transfer,
            patches,
            middles,
            mounts,
            networks,
            ports,
            redirects,
            securities,
        }
    }
}
