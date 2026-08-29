use std::{convert::Infallible, sync::Arc};

use axum::{Router as AxumRouter, body::Body as AxumBody};
use bytes::Bytes;
use connectrpc::{ConnectRpcBody, ConnectRpcService};
use http_body_util::{BodyExt, combinators::UnsyncBoxBody};
use hyper::{Request, Response, body::Incoming, service::service_fn};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use tokio::net::TcpListener;
use tower::ServiceExt;

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type ResponseBody = UnsyncBoxBody<Bytes, BoxError>;

pub async fn serve(
    listener: TcpListener,
    legacy: AxumRouter,
    connect: ConnectRpcService,
) -> std::io::Result<()> {
    let legacy = Arc::new(legacy);
    let connect = Arc::new(connect);

    loop {
        let (stream, peer) = listener.accept().await?;
        let legacy = Arc::clone(&legacy);
        let connect = Arc::clone(&connect);

        tokio::spawn(async move {
            let service = service_fn(move |request| {
                dispatch(request, Arc::clone(&legacy), Arc::clone(&connect))
            });
            let io = TokioIo::new(stream);
            let builder = Builder::new(TokioExecutor::new());

            if let Err(error) = builder.serve_connection_with_upgrades(io, service).await {
                tracing::debug!(%peer, %error, "HTTP connection closed with an error");
            }
        });
    }
}

async fn dispatch(
    request: Request<Incoming>,
    legacy: Arc<AxumRouter>,
    connect: Arc<ConnectRpcService>,
) -> Result<Response<ResponseBody>, Infallible> {
    if request.uri().path().starts_with("/openoxide.api.") {
        let response = connect.as_ref().clone().oneshot(request).await?;
        return Ok(response.map(box_connect_body));
    }

    let response = legacy.as_ref().clone().oneshot(request).await?;
    Ok(response.map(box_axum_body))
}

fn box_connect_body(body: ConnectRpcBody) -> ResponseBody {
    body.map_err(|error| match error {}).boxed_unsync()
}

fn box_axum_body(body: AxumBody) -> ResponseBody {
    body.map_err(|error| -> BoxError { Box::new(error) })
        .boxed_unsync()
}
