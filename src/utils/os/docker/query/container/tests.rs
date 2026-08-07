use super::create::ContainerCreate;
use super::types::RestartPolicy;
use crate::utils::docker::DockerCli;

#[test]
fn restart_policy_display() {
    assert_eq!(RestartPolicy::No.to_string(), "no");
    assert_eq!(RestartPolicy::Always.to_string(), "always");
    assert_eq!(RestartPolicy::OnFailure(5).to_string(), "on-failure:5");
    assert_eq!(RestartPolicy::UnlessStopped.to_string(), "unless-stopped");
}

#[test]
fn create_builds_args_correctly() {
    let cli = DockerCli::new_local();
    let builder = ContainerCreate::new(&cli, "nginx:latest")
        .name("web")
        .env("PORT", "80")
        .label("app", "web")
        .network("bridge")
        .publish(8080, 80)
        .volume("web-data", "/data")
        .bind_ro("/etc/nginx/nginx.conf", "/etc/nginx/nginx.conf")
        .memory("256m")
        .restart(RestartPolicy::UnlessStopped)
        .tty(false)
        .interactive()
        .detach()
        .privileged()
        .cap_add("NET_ADMIN")
        .add_host("host.docker.internal:host-gateway")
        .dns("8.8.8.8")
        .init();

    let opts = builder.build_opts();
    assert!(opts.contains(&"--name".to_string()));
    assert!(opts.contains(&"web".to_string()));
    assert!(opts.contains(&"--env".to_string()));
    assert!(opts.contains(&"PORT=80".to_string()));
    assert!(opts.contains(&"8080:80/tcp".to_string()));
    assert!(opts.contains(&"--restart".to_string()));
    assert!(opts.contains(&"unless-stopped".to_string()));
    assert!(opts.contains(&"--privileged".to_string()));
    assert!(opts.contains(&"--cap-add".to_string()));
    assert!(opts.contains(&"NET_ADMIN".to_string()));
    assert!(opts.contains(&"--add-host".to_string()));
    assert!(opts.contains(&"host.docker.internal:host-gateway".to_string()));
    assert!(opts.contains(&"--dns".to_string()));
    assert!(opts.contains(&"8.8.8.8".to_string()));
    assert!(opts.contains(&"--init".to_string()));
    assert!(!opts.contains(&"--tty".to_string()));
    assert!(opts.contains(&"--interactive".to_string()));
    assert!(opts.contains(&"--detach".to_string()));
    assert_eq!(opts.last(), Some(&"nginx:latest".to_string()));
}

#[test]
fn tty_enabled_when_true() {
    let cli = DockerCli::new_local();
    let opts = ContainerCreate::new(&cli, "alpine").tty(true).build_opts();
    assert!(opts.contains(&"--tty".to_string()));
}
