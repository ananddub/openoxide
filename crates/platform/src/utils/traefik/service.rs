use super::builder::TraefikBuilder;

pub struct ServiceBuilder {
    pub(crate) builder: TraefikBuilder,
    pub(crate) name: String,
}

impl ServiceBuilder {
    fn key(&self, suffix: &str) -> String {
        format!("traefik.http.services.{}.{}", self.name, suffix)
    }

    pub fn port(mut self, port: u16) -> Self {
        self.builder
            .labels
            .insert(self.key("loadbalancer.server.port"), port.to_string());
        self
    }

    pub fn scheme(mut self, scheme: impl Into<String>) -> Self {
        self.builder
            .labels
            .insert(self.key("loadbalancer.server.scheme"), scheme.into());
        self
    }

    pub fn finish(self) -> TraefikBuilder {
        self.builder
    }
}
