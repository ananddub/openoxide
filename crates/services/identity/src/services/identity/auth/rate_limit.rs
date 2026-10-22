use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use auto_di::singleton;

/// Beyond this the map is pruned; a brute-force attempt burst should never
/// grow the limiter into unbounded memory.
const MAX_TRACKED_KEYS: usize = 4096;
const ENTRY_TTL: Duration = Duration::from_secs(60 * 60);

struct Bucket {
    window_start: Instant,
    attempts: u32,
    blocked_until: Option<Instant>,
}

/// Process-local fixed-window limiter for authentication endpoints.
///
/// In-memory on purpose: the panel is a single process and the goal is to
/// stop online password/TOTP guessing, not to enforce a distributed quota.
pub struct AuthRateLimiter {
    buckets: Mutex<HashMap<String, Bucket>>,
}

#[singleton]
impl AuthRateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Counts one attempt against `key`. Returns the retry delay once the
    /// caller exceeds `limit` attempts inside `window`.
    pub fn check(&self, key: &str, limit: u32, window: Duration) -> Result<(), Duration> {
        let now = Instant::now();
        let mut buckets = self
            .buckets
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if buckets.len() >= MAX_TRACKED_KEYS {
            buckets.retain(|_, bucket| {
                bucket.blocked_until.is_some_and(|until| until > now)
                    || now.duration_since(bucket.window_start) < ENTRY_TTL
            });
        }

        let bucket = buckets.entry(key.to_string()).or_insert_with(|| Bucket {
            window_start: now,
            attempts: 0,
            blocked_until: None,
        });

        if let Some(until) = bucket.blocked_until {
            if until > now {
                return Err(until.duration_since(now));
            }
            bucket.blocked_until = None;
            bucket.window_start = now;
            bucket.attempts = 0;
        }

        if now.duration_since(bucket.window_start) >= window {
            bucket.window_start = now;
            bucket.attempts = 0;
        }

        if bucket.attempts >= limit {
            // Lock the key out for a full window after the limit is hit so
            // guessing cannot continue at full speed.
            bucket.blocked_until = Some(now + window);
            return Err(window);
        }

        bucket.attempts += 1;
        Ok(())
    }

    /// Clears an account-scoped counter after a successful sign-in; IP
    /// counters stay in place so a single host cannot spray many accounts.
    pub fn clear(&self, key: &str) {
        let mut buckets = self
            .buckets
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        buckets.remove(key);
    }
}

impl Default for AuthRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_after_the_limit_and_recovers_after_the_window() {
        let limiter = AuthRateLimiter::new();
        let window = Duration::from_millis(40);
        for _ in 0..3 {
            assert!(limiter.check("login:test", 3, window).is_ok());
        }
        assert!(limiter.check("login:test", 3, window).is_err());

        std::thread::sleep(window + Duration::from_millis(10));
        assert!(limiter.check("login:test", 3, window).is_ok());
    }

    #[test]
    fn clear_resets_an_account_counter() {
        let limiter = AuthRateLimiter::new();
        let window = Duration::from_secs(60);
        for _ in 0..2 {
            assert!(limiter.check("acct", 2, window).is_ok());
        }
        limiter.clear("acct");
        assert!(limiter.check("acct", 2, window).is_ok());
    }
}
