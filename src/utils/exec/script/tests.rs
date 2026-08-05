use crate::utils::exec::Condition;
use crate::utils::exec::ScriptPipeline;

use super::*;
use crate::pipeline;
use crate::utils::exec::{CommandExecutor, LocalExecutor};
use crate::utils::rclone::{RcloneBuilder, RcloneCommand, RcloneTarget};
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn test_pipeline_execution() {
    let executor = CommandExecutor::Local(LocalExecutor::new());

    let pipeline = ScriptPipeline::new()
        .cmd("echo 'line 1'")
        .cmd("echo 'line 2'");

    let output = pipeline.execute(&executor).await.unwrap();
    assert!(output.status.success());
    assert!(output.stdout.contains("line 1"));
    assert!(output.stdout.contains("line 2"));
}

#[tokio::test]
async fn test_pipeline_with_rclone_compilation() {
    let src = RcloneTarget::Local {
        path: "/tmp/src.txt".to_string(),
    };
    let dest = RcloneTarget::Local {
        path: "/tmp/dest.txt".to_string(),
    };
    let rclone = RcloneBuilder::new(RcloneCommand::Copyto)
        .source(src)
        .destination(dest);

    let pipeline = ScriptPipeline::new()
        .cmd("echo 'starting rclone'")
        .cmd(rclone)
        .cmd("echo 'finished rclone'");

    let script = pipeline.compile();
    assert!(script.contains("set -e"));
    assert!(script.contains("starting rclone"));
    assert!(script.contains("rclone copyto /tmp/src.txt /tmp/dest.txt"));
    assert!(script.contains("finished rclone"));
}

#[tokio::test]
async fn test_pipeline_with_various_builders() {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    let cli = crate::utils::docker::DockerCli::new_local();

    // Bind the handles to local variables to extend their lifetimes
    let containers_handle = cli.containers();
    let images_handle = cli.images();
    let compose_handle = cli.compose();
    let services_handle = cli.services();
    let stacks_handle = cli.stacks();

    // 1. ContainerCreate builder
    let container = containers_handle
        .create("alpine:latest")
        .name("test-pipeline-container")
        .tty(true);

    // 2. ContainerStartBuilder
    let start = containers_handle.start("test-pipeline-container");

    // 3. NixpacksBuildBuilder
    let nixpacks_cli = crate::utils::builder::packs::nixpacks::NixpacksCli::new(&executor);
    let nixpacks = nixpacks_cli.build("/path/to/project").name("my-app");

    // 4. Image BuildBuilder
    let img_build = images_handle.build("/path/to/ctx").tag("my-image:latest");

    // 5. Compose UpBuilder
    let compose_up = compose_handle.up().detach().build();

    // 6. Service CreateBuilder
    let svc_create = services_handle.create("nginx:latest").name("my-service");

    // 7. Stack DeployBuilder
    let stack_deploy = stacks_handle
        .deploy("my-stack")
        .compose_file("docker-compose.yml");

    // Pipeline combining all
    let pipeline = ScriptPipeline::new()
        .cmd(container)
        .cmd(start)
        .cmd(nixpacks)
        .cmd(img_build)
        .cmd(compose_up)
        .cmd(svc_create)
        .cmd(stack_deploy);

    let script = pipeline.compile();
    assert!(
        script.contains("docker container run --name test-pipeline-container --tty alpine:latest")
    );
    assert!(script.contains("docker container start test-pipeline-container"));
    assert!(script.contains("nixpacks build /path/to/project --name my-app"));
    assert!(script.contains("docker image build --tag my-image:latest /path/to/ctx"));
    assert!(script.contains("docker compose up --detach --build"));
    assert!(script.contains("docker service create --name my-service nginx:latest"));
    assert!(script.contains("docker stack deploy --compose-file docker-compose.yml my-stack"));
}

#[tokio::test]
async fn test_pipeline_advanced_features() {
    let pipeline = ScriptPipeline::new()
        .working_dir("/var/www/app")
        .env("ENV_KEY", "env_val")
        .trace(true)
        .verbose_headers(true)
        .cmd("echo 'Step 1'")
        .and("echo 'Runs only if Step 1 succeeded'")
        .or("echo 'Runs only if Step 1 failed'")
        .pipe("grep -i step");

    let script = pipeline.compile();
    assert!(script.contains("set -x"));
    assert!(script.contains("export ENV_KEY='env_val'"));
    assert!(script.contains("cd '/var/www/app'"));
    assert!(script.contains("echo '=== [Step 1] ==='"));
    assert!(script.contains("echo 'Step 1' && echo 'Runs only if Step 1 succeeded' || echo 'Runs only if Step 1 failed' | grep -i step"));
}

