pub mod db;
// FlashORM-generated, schema-safe SQLite query layer.
#[allow(non_snake_case)]
pub mod flash_gen;

// migration façade: keep the moved db code on its original crate:: paths.
pub mod core {
    pub use openoxide_core::*;
}

pub mod utils {
    pub use openoxide_core::{jwt, paths};
    pub use os;
    pub use os::*;
}
