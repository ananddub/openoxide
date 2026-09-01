use std::future::Future;
use std::sync::{Arc, Mutex};

use crate::db::reactive::bus::event_bus;
use crate::db::reactive::types::DbChangeEvent;

tokio::task_local! {
    pub static REQUEST_CHANGES: Arc<Mutex<Vec<DbChangeEvent>>>;
}

pub async fn request_scope<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let changes = Arc::new(Mutex::new(Vec::new()));
    let result = REQUEST_CHANGES.scope(Arc::clone(&changes), future).await;
    let changes = changes
        .lock()
        .map(|mut changes| std::mem::take(&mut *changes))
        .unwrap_or_default();
    event_bus().publish(changes);
    result
}
