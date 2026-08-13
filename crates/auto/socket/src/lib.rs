#![doc = include_str!("../README.md")]

use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};

use auto_di::{BoxFuture, Container, DiError};
use serde::{Deserialize, Serialize};
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, TryData},
    socket::DisconnectReason,
};

pub use auto_socket_macros::{auto_socket, on};

mod descriptors;
pub use descriptors::{
    LiveAccessDescriptor, LiveRefreshDescriptor, LiveTableDescriptor, SocketDescriptor,
    SocketRegistrar,
};

static SOCKET_IO: OnceLock<SocketIo> = OnceLock::new();
static LIVE_REFRESHERS: OnceLock<Vec<ResolvedLiveRefresher>> = OnceLock::new();
static LATEST_CHANNELS: OnceLock<
    std::sync::Mutex<HashMap<String, tokio::sync::watch::Sender<serde_json::Value>>>,
> = OnceLock::new();
static STREAM_CHANNELS: OnceLock<
    std::sync::Mutex<HashMap<String, tokio::sync::mpsc::Sender<serde_json::Value>>>,
> = OnceLock::new();
static STREAM_HISTORY: OnceLock<std::sync::Mutex<HashMap<String, StreamHistory>>> = OnceLock::new();
static AUTHENTICATOR: OnceLock<LiveAuthenticator> = OnceLock::new();
static AUTHORIZER: OnceLock<LiveAuthorizer> = OnceLock::new();
static ACCESS_ENDPOINTS: OnceLock<HashMap<&'static str, Option<(&'static str, &'static str)>>> =
    OnceLock::new();
static SUBSCRIPTION_REFRESH_GATES: OnceLock<
    std::sync::Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>,
> = OnceLock::new();
static ACTIVE_LIVE_ROOMS: OnceLock<std::sync::Mutex<HashMap<String, ActiveLiveRoom>>> =
    OnceLock::new();

type LiveAuthenticator =
    Arc<dyn Fn(String) -> BoxFuture<'static, Result<LiveIdentity, String>> + Send + Sync>;
type LiveAuthorizer = Arc<
    dyn Fn(LiveIdentity, &'static str, &'static str) -> BoxFuture<'static, Result<bool, String>>
        + Send
        + Sync,
>;

#[derive(Clone, Debug)]
pub struct LiveIdentity {
    pub user_id: i64,
    pub organization_id: Option<i64>,
}

pub fn set_authorizer<F, Fut>(authorize: F) -> Result<(), &'static str>
where
    F: Fn(LiveIdentity, &'static str, &'static str) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<bool, String>> + Send + 'static,
{
    AUTHORIZER
        .set(Arc::new(move |identity, resource, operation| {
            Box::pin(authorize(identity, resource, operation))
        }))
        .map_err(|_| "live authorizer already configured")
}

#[derive(Clone, Debug, Deserialize)]
struct LiveConnectAuth {
    token: Option<String>,
}

#[derive(Clone, Debug)]
struct SocketIdentity(LiveIdentity);

pub fn set_authenticator<F, Fut>(authenticate: F) -> Result<(), &'static str>
where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<LiveIdentity, String>> + Send + 'static,
{
    AUTHENTICATOR
        .set(Arc::new(move |token| Box::pin(authenticate(token))))
        .map_err(|_| "live authenticator already configured")
}

struct StreamHistory {
    limit: usize,
    events: VecDeque<serde_json::Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveStrategy {
    Publish,
    Sqlite,
    Latest,
    Stream { capacity: usize, replay: usize },
}

type LiveRefresher = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static>;

struct ResolvedLiveRefresher {
    endpoint: &'static str,
    tables: &'static [&'static str],
    refresh: LiveRefresher,
    running: Arc<AtomicBool>,
    pending: Arc<AtomicBool>,
}

#[derive(Debug, Deserialize)]
struct LiveSubscriptionRequest {
    endpoint: String,
    args: serde_json::Value,
}

#[derive(Clone, Debug)]
struct ActiveLiveRoom {
    namespace: &'static str,
    room: String,
    endpoint: String,
    args: serde_json::Value,
    subscribers: usize,
}

#[derive(Clone, Debug, Default)]
struct SocketSubscriptions(std::collections::HashSet<String>);

fn retain_live_room(namespace: &'static str, room: &str, subscription: &LiveSubscriptionRequest) {
    let rooms = ACTIVE_LIVE_ROOMS.get_or_init(Default::default);
    let mut rooms = rooms.lock().expect("active live room registry poisoned");
    rooms
        .entry(room.to_owned())
        .and_modify(|active| active.subscribers += 1)
        .or_insert_with(|| ActiveLiveRoom {
            namespace,
            room: room.to_owned(),
            endpoint: subscription.endpoint.clone(),
            args: subscription.args.clone(),
            subscribers: 1,
        });
}

