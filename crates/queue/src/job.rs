use std::future::Future;
use tokio_util::sync::CancellationToken;

use crate::types::Priority;

pub trait TaskJob: Send + Sync + 'static {
    type Output: Send + Clone + 'static;
    type Error: std::fmt::Display + Send + Clone + 'static;

    fn id(&self) -> String;

    fn priority(&self) -> Priority {
        Priority::Normal
    }

    fn execute(
        self,
        cancel_token: CancellationToken,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send;
}
