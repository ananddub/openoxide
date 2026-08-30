use std::sync::Arc;

use auto_di::singleton;

use crate::services::compose::management::{
    ComposeManagementService, ComposeMountService, ComposePatchService, ComposeTransferService,
};
use crate::services::compose::ComposeService as AppComposeService;

pub struct ComposeController {
    pub(super) service: Arc<AppComposeService>,
    pub(super) management: Arc<ComposeManagementService>,
    pub(super) transfer: Arc<ComposeTransferService>,
    pub(super) mounts: Arc<ComposeMountService>,
    pub(super) patches: Arc<ComposePatchService>,
}

#[singleton]
impl ComposeController {
    #[allow(clippy::too_many_arguments)]
    fn new(
        service: Arc<AppComposeService>,
        management: Arc<ComposeManagementService>,
        transfer: Arc<ComposeTransferService>,
        mounts: Arc<ComposeMountService>,
        patches: Arc<ComposePatchService>,
    ) -> Self {
        Self { service, management, transfer, mounts, patches }
    }
}
