//! Typed builders for database client CLIs (`pg_dump`, `mysqldump`, `mongodump`,
//! `redis-cli`, …).
//!
//! Every builder accumulates argv tokens and implements
//! [`IntoCommand`](crate::exec::script::IntoCommand), so a client command can be
//! used directly as a [`ScriptPipeline`](crate::exec::script::ScriptPipeline)
//! step. Every token is single-quoted in the rendered command string (Gate G3):
//! these strings are frequently nested inside `docker exec … sh -c '<inner>'`,
//! so a credential or database name containing a quote must not be able to
//! break out of its token at either shell layer.

pub mod mariadb;
pub mod mongo;
pub mod mysql;
pub mod postgres;
pub mod redis;

pub use mariadb::{MariadbAdminBuilder, MariadbClientBuilder, MariadbDumpBuilder};
pub use mongo::{MongodumpBuilder, MongorestoreBuilder, MongoshBuilder};
pub use mysql::{MysqlAdminBuilder, MysqlClientBuilder, MysqldumpBuilder};
pub use postgres::{PgDumpBuilder, PgIsReadyBuilder, PgRestoreBuilder};
pub use redis::RedisCliBuilder;

use crate::exec::script::shell_single_quote;

/// Renders `binary <quoted arg> …` — the shared `IntoCommand` shape for all
/// database client builders. The binary name is a fixed literal; every
/// caller-supplied token is single-quoted.
pub(crate) fn build_command(binary: &str, args: &[String]) -> String {
    let mut parts = Vec::with_capacity(args.len() + 1);
    parts.push(binary.to_string());
    parts.extend(args.iter().map(|a| shell_single_quote(a)));
    parts.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec::script::IntoCommand;

    struct Fake;
    impl IntoCommand for Fake {
        fn build_str(&self) -> String {
            build_command("tool", &["a'b".into(), "-x".into()])
        }
    }

    #[test]
    fn every_token_is_single_quoted_and_escape_safe() {
        assert_eq!(Fake.build_str(), "tool 'a'\\''b' '-x'");
    }
}