fn release_live_room(room: &str) {
    let rooms = ACTIVE_LIVE_ROOMS.get_or_init(Default::default);
    let mut rooms = rooms.lock().expect("active live room registry poisoned");
    let remove = rooms.get_mut(room).is_some_and(|active| {
        if active.subscribers > 1 {
            active.subscribers -= 1;
            false
        } else {
            true
        }
    });
    if remove {
        rooms.remove(room);
    }
}

fn scoped_room(
    identity: Option<&LiveIdentity>,
    endpoint: &str,
    args: &serde_json::Value,
) -> String {
    match identity {
        Some(identity) => format!("user:{}:{endpoint}:{args}", identity.user_id),
        None => format!("{endpoint}:{args}"),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    #[error("auto_socket has not been registered")]
    NotRegistered,
    #[error("failed to serialize live endpoint arguments: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to publish socket event: {0}")]
    Broadcast(#[from] socketioxide::BroadcastError),
    #[error("live publisher requires endpoint room arguments")]
    MissingRoomArguments,
}

/// A type-safe handle for publishing a live endpoint result.
pub struct LivePublisher<T> {
    namespace: &'static str,
    endpoint: &'static str,
    event: &'static str,
    room_args: Option<serde_json::Value>,
    strategy: LiveStrategy,
    identity: Option<LiveIdentity>,
    marker: PhantomData<fn() -> T>,
}

/// Typed client-side subscription descriptor generated for every live endpoint.
pub struct LiveSubscription<T> {
    namespace: &'static str,
    endpoint: &'static str,
    event: &'static str,
    client_name: &'static str,
    args: serde_json::Value,
    marker: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct LiveSubscriptionMessage {
    pub endpoint: &'static str,
    pub args: serde_json::Value,
}

impl<T> LiveSubscription<T> {
    pub fn new<A: Serialize>(
        namespace: &'static str,
        endpoint: &'static str,
        event: &'static str,
        client_name: &'static str,
        args: A,
    ) -> Result<Self, PublishError> {
        Ok(Self {
            namespace,
            endpoint,
            event,
            client_name,
            args: serde_json::to_value(args)?,
            marker: PhantomData,
        })
    }

    pub fn namespace(&self) -> &'static str {
        self.namespace
    }
    pub fn endpoint(&self) -> &'static str {
        self.endpoint
    }
    pub fn event(&self) -> &'static str {
        self.event
    }
    pub fn client_name(&self) -> &'static str {
        self.client_name
    }
    pub fn args(&self) -> &serde_json::Value {
        &self.args
    }
    pub fn message(&self) -> LiveSubscriptionMessage {
        LiveSubscriptionMessage {
            endpoint: self.endpoint,
            args: self.args.clone(),
        }
    }
    pub fn decode(&self, payload: serde_json::Value) -> Result<T, serde_json::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_value(payload)
    }
}

impl<T> LivePublisher<T>
where
    T: Serialize,
{
    pub fn new(namespace: &'static str, endpoint: &'static str, event: &'static str) -> Self {
        Self {
            namespace,
            endpoint,
            event,
            room_args: None,
            strategy: LiveStrategy::Publish,
            identity: None,
            marker: PhantomData,
        }
    }

    pub fn room<A: Serialize>(mut self, args: A) -> Result<Self, PublishError> {
        self.room_args = Some(serde_json::to_value(args)?);
        Ok(self)
    }

    pub fn strategy(mut self, strategy: LiveStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn user(mut self, user_id: i64) -> Self {
        self.identity = Some(LiveIdentity {
            user_id,
            organization_id: None,
        });
        self
    }

    pub async fn publish(self, data: T) -> Result<(), PublishError> {
        let args = self.room_args.ok_or(PublishError::MissingRoomArguments)?;
        let message = serde_json::json!({"endpoint": self.endpoint, "args": args, "data": data});
        let room = scoped_room(self.identity.as_ref(), self.endpoint, &message["args"]);
        match self.strategy {
            LiveStrategy::Publish | LiveStrategy::Sqlite => {
                emit_room(self.namespace, room, message).await
            }
            LiveStrategy::Latest => publish_latest(self.namespace, room, message),
            LiveStrategy::Stream { capacity, replay } => {
                publish_stream(self.namespace, room, message, capacity, replay).await
            }
        }
    }

    /// Emits this endpoint's typed payload to one explicitly selected socket.
    pub fn emit(self, socket: &SocketRef, data: T) -> Result<(), socketioxide::SendError> {
        socket.emit(
            "live:update",
            &serde_json::json!({"endpoint": self.endpoint, "args": self.room_args, "data": data}),
        )
    }

    /// Broadcasts this endpoint's typed payload to every client in its socket group.
    pub async fn broadcast(self, data: T) -> Result<(), PublishError> {
        let io = SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
        if let Some(namespace) = io.of(self.namespace) {
            namespace
                .emit(
                    "live:update",
                    &serde_json::json!({"endpoint": self.endpoint, "args": self.room_args, "data": data}),
                )
                .await?;
        }
        Ok(())
    }

    pub fn endpoint(&self) -> &'static str {
        self.endpoint
    }
}

