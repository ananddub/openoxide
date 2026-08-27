pub use db::repository;

// Required by pipeline! macro — $crate::ScriptPipeline and $crate::shell_single_quote
pub use utils::exec::script::ScriptPipeline;
#[doc(hidden)]
pub use utils::exec::script::shell_single_quote;

pub mod api;
pub mod core;
pub mod db;
// FlashORM-generated, schema-safe SQLite query layer.
pub mod flash_gen;
pub mod services;
pub mod utils;
pub mod websocket;
