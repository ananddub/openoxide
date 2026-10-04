//! PostgreSQL client tools: `pg_dump`, `pg_restore`, `pg_isready`.

use super::build_command;
use crate::exec::script::IntoCommand;

macro_rules! client_builder {
    ($name:ident, $binary:literal, $(#[$meta:meta])?) => {
        $(#[$meta])?
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

client_builder!(
    PgDumpBuilder,
    "pg_dump",
    /// Builds a `pg_dump` invocation.
);
client_builder!(
    PgRestoreBuilder,
    "pg_restore",
    /// Builds a `pg_restore` invocation.
);
client_builder!(
    PgIsReadyBuilder,
    "pg_isready",
    /// Builds a `pg_isready` connection check.
);

impl PgDumpBuilder {
    /// Custom archive format (`-Fc`).
    pub fn format_custom(self) -> Self {
        self.flag("-Fc")
    }
    /// Skip ACL statements (`--no-acl`).
    pub fn no_acl(self) -> Self {
        self.flag("--no-acl")
    }
    /// Skip owner statements (`--no-owner`).
    pub fn no_owner(self) -> Self {
        self.flag("--no-owner")
    }
    /// Connect to host (`-h`).
    pub fn host(self, host: impl Into<String>) -> Self {
        self.pair("-h", host)
    }
    /// Operating-system user name to connect as (`-U`).
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-U", user)
    }
    /// Never prompt for a password (`--no-password`); supply `PGPASSWORD` via
    /// the pipeline environment instead of the command line.
    pub fn no_password(self) -> Self {
        self.flag("--no-password")
    }
    /// Positional database name.
    pub fn database(mut self, db: impl Into<String>) -> Self {
        self.args.push(db.into());
        self
    }
}

impl PgRestoreBuilder {
    /// User to connect as (`-U`).
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-U", user)
    }
    /// Destination database (`-d`).
    pub fn database(self, db: impl Into<String>) -> Self {
        self.pair("-d", db)
    }
    /// Do not restore ownership (`-O`).
    pub fn no_owner(self) -> Self {
        self.flag("-O")
    }
    /// Issue drop commands before creating (`--clean`).
    pub fn clean(self) -> Self {
        self.flag("--clean")
    }
    /// Suppress errors for non-existent objects (`--if-exists`).
    pub fn if_exists(self) -> Self {
        self.flag("--if-exists")
    }
}

impl PgIsReadyBuilder {
    /// Report on host (`-h`).
    pub fn host(self, host: impl Into<String>) -> Self {
        self.pair("-h", host)
    }
    /// User name (`-U`).
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-U", user)
    }
    /// Target database (`-d`).
    pub fn database(self, db: impl Into<String>) -> Self {
        self.pair("-d", db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pg_dump_backup_shape() {
        let cmd = PgDumpBuilder::new()
            .format_custom()
            .no_acl()
            .no_owner()
            .host("localhost")
            .username("app")
            .no_password()
            .database("orders")
            .build_str();
        assert_eq!(
            cmd,
            "pg_dump '-Fc' '--no-acl' '--no-owner' '-h' 'localhost' '-U' 'app' '--no-password' 'orders'"
        );
    }

    #[test]
    fn credential_quote_cannot_break_out() {
        // Gate G3: `'` inside a username stays inside its token.
        let cmd = PgIsReadyBuilder::new().username("o'brien").build_str();
        assert_eq!(cmd, "pg_isready '-U' 'o'\\''brien'");
    }

    #[test]
    fn pg_restore_shape() {
        let cmd = PgRestoreBuilder::new()
            .username("app")
            .database("orders")
            .no_owner()
            .clean()
            .if_exists()
            .build_str();
        assert_eq!(
            cmd,
            "pg_restore '-U' 'app' '-d' 'orders' '-O' '--clean' '--if-exists'"
        );
    }
}
