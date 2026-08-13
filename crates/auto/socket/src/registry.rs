use crate::{
    LiveRefreshDescriptor,
    state::{LIVE_REFRESHERS, ResolvedLiveRefresher},
};
use auto_di::{Container, DiError};
use std::sync::{Arc, atomic::AtomicBool};

pub(crate) async fn install_refreshers(container: &Container) -> Result<(), DiError> {
    let mut refreshers = Vec::new();
    for descriptor in inventory::iter::<LiveRefreshDescriptor> {
        refreshers.push(ResolvedLiveRefresher {
            endpoint: descriptor.endpoint,
            tables: descriptor.tables,
            refresh: (descriptor.factory)(container).await?,
            running: Arc::new(AtomicBool::new(false)),
            pending: Arc::new(AtomicBool::new(false)),
        });
    }
    let _ = LIVE_REFRESHERS.set(refreshers);
    Ok(())
}
