use super::*;
use crate::docker::{
    DockerCli,
    core::{Cpu, Memory, Mount, Platform, Port},
    query::filter::{ContainerFilter, ContainerStatus},
};

fn cli() -> DockerCli {
    DockerCli::new_local()
}

#[test]
fn container_create_args() {
    let temp = cli();
    let c = ContainerCreate::new(&temp, "nginx:latest")
        .name("web")
        .network("bridge")
        .publish(Port::tcp(8080, 80))
        .mount(Mount::volume("data", "/data"))
        .env("PORT", "80")
        .memory(Memory::mb(256))
        .cpus(Cpu::cores(0.5))
        .platform(Platform::LinuxAmd64)
        .restart(RestartPolicy::UnlessStopped)
        .tty(false)
        .detach()
        .privileged()
        .cap_add("NET_ADMIN")
        .add_host("host.docker.internal:host-gateway")
        .dns("8.8.8.8")
        .init();

    let preview = c.print_run();
    assert!(preview.contains("container run"));
    assert!(preview.contains("--name web"));
    assert!(preview.contains("8080:80/tcp"));
    assert!(preview.contains("256m"));
    assert!(preview.contains("0.50"));
    assert!(preview.contains("linux/amd64"));
    assert!(preview.contains("--privileged"));
    assert!(preview.contains("--cap-add NET_ADMIN"));
    assert!(preview.contains("--add-host host.docker.internal:host-gateway"));
    assert!(preview.contains("--dns 8.8.8.8"));
    assert!(preview.contains("--init"));
    assert!(!preview.contains("--tty"));
}

#[test]
fn container_create_tty_enabled() {
    let temp = cli();
    let c = ContainerCreate::new(&temp, "alpine").tty(true);
    assert!(c.print_run().contains("--tty"));
}

#[test]
fn restart_policy() {
    assert_eq!(RestartPolicy::OnFailure(3).to_string(), "on-failure:3");
    assert_eq!(RestartPolicy::UnlessStopped.to_string(), "unless-stopped");
}

#[test]
fn container_query_print() {
    let tmp = cli();
    let q = ContainerQuery::new(&tmp)
        .all()
        .filter(ContainerFilter::Status(ContainerStatus::Running));
    assert!(q.print().contains("--all"));
    assert!(q.print().contains("status=running"));
}
