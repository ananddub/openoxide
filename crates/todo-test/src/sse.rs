use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response, Sse, sse::Event},
};
use std::convert::Infallible;

#[auto_route::get("/_rustploy/html/events/{session}")]
pub async fn events(Path(session): Path<String>) -> Response {
    let Some(receiver) = html_rt::take_session(&session) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let stream = futures::stream::unfold(receiver, |mut receiver| async move {
        receiver.recv().await.map(|patch| {
            let data = serde_json::json!({"slot": patch.slot, "html": patch.html});
            (
                Ok::<Event, Infallible>(Event::default().data(data.to_string())),
                receiver,
            )
        })
    });
    Sse::new(stream).into_response()
}
