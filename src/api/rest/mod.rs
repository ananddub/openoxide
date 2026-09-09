use std::convert::Infallible;

use bytes::Bytes;
use http_body_util::{BodyExt, Full, combinators::UnsyncBoxBody};
use hyper::{Method, Response, body::Incoming, header::CONTENT_TYPE};

type ResponseB = Response<UnsyncBoxBody<Bytes, Infallible>>;

/// Routes the legacy REST surface (`/api/*`) that predates the ConnectRPC
/// migration. These are plain-text / streaming endpoints (log tailing,
/// docker stats) that don't fit the Connect protocol, so the dispatcher
/// handles them here instead of falling through to the SPA fallback.
pub async fn handle(
    method: &Method,
    path: &str,
    query: Option<&str>,
    _body: Incoming,
) -> ResponseB {
    let path = path.trim_start_matches("/api");

    match (method, path) {
        (m, p) if *m == Method::GET && p.starts_with("/deployments/") && p.ends_with("/logs") => {
            let id = p
                .trim_start_matches("/deployments/")
                .trim_end_matches("/logs");
            deployment_log_file(id).await
        }
        (m, p)
            if *m == Method::GET && p.starts_with("/deployments/docker/service/") =>
        {
            docker_service_logs(p, query).await
        }
        _ => Response::builder()
            .status(404)
            .header(CONTENT_TYPE, "application/json")
            .body(empty())
            .expect("static 404 response is always valid"),
    }
}

/// GET /api/deployments/:id/logs — serves the deployment's log file.
async fn deployment_log_file(id: &str) -> ResponseB {
    if !id.chars().all(|c| c.is_ascii_digit()) || id.is_empty() {
        return text_response(400, "{\"code\":\"invalid_argument\",\"message\":\"invalid deployment id\"}");
    }

    let path = crate::utils::paths::openoxide_paths()
        .deployment_log_file(id.parse::<i64>().unwrap_or(0));
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => text_response(200, &content),
        Err(_) => text_response(
            404,
            "{\"code\":\"not_found\",\"message\":\"Log file not found\"}",
        ),
    }
}

/// GET /api/deployments/docker/service/:name/logs — fetches current docker
/// logs for the container/service (plain text; follow param is ignored —
/// the client re-polls live mode on an interval).
async fn docker_service_logs(path: &str, query: Option<&str>) -> ResponseB {
    let name = path
        .trim_start_matches("/deployments/docker/service/")
        .split('/')
        .next()
        .unwrap_or("")
        .to_string();
    if name.is_empty() {
        return text_response(400, "{\"code\":\"invalid_argument\",\"message\":\"missing service name\"}");
    }

    let tail = query
        .and_then(|q| {
            q.split('&').find_map(|kv| {
                let (k, v) = kv.split_once('=')?;
                (k == "tail").then(|| v.to_string())
            })
        })
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(200);

    let service = match auto_di::resolve::<std::sync::Arc<
        crate::services::compute::engine::DeploymentService,
    >>()
    .await
    {
        Ok(service) => service,
        Err(error) => {
            tracing::error!(%error, "deployment service unavailable for docker logs");
            return text_response(
                503,
                "{\"code\":\"unavailable\",\"message\":\"service not initialized\"}",
            );
        }
    };
    let output = match service
        .docker_container_logs(
            None,
            name.clone(),
            crate::services::compute::engine::DockerLogOptions {
                tail,
                ..Default::default()
            },
        )
        .await
    {
        Ok(output) => output,
        Err(error) => {
            tracing::warn!(%error, service = %name, "docker service logs failed");
            return text_response(
                502,
                &format!("docker logs unavailable: {error}"),
            );
        }
    };

    text_response(200, &format!("{}{}", output.stdout, output.stderr))
}

fn text_response(status: u16, body: &str) -> ResponseB {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(full(body.to_string().into_bytes()))
        .expect("text response is always valid")
}

fn empty() -> UnsyncBoxBody<Bytes, Infallible> {
    Full::<Bytes>::default()
        .map_err(|e| match e {})
        .boxed_unsync()
}

fn full(bytes: Vec<u8>) -> UnsyncBoxBody<Bytes, Infallible> {
    Full::new(Bytes::from(bytes))
        .map_err(|e| match e {})
        .boxed_unsync()
}
