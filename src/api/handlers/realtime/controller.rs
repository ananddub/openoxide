use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, RealtimeService, RealtimeServiceExt, WatchTablesEvent,
};
use crate::services::platform::realtime::TableWatchStream;

use super::init::RealtimeController;

#[auto_grpc::service(connect = true, auth = true)]
impl RealtimeService for RealtimeController {
    #[permission(Organization, CanRead)]
    async fn watch_tables(
        &self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<impl connectrpc::Encodable<WatchTablesEvent> + Send + use<>>,
    > {
        // The notification channel only ever carries table names — row data
        // still flows through the caller's regular permission-checked RPCs,
        // so org scoping here is purely routing, not an access boundary.
        let (receiver, resync) = self.service.watch(auth.organization_id);
        Response::stream_ok(TableWatchStream::new(receiver, resync))
    }
}