async fn emit_room(
    namespace: &'static str,
    room: String,
    message: serde_json::Value,
) -> Result<(), PublishError> {
    let io = SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
    if let Some(socket_namespace) = io.of(namespace) {
        socket_namespace
            .to(room)
            .emit("live:update", &message)
            .await?;
    }
    Ok(())
}

fn publish_latest(
    namespace: &'static str,
    room: String,
    message: serde_json::Value,
) -> Result<(), PublishError> {
    SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
    let key = format!("{namespace}:{room}");
    let channels = LATEST_CHANNELS.get_or_init(Default::default);
    let mut channels = channels
        .lock()
        .expect("latest live channel registry poisoned");
    if let Some(sender) = channels.get(&key) {
        sender.send_replace(message);
        return Ok(());
    }
    let (sender, mut receiver) = tokio::sync::watch::channel(message);
    channels.insert(key, sender);
    tokio::spawn(async move {
        loop {
            let value = receiver.borrow_and_update().clone();
            let _ = emit_room(namespace, room.clone(), value).await;
            if receiver.changed().await.is_err() {
                break;
            }
        }
    });
    Ok(())
}

async fn publish_stream(
    namespace: &'static str,
    room: String,
    message: serde_json::Value,
    capacity: usize,
    replay: usize,
) -> Result<(), PublishError> {
    SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
    let key = format!("{namespace}:{room}");
    if replay > 0 {
        let histories = STREAM_HISTORY.get_or_init(Default::default);
        let mut histories = histories.lock().expect("stream history registry poisoned");
        let history = histories
            .entry(key.clone())
            .or_insert_with(|| StreamHistory {
                limit: replay,
                events: VecDeque::with_capacity(replay),
            });
        history.limit = replay;
        while history.events.len() >= history.limit {
            history.events.pop_front();
        }
        history.events.push_back(message.clone());
    }
    let sender = {
        let channels = STREAM_CHANNELS.get_or_init(Default::default);
        let mut channels = channels
            .lock()
            .expect("stream live channel registry poisoned");
        if let Some(sender) = channels.get(&key) {
            sender.clone()
        } else {
            let (sender, mut receiver) = tokio::sync::mpsc::channel(capacity.max(1));
            channels.insert(key, sender.clone());
            tokio::spawn(async move {
                while let Some(value) = receiver.recv().await {
                    let _ = emit_room(namespace, room.clone(), value).await;
                }
            });
            sender
        }
    };
    sender
        .send(message)
        .await
        .map_err(|_| PublishError::NotRegistered)
}

