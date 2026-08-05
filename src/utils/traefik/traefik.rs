use super::middleware::Middleware;
use super::router::RouterBuilder;
use super::service::ServiceBuilder;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct TraefikBuilder {
    pub(crate) labels: BTreeMap<String, String>,
}

impl TraefikBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(mut self) -> Self {
        self.labels.insert("traefik.enable".into(), "true".into());
        self
    }

    pub fn network(mut self, network: impl Into<String>) -> Self {
        self.labels
            .insert("traefik.docker.network".into(), network.into());
        self
    }

    /// Start building a named router, returning a [`RouterBuilder`] for
    /// chaining router-specific label methods.
    pub fn router(self, name: impl Into<String>) -> RouterBuilder {
        RouterBuilder {
            builder: self,
            name: name.into(),
        }
    }

    /// Start building a named service, returning a [`ServiceBuilder`] for
    /// chaining service-specific label methods.
    pub fn service(self, name: impl Into<String>) -> ServiceBuilder {
        ServiceBuilder {
            builder: self,
            name: name.into(),
        }
    }

    /// Attach all Traefik labels produced by the given [`Middleware`] directly
    /// into this builder's label map.
    ///
    /// This lets you register middleware definitions (e.g. `stripPrefix`,
    /// `addPrefix`) alongside router/service labels in one pass.
    pub fn middleware(mut self, mw: Middleware) -> Self {
        for (key, value) in mw.labels() {
            self.labels.insert(key, value);
        }
        self
    }

    /// Merge another completed Traefik builder into this one while preserving
    /// the one-label-per-key invariant.
    pub fn append(&mut self, other: TraefikBuilder) {
        self.labels.extend(other.labels);
    }

    /// Attach a pre-built [`RouterBuilder`]'s labels into this builder and
    /// return `self` for continued chaining.
    ///
    /// Useful when you want to configure a router inline and then keep adding
    /// more routers or services.
    pub fn add_router(mut self, router: RouterBuilder) -> Self {
        // Merge all labels from the sub-builder, overwriting on collision.
        self.labels.extend(router.builder.labels);
        self
    }

    /// Attach a pre-built [`ServiceBuilder`]'s labels into this builder and
    /// return `self` for continued chaining.
    pub fn add_service(mut self, service: ServiceBuilder) -> Self {
        self.labels.extend(service.builder.labels);
        self
    }

    /// Render all accumulated labels as `key=value` strings suitable for
    /// Docker Swarm stack files or `docker run --label` arguments.
    pub fn build(self) -> Vec<String> {
        self.labels
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect()
    }
}
