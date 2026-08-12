#![doc = include_str!("../README.md")]

use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, OnceLock},
};

use auto_di::{BoxFuture, Container, DiError};
use serde::{Deserialize, Serialize};
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
};

pub use auto_socket_macros::{auto_socket, on};

static SOCKET_IO: OnceLock<SocketIo> = OnceLock::new();

#[derive(Debug, Deserialize)]
struct LiveSubscription {
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
}

/// A type-safe handle for publishing a live endpoint result.
pub struct LivePublisher<T> {
    namespace: &'static str,
    endpoint: &'static str,
    event: &'static str,
    marker: PhantomData<fn() -> T>,
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
            marker: PhantomData,
        }
    }

    pub async fn publish<A: Serialize>(self, args: A, data: T) -> Result<(), PublishError> {
        let io = SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
        let args = serde_json::to_value(args)?;
        let room = format!("{}:{args}", self.endpoint);
        if let Some(namespace) = io.of(self.namespace) {
            namespace.to(room).emit(self.event, &data).await?;
        }
        Ok(())
    }

    /// Emits this endpoint's typed payload to one explicitly selected socket.
    pub fn emit(self, socket: &SocketRef, data: T) -> Result<(), socketioxide::SendError> {
        socket.emit(self.event, &data)
    }

    /// Broadcasts this endpoint's typed payload to every client in its socket group.
    pub async fn broadcast(self, data: T) -> Result<(), PublishError> {
        let io = SOCKET_IO.get().ok_or(PublishError::NotRegistered)?;
        if let Some(namespace) = io.of(self.namespace) {
            namespace.emit(self.event, &data).await?;
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

/// Resolves socket handler objects and registers each namespace exactly once.
pub async fn register(io: &SocketIo, container: &Container) -> Result<(), DiError> {
    let _ = SOCKET_IO.set(io.clone());
    let mut namespaces: HashMap<&'static str, Vec<SocketRegistrar>> = HashMap::new();

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
                    |socket: SocketRef, Data(subscription): Data<LiveSubscription>| async move {
                        socket.join(format!("{}:{}", subscription.endpoint, subscription.args));
                    },
                );
                socket.on(
                    "live:unsubscribe",
                    |socket: SocketRef, Data(subscription): Data<LiveSubscription>| async move {
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