/// Refreshes every live endpoint affected by committed table changes.
/// Each registered endpoint resolver runs once, independent of subscriber count.
pub fn notify_table_changes<'a>(tables: impl IntoIterator<Item = &'a str>) {
    let Some(refreshers) = LIVE_REFRESHERS.get() else {
        return;
    };
    let changed = tables.into_iter().collect::<std::collections::HashSet<_>>();
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
                // Close the race where a notification arrived just before
                // `running` was cleared and therefore did not spawn a worker.
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

async fn refresh_for_subscription(endpoint: &str) {
    let Some(refreshers) = LIVE_REFRESHERS.get() else {
        return;
    };
    for entry in refreshers.iter().filter(|entry| entry.endpoint == endpoint) {
        let gate = {
            let gates = SUBSCRIPTION_REFRESH_GATES.get_or_init(Default::default);
            let mut gates = gates
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

/// Resolves socket handler objects and registers each namespace exactly once.
pub async fn register(io: &SocketIo, container: &Container) -> Result<(), DiError> {
    let _ = SOCKET_IO.set(io.clone());
    let _ = ACCESS_ENDPOINTS.set(
        inventory::iter::<LiveAccessDescriptor>
            .into_iter()
            .map(|item| (item.endpoint, item.permission))
            .collect(),
    );
    let mut namespaces: HashMap<&'static str, Vec<SocketRegistrar>> = HashMap::new();
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

    for descriptor in inventory::iter::<SocketDescriptor> {
        namespaces
            .entry(descriptor.namespace)
            .or_default()
            .push((descriptor.factory)(container).await?);
    }

    for (namespace, registrars) in namespaces {
        io.ns(namespace, move |socket: SocketRef, TryData(auth): TryData<LiveConnectAuth>| {
            let registrars = registrars.clone();
            async move {
                if let Some(authenticator) = AUTHENTICATOR.get() {
                    let Some(token) = auth.ok().and_then(|auth| auth.token) else {
                        let _ = socket.clone().disconnect();
                        return;
                    };
                    match authenticator(token).await {
                        Ok(identity) => { socket.extensions.insert(SocketIdentity(identity)); }
                        Err(_) => { let _ = socket.clone().disconnect(); return; }
                    }
                }
                socket.on(
                    "live:subscribe",
                    move |socket: SocketRef, Data(subscription): Data<LiveSubscriptionRequest>| async move {
                        let access = ACCESS_ENDPOINTS.get().and_then(|endpoints| endpoints.get(subscription.endpoint.as_str())).copied();
                        let identity = access.is_some().then(|| socket.extensions.get::<SocketIdentity>().map(|value| value.0)).flatten();
                        let Some(identity) = identity.or_else(|| access.is_none().then(|| LiveIdentity { user_id: 0, organization_id: None })) else { return };
                        if let Some(Some((resource, operation))) = access {
                            let Some(authorizer) = AUTHORIZER.get() else { return };
                            match authorizer(identity.clone(), resource, operation).await {
                                Ok(true) => {}
                                _ => return,
                            }
                        }
                        let scoped_identity = access.is_some().then_some(&identity);
                        let room = scoped_room(scoped_identity, &subscription.endpoint, &subscription.args);
                        // Join before refreshing so the initial snapshot cannot
                        // race past a newly connected subscriber.
                        socket.join(room.clone());
                        let first_for_socket = {
                            let mut subscriptions = socket.extensions.get::<SocketSubscriptions>()
                                .map(|value| value.0)
                                .unwrap_or_default();
                            let inserted = subscriptions.insert(room.clone());
                            socket.extensions.insert(SocketSubscriptions(subscriptions));
                            inserted
                        };
                        if first_for_socket { retain_live_room(namespace, &room, &subscription); }
                        // Populate a fresh snapshot after joining. For the
                        // fixed-room SQLite endpoints this invokes the same
                        // deduplicated refresher used by commit notifications.
                        refresh_for_subscription(&subscription.endpoint).await;
                        let history_key = format!("{namespace}:{room}");
                        let latest = LATEST_CHANNELS
                            .get_or_init(Default::default)
                            .lock()
                            .ok()
                            .and_then(|channels| {
                                channels
                                    .get(&history_key)
                                    .map(|sender| sender.borrow().clone())
                            });
                        if let Ok(histories) = STREAM_HISTORY.get_or_init(Default::default).lock() {
                            if let Some(latest) = latest {
                                let _ = socket.emit("live:update", &latest);
                            }
                            if let Some(history) = histories.get(&history_key) {
                                for event in &history.events {
                                    let _ = socket.emit("live:update", event);
                                }
                            }
                        } else {
                            if let Some(latest) = latest {
                                let _ = socket.emit("live:update", &latest);
                            }
                        }
                    },
                );
                socket.on(
                    "live:unsubscribe",
                    |socket: SocketRef, Data(subscription): Data<LiveSubscriptionRequest>| async move {
                        let requires_auth = ACCESS_ENDPOINTS.get().is_some_and(|endpoints| endpoints.contains_key(subscription.endpoint.as_str()));
                        let identity = requires_auth.then(|| socket.extensions.get::<SocketIdentity>().map(|value| value.0)).flatten();
                        let room = scoped_room(identity.as_ref(), &subscription.endpoint, &subscription.args);
                        socket.leave(room.clone());
                        let removed = {
                            let mut subscriptions = socket.extensions.get::<SocketSubscriptions>()
                                .map(|value| value.0)
                                .unwrap_or_default();
                            let removed = subscriptions.remove(&room);
                            socket.extensions.insert(SocketSubscriptions(subscriptions));
                            removed
                        };
                        if removed { release_live_room(&room); }
                    },
                );
                socket.on_disconnect(|socket: SocketRef, _reason: DisconnectReason| async move {
                    if let Some(subscriptions) = socket.extensions.get::<SocketSubscriptions>() {
                        for room in subscriptions.0 { release_live_room(&room); }
                    }
                });
                for registrar in registrars {
                    registrar(socket.clone());
                }
            }
        });
    }

    Ok(())
}

/// Registers all socket handlers using auto-di's global container.
pub async fn register_global(io: &SocketIo) -> Result<(), DiError> {
    register(io, auto_di::global_container()?).await
}

#[doc(hidden)]
pub mod __private {
    pub use auto_di;
    pub use inventory;
    pub use socketioxide;
}
