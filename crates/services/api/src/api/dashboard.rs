use rust_embed::RustEmbed;

use bytes::Bytes;
use http_body_util::{BodyExt, Full, combinators::UnsyncBoxBody};
use hyper::{Method, Response};
use std::{convert::Infallible, fs};

type ResponseBody = UnsyncBoxBody<Bytes, Infallible>;

/// Directory the dev stack bind-mounts over the panel container. When the
/// mount exists the SPA is served from it, so web-only changes deploy by
/// rebuilding the bundle on the host — no binary recompile, no image
/// build. Production (scratch image, nothing mounted) finds no such
/// directory and silently falls back to the embedded assets.
const WEB_DIST_OVERRIDE: &str = "/app/web-dist";

/// The compiled OpenOxide dashboard (vite build output), embedded into the
/// panel binary so a single process serves the SPA over HTTPS/HTTP-2 — no
/// reverse proxy hop for the browser. Populated by the Docker build
/// (web/openoxide_react/dist); a debug cargo build embeds whatever dist
/// currently exists on disk, falling back to API-only serving when empty.
#[derive(RustEmbed)]
#[folder = "../../../web/openoxide_react/dist/"]
pub struct DashboardAssets;

/// Immutable decoded asset: content type + body bytes.
pub struct EmbeddedAsset {
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

pub fn lookup(path: &str) -> Option<EmbeddedAsset> {
    // Root serves index.html; vite emits hashed asset files and plain
    // files (favicon, manifest) at the root.
    let normalized = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    // Dev mount first (fresh host build, picked up per request), then the
    // compile-time embed.
    lookup_disk(normalized).or_else(|| lookup_embedded(normalized))
}

/// Reads an asset from the bind-mounted dev dist dir. Traversal is
/// rejected outright; the mount only ever exists in the dev stack.
fn lookup_disk(normalized: &str) -> Option<EmbeddedAsset> {
    if normalized.contains("..") {
        return None;
    }
    let body = fs::read(format!("{WEB_DIST_OVERRIDE}/{normalized}")).ok()?;
    Some(EmbeddedAsset {
        content_type: content_type_for(normalized),
        body,
    })
}

fn lookup_embedded(normalized: &str) -> Option<EmbeddedAsset> {
    let file = DashboardAssets::get(normalized)?;

    Some(EmbeddedAsset {
        content_type: content_type_for(normalized),
        body: file.data.to_vec(),
    })
}

fn content_type_for(normalized: &str) -> &'static str {
    match normalized.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" | "webmanifest" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "eot" => "application/vnd.ms-fontobject",
        "txt" => "text/plain; charset=utf-8",
        "map" => "application/json",
        _ => "application/octet-stream",
    }
}

/// Serves an embedded dashboard asset for non-RPC requests, with SPA
/// history-mode fallback (unknown paths render index.html). GET/HEAD only.
pub fn serve_spa(method: &Method, path: &str) -> Response<ResponseBody> {
    let is_get = *method == Method::GET;
    let is_head = *method == Method::HEAD;
    if !is_get && !is_head {
        return Response::builder()
            .status(405)
            .header("allow", "GET, HEAD")
            .body(empty_body())
            .expect("static 405 response is always valid");
    }

    match lookup(path).or_else(|| lookup("/index.html")) {
        Some(asset) => {
            let mut response = Response::builder()
                .status(200)
                .header("content-type", asset.content_type)
                .header("cache-control", cache_control_for(path));
            if is_head {
                response = response.header("content-length", asset.body.len());
            }
            let body: Full<Bytes> = if is_head {
                Full::default()
            } else {
                Full::new(Bytes::from(asset.body))
            };
            response
                .body(body.map_err(|error| match error {}).boxed_unsync())
                .expect("static asset response is always valid")
        }
        // No dist embedded (dev build before the web bundle exists): the
        // panel is API-only until `bun run build` populates the dist dir.
        None => Response::builder()
            .status(503)
            .header("content-type", "text/plain; charset=utf-8")
            .body(empty_body())
            .expect("static 503 response is always valid"),
    }
}

fn cache_control_for(path: &str) -> &'static str {
    // Hashed vite assets are immutable; index.html must revalidate so
    // users pick up new deploys immediately.
    if path.starts_with("/assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    }
}

fn empty_body() -> UnsyncBoxBody<Bytes, Infallible> {
    Full::<Bytes>::default()
        .map_err(|error| match error {})
        .boxed_unsync()
}
