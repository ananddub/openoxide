pub mod ai;
pub mod background;
pub mod backup;
pub mod builder;
pub use os::{cgroup, docker, git, rclone, ssh};
pub mod exec;
pub mod jwt;
pub mod macros;
pub mod paths;
pub mod provider;
pub mod setup;
pub mod traefik;
pub mod watch_paths;
// pub mod k8s;
pub mod os;

#[tokio::test]
async fn test_docker() {
    use crate::utils::docker::DockerCli;
    let docker = DockerCli::new_local();
    let data = docker.container("b671b4542569").inspect().await.unwrap();
    println!("{:?}", data);
}