#[tokio::test]
async fn test_pipeline_if_else() {
    let then_branch = ScriptPipeline::new()
        .cmd("echo 'directory exists'")
        .cmd("ls -la");

    let else_branch = ScriptPipeline::new()
        .cmd("echo 'directory does not exist'")
        .cmd("mkdir -p /tmp/test-dir");

    let pipeline =
        ScriptPipeline::new().if_else("[ -d /tmp/test-dir ]", then_branch, Some(else_branch));

    let script = pipeline.compile();
    assert!(script.contains("if [ -d /tmp/test-dir ]; then"));
    assert!(script.contains("    echo 'directory exists'"));
    assert!(script.contains("    ls -la"));
    assert!(script.contains("else"));
    assert!(script.contains("    echo 'directory does not exist'"));
    assert!(script.contains("    mkdir -p /tmp/test-dir"));
    assert!(script.contains("fi"));
}

#[tokio::test]
async fn test_pipeline_git_builders() {
    let git_cli = crate::utils::git::client::GitCli::new_local().with_repository("/tmp/repo");

    let clone = git_cli
        .clone("https://github.com/test/repo.git")
        .destination("/tmp/repo");
    let fetch = git_cli.fetch().all();
    let reset = git_cli.reset().hard().commit("origin/main");

    let pipeline = ScriptPipeline::new().cmd(clone).cmd(fetch).cmd(reset);

    let script = pipeline.compile();
    assert!(script.contains("git clone https://github.com/test/repo.git /tmp/repo"));
    assert!(script.contains("git -c safe.directory=/tmp/repo -C /tmp/repo fetch --all"));
    assert!(
        script.contains("git -c safe.directory=/tmp/repo -C /tmp/repo reset --hard origin/main")
    );
}

#[tokio::test]
async fn test_pipeline_convenience_conditions() {
    let then_branch = ScriptPipeline::new().cmd("echo 'then'");
    let else_branch = ScriptPipeline::new().cmd("echo 'else'");

    let pipeline = ScriptPipeline::new()
        .if_dir_exists(
            "/tmp/test-dir",
            then_branch.clone(),
            Some(else_branch.clone()),
        )
        .if_file_exists(
            "/tmp/test-file",
            then_branch.clone(),
            Some(else_branch.clone()),
        )
        .if_cmd_succeeds("git status", then_branch.clone(), Some(else_branch.clone()))
        .if_env_set("MY_VAR", then_branch, Some(else_branch));

    let script = pipeline.compile();
    assert!(script.contains("if [ -d '/tmp/test-dir' ]; then"));
    assert!(script.contains("if [ -f '/tmp/test-file' ]; then"));
    assert!(script.contains("if git status; then"));
    assert!(script.contains("if [ -n \"${MY_VAR}\" ]; then"));
}

#[tokio::test]
async fn test_pipeline_condition_enum_and_fluent_api() {
    let cond = Condition::dir_exists("/tmp/repo")
        & Condition::file_exists("/tmp/repo/config.json")
        & Condition::cmd_succeeds("git status")
        | Condition::env_set("FORCE_DEPLOY").not();

    let pipeline = ScriptPipeline::new()
        .if_condition(cond)
        .then(|p| p.cmd("echo 'deploying'").cmd("cargo build"))
        .otherwise(|p| p.cmd("echo 'skipping'"))
        .cmd("echo 'done'");
    let script = pipeline.compile();
    assert!(script.contains("if ([ -d '/tmp/repo' ] && [ -f '/tmp/repo/config.json' ] && git status) || ! [ -n \"${FORCE_DEPLOY}\" ]; then"));
    assert!(script.contains("    echo 'deploying'"));
    assert!(script.contains("    cargo build"));
    assert!(script.contains("else"));
    assert!(script.contains("    echo 'skipping'"));
    assert!(script.contains("fi"));
    assert!(script.contains("echo 'done'"));
}
#[tokio::test]
async fn test_pipeline_condition_enum_and_fluent_macros() {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    let nixpacks_cli = crate::utils::builder::packs::nixpacks::NixpacksCli::new(&executor);
    let git_cli = crate::utils::git::client::GitCli::new_local().with_repository("/tmp/repo");

    let pipeline = pipeline! {
        if (dir("/tmp/repo") && file("/tmp/repo/config.json")&& cmd("git status")|| !env("FORCE_DEPLOY")) {
            echo("deploying");
            cargo("build");
        } else {
            nixpacks_cli.build("/path/to/project").name("my-app");
            git_cli.clone("").destination("/tmp/repo");
            echo("skipping");
        }
        echo("done");
    };

    let script = pipeline.compile();
    assert!(script.contains("if ([ -d '/tmp/repo' ] && [ -f '/tmp/repo/config.json' ] && git status) || ! [ -n \"${FORCE_DEPLOY}\" ]; then"));
    assert!(script.contains("    echo 'deploying'"));
    assert!(script.contains("    cargo 'build'"));
    assert!(script.contains("else"));
    assert!(script.contains("    nixpacks build /path/to/project --name my-app"));
    assert!(script.contains("    echo 'skipping'"));
    assert!(script.contains("fi"));
    assert!(script.contains("echo 'done'"));
}

