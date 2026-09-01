pub mod typed;

use std::time::Duration;

pub use typed::RealtimeTypedExt;

/// Debounce window duration for query re-evaluation.
/// The evaluator loop in `typed.rs` uses `tokio::select!` with this constant
/// to implement leading-edge + trailing-edge debouncing:
///
/// ```text
/// Events:  ─E1──E2──E3────────────────E4──E5────────────
///                                                        
/// Runs:    ─R1─────────────R2──────────R3────────R4──────
///           ^               ^          ^          ^
///           leading         trailing   leading    trailing
///           (immediate)     (50ms)     (immediate)(50ms)
/// ```
///
/// This ensures the subscriber always sees the latest data after a burst,
/// unlike a leading-edge-only debouncer that would drop E2 and E3 forever.
pub const DEBOUNCE_WINDOW_MS: u64 = 50;

/// Returns the debounce window as a `Duration`.
pub fn debounce_window() -> Duration {
    Duration::from_millis(DEBOUNCE_WINDOW_MS)
}
