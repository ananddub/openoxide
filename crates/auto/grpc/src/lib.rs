use auto_di::{Container, DiError};
use futures::future::BoxFuture;
use tonic::service::RoutesBuilder;

pub use auto_grpc_macros::middleware;
pub use auto_grpc_macros::service;

pub trait MiddlewareOrder {
    const ORDER: i32;
}

#[doc(hidden)]
pub mod __private {
    pub use auto_di;
    pub use futures;
    pub use inventory;
    pub use tonic;
}

pub type Registrar =
    for<'a> fn(&'a Container, &'a mut RoutesBuilder) -> BoxFuture<'a, Result<(), DiError>>;

pub struct Descriptor {
    pub registrar: Registrar,
}

inventory::collect!(Descriptor);

pub async fn register(container: &Container, builder: &mut RoutesBuilder) -> Result<(), DiError> {
    for descriptor in inventory::iter::<Descriptor> {
        (descriptor.registrar)(container, builder).await?;
    }
    Ok(())
}