#[tokio::test]
async fn test_direct_builder_execution() {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    let cmd = "echo 'direct execution works'";

    let out = cmd.execute(&executor).await.unwrap();
    assert!(out.status.success());
    assert!(out.stdout.contains("direct execution works"));

    let cancel = CancellationToken::new();
    let out_cancelled = cmd.execute_cancelled(&executor, &cancel).await.unwrap();
    assert!(out_cancelled.status.success());
    assert!(out_cancelled.stdout.contains("direct execution works"));
}

#[tokio::test]
async fn test_pipeline_array_and_vector_commands() {
    let pipeline = pipeline! {
        cmd(&["echo", "hello", "world"]);
        cmd(vec!["echo", "foo", "bar"]);
    };

    let script = pipeline.compile();
    assert!(script.contains("'echo' 'hello' 'world'"));
    assert!(script.contains("'echo' 'foo' 'bar'"));
}

#[test]
fn test_shell_ir_compilation() {
    use super::dsl::{ArgToken, CaptureSource, Command, ShellIR, Statement};

    let systemctl_cmd = ShellIR::Command(Command {
        name: "systemctl".to_string(),
        args: vec![
            ArgToken::Literal("is-active".to_string()),
            ArgToken::Literal("sshd".to_string()),
        ],
    });

    assert_eq!(systemctl_cmd.to_bash(), "systemctl 'is-active' 'sshd'");

    let redirect =
        systemctl_cmd
            .clone()
            .stdout(crate::utils::exec::script::dsl::OutputTarget::file(
                "/var/log/active.log",
            ));
    assert_eq!(
        redirect.to_bash(),
        "systemctl 'is-active' 'sshd' > '/var/log/active.log'"
    );

    let assign = ShellIR::Statement(Statement::VarAssign {
        name: "active".to_string(),
        val: Box::new(ShellIR::Capture {
            cmd: Box::new(systemctl_cmd),
            source: CaptureSource::Status,
        }),
        default: None,
    });

    assert_eq!(
        assign.to_bash(),
        "active=$(if systemctl 'is-active' 'sshd'; then echo true; else echo false; fi)"
    );
}

#[test]
fn test_sh_macro_compilation() {
    use super::sh;

    let script_ir = sh!(
        let active = any![
            systemctl!("is-active", "--quiet", "sshd"),
            systemctl!("is-active", "--quiet", "ssh")
        ];

        if active {
            let config = capture_stdout! {
                sudo("sshd", "-T");
            }.default("/etc/ssh/sshd_config");
        }
    );

    assert_eq!(script_ir.len(), 2);
    assert!(script_ir[0].to_bash().contains("active="));
    assert!(script_ir[0].to_bash().contains(
        "systemctl 'is-active' '--quiet' 'sshd' || systemctl 'is-active' '--quiet' 'ssh'"
    ));
    assert!(script_ir[1].to_bash().contains("if \"$active\"; then"));
    assert!(script_ir[1].to_bash().contains("config=$("));
}

