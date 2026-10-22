#[derive(Debug, Clone)]
pub enum Middleware {
    StripPrefix {
        name: String,
        prefixes: Vec<String>,
    },

    AddPrefix {
        name: String,
        prefix: String,
    },

    RedirectScheme {
        name: String,
        scheme: String,
        permanent: bool,
    },

    RedirectRegex {
        name: String,
        regex: String,
        replacement: String,
        permanent: bool,
    },

    BasicAuth {
        name: String,
        users: Vec<(String, String)>,
    },

    Compress {
        name: String,
    },

    RequestHeaders {
        name: String,
        headers: Vec<(String, String)>,
    },

    ResponseHeaders {
        name: String,
        headers: Vec<(String, String)>,
    },

    RateLimit {
        name: String,
        average: i64,
        burst: i64,
    },

    IpAllowList {
        name: String,
        source_ranges: Vec<String>,
    },

    Custom {
        name: String,
    },
}

impl Middleware {
    pub fn labels(&self) -> Vec<(String, String)> {
        match self {
            Middleware::StripPrefix { name, prefixes } => vec![(
                format!("traefik.http.middlewares.{name}.stripprefix.prefixes"),
                prefixes.join(","),
            )],

            Middleware::AddPrefix { name, prefix } => vec![(
                format!("traefik.http.middlewares.{name}.addprefix.prefix"),
                prefix.clone(),
            )],

            Middleware::RedirectScheme {
                name,
                scheme,
                permanent,
            } => vec![
                (
                    format!("traefik.http.middlewares.{name}.redirectscheme.scheme"),
                    scheme.clone(),
                ),
                (
                    format!("traefik.http.middlewares.{name}.redirectscheme.permanent"),
                    permanent.to_string(),
                ),
            ],

            Middleware::RedirectRegex {
                name,
                regex,
                replacement,
                permanent,
            } => vec![
                (
                    format!("traefik.http.middlewares.{name}.redirectregex.regex"),
                    regex.clone(),
                ),
                (
                    format!("traefik.http.middlewares.{name}.redirectregex.replacement"),
                    replacement.clone(),
                ),
                (
                    format!("traefik.http.middlewares.{name}.redirectregex.permanent"),
                    permanent.to_string(),
                ),
            ],

            Middleware::BasicAuth { name, users } => vec![(
                format!("traefik.http.middlewares.{name}.basicauth.users"),
                users
                    .iter()
                    .map(|(username, password)| {
                        format!("{username}:{}", password.replace('$', "$$"))
                    })
                    .collect::<Vec<_>>()
                    .join(","),
            )],

            Middleware::Compress { name } => vec![(
                format!("traefik.http.middlewares.{name}.compress"),
                "true".into(),
            )],

            Middleware::RequestHeaders { name, headers } => headers
                .iter()
                .map(|(k, v)| {
                    (
                        format!("traefik.http.middlewares.{name}.headers.customrequestheaders.{k}"),
                        v.clone(),
                    )
                })
                .collect(),

            Middleware::ResponseHeaders { name, headers } => headers
                .iter()
                .map(|(k, v)| {
                    (
                        format!(
                            "traefik.http.middlewares.{name}.headers.customresponseheaders.{k}"
                        ),
                        v.clone(),
                    )
                })
                .collect(),

            Middleware::RateLimit {
                name,
                average,
                burst,
            } => vec![
                (
                    format!("traefik.http.middlewares.{name}.ratelimit.average"),
                    average.to_string(),
                ),
                (
                    format!("traefik.http.middlewares.{name}.ratelimit.burst"),
                    burst.to_string(),
                ),
            ],

            Middleware::IpAllowList {
                name,
                source_ranges,
            } => vec![(
                format!("traefik.http.middlewares.{name}.ipallowlist.sourcerange"),
                source_ranges.join(","),
            )],

            Middleware::Custom { .. } => vec![],
        }
    }

    pub fn reference(&self) -> String {
        match self {
            Middleware::StripPrefix { name, .. } => name.clone(),
            Middleware::AddPrefix { name, .. } => name.clone(),
            Middleware::RedirectScheme { name, .. } => name.clone(),
            Middleware::RedirectRegex { name, .. } => name.clone(),
            Middleware::BasicAuth { name, .. } => name.clone(),
            Middleware::Compress { name } => name.clone(),
            Middleware::RequestHeaders { name, .. } => name.clone(),
            Middleware::ResponseHeaders { name, .. } => name.clone(),
            Middleware::RateLimit { name, .. } => name.clone(),
            Middleware::IpAllowList { name, .. } => name.clone(),
            Middleware::Custom { name } => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Middleware;

    #[test]
    fn application_middlewares_render_typed_labels() {
        let cases = [
            Middleware::RedirectRegex {
                name: "redirect".into(),
                regex: "^http://old/(.*)".into(),
                replacement: "https://new/$1".into(),
                permanent: true,
            },
            Middleware::BasicAuth {
                name: "auth".into(),
                users: vec![("admin".into(), "$2b$12$hash".into())],
            },
            Middleware::RateLimit {
                name: "limit".into(),
                average: 100,
                burst: 20,
            },
            Middleware::IpAllowList {
                name: "allow".into(),
                source_ranges: vec!["10.0.0.0/8".into()],
            },
        ];
        let labels = cases
            .into_iter()
            .flat_map(|middleware| middleware.labels())
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            labels["traefik.http.middlewares.redirect.redirectregex.permanent"],
            "true"
        );
        assert_eq!(
            labels["traefik.http.middlewares.auth.basicauth.users"],
            "admin:$$2b$$12$$hash"
        );
        assert_eq!(
            labels["traefik.http.middlewares.limit.ratelimit.average"],
            "100"
        );
        assert_eq!(
            labels["traefik.http.middlewares.allow.ipallowlist.sourcerange"],
            "10.0.0.0/8"
        );
    }

    #[test]
    fn request_and_response_headers_are_distinct() {
        let request = Middleware::RequestHeaders {
            name: "request".into(),
            headers: vec![("X-Request".into(), "one".into())],
        };
        let response = Middleware::ResponseHeaders {
            name: "response".into(),
            headers: vec![("X-Response".into(), "two".into())],
        };

        assert!(request.labels()[0].0.contains("customrequestheaders"));
        assert!(response.labels()[0].0.contains("customresponseheaders"));
    }
}
