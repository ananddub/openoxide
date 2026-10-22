pub mod utils;

// migration façade: keep the moved code on its original crate:: paths.
pub mod core {
    pub use openoxide_core::*;
}

pub use openoxide_db::db;
pub use openoxide_db::flash_gen;
