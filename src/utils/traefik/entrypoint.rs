use std::fmt;

/// A reference to a Traefik entrypoint name, as defined in the static
/// configuration (e.g. `--entrypoints.web.address=:80`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entrypoint {
    Web,
    WebSecure,
    Custom(String),
}

impl Entrypoint {
    pub fn custom(name: impl Into<String>) -> Self {
        Entrypoint::Custom(name.into())
    }

    pub fn name(&self) -> &str {
        match self {
            Entrypoint::Web => "web",
            Entrypoint::WebSecure => "websecure",
            Entrypoint::Custom(name) => name,
        }
    }
}

impl fmt::Display for Entrypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(Entrypoint::Web.name(), "web");
        assert_eq!(Entrypoint::WebSecure.name(), "websecure");
        assert_eq!(Entrypoint::custom("metrics").name(), "metrics");
    }
}
