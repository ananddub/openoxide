pub mod ai;
pub mod background;
pub mod backup;
pub mod builder;
pub use os::exec;
pub use os::{cgroup, docker, git, rclone, ssh};
pub mod jwt;
pub use os::macros;
pub mod paths;
pub mod provider;
pub mod setup;
pub mod traefik;
pub mod upload;
pub mod watch_paths;
// pub mod k8s;
pub use os;

#[tokio::test]
async fn test_docker() {
    use crate::utils::docker::DockerCli;
    let docker = DockerCli::new_local();
    let data = match docker.container("b671b4542569").inspect().await {
        Ok(data) => data,
        Err(error) => {
            let detail = error.to_string();
            if detail.contains("permission denied") || detail.contains("Cannot connect") {
                eprintln!("skipping docker integration test: {detail}");
                return;
            }
            panic!("docker integration test failed: {detail}");
        }
    };
    println!("{:?}", data);
}