#[test]
fn test_sh_macro_advanced_features() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;
    let temp = "test";
    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);
    let script_ir = sh!(
        defer! {
            cmd("echo", "cleanup-done");
        }
        fn restart_service(name){
            os.service(name).restart().sudo();
            cmd("systemctl", "-R");
        }

        let services = ["nginx", "sshd"];

        for service in services {
            restart_service(service);
        }
        os.service(temp).restart().sudo();
        let logs = glob!("*.log");
        let user = shell_env!("USER");

        cmd("rm", logs);
        restart_service(temp);
        cmd("echo", "deploy-finished").stdout(crate::utils::exec::script::dsl::OutputTarget::file("/var/log/deploy.log"));
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");

    // 1. Closure function test
    assert!(bash.contains("restart_service() {"));
    assert!(bash.contains("local name=\"$1\""));
    assert!(bash.contains("sudo systemctl restart \"$name\""));

    // 2. Array loop test
    assert!(bash.contains("services=('nginx' 'sshd')"));
    assert!(bash.contains("for service in \"${services[@]}\"; do"));
    assert!(bash.contains("restart_service \"$service\""));

    // 3. Glob & EnvVar test
    assert!(bash.contains("logs=*.log"));
    assert!(bash.contains("user=\"$USER\""));
    assert!(bash.contains("rm \"$logs\""));

    // 4. Redirection test
    assert!(bash.contains("echo 'deploy-finished' > '/var/log/deploy.log'"));

    // 5. Defer (trap cleanup) test
    assert!(bash.contains("_cleanup() {"));
    assert!(bash.contains("echo 'cleanup-done'"));
    assert!(bash.contains("trap _cleanup EXIT"));
}

#[test]
fn test_sh_macro_while_and_pipeline() {
    use super::sh;

    let script = sh!(while cmd("test", "1", "-eq", "1") {
        let needle = "ready";
        pipe![
            cmd("printf", "ready"),
            cmd("grep", "-q", word!["^", needle, "$"])
        ];
        break;
    });
    let bash = script.build_str();
    assert!(bash.contains("while test '1' '-eq' '1'; do"));
    assert!(bash.contains("printf 'ready' | grep '-q' \"^${needle}\\$\""));
    assert!(bash.contains("break"));
}

#[test]
fn test_sh_macro_validated_text_tools() {
    use super::sh;

    let filter = ".items[]";
    let script = sh!(
        pipe![cmd("cat", "data.json"), jq!(dynamic!(filter)), grep!("active")];
        jq(".server.port", "config.json");
        pipe![
            cmd("printf", "alpha beta"),
            awk! {
                for field in fields {
                    if field != "" {
                        print(field);
                        exit;
                    }
                }
            }
        ];
        sed("s/foo/bar/g", "config.txt");
    );

    let bash = script
        .iter()
        .map(|step| step.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("cat 'data.json' | jq '-r' '.items[]' | grep 'active'"));
    assert!(bash.contains("jq '-r' '.server.port' 'config.json'"));
    assert!(bash.contains(
        "printf 'alpha beta' | awk '{ for (i=1; i<=NF; i++) if ($i != \"\") { print $i; exit } }'"
    ));
    assert!(bash.contains("sed 's/foo/bar/g' 'config.txt'"));
}

#[test]
fn test_sh_macro_convenience_dsls() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        info!("Deploy starting");

        let tmp = temp_file();

        os.file("$tmp").write("hello");
        os.file("/etc/passwd").read();
        os.file("$tmp").exists();
        os.file("$tmp").delete();
        os.file("$tmp").chmod(crate::utils::os::file::FileMode::OwnerAllGroupReadExecuteWorldReadExecute);
        os.file("$tmp").chown("root");

        os.dir("/tmp/test_dir").create();
        os.dir("/tmp/test_dir").exists();
        os.dir("/tmp/test_dir").delete();

        os.service("nginx").restart().sudo();
        os.service("nginx").status();

        os.process("nginx").kill();

        os.package("nginx").install();

        let oid = os_id();
        let family = os_family();
        let arch = os_arch();

        retry!(5, {
            cmd("curl", "http://example.com");
        });

        parallel! {
            cmd("sleep", "1");
            cmd("sleep", "2");
        }
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");

    // 1. Logging and temp file
    assert!(bash.contains("echo '[INFO] Deploy starting'"));
    assert!(bash.contains("tmp=$(mktemp)"));
    assert!(bash.contains("oid=$(sh '-c' '. /etc/os-release && echo \"$ID\"')"));
    assert!(bash.contains("family=$(uname '-s')"));
    assert!(bash.contains("arch=$(uname '-m')"));

    // 2. File DSL
    assert!(bash.contains("printf '%b' 'hello' > \"$tmp\""));
    assert!(bash.contains("cat '/etc/passwd'"));
    assert!(bash.contains("test -f \"$tmp\""));
    assert!(bash.contains("rm -f \"$tmp\""));
    assert!(bash.contains("chmod '755' \"$tmp\""));
    assert!(bash.contains("chown 'root' \"$tmp\""));

    // 3. Directory DSL
    assert!(bash.contains("mkdir -p '/tmp/test_dir'"));
    assert!(bash.contains("test -d '/tmp/test_dir'"));
    assert!(bash.contains("rm -rf '/tmp/test_dir'"));

    // 4. Service & Process DSL
    assert!(bash.contains("sudo systemctl restart 'nginx'"));
    assert!(bash.contains("systemctl status 'nginx'"));
    assert!(bash.contains("'kill' '-9' 'nginx'"));

    // 5. Package Managers DSL
    assert!(bash.contains("apt-get 'install' '-y'"));

    // 6. Retry DSL
    assert!(bash.contains("for i in $(seq 1 '5'); do"));
    assert!(bash.contains("curl 'http://example.com'"));
    assert!(bash.contains("break || sleep 1"));

    // 7. Parallel DSL
    assert!(bash.contains("sleep '1' &"));
    assert!(bash.contains("sleep '2' &"));
    assert!(bash.contains("wait"));
}

