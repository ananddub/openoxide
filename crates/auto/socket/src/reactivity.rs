use crate::{
    LiveTableDescriptor,
    rooms::ACTIVE_LIVE_ROOMS,
    state::{LIVE_REFRESHERS, SOCKET_IO, SUBSCRIPTION_REFRESH_GATES},
};
use std::sync::Arc;

pub fn notify_table_changes<'a>(tables: impl IntoIterator<Item = &'a str>) {
    let changed = tables.into_iter().collect::<std::collections::HashSet<_>>();
    tracing::info!(tables = ?changed, "live table notification received");
    refresh_or_invalidate_rooms(&changed);
}

fn refresh_or_invalidate_rooms(changed: &std::collections::HashSet<&str>) {
    let affected = inventory::iter::<LiveTableDescriptor>
        .into_iter()
        .filter(|entry| entry.tables.iter().any(|table| changed.contains(table)))
        .map(|entry| entry.endpoint)
        .collect::<std::collections::HashSet<_>>();
    if affected.is_empty() {
        tracing::info!("live table notification has no matching endpoint");
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
    tracing::info!(endpoints = ?affected, rooms = rooms.len(), "live rooms selected for refresh");
    tokio::spawn(async move {
        let Some(io) = SOCKET_IO.get() else {
            return;
        };
        for room in rooms {
            if let Some(refresher) = LIVE_REFRESHERS
                .get()
                .and_then(|items| items.iter().find(|item| item.endpoint == room.endpoint))
            {
                tracing::info!(endpoint = room.endpoint, room = %room.room, args = ?room.args, "refreshing live room");
                (refresher.refresh)(room.args.clone(), room.identity.clone()).await;
                tracing::info!(endpoint = room.endpoint, room = %room.room, "live room refresh completed");
                continue;
            }
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

pub(crate) async fn refresh_subscription(
    endpoint: &str,
    args: serde_json::Value,
    identity: Option<crate::LiveIdentity>,
) {
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
        (entry.refresh)(args.clone(), identity.clone()).await;
    }
}
