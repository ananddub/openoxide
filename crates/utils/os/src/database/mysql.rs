//! MySQL client tools: `mysqldump`, `mysql`, `mysqladmin`.

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

            #[allow(dead_code)]
            fn positionals(mut self, v: impl Into<String>) -> Self {
                self.args.push(v.into());
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

client_builder!(MysqldumpBuilder, "mysqldump");
client_builder!(MysqlClientBuilder, "mysql");
client_builder!(MysqlAdminBuilder, "mysqladmin");

impl MysqldumpBuilder {
    /// `--default-character-set=<cs>`.
    pub fn default_character_set(self, cs: impl Into<String>) -> Self {
        self.assign("--default-character-set", cs)
    }
    /// `-u <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-u", user)
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
    /// `--no-tablespaces` — required against MySQL 8 without the TABLESPACE
    /// privilege.
    pub fn no_tablespaces(self) -> Self {
        self.flag("--no-tablespaces")
    }
    /// `--quick` — stream rows instead of buffering the whole table.
    pub fn quick(self) -> Self {
        self.flag("--quick")
    }
    /// Positional database name.
    pub fn database(self, db: impl Into<String>) -> Self {
        self.positionals(db)
    }
}

impl MysqlClientBuilder {
    /// `-u <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-u", user)
    }
    /// `--password=<pw>`. Prefer the `MYSQL_PWD` environment variable.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.assign("--password", pw)
    }
    /// Positional database name.
    pub fn database(self, db: impl Into<String>) -> Self {
        self.positionals(db)
    }
}

impl MysqlAdminBuilder {
    /// `-u <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-u", user)
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
    fn mysqldump_backup_shape() {
        let cmd = MysqldumpBuilder::new()
            .default_character_set("utf8mb4")
            .username("root")
            .single_transaction()
            .no_tablespaces()
            .quick()
            .database("orders")
            .build_str();
        assert_eq!(
            cmd,
            "mysqldump '--default-character-set=utf8mb4' '-u' 'root' '--single-transaction' '--no-tablespaces' '--quick' 'orders'"
        );
    }

    #[test]
    fn password_token_quote_is_escape_safe() {
        // Gate G3: a `'` in the password must not break out of its token.
        let cmd = MysqlAdminBuilder::new().password("a'b").ping().build_str();
        assert_eq!(cmd, "mysqladmin '--password=a'\\''b' 'ping'");
    }
}
