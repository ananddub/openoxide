pub mod utils;

// migration façades: keep the moved code on its original crate:: paths.
pub mod core {
    pub use openoxide_core::*;
}

pub use openoxide_db::db;

pub mod services {
    pub use openoxide_compute::services::{
        builder, compute, infrastructure, integrations, migration, observability,
    };
    pub use openoxide_domain::services::{automation, networking, platform, workspace};
}
