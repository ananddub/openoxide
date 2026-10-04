use std::sync::Arc;

use auto_di::singleton;

use crate::services::compute::application::ApplicationService as AppApplicationService;
use crate::services::compute::application::import_export::ApplicationTransferService;

pub struct ApplicationController {
    pub(super) service: Arc<AppApplicationService>,
    pub(super) transfer: Arc<ApplicationTransferService>,
    pub(super) patches: Arc<crate::services::compute::application::patch::PatchService>,
    pub(super) middles:
        Arc<crate::services::compute::application::middleware::ApplicationMiddlewareService>,
    pub(super) mounts: Arc<crate::services::compute::application::mount::MountService>,
    pub(super) networks:
        Arc<crate::services::compute::application::network::ApplicationNetworkService>,
    pub(super) ports: Arc<crate::services::compute::application::port::PortService>,
    pub(super) redirects: Arc<crate::services::compute::application::redirect::RedirectService>,
    pub(super) securities: Arc<crate::services::compute::application::security::SecurityService>,
}

#[singleton]
impl ApplicationController {
    // `auto_di` resolves one parameter per dependency, so the arity is fixed by the
    // controller's fields and cannot be reduced by grouping arguments.
    #[allow(clippy::too_many_arguments)]
    fn new(
        service: Arc<AppApplicationService>,
        transfer: Arc<ApplicationTransferService>,
        patches: Arc<crate::services::compute::application::patch::PatchService>,
        middles: Arc<
            crate::services::compute::application::middleware::ApplicationMiddlewareService,
        >,
        mounts: Arc<crate::services::compute::application::mount::MountService>,
        networks: Arc<crate::services::compute::application::network::ApplicationNetworkService>,
        ports: Arc<crate::services::compute::application::port::PortService>,
        redirects: Arc<crate::services::compute::application::redirect::RedirectService>,
        securities: Arc<crate::services::compute::application::security::SecurityService>,
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
