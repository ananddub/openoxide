//! `redis-cli` — used for `PING` health checks and `BGSAVE` snapshots.

use super::build_command;
use crate::exec::script::IntoCommand;

/// Builds a `redis-cli` invocation.
#[derive(Default, Clone, Debug)]
pub struct RedisCliBuilder {
    args: Vec<String>,
}

impl RedisCliBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    fn flag(mut self, f: &str) -> Self {
        self.args.push(f.to_string());
        self
    }

    fn pair(mut self, f: &str, v: impl Into<String>) -> Self {
        self.args.push(f.to_string());
        self.args.push(v.into());
        self
    }

    /// `-h <host>`.
    pub fn host(self, host: impl Into<String>) -> Self {
        self.pair("-h", host)
    }

    /// `-p <port>`.
    pub fn port(self, port: impl Into<String>) -> Self {
        self.pair("-p", port)
    }

    /// `--password <pw>` / `RedisCore` auth (`-a`).
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.pair("-a", pw)
    }

    /// `PING` command.
    pub fn ping(self) -> Self {
        self.flag("PING")
    }

    /// `BGSAVE` command — fork and persist to the RDB file.
    pub fn bgsave(self) -> Self {
        self.flag("BGSAVE")
    }

    /// `--no-auth-warning` — keep `-a` usage off stderr noise.
    pub fn no_auth_warning(self) -> Self {
        self.flag("--no-auth-warning")
    }

    /// Accumulated argv (binary excluded).
    pub fn build_args(&self) -> Vec<String> {
        self.args.clone()
    }
}

impl IntoCommand for RedisCliBuilder {
    fn build_str(&self) -> String {
        build_command("redis-cli", &self.args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bgsave_shape() {
        assert_eq!(
            RedisCliBuilder::new().bgsave().build_str(),
            "redis-cli 'BGSAVE'"
        );
    }

    #[test]
    fn auth_password_quoted() {
        let cmd = RedisCliBuilder::new()
            .password("s'ecr et")
            .no_auth_warning()
            .ping()
            .build_str();
        assert_eq!(
            cmd,
            "redis-cli '-a' 's'\\''ecr et' '--no-auth-warning' 'PING'"
        );
    }
}
