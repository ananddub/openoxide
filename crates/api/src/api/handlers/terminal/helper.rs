use std::pin::Pin;
use std::task::{Context, Poll};

use connectrpc::ConnectError;
use futures::Stream;

use crate::api::connect::generated::openoxide::api::v1::TerminalEvent;
use crate::services::infrastructure::terminal::TerminalSession;

/// Response body of `OpenTerminal`: adapts the service's event channel to
/// the ConnectRPC stream item type. Session teardown happens in
/// `TerminalSession`'s own `Drop` when this stream ends.
pub struct TerminalEventStream {
    session: TerminalSession,
}

impl TerminalEventStream {
    pub fn new(session: TerminalSession) -> Self {
        Self { session }
    }
}

impl Stream for TerminalEventStream {
    type Item = Result<TerminalEvent, ConnectError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut()
            .session
            .poll_next(cx)
            .map(|event| event.map(Ok))
    }
}
