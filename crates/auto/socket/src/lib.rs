#![doc = include_str!("../README.md")]

use std::{
    collections::HashMap,
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
    extract::{Data, SocketRef},
};

pub use auto_socket_macros::{auto_socket, on};

static SOCKET_IO: OnceLock<SocketIo> = OnceLock::new();
static LIVE_REFRESHERS: OnceLock<Vec<ResolvedLiveRefresher>> = OnceLock::new();

type LiveRefresher = Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static>;

struct ResolvedLiveRefresher {
    table: &'static str,
    refresh: LiveRefresher,
    running: Arc<AtomicBool>,
    pending: Arc<AtomicBool>,
}

#[derive(Debug, Deserialize)]
struct LiveSubscriptionRequest {
    endpoint: String,
    args: serde_json::Value,
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
            marker: PhantomData,
        }
    }

    pub fn room<A: Serialize>(mut self, args: A) -> Result<Self, PublishError> {
        self.room_args = Some(serde_json::to_value(args)?);
        Ok(self)
    }

    pub async fn publish(self, data: T) -> Result<(), PublishError> {
        let io = SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
        let args = self.room_args.ok_or(PublishError::MissingRoomArguments)?;
        let room = format!("{}:{args}", self.endpoint);
        if let Some(namespace) = io.of(self.namespace) {
            namespace
                .to(room)
                .emit(
                    "live:update",
                    &serde_json::json!({
                        "endpoint": self.endpoint,
                        "args": args,
                        "data": data,
                    }),
                )
                .await?;
        }
        Ok(())
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

#[doc(hidden)]
pub type SocketRegistrar = Arc<dyn Fn(SocketRef) + Send + Sync + 'static>;

/// A namespace event registrar submitted by the socket macros.
#[doc(hidden)]
pub struct SocketDescriptor {
    namespace: &'static str,
    factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<SocketRegistrar, DiError>>,
}

impl SocketDescriptor {
    #[doc(hidden)]
    pub const fn new(
        namespace: &'static str,
        factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<SocketRegistrar, DiError>>,
    ) -> Self {
        Self { namespace, factory }
    }
}

inventory::collect!(SocketDescriptor);

/// A table-backed live endpoint refresh resolver generated by `auto_route`.
#[doc(hidden)]
pub struct LiveRefreshDescriptor {
    table: &'static str,
    factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<LiveRefresher, DiError>>,
}

impl LiveRefreshDescriptor {
    pub const fn new(
        table: &'static str,
        factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<LiveRefresher, DiError>>,
    ) -> Self {
        Self { table, factory }
    }
}

inventory::collect!(LiveRefreshDescriptor);

/// Refreshes every live endpoint affected by committed table changes.
/// Each registered endpoint resolver runs once, independent of subscriber count.
pub fn notify_table_changes<'a>(tables: impl IntoIterator<Item = &'a str>) {
    let Some(refreshers) = LIVE_REFRESHERS.get() else {
        return;
    };
    let changed = tables.into_iter().collect::<std::collections::HashSet<_>>();
    for entry in refreshers {
        if !changed.contains(entry.table) {
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
}

/// Resolves socket handler objects and registers each namespace exactly once.
pub async fn register(io: &SocketIo, container: &Container) -> Result<(), DiError> {
    let _ = SOCKET_IO.set(io.clone());
    let mut namespaces: HashMap<&'static str, Vec<SocketRegistrar>> = HashMap::new();
    let mut refreshers = Vec::new();

    for descriptor in inventory::iter::<LiveRefreshDescriptor> {
        refreshers.push(ResolvedLiveRefresher {
            table: descriptor.table,
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
        io.ns(namespace, move |socket: SocketRef| {
            let registrars = registrars.clone();
            async move {
                socket.on(
                    "live:subscribe",
                    |socket: SocketRef, Data(subscription): Data<LiveSubscriptionRequest>| async move {
                        socket.join(format!("{}:{}", subscription.endpoint, subscription.args));
                    },
                );
                socket.on(
                    "live:unsubscribe",
                    |socket: SocketRef, Data(subscription): Data<LiveSubscriptionRequest>| async move {
                        socket.leave(format!("{}:{}", subscription.endpoint, subscription.args));
                    },
                );
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