#[test]
fn test_sh_macro_with_rust_builders() {
    use super::sh;
    let cli = crate::utils::docker::DockerCli::new_local();
    let containers = cli.containers();
    let container = containers
        .create("alpine:latest")
        .name("test-sh-macro-container")
        .tty(true);

    let script_ir = sh!(
        container;
    );

    assert_eq!(script_ir.len(), 1);
    let bash = script_ir[0].to_bash();
    assert!(
        bash.contains("docker container run --name test-sh-macro-container --tty alpine:latest")
    );
}

#[test]
fn test_sh_macro_builder_inside_if_branch() {
    use super::sh;
    let cli = crate::utils::docker::DockerCli::new_local();
    let containers = cli.containers();
    let start = containers.start("web");

    let script_ir = sh!(if cmd("test", "1", "=", "1") {
        start;
    });

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("if test '1' '=' '1'; then"));
    assert!(bash.contains("docker container start web"));
}

#[test]
fn test_sh_macro_docker_chain_inside_if_branch() {
    use super::sh;
    let docker = crate::utils::docker::DockerCli::new_local();

    let script_ir = sh!(if docker
        .containers()
        .inspect_cmd("web")
        .stdout(crate::utils::exec::script::dsl::OutputTarget::Null)
        .stderr(crate::utils::exec::script::dsl::OutputTarget::Null)
    {
        docker.containers().start("web");
    });

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("if docker container inspect web > /dev/null 2> /dev/null; then"));
    assert!(bash.contains("docker container start web"));
}

#[test]
fn test_sh_macro_structured_awk() {
    use super::sh;

    let script_ir = sh!(
        pipe![
            cmd("hostname", "-I"),
            awk! {
                for field in fields {
                if field != "127.0.0.1" {
                    print(field);
                    exit;
                }
                }
            }
        ];
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("hostname '-I' | awk '{ for (i=1; i<=NF; i++) if ($i != \"127.0.0.1\") { print $i; exit } }'"));
}

#[test]
fn test_sh_macro_break_continue() {
    use super::sh;
    let script_ir = sh!(
        let services = ["nginx", "sshd"];
        for s in services {
            if s {
                continue;
            }
            break;
        }
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("for s in \"${services[@]}\"; do"));
    assert!(bash.contains("if \"$s\"; then"));
    assert!(bash.contains("continue"));
    assert!(bash.contains("break"));
}

#[test]
fn test_sh_macro_linux_commands_validation() {
    use super::sh;
    let script_ir = sh!(
        grep!("-i", "^hello", "file.txt");
        sed!("s/foo/bar/g", "file.txt");
        awk! {
            for field in fields {
                if field != "" {
                    print(field);
                    exit;
                }
            }
        };
        find!("/tmp", "-name", "*.txt", "-type", "f");
        xargs!("-0", "rm");
        tar!("-czf", "archive.tar.gz", "dir");
        curl!("-s", "-L", "http://example.com");
    );

    assert_eq!(script_ir.len(), 7);
    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("grep '-i' '^hello' 'file.txt'"));
    assert!(bash.contains("sed 's/foo/bar/g' 'file.txt'"));
    assert!(bash.contains("awk '{ for (i=1; i<=NF; i++) if ($i != \"\") { print $i; exit } }'"));
    assert!(bash.contains("find '/tmp' '-name' '*.txt' '-type' 'f'"));
    assert!(bash.contains("xargs '-0' 'rm'"));
    assert!(bash.contains("tar '-czf' 'archive.tar.gz' 'dir'"));
    assert!(bash.contains("curl '-s' '-L' 'http://example.com'"));
}

