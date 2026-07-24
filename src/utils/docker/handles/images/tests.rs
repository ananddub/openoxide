use super::*;
use crate::utils::docker::{DockerCli, core::Platform};

fn cli() -> DockerCli {
    DockerCli::new_local()
}

#[test]
fn build_preview() {
    let tmp = cli();
    let b = BuildBuilder::new(&tmp, ".")
        .tag("api:latest")
        .target("release")
        .build_arg("ENV", "prod")
        .no_cache()
        .platform(Platform::LinuxArm64);
    let p = b.print();
    assert!(p.contains("image build"));
    assert!(p.contains("--tag api:latest"));
    assert!(p.contains("--target release"));
    assert!(p.contains("ENV=prod"));
    assert!(p.contains("--no-cache"));
    assert!(p.contains("linux/arm64"));
    assert!(p.ends_with("."));
}

#[test]
fn pull_preview() {
    let p = PullBuilder::new(&cli(), "nginx:latest")
        .platform(Platform::LinuxAmd64)
        .print();
    assert!(p.contains("image pull"));
    assert!(p.contains("linux/amd64"));
    assert!(p.ends_with("nginx:latest"));
}
