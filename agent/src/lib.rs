// Every gRPC surface here returns `Result<T, tonic::Status>`; the error type is
// fixed by the tonic-generated service trait, so its 176-byte size is not ours
// to box.
#![allow(clippy::result_large_err)]

pub mod api;
pub mod app;
pub mod core;
pub mod services;
pub mod utils;
pub use core::{config, context, error};
pub use utils::docker;
pub use utils::telemetry;
pub use utils::telemetry::collector;