#[test]
fn test_sh_macro_generic_unix_dsl() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        os.system().info();
        os.system().hostname();
        os.process_api().list();
        os.package_api().update_index();
        os.dir_api().current();
        os.network().ping("1.1.1.1");
        os.env().get("USER");
        sleep(5);
    );

    assert_eq!(script_ir.len(), 8);
    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("uname '-a'"));
    assert!(bash.contains("hostname"));
    assert!(bash.contains("ps '-ef'"));
    assert!(bash.contains("command '-v' 'apt-get'"));
    assert!(bash.contains("pwd"));
    assert!(bash.contains("ping '-c' '4' '1.1.1.1'"));
    assert!(bash.contains("printenv 'USER'"));
    assert!(bash.contains("sleep '5'"));
}

#[test]
fn test_sh_macro_mvp_deploy_dsl() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        os.port().free(8080);
        os.port().check(8080);
        os.lock().acquire("deploy");
        os.lock().release("deploy");
        os.http().wait_healthy("http://localhost:8080", 30);
        os.symlink("v2", "current").create();
        os.mount("src", "tgt").bind();
        os.mount_ref("tgt").unmount();
    );

    assert_eq!(script_ir.len(), 8);
    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("while ss '-tuln' | grep '-q' \":${port} \"; do"));
    assert!(bash.contains("ss '-tuln' | grep '-q' \":8080 \""));
    assert!(bash.contains("while ! mkdir \"/tmp/rustploy_lock_deploy\" 2> /dev/null; do"));
    assert!(bash.contains("rmdir \"/tmp/rustploy_lock_deploy\""));
    assert!(bash.contains("start=$(date '+%s')"));
    assert!(bash.contains("ln -sf 'v2' 'current'"));
    assert!(bash.contains("mount '--bind' 'src' 'tgt'"));
    assert!(bash.contains("umount 'tgt'"));
}

#[test]
fn test_pipeline_and_sh_unification() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    // 1. Compile DSL statements via sh!
    let setup = sh!(
        os.lock().acquire("my_deploy_lock");
        os.port().free(9000);
    );

    // 2. Convert the list of ShellIRs into a single combined Bash string
    let setup_bash = setup
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");

    // 3. Feed the generated script into the pipeline builder block
    let p = pipeline!(
        and setup_bash;
        and "echo 'deploying now'";
    );

    let compiled = p.compile();
    assert!(compiled.contains("while ! mkdir \"/tmp/rustploy_lock_my_deploy_lock\""));
    assert!(compiled.contains("while ss '-tuln' | grep '-q' \":${port} \"; do"));
    assert!(compiled.contains("echo 'deploying now'"));
}

#[tokio::test]
async fn test_sh_macro_direct_execution() {
    use super::{IntoCommand, sh};
    use crate::utils::exec::{CommandExecutor, LocalExecutor};

    let executor = CommandExecutor::Local(LocalExecutor::new());

    // We can run execute directly on the sh! output!
    let out = sh!(
        echo("hello from sh macro");
    )
    .execute(&executor)
    .await
    .unwrap();

    assert!(out.status.success());
    assert!(out.stdout.contains("hello from sh macro"));
}

#[test]
fn test_rust_dsl_api_usage() {
    use crate::utils::exec::script::IntoCommand;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    // 1. We can write type-safe regular Rust code using the DSL
    let p_free = os.port().free(8080);
    let p_check = os.port().check(8080);
    let lock_acq = os.lock().acquire("deploy");
    let health = os.http().wait_healthy("http://localhost:8080", 30);
    let sym = os.symlink("v2", "current").create();
    let config = os.file("config.json").write("production");

    // 2. Verify their generated bash command output
    assert!(p_free.build_str().contains("while ss '-tuln' | grep '-q'"));
    assert!(
        p_check
            .build_str()
            .contains("ss '-tuln' | grep '-q' \":8080 \"")
    );
    assert!(
        lock_acq
            .build_str()
            .contains("while ! mkdir \"/tmp/rustploy_lock_deploy\" 2> /dev/null; do")
    );
    assert!(health.build_str().contains("start=$(date '+%s')"));
    assert!(sym.build_str().contains("ln -sf 'v2' 'current'"));
    assert!(
        config
            .build_str()
            .contains("printf '%b' 'production' > 'config.json'")
    );
}

