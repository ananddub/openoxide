use super::tls::TlsConfig;
use super::traefik::TraefikBuilder;

pub struct RouterBuilder {
    pub(crate) builder: TraefikBuilder,
    pub(crate) name: String,
}

impl RouterBuilder {
    fn key(&self, suffix: &str) -> String {
        format!("traefik.http.routers.{}.{}", self.name, suffix)
    }

    /// Set the routing rule (accepts anything that displays as a Traefik rule
    /// string — including the [`Rule`](super::rule::Rule) type).
    pub fn rule(mut self, value: impl ToString) -> Self {
        self.builder
            .labels
            .insert(self.key("rule"), value.to_string());
        self
    }

    /// Set multiple entrypoints (joined with `,`).
    pub fn entrypoints<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.builder.labels.insert(
            self.key("entrypoints"),
            values
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }

    /// Convenience shorthand for a single entrypoint.
    pub fn entrypoint(self, value: impl ToString) -> Self {
        self.entrypoints([value])
    }

    /// Enable (or disable) TLS on this router.
    pub fn tls(mut self, enabled: bool) -> Self {
        self.builder
            .labels
            .insert(self.key("tls"), enabled.to_string());
        self
    }

    /// Set the TLS cert resolver for this router.
    pub fn cert_resolver(mut self, resolver: impl Into<String>) -> Self {
        self.builder
            .labels
            .insert(self.key("tls.certresolver"), resolver.into());
        self
    }

    pub fn tls_config(mut self, config: TlsConfig) -> Self {
        self.builder.labels.insert(self.key("tls"), "true".into());
        if let Some(resolver) = config.cert_resolver {
            self.builder
                .labels
                .insert(self.key("tls.certresolver"), resolver);
        }
        self
    }

    /// Set multiple middlewares (joined with `,`).
    pub fn middlewares<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.builder.labels.insert(
            self.key("middlewares"),
            values
                .into_iter()
                .map(|v| v.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }

    /// Convenience shorthand for a single middleware reference.
    pub fn middleware(self, value: impl AsRef<str>) -> Self {
        self.middlewares([value.as_ref().to_owned()])
    }

    /// Finalize this router by setting its backing service name and returning
    /// the parent [`TraefikBuilder`] for continued chaining.
    pub fn service(mut self, service: impl Into<String>) -> TraefikBuilder {
        self.builder
            .labels
            .insert(self.key("service"), service.into());
        self.builder
    }

    /// Finalize this router without a service override.  Returns the parent
    /// [`TraefikBuilder`] so you can chain more routers / services.
    pub fn finish(self) -> TraefikBuilder {
        self.builder
    }
}
