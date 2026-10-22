use serde::{Deserialize, Serialize};

use crate::utils::traefik::middleware::Middleware;

use super::types::{StructuredMiddlewareDto, StructuredMiddlewareResponseDto};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StructuredMiddlewareKind {
    StripPrefix,
    AddPrefix,
    RedirectScheme,
    Compress,
    RateLimit,
    IpAllowList,
    RequestHeaders,
    ResponseHeaders,
}

pub fn render_structured_middleware(
    input: StructuredMiddlewareDto,
) -> Result<StructuredMiddlewareResponseDto, String> {
    let name = input.name.trim().to_owned();
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
    {
        return Err("invalid middleware name".into());
    }
    let value = |key: &str| {
        input
            .values
            .get(key)
            .cloned()
            .ok_or_else(|| format!("missing middleware value: {key}"))
    };
    let middleware = match input.kind {
        StructuredMiddlewareKind::StripPrefix => Middleware::StripPrefix {
            name,
            prefixes: input.list,
        },
        StructuredMiddlewareKind::AddPrefix => Middleware::AddPrefix {
            name,
            prefix: value("prefix")?,
        },
        StructuredMiddlewareKind::RedirectScheme => Middleware::RedirectScheme {
            name,
            scheme: value("scheme")?,
            permanent: input.values.get("permanent").is_some_and(|v| v == "true"),
        },
        StructuredMiddlewareKind::Compress => Middleware::Compress { name },
        StructuredMiddlewareKind::RateLimit => Middleware::RateLimit {
            name,
            average: value("average")?.parse().map_err(|_| "invalid average")?,
            burst: value("burst")?.parse().map_err(|_| "invalid burst")?,
        },
        StructuredMiddlewareKind::IpAllowList => Middleware::IpAllowList {
            name,
            source_ranges: input.list,
        },
        StructuredMiddlewareKind::RequestHeaders => Middleware::RequestHeaders {
            name,
            headers: input.values.into_iter().collect(),
        },
        StructuredMiddlewareKind::ResponseHeaders => Middleware::ResponseHeaders {
            name,
            headers: input.values.into_iter().collect(),
        },
    };
    Ok(StructuredMiddlewareResponseDto {
        reference: middleware.reference(),
        labels: middleware.labels().into_iter().collect(),
    })
}