#[test]
fn test_file_write_escapes_multiline_literals() {
    use crate::utils::exec::script::IntoCommand;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let command = os
        .file("traefik.yml")
        .write("global:\n  sendAnonymousUsage: false\nproviders:\n  docker: {}\n")
        .build_str();

    assert!(command.contains("printf '%b'"));
    assert!(command.contains("global:\\n  sendAnonymousUsage: false\\nproviders:"));
    assert!(!command.contains("global:\n"));
}

#[test]
fn test_package_builders_and_macro_dsl() {
    use crate::utils::exec::script::{IntoCommand, sh};
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    // Test direct builder string generation
    let pkg_install = os.package("curl").install();
    let pkg_install_bash = pkg_install.build_str();
    assert!(pkg_install_bash.contains("apt-get 'install' '-y'"));
    assert!(pkg_install_bash.contains("xbps-install '-Sy'"));
    assert!(pkg_install_bash.contains("nix-env '-i'"));
    assert!(pkg_install_bash.contains("brew 'install'"));

    let macro_script = sh!(
        os.package("nginx").install().manager(crate::utils::os::package::PackageManager::Emerge);
        os.package("git").install().manager(crate::utils::os::package::PackageManager::Nix);
        os.package("docker").install();
    );

    let macro_bash = macro_script
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(macro_bash.contains("emerge 'nginx'"));
    assert!(macro_bash.contains("nix-env '-i' 'git'"));
    assert!(macro_bash.contains("command '-v' 'apt-get'"));
    assert!(macro_bash.contains("command '-v' 'xbps-install'"));
    assert!(macro_bash.contains("command '-v' 'nix-env'"));
}

#[test]
fn test_sh_macro_json() {
    use super::sh;
    let script_ir = sh!(
        let isEnabled = "true";
        let permitRootLogin = "prohibit-password";
        json!({
            "enabled": isEnabled,
            "permitRootLogin": permitRootLogin,
            "port": 22,
            "debug": false
        });
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("echo \"{\\\"enabled\\\": $isEnabled, \\\"permitRootLogin\\\": \\\"$permitRootLogin\\\", \\\"port\\\": 22, \\\"debug\\\": false}\""));
}

#[test]
fn test_sh_macro_binary_and_unary_conditions() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        if systemctl!("is-active", "--quiet", "sshd") || systemctl!("is-active", "--quiet", "ssh") {
            echo("sshd is running");
        }
        if !cmd("test", "-f", "/etc/sshd_config") {
            echo("no config");
        }
        if os.file("/etc/passwd").exists() || grep!("-q", "root", "/etc/passwd") {
            echo("passwd is secure");
        }
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains(
        "if systemctl 'is-active' '--quiet' 'sshd' || systemctl 'is-active' '--quiet' 'ssh'; then"
    ));
    assert!(bash.contains("if ! test '-f' '/etc/sshd_config'; then"));
    assert!(bash.contains("if test -f '/etc/passwd' || grep '-q' 'root' '/etc/passwd'; then"));
}

#[test]
fn test_sh_macro_sudo() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        sudo!(systemctl!("restart", "nginx"));
        sudo!(os.file("/etc/shadow").read());
        sudo!(cmd("apt-get", "update"));
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("sudo 'systemctl' 'restart' 'nginx'"));
    assert!(bash.contains("sudo cat '/etc/shadow'"));
    assert!(bash.contains("sudo 'apt-get' 'update'"));
}

#[test]
fn test_sh_macro_capture_success_failure() {
    use super::sh;
    let script_ir = sh!(
        let active = capture_status! {
            systemctl!("is-active", "sshd");
        };
        if active.success() {
            echo("sshd is up");
        }
        if active.failure() {
            echo("sshd is down");
        }
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains(
        "active=$(if systemctl 'is-active' 'sshd'; then echo true; else echo false; fi)"
    ));
    assert!(bash.contains("active") && bash.contains("true"));
    assert!(bash.contains("active") && bash.contains("false"));
}

