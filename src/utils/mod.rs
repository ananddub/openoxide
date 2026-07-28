pub mod backup;
pub mod builder;
pub mod cgroup;
pub mod docker;
pub mod exec;
pub mod git;
pub mod jwt;
pub mod macros;
pub mod paths;
pub mod provider;
pub mod rclone;
pub mod setup;
pub mod ssh;
pub mod traefik;
pub mod zip;
// pub mod k8s;
pub mod os;

#[tokio::test]
async fn test_docker() {
    use crate::utils::docker::DockerCli;
    let docker = DockerCli::new_local();
    let data = docker.containers().inspect("b671b4542569").await.unwrap();
    println!("{:?}", data);
}
