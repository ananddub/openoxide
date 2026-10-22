pub mod automation;
pub mod networking;
pub mod platform;
pub mod workspace;

// migration façades: these modules moved to openoxide-compute.
pub use openoxide_compute::services::{
    builder, compute, infrastructure, integrations, migration, observability,
};
