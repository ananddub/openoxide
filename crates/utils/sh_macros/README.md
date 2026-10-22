# OpenOxide Shell Scripting Macro (`sh!`)

`openoxide_sh_macros` is a powerful procedural macro DSL (`sh!`) that compiles clean, type-safe, Rust-like syntax into a robust, POSIX-compliant Bash shell script representation (`ShellIR`). The macro guarantees variable scope checking and structure validation at compile-time, protecting against command injection vulnerabilities and shell scripting bugs.

---

## Table of Contents
1. [Core Design Concepts](#core-design-concepts)
2. [Unified Command Capture (`capture!`)](#unified-command-capture-capture)
3. [Fluent System Utilities (`os` API)](#fluent-system-utilities-os-api)
4. [Sudo Wrap Macro (`sudo!`)](#sudo-wrap-macro-sudo)
5. [Syntax Reference](#syntax-reference)
   - [Command Invocation](#command-invocation)
   - [Variables & Scope](#variables--scope)
   - [Dynamic Rust Interpolation (`rust!`)](#dynamic-rust-interpolation-rust)
   - [Conditionals (`if` / `else if` / `else`)](#conditionals-if--else-if--else)
   - [Output Redirection & Chaining](#output-redirection--chaining)
6. [Supported Package Managers](#supported-package-managers)
7. [Comprehensive Production-Grade Examples](#comprehensive-production-grade-examples)

---

## Core Design Concepts

When writing shell logic inside `sh!`, the macro parses your Rust code and generates a `Vec<ShellIR>` intermediate representation.
* **Compile-time Scope Validation**: All variable usage is checked at compile-time to prevent undefined variable references in bash.
* **Mathematical Injection Safety**: The DSL distinguishes variables and literals. Literals are compiled into single-quoted string tokens `'literal'`, while variable arguments are compiled into distinct variable tokens and safely formatted during shell rendering.
* **Platform Agnostic Abstractions**: Provides native DSL helpers for packages, files, JSON, text processing, and execution wrappers.

---

## Unified Command Capture (`capture!`)

Instead of split macros, a single unified **`capture!`** block executes any block of commands, simultaneously capturing their standard output, standard error, and exit status. Under the hood, it routes outputs through temporary file descriptors, loads them into local variables, and cleans up files instantly to prevent filesystem garbage.

### Captured Object API
When you assign a `capture!` block to a variable:
* `res.stdout()`: Accesses the captured standard output variable.
* `res.stderr()`: Accesses the captured standard error variable.
* `res.success()`: Generates a status check to verify the commands completed with exit code `0`.
* `res.failure()`: Generates a status check to verify the commands failed (exit code non-zero).

### Example DSL Call
```rust
let script = sh!(
    let res = capture! {
        cmd("curl", "-s", "http://example.com/api");
        echo("Finished query execution.");
    };

    if res.success() {
        echo("Success Output:");
        echo(res.stdout());
    } else {
        echo("Error Log:");
        echo(res.stderr());
    }
);
```

### Generated Bash Execution Flow
```bash
res_stdout_file=$(mktemp)
res_stderr_file=$(mktemp)
if (
    curl -s 'http://example.com/api'
    echo 'Finished query execution.'
) > "$res_stdout_file" 2> "$res_stderr_file"; then
    res_status=true
else
    res_status=false
fi
res_stdout=$(cat "$res_stdout_file")
res_stderr=$(cat "$res_stderr_file")
rm -f "$res_stdout_file" "$res_stderr_file"

if [ "${res_status:-}" = "true" ]; then
    echo 'Success Output:'
    echo "$res_stdout"
else
    echo 'Error Log:'
    echo "$res_stderr"
fi
```

---

## Fluent System Utilities (`os` API)

To keep the DSL syntax clean, common UNIX command-line utilities are unified under the `os` receiver object. The procedural macro detects `os` and compiles them directly into native `OsCli` builder struct calls:

| Operation | DSL Helper Method | Generated Shell Format |
|---|---|---|
| **Stdout Capture** | `os.capture_stdout(cmd)` | `$(cmd)` |
| **Status Capture** | `os.capture_status(cmd)` | `$(if cmd; then echo true; else echo false; fi)` |
| **Command Check** | `os.has_command(bin)` | `command -v 'bin'` |
| **JSON Parser** | `os.jq(var, query)` | `$(echo $var \| jq -r 'query')` |
| **JSON File Parser** | `os.jq_file(file, query)` | `$(jq -r 'query' 'file')` |
| **Column Processing** | `awk! { for field in fields { ... } }` inside `pipe![...]` | `awk '{ for (i=1; i<=NF; i++) ... }'` |
| **In-place Replace** | `os.sed_file(file, pattern)` | `sed -i 'pattern' 'file'` |
| **Text Filter** | `os.grep(target, pattern)` | `$(echo $target \| grep 'pattern')` or `$({cmd} \| grep 'pattern')` |
| **File Filter** | `os.grep_file(file, pattern)` | `$(grep 'pattern' 'file')` |

### DSL Usage
```rust
let script = sh!(
    let text = os.capture_stdout("curl http://example.com");
    let status = os.capture_status("systemctl is-active sshd");
    let has_nginx = os.has_command("nginx");

    let user = os.jq(text, ".user.name");
    let port = os.jq_file("config.json", ".server.port");

    let pid = capture_stdout! {
        pipe![
            cmd("ps", "-ef"),
            awk! {
                for field in fields {
                    if field != "root" {
                        print(field);
                        exit;
                    }
                }
            }
        ];
    };
    os.sed_file("app.conf", "s/80/8080/g");
    
    let errors = os.grep(text, "error");
    let failed_lines = os.grep_file("app.log", "failed");
);
```

---

## Sudo Wrap Macro (`sudo!`)

Any command statement or file action can be wrapped with elevated privileges using **`sudo!(...)`**:

```rust
let script = sh!(
    sudo!(systemctl!("restart", "nginx"));
    sudo!(os.file("/etc/shadow").read());
    sudo!(cmd("apt-get", "update"));
);
```

---

## Syntax Reference

### Command Invocation
Invoke binaries using the `cmd(...)` helper or common system wrappers (`echo`, `sleep`, `exit`):

```rust
let script = sh!(
    cmd("git", "clone", "https://github.com/example/repo.git");
    echo("Repository cloned!");
    sleep(2);
    exit(0);
);
```

### Variables & Scope
Declare variables using `let` bindings. Variable scoping is validated at compile-time:

```rust
let script = sh!(
    let file_path = "/var/log/app.log";
    cmd("tail", "-f", file_path); // Compiles to: tail -f "$file_path"
);
```

### Dynamic Rust Interpolation (`rust!`)
To inject dynamic Rust expressions evaluated at runtime, wrap them in `rust!(...)`:

```rust
let pkg = "nginx";
let script = sh!(
    cmd("apt-get", "install", "-y", rust!(pkg));
);
```

### Conditionals (`if` / `else if` / `else`)
Use standard Rust conditional checks. The condition can check the output or status of any command:

```rust
let script = sh!(
    if cmd("command", "-v", "systemctl").stdout("/dev/null") {
        cmd("systemctl", "restart", "docker");
    } else {
        cmd("service", "docker", "restart");
    }
);
```

### Output Redirection & Chaining
Redirect standard descriptor streams using builder-like methods:

```rust
let script = sh!(
    cmd("apt-get", "update").stdout("/dev/null").stderr("/var/log/apt_err.log");
    cmd("echo", "new config line").append("/etc/app.conf");
);
```

---

## Supported Package Managers

The DSL natively translates package manager method calls dynamically or statically:

| Manager | DSL Helper | Example |
|---|---|---|
| **APT** (Debian/Ubuntu) | `apt()` | `apt().install("curl")` |
| **APK** (Alpine) | `apk()` | `apk().install("curl")` |
| **DNF** (Fedora) | `dnf()` | `dnf().install("curl")` |
| **YUM** (RHEL/CentOS) | `yum()` | `yum().install("curl")` |
| **PACMAN** (Arch) | `pacman()` | `pacman().install("curl")` |
| **ZYPPER** (openSUSE) | `zypper()` | `zypper().install("curl")` |
| **XBPS** (Void Linux) | `xbps()` | `xbps().install("curl")` |
| **EMERGE** (Gentoo) | `emerge()` | `emerge().install("curl")` |
| **NIX** (NixOS) | `nix()` | `nix().install("curl")` |
| **BREW** (macOS) | `brew()` | `brew().install("curl")` |
| **Auto-Detect** | `package()` | `package().install("curl")` |

---

## Comprehensive Production-Grade Examples

### Automated Web Service Deployment
This example checks if Docker is available, fetches the latest container configuration, parses port parameters, updates setup scripts, and starts the service.

```rust
use openoxide_sh_macros::sh;
use crate::utils::os::OsCli;
use crate::utils::exec::CommandExecutor;

pub fn deploy_service(os: &OsCli) -> String {
    let script_ir = sh!(
        // 1. Check commands
        if os.has_command("docker").failure() {
            echo("Docker is not installed! Exiting...").stderr("/dev/stderr");
            exit(1);
        }

        // 2. Fetch config JSON and query target port
        let config_data = capture! {
            cmd("curl", "-s", "http://config-server.local/app.json");
        };

        if config_data.failure() {
            echo("Failed to fetch configuration!").stderr("/dev/stderr");
            exit(2);
        }

        let app_port = os.jq(config_data.stdout(), ".ports.http");
        echo(cmd("echo", "Configured port is:", app_port));

        // 3. Update configuration file in-place
        os.sed_file("app.env", rust!(format!("s/PORT=.*/PORT={}/g", "8080")));

        // 4. Start the service
        sudo!(cmd("docker", "run", "-d", "-p", rust!(format!("{}:80", "8080")), "nginx:alpine"));
    );

    script_ir.iter().map(|s| s.to_bash()).collect::<Vec<_>>().join("\n")
}
```