#[test]
fn test_sh_macro_new_features() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        os.file("config.txt").replace("old_ip", "new_ip");
        os.has_command("nginx");
        let info = "some_json_str";
        let user = jq!(info, ".user.name");
        let port = jq_file!("config.json", ".server.port");
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("sed -i 's|old_ip|new_ip|g' 'config.txt'"));
    assert!(bash.contains("command '-v' 'nginx'"));
    assert!(
        bash.contains("user=$(echo \"$info\" | jq '-r' '.user.name')"),
        "generated shell:\n{bash}"
    );
    assert!(bash.contains("port=$(jq -r '.server.port' 'config.json')"));
}

#[test]
fn test_sh_macro_nested_captures() {
    use super::sh;
    let script_ir = sh!(
        let status = capture_status! {
            let out = capture_stdout! {
                cmd("curl", "http://example.com");
            };
        };
        if status.success() {
            echo("fetch succeeded");
        }
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains(
        "status=$(if out=$(curl 'http://example.com'); then echo true; else echo false; fi)"
    ));
    assert!(bash.contains("status") && bash.contains("true"));
}

#[test]
fn test_sh_macro_os_api_utilities() {
    use super::sh;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;

    let executor = CommandExecutor::Local(LocalExecutor::new());
    let os = OsCli::new(&executor);

    let script_ir = sh!(
        let text = os.capture_stdout("curl http://example.com");
        let status = os.capture_status("systemctl is-active sshd");
        let user = os.jq(text, ".user.name");
        let port = os.jq_file("config.json", ".server.port");
        let col = capture_stdout! {
            pipe![
                cmd("printf", "a b"),
                awk! {
                    for field in fields {
                        if field != "a" {
                            print(field);
                            exit;
                        }
                    }
                }
            ];
        };
        let grep_res = os.grep(text, "error");
        let file_grep = os.grep_file("app.log", "failed");
        os.sed_file("config.json", "s/foo/bar/g");
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("text=$(curl http://example.com)"));
    assert!(
        bash.contains("status=$(if systemctl is-active sshd; then echo true; else echo false; fi)")
    );
    assert!(bash.contains("user=$(echo \"$text\" | jq '-r' '.user.name')"));
    assert!(bash.contains("port=$(jq '-r' '.server.port' 'config.json')"));
    assert!(bash.contains(
        "col=$(printf 'a b' | awk '{ for (i=1; i<=NF; i++) if ($i != \"a\") { print $i; exit } }')"
    ));
    assert!(bash.contains("grep_res=$(echo \"$text\" | grep 'error')"));
    assert!(bash.contains("file_grep=$(grep 'failed' 'app.log')"));
    assert!(bash.contains("sed -i 's/foo/bar/g' 'config.json'"));
}

#[test]
fn test_sh_macro_unified_capture() {
    use super::sh;
    let script_ir = sh!(
        let res = capture! {
            cmd("curl", "http://example.com");
            echo("block line 2");
        };

        if res.success() {
            echo(res.stdout());
        } else {
            echo(res.stderr());
        }
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(bash.contains("res_stdout_file=$(mktemp)"));
    assert!(bash.contains("res_stderr_file=$(mktemp)"));
    assert!(bash.contains("curl 'http://example.com'"));
    assert!(bash.contains("echo 'block line 2'"));
    assert!(bash.contains("res_status=true"));
    assert!(bash.contains("res_stdout=$(cat \"$res_stdout_file\")"));
    assert!(bash.contains("res_stderr=$(cat \"$res_stderr_file\")"));
    assert!(bash.contains("rm -f \"$res_stdout_file\" \"$res_stderr_file\""));
    assert!(bash.contains("res_status") && bash.contains("true"));
    assert!(bash.contains("echo \"$res_stdout\""));
    assert!(bash.contains("echo \"$res_stderr\""));
}

#[test]
fn test_auto_detect_outer_rust_vars() {
    let outer_var = "my_custom_service";
    let script_ir = sh!(
        cmd("systemctl", "restart", outer_var);
    );

    let bash = script_ir
        .iter()
        .map(|s| s.to_bash())
        .collect::<Vec<_>>()
        .join("\n");
    println!("BASH GENERATED: {}", bash);
    assert!(bash.contains("systemctl 'restart' 'my_custom_service'"));
}
