use super::container::{ContainerFilter, ContainerStatus, HealthStatus};
use super::image::ImageFilter;
use super::network::NetworkFilter;
use super::service::{ServiceFilter, ServiceMode};
use super::volume::VolumeFilter;
use crate::docker::{NetworkScope, NetworkType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_filter_display() {
        assert_eq!(
            ContainerFilter::Status(ContainerStatus::Running).to_string(),
            "status=running"
        );
        assert_eq!(
            ContainerFilter::Health(HealthStatus::Healthy).to_string(),
            "health=healthy"
        );
        assert_eq!(
            ContainerFilter::label("app", "api").to_string(),
            "label=app=api"
        );
        assert_eq!(
            ContainerFilter::label_key("traefik.enable").to_string(),
            "label=traefik.enable"
        );
        assert_eq!(ContainerFilter::Exited(0).to_string(), "exited=0");
        assert_eq!(
            ContainerFilter::ancestor("nginx:latest").to_string(),
            "ancestor=nginx:latest"
        );
    }

    #[test]
    fn image_filter_display() {
        assert_eq!(
            ImageFilter::reference("api:*").to_string(),
            "reference=api:*"
        );
        assert_eq!(ImageFilter::Dangling(true).to_string(), "dangling=true");
    }

    #[test]
    fn service_filter_display() {
        assert_eq!(
            ServiceFilter::Mode(ServiceMode::Replicated).to_string(),
            "mode=replicated"
        );
        assert_eq!(
            ServiceFilter::name("myservice").to_string(),
            "name=myservice"
        );
    }

    #[test]
    fn network_filter_display() {
        assert_eq!(
            NetworkFilter::Scope(NetworkScope::Swarm).to_string(),
            "scope=swarm"
        );
        assert_eq!(
            NetworkFilter::Type(NetworkType::Custom).to_string(),
            "type=custom"
        );
    }

    #[test]
    fn volume_filter_display() {
        assert_eq!(VolumeFilter::Dangling(false).to_string(), "dangling=false");
        assert_eq!(VolumeFilter::driver("local").to_string(), "driver=local");
    }
}
