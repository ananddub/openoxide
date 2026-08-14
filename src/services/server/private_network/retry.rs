use std::{future::Future, time::Duration};

#[derive(Debug, Clone, Copy)]
pub(super) struct RetryPolicy {
    attempts: usize,
    initial_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    pub(super) const fn network() -> Self {
        Self {
            attempts: 6,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(3),
        }
    }

    pub(super) const fn fast_network() -> Self {
        Self {
            attempts: 2,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(1),
        }
    }

    #[cfg(test)]
    const fn immediate(attempts: usize) -> Self {
        Self {
            attempts,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        }
    }

    pub(super) async fn run<T, E, F, Fut>(self, mut operation: F) -> Result<T, E>
    where
        F: FnMut(usize) -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        debug_assert!(self.attempts > 0);
        let mut delay = self.initial_delay;
        for attempt in 1..=self.attempts {
            match operation(attempt).await {
                Ok(value) => return Ok(value),
                Err(error) if attempt == self.attempts => return Err(error),
                Err(_) => {
                    tokio::time::sleep(delay).await;
                    delay = delay.saturating_mul(2).min(self.max_delay);
                }
            }
        }
        unreachable!("retry policy always performs at least one attempt")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::RetryPolicy;

    #[tokio::test]
    async fn retries_transient_failures_until_success() {
        let attempts = AtomicUsize::new(0);
        let result = RetryPolicy::immediate(4)
            .run(|_| async {
                let current = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                (current == 3).then_some("ready").ok_or("disconnected")
            })
            .await;
        assert_eq!(result, Ok("ready"));
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn returns_last_error_after_attempt_budget() {
        let attempts = AtomicUsize::new(0);
        let result: Result<(), usize> = RetryPolicy::immediate(3)
            .run(|attempt| {
                let attempts = &attempts;
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Err(attempt)
                }
            })
            .await;
        assert_eq!(result, Err(3));
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }
}
