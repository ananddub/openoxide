use super::{
    container::{ContainerCreate, ContainerQuery},
    image::ImageQuery,
    network::{NetworkCreate, NetworkQuery},
    service::{ServiceQuery, ServiceUpdate},
    volume::{VolumeCreate, VolumeQuery},
};
use crate::utils::docker::DockerCli;

/// Root entry point for the typesafe Docker query builder.
///
/// Obtain one via [`DockerCli::query()`].
///
/// # Example
/// ```rust,no_run
/// use crate::utils::docker::query::filter::{ContainerFilter, ContainerStatus};
///
/// let running = docker.query()
///     .containers()
///     .all()
///     .filter(ContainerFilter::Status(ContainerStatus::Running))
///     .filter(ContainerFilter::label("app", "api"))
///     .list()
///     .await?;
/// ```
pub struct DockerQuery<'a> {
    pub(super) cli: &'a DockerCli,
}

impl<'a> DockerQuery<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    // ── Container ─────────────────────────────────────────────────────────────

    /// Build a `docker container ls` query.
    pub fn containers(self) -> ContainerQuery<'a> {
        ContainerQuery::new(self.cli)
    }

    /// Build a `docker container create` / `docker container run` command.
    pub fn create_container(self, image: impl Into<String>) -> ContainerCreate<'a> {
        ContainerCreate::new(self.cli, image)
    }

    // ── Image ─────────────────────────────────────────────────────────────────

    /// Build a `docker image ls` query.
    pub fn images(self) -> ImageQuery<'a> {
        ImageQuery::new(self.cli)
    }

    // ── Service (Swarm) ───────────────────────────────────────────────────────

    /// Build a `docker service ls` query.
    pub fn services(self) -> ServiceQuery<'a> {
        ServiceQuery::new(self.cli)
    }

    /// Build a `docker service update` command for the named service.
    pub fn update_service(self, name: impl Into<String>) -> ServiceUpdate<'a> {
        ServiceUpdate::new(self.cli, name)
    }

    // ── Network ───────────────────────────────────────────────────────────────

    /// Build a `docker network ls` query.
    pub fn networks(self) -> NetworkQuery<'a> {
        NetworkQuery::new(self.cli)
    }

    /// Build a `docker network create` command.
    pub fn create_network(self, name: impl Into<String>) -> NetworkCreate<'a> {
        NetworkCreate::new(self.cli, name)
    }

    // ── Volume ────────────────────────────────────────────────────────────────

    /// Build a `docker volume ls` query.
    pub fn volumes(self) -> VolumeQuery<'a> {
        VolumeQuery::new(self.cli)
    }

    /// Build a `docker volume create` command.
    pub fn create_volume(self, name: impl Into<String>) -> VolumeCreate<'a> {
        VolumeCreate::new(self.cli, name)
    }
}
