//! MariaDB client tools: `mariadb-dump`, `mariadb`, `mariadb-admin`.

use super::build_command;
use crate::exec::script::IntoCommand;

macro_rules! client_builder {
    ($name:ident, $binary:literal) => {
        #[doc = concat!("Builds a `", $binary, "` invocation.")]
        #[derive(Default, Clone, Debug)]
        pub struct $name {
            args: Vec<String>,
        }

        impl $name {
            pub fn new() -> Self {
                Self::default()
            }

            #[allow(dead_code)]
            fn flag(mut self, f: &str) -> Self {
                self.args.push(f.to_string());
                self
            }

            #[allow(dead_code)]
            fn pair(mut self, f: &str, v: impl Into<String>) -> Self {
                self.args.push(f.to_string());
                self.args.push(v.into());
                self
            }

            #[allow(dead_code)]
            fn assign(mut self, f: &str, v: impl Into<String>) -> Self {
                self.args.push(format!("{f}={}", v.into()));
                self
            }

            /// Accumulated argv (binary excluded).
            pub fn build_args(&self) -> Vec<String> {
                self.args.clone()
            }
        }

        impl IntoCommand for $name {
            fn build_str(&self) -> String {
                build_command($binary, &self.args)
            }
        }
    };
}

client_builder!(MariadbDumpBuilder, "mariadb-dump");
client_builder!(MariadbClientBuilder, "mariadb");
client_builder!(MariadbAdminBuilder, "mariadb-admin");

impl MariadbDumpBuilder {
    /// `--user=<user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.assign("--user", user)
    }
    /// `--password=<pw>`. Prefer passing `MYSQL_PWD` through the pipeline
    /// environment so the secret never appears in argv.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.assign("--password", pw)
    }
    /// `--single-transaction` — consistent snapshot without locking.
    pub fn single_transaction(self) -> Self {
        self.flag("--single-transaction")
    }
    /// `--quick` — stream rows instead of buffering the whole table.
    pub fn quick(self) -> Self {
        self.flag("--quick")
    }
    /// `--databases <db>` — dump with `CREATE DATABASE`/`USE` statements.
    pub fn databases(mut self, db: impl Into<String>) -> Self {
        self.args.push("--databases".into());
        self.args.push(db.into());
        self
    }
}

impl MariadbClientBuilder {
    /// `-u <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-u", user)
    }
    /// `--password=<pw>`. Prefer the `MYSQL_PWD` environment variable.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.assign("--password", pw)
    }
    /// Positional database name.
    pub fn database(mut self, db: impl Into<String>) -> Self {
        self.args.push(db.into());
        self
    }
}

impl MariadbAdminBuilder {
    /// `--user=<user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.assign("--user", user)
    }
    /// `--password=<pw>`. Prefer the `MYSQL_PWD` environment variable.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.assign("--password", pw)
    }
    /// `ping` subcommand.
    pub fn ping(self) -> Self {
        self.flag("ping")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mariadb_dump_shape() {
        let cmd = MariadbDumpBuilder::new()
            .username("app")
            .single_transaction()
            .quick()
            .databases("orders")
            .build_str();
        assert_eq!(
            cmd,
            "mariadb-dump '--user=app' '--single-transaction' '--quick' '--databases' 'orders'"
        );
    }

    #[test]
    fn admin_ping_shape() {
        let cmd = MariadbAdminBuilder::new()
            .username("app")
            .ping()
            .build_str();
        assert_eq!(cmd, "mariadb-admin '--user=app' 'ping'");
    }
}
