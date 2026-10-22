pub mod api;

// migration façades: required by auto_grpc expansions, which emit
// `crate::core::middleware::auth_context::AuthContext` and
// `crate::services::identity::permission::{...}`.
pub mod core {
    pub use openoxide_core::*;

    pub mod cache {
        pub use openoxide_compute::cache::*;
    }

    pub mod middleware {
        pub mod auth_context {
            pub use openoxide_identity::services::identity::auth_context::AuthContext;
        }
    }
}

pub mod services {
    pub use openoxide_compute::services::{
        builder, compute, infrastructure, integrations, migration, observability,
    };
    pub use openoxide_identity::services::identity;
    pub use openoxide_domain::services::{automation, networking, platform, workspace};
}

pub use openoxide_db::db;
pub use openoxide_db::flash_gen;

pub mod utils {
    pub use openoxide_core::{jwt, paths};
    pub use openoxide_platform::utils::*;
    pub use os;

    pub mod builder {
        pub use openoxide_builder::utils::builder::*;
        pub use openoxide_compute::services::builder::{database, queue};
    }
}
