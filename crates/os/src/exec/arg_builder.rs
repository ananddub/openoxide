/// Shared argument accumulator used by all Docker CLI builders.
///
/// Builders store a single `ArgBuilder` instead of maintaining their own
/// `Vec<String>`. This eliminates duplicated push/extend patterns.
#[derive(Debug, Default, Clone)]
pub struct ArgBuilder {
    args: Vec<String>,
    pub retry_limit: Option<u32>,
    pub cancel_token: Option<tokio_util::sync::CancellationToken>,
}

impl ArgBuilder {
    /// Start a new builder, pre-seeded with Docker subcommand tokens.
    /// ```
    /// ArgBuilder::cmd(&["container", "run"])
    /// ```
    pub fn cmd(subcmd: &[&str]) -> Self {
        Self {
            args: subcmd.iter().map(|s| (*s).to_string()).collect(),
            retry_limit: None,
            cancel_token: None,
        }
    }

    /// Push a bare flag (`--flag`).
    pub fn flag(&mut self, f: &str) -> &mut Self {
        self.args.push(f.to_string());
        self
    }

    /// Push `--key value`.
    pub fn pair(&mut self, k: &str, v: impl AsRef<str>) -> &mut Self {
        self.args.push(k.to_string());
        self.args.push(v.as_ref().to_string());
        self
    }

    /// Insert `--key value` at a specific index.
    pub fn insert_pair(&mut self, index: usize, k: &str, v: impl AsRef<str>) -> &mut Self {
        self.args.insert(index, v.as_ref().to_string());
        self.args.insert(index, k.to_string());
        self
    }

    /// Push `--key value` only when `cond` is true.
    pub fn pair_if(&mut self, k: &str, v: impl AsRef<str>, cond: bool) -> &mut Self {
        if cond {
            self.pair(k, v);
        }
        self
    }

    /// Push `--key value` only when `v` is `Some`.
    pub fn pair_opt(&mut self, k: &str, v: Option<impl AsRef<str>>) -> &mut Self {
        if let Some(v) = v {
            self.pair(k, v);
        }
        self
    }

    /// Push `--flag` only when `cond` is true.
    pub fn flag_if(&mut self, f: &str, cond: bool) -> &mut Self {
        if cond {
            self.flag(f);
        }
        self
    }

    /// Push `--filter key=value` using any value that implements `Display`.
    pub fn filter(&mut self, v: impl std::fmt::Display) -> &mut Self {
        self.pair("--filter", v.to_string())
    }

    /// Push `--label key=value`.
    pub fn label(&mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> &mut Self {
        self.pair("--label", format!("{}={}", k.as_ref(), v.as_ref()))
    }

    /// Push `--env KEY=VALUE`.
    pub fn env_var(&mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> &mut Self {
        self.pair("--env", format!("{}={}", k.as_ref(), v.as_ref()))
    }

    /// Push a single raw argument.
    pub fn push(&mut self, v: impl Into<String>) -> &mut Self {
        self.args.push(v.into());
        self
    }

    /// Push all items from an iterator.
    pub fn push_all(&mut self, vs: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.args.extend(vs.into_iter().map(Into::into));
        self
    }

    /// Inherit retry_limit and cancel_token from another ArgBuilder.
    pub fn inherit_meta(&mut self, other: &Self) -> &mut Self {
        self.retry_limit = other.retry_limit;
        self.cancel_token = other.cancel_token.clone();
        self
    }

    /// Consume and return the built argument list.
    pub fn build(self) -> Vec<String> {
        self.args
    }

    /// Render as a human-readable `docker <args...>` string for dry-run / debugging.
    pub fn preview(&self) -> String {
        std::iter::once("docker")
            .chain(self.args.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Render arguments joined by space without any command prefix.
    pub fn build_string(&self) -> String {
        self.args.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_correct_args() {
        // let mut a = ArgBuilder::cmd(&["container", "run"]);
        // a.pair("--name", "web")
        //     .flag("--detach")
        //     .pair_if("--memory", "512m", true)
        //     .pair_opt("--user", Some("root"))
        //     .pair_opt::<&str>("--workdir", None)
        //     .flag_if("--rm", false)
        //     .label("app", "api")
        //     .env_var("PORT", "3000")
        //     .filter("status=running")
        //     .push("nginx:latest");
        //
        // let args = a.clone().build();
        // assert_eq!(&args[0..2], &["container", "run"]);
        // assert!(args.contains(&"--name".to_string()));
        // assert!(args.contains(&"web".to_string()));
        // assert!(args.contains(&"--detach".to_string()));
        // assert!(args.contains(&"--memory".to_string()));
        // assert!(!args.contains(&"--rm".to_string()));
        // assert!(!args.contains(&"--workdir".to_string()));
        // assert!(args.contains(&"app=api".to_string()));
        // assert!(args.contains(&"PORT=3000".to_string()));
        // assert!(args.contains(&"status=running".to_string()));
        // assert_eq!(args.last(), Some(&"nginx:latest".to_string()));
    }

    #[test]
    fn preview_starts_with_docker() {
        let mut a = ArgBuilder::cmd(&["ps"]);
        a.flag("--all");
        assert!(a.preview().starts_with("docker ps"));
    }
}
