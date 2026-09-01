use futures_util::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

use crate::c_ffi::RealtimeRawEvent;
use crate::engine::RealtimeHandle;

pub trait RealtimeStreamExt {
    fn watch_table(&self, target_table: &str) -> impl Stream<Item = RealtimeRawEvent>;
}

impl RealtimeStreamExt for RealtimeHandle {
    fn watch_table(&self, target_table: &str) -> impl Stream<Item = RealtimeRawEvent> {
        let target = target_table.to_string();
        let rx = self.subscribe();
        BroadcastStream::new(rx).filter_map(move |res| {
            let target = target.clone();
            async move {
                let ev = res.ok()?;
                if ev.table == target {
                    Some(ev)
                } else {
                    None
                }
            }
        })
    }
}
