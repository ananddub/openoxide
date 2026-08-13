use crate::{
    LiveTableDescriptor,
    rooms::ACTIVE_LIVE_ROOMS,
    state::{LIVE_REFRESHERS, SOCKET_IO, SUBSCRIPTION_REFRESH_GATES},
};
use std::sync::{Arc, atomic::Ordering};

pub fn notify_table_changes<'a>(tables: impl IntoIterator<Item = &'a str>) {
    let changed = tables.into_iter().collect::<std::collections::HashSet<_>>();
    if let Some(refreshers) = LIVE_REFRESHERS.get() {
        for entry in refreshers {
            if !entry.tables.iter().any(|table| changed.contains(table)) {
                continue;
            }
            entry.pending.store(true, Ordering::Release);
            if entry.running.swap(true, Ordering::AcqRel) {
                continue;
            }
            let refresh = entry.refresh.clone();
            let running = entry.running.clone();
            let pending = entry.pending.clone();
            tokio::spawn(async move {
                loop {
                    pending.store(false, Ordering::Release);
                    refresh().await;
                    if pending.load(Ordering::Acquire) {
                        continue;
                    }
                    running.store(false, Ordering::Release);
                    if !pending.load(Ordering::Acquire)
                        || running
                            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                            .is_err()
                    {
                        break;
                    }
                }
            });
        }
    }
    invalidate_rooms(&changed);
}

fn invalidate_rooms(changed: &std::collections::HashSet<&str>) {
    let affected = inventory::iter::<LiveTableDescriptor>
        .into_iter()
        .filter(|entry| entry.tables.iter().any(|table| changed.contains(table)))
        .map(|entry| entry.endpoint)
        .collect::<std::collections::HashSet<_>>();
    if affected.is_empty() {
        return;
    }
    let rooms = ACTIVE_LIVE_ROOMS
        .get_or_init(Default::default)
        .lock()
        .map(|rooms| {
            rooms
                .values()
                .filter(|room| affected.contains(room.endpoint.as_str()))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    tokio::spawn(async move {
        let Some(io) = SOCKET_IO.get() else {
            return;
        };
        for room in rooms {
            if let Some(namespace) = io.of(room.namespace) {
                let _ = namespace
                    .to(room.room)
                    .emit(
                        "live:invalidate",
                        &serde_json::json!({"endpoint": room.endpoint, "args": room.args}),
                    )
                    .await;
            }
        }
    });
}

pub(crate) async fn refresh_subscription(endpoint: &str) {
    let Some(refreshers) = LIVE_REFRESHERS.get() else {
        return;
    };
    for entry in refreshers.iter().filter(|entry| entry.endpoint == endpoint) {
        let gate = {
            let mut gates = SUBSCRIPTION_REFRESH_GATES
                .get_or_init(Default::default)
                .lock()
                .expect("subscription refresh registry poisoned");
            gates
                .entry(endpoint.to_owned())
                .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
                .clone()
        };
        let _guard = gate.lock().await;
        (entry.refresh)().await;
    }
}
