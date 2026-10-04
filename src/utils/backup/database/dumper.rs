use os::archive::tar::TarCompression;
use os::archive::{GunzipBuilder, GzipBuilder};
use os::database::{
    MariadbAdminBuilder, MariadbClientBuilder, MariadbDumpBuilder, MongodumpBuilder,
    MongorestoreBuilder, MongoshBuilder, MysqlAdminBuilder, MysqlClientBuilder, MysqldumpBuilder,
    PgDumpBuilder, PgIsReadyBuilder, PgRestoreBuilder, RedisCliBuilder,
};
use os::exec::CommandExecutor;
use os::{
    OsCli,
    exec::script::{ScriptPipeline, sh},
};

#[derive(Debug, Clone)]
pub struct DbCredentials {
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone)]
pub struct ContainerTarget {
    pub service_name: String,
}

#[derive(Debug, Clone)]
pub enum DatabaseDumper {
    Postgres {
        creds: DbCredentials,
        target: ContainerTarget,
    },
    Mysql {
        creds: DbCredentials,
        target: ContainerTarget,
    },
    MariaDb {
        creds: DbCredentials,
        target: ContainerTarget,
    },
    Mongo {
        creds: DbCredentials,
        target: ContainerTarget,
    },
    LibSql {
        database: String,
        target: ContainerTarget,
    },
    Redis {
        target: ContainerTarget,
    },
}

impl DatabaseDumper {
    /// The in-container dump command as a compiled shell script. Each stage is
    /// a typed `os` builder (fully shell-quoted, Gate G3) written inline in
    /// `sh!`, so the code reads exactly like the shell it emits.
    pub fn inner_dump_command(&self, executor: &CommandExecutor) -> String {
        let script = match self {
            Self::Postgres { creds: c, .. } => sh!(
                pipe![
                    PgDumpBuilder::new()
                        .format_custom()
                        .no_acl()
                        .no_owner()
                        .host("localhost")
                        .username(&c.user)
                        .no_password()
                        .database(&c.database),
                    GzipBuilder::new()
                ];
            ),
            Self::Mysql { creds: c, .. } => sh!(
                // Password is delivered via MYSQL_PWD (see `secret_env`), not argv.
                pipe![
                    MysqldumpBuilder::new()
                        .default_character_set("utf8mb4")
                        .username("root")
                        .single_transaction()
                        .no_tablespaces()
                        .quick()
                        .database(&c.database),
                    GzipBuilder::new()
                ];
            ),
            Self::MariaDb { creds: c, .. } => sh!(
                pipe![
                    MariadbDumpBuilder::new()
                        .username(&c.user)
                        .single_transaction()
                        .quick()
                        .databases(&c.database),
                    GzipBuilder::new()
                ];
            ),
            Self::Mongo { creds: c, .. } => sh!(
                MongodumpBuilder::new()
                    .database(&c.database)
                    .username(&c.user)
                    .password(&c.password)
                    .archive()
                    .authentication_database("admin")
                    .gzip();
            ),
            Self::LibSql { database, .. } => sh!(
                pipe![
                    OsCli::new(executor)
                        .archive("/dev/null")
                        .tar()
                        .compression(TarCompression::None)
                        .create()
                        .to_stdout()
                        .entry_from("/var/lib/sqld", database),
                    GzipBuilder::new()
                ];
            ),
            Self::Redis { .. } => sh!(
                RedisCliBuilder::new().bgsave();
                cmd("sleep", "1");
                pipe![cmd("cat", "/data/dump.rdb"), GzipBuilder::new()];
            ),
        };
        ScriptPipeline::new().pipefail(true).cmd(script).compile()
    }

    /// The in-container restore command as a compiled shell script (reads the
    /// backup stream on stdin).
    pub fn inner_restore_command(&self, executor: &CommandExecutor) -> String {
        let script = match self {
            Self::Postgres { creds: c, .. } => sh!(
                pipe![
                    GunzipBuilder::new(),
                    PgRestoreBuilder::new()
                        .username(&c.user)
                        .database(&c.database)
                        .no_owner()
                        .clean()
                        .if_exists()
                ];
            ),
            Self::Mysql { creds: c, .. } => sh!(
                pipe![
                    GunzipBuilder::new(),
                    MysqlClientBuilder::new().username("root").database(&c.database)
                ];
            ),
            Self::MariaDb { creds: c, .. } => sh!(
                pipe![
                    GunzipBuilder::new(),
                    MariadbClientBuilder::new().username(&c.user).database(&c.database)
                ];
            ),
            Self::Mongo { creds: c, .. } => sh!(
                MongorestoreBuilder::new()
                    .username(&c.user)
                    .password(&c.password)
                    .authentication_database("admin")
                    .database(&c.database)
                    .archive()
                    .gzip()
                    .drop();
            ),
            Self::LibSql { .. } => sh!(
                pipe![
                    GunzipBuilder::new(),
                    OsCli::new(executor)
                        .archive("/dev/null")
                        .tar()
                        .compression(TarCompression::None)
                        .extract_to("/var/lib/sqld")
                        .from_stdin()
                ];
            ),
            Self::Redis { .. } => sh!(
                cmd("gunzip").stdout(os::exec::script::dsl::OutputTarget::file("/data/dump.rdb"));
            ),
        };
        ScriptPipeline::new().pipefail(true).cmd(script).compile()
    }

    /// Env var that carries the database password into the container so it
    /// never appears in process argv. `None` means the client command embeds
    /// its own auth or needs none.
    pub fn secret_env(&self) -> Option<(&'static str, String)> {
        match self {
            Self::Mysql { creds: c, .. } | Self::MariaDb { creds: c, .. } => {
                Some(("MYSQL_PWD", c.password.clone()))
            }
            _ => None,
        }
    }

    pub fn service_name(&self) -> &str {
        match self {
            Self::Postgres { target, .. }
            | Self::Mysql { target, .. }
            | Self::MariaDb { target, .. }
            | Self::Mongo { target, .. }
            | Self::LibSql { target, .. }
            | Self::Redis { target, .. } => &target.service_name,
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Postgres { .. } => "dump.gz",
            Self::Mysql { .. } | Self::MariaDb { .. } => "sql.gz",
            Self::Mongo { .. } => "archive.gz",
            Self::LibSql { .. } => "tar.gz",
            Self::Redis { .. } => "rdb.gz",
        }
    }

    pub fn container_label(&self) -> &'static str {
        "com.docker.swarm.service.name"
    }

    pub fn connection_check_command(&self) -> String {
        let script = match self {
            Self::Postgres { creds: c, .. } => sh!(
                PgIsReadyBuilder::new()
                    .host("localhost")
                    .username(&c.user)
                    .database(&c.database);
            ),
            Self::Mysql { .. } => sh!(
                MysqlAdminBuilder::new().username("root").ping();
            ),
            Self::MariaDb { creds: c, .. } => sh!(
                MariadbAdminBuilder::new().username(&c.user).ping();
            ),
            Self::Mongo { creds: c, .. } => sh!(
                MongoshBuilder::new()
                    .quiet()
                    .username(&c.user)
                    .password(&c.password)
                    .eval_script("db.runCommand({ping:1}).ok")
                    .authentication_database("admin");
            ),
            Self::LibSql { .. } => sh!(cmd("test", "-d", "/var/lib/sqld");),
            Self::Redis { .. } => sh!(RedisCliBuilder::new().ping();),
        };
        ScriptPipeline::new().cmd(script).compile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use os::exec::LocalExecutor;

    fn executor() -> CommandExecutor {
        CommandExecutor::Local(LocalExecutor::new())
    }

    fn pg(user: &str, password: &str, db: &str) -> DatabaseDumper {
        DatabaseDumper::Postgres {
            creds: DbCredentials {
                user: user.into(),
                password: password.into(),
                database: db.into(),
            },
            target: ContainerTarget {
                service_name: "svc".into(),
            },
        }
    }

    fn mysql(user: &str, password: &str, db: &str) -> DatabaseDumper {
        DatabaseDumper::Mysql {
            creds: DbCredentials {
                user: user.into(),
                password: password.into(),
                database: db.into(),
            },
            target: ContainerTarget {
                service_name: "svc".into(),
            },
        }
    }

    #[test]
    fn single_quote_in_credential_cannot_break_out() {
        // Gate G3: a `'` in a username must be escaped as '\'', not terminate
        // the surrounding single-quoted token inside the container shell.
        let dump = pg("o'brien", "ignored", "orders").inner_dump_command(&executor());
        assert!(
            dump.contains("'-U' 'o'\\''brien'"),
            "username not safely quoted, got: {dump}"
        );
        // The injected fragment must not survive as bare shell syntax.
        assert!(!dump.contains("o'brien '--no-password'"));
    }

    #[test]
    fn normal_credentials_are_wrapped_intact() {
        let dump = pg("app", "p@ss w0rd", "mydb").inner_dump_command(&executor());
        assert!(dump.contains("'-U' 'app'"));
        assert!(dump.contains("'mydb' | gzip"));
    }

    #[test]
    fn mysql_password_never_appears_in_dump_argv() {
        let dump = mysql("app", "s3cr3t", "orders").inner_dump_command(&executor());
        assert!(
            !dump.contains("s3cr3t"),
            "password leaked into script: {dump}"
        );
        assert_eq!(
            mysql("app", "s3cr3t", "orders").secret_env(),
            Some(("MYSQL_PWD", "s3cr3t".into()))
        );
    }

    #[test]
    fn pipefail_is_set_for_dump_chains() {
        let dump = pg("app", "x", "db").inner_dump_command(&executor());
        assert!(dump.starts_with("set -eo pipefail"));
    }

    fn creds(user: &str, password: &str, database: &str) -> DbCredentials {
        DbCredentials {
            user: user.into(),
            password: password.into(),
            database: database.into(),
        }
    }

    fn target() -> ContainerTarget {
        ContainerTarget {
            service_name: "svc".into(),
        }
    }

    #[test]
    fn golden_generated_scripts_for_every_engine() {
        let e = executor();

        let dump = |d: &DatabaseDumper| d.inner_dump_command(&e);
        let restore = |d: &DatabaseDumper| d.inner_restore_command(&e);
        let check = |d: &DatabaseDumper| d.connection_check_command();

        let postgres = pg("app", "pw", "mydb");
        assert_eq!(
            dump(&postgres),
            "set -eo pipefail\npg_dump '-Fc' '--no-acl' '--no-owner' '-h' 'localhost' '-U' 'app' '--no-password' 'mydb' | gzip\n"
        );
        assert_eq!(
            restore(&postgres),
            "set -eo pipefail\ngunzip | pg_restore '-U' 'app' '-d' 'mydb' '-O' '--clean' '--if-exists'\n"
        );
        assert_eq!(
            check(&postgres),
            "set -e\npg_isready '-h' 'localhost' '-U' 'app' '-d' 'mydb'\n"
        );

        let mysql = mysql("app", "pw", "orders");
        assert_eq!(
            dump(&mysql),
            "set -eo pipefail\nmysqldump '--default-character-set=utf8mb4' '-u' 'root' '--single-transaction' '--no-tablespaces' '--quick' 'orders' | gzip\n"
        );
        assert_eq!(
            restore(&mysql),
            "set -eo pipefail\ngunzip | mysql '-u' 'root' 'orders'\n"
        );
        assert_eq!(check(&mysql), "set -e\nmysqladmin '-u' 'root' 'ping'\n");

        let mariadb = DatabaseDumper::MariaDb {
            creds: creds("maria", "pw", "shop"),
            target: target(),
        };
        assert_eq!(
            dump(&mariadb),
            "set -eo pipefail\nmariadb-dump '--user=maria' '--single-transaction' '--quick' '--databases' 'shop' | gzip\n"
        );
        assert_eq!(
            restore(&mariadb),
            "set -eo pipefail\ngunzip | mariadb '-u' 'maria' 'shop'\n"
        );
        assert_eq!(
            check(&mariadb),
            "set -e\nmariadb-admin '--user=maria' 'ping'\n"
        );

        let mongo = DatabaseDumper::Mongo {
            creds: creds("mongo", "pw", "logs"),
            target: target(),
        };
        assert_eq!(
            dump(&mongo),
            "set -eo pipefail\nmongodump '-d' 'logs' '-u' 'mongo' '-p' 'pw' '--archive' '--authenticationDatabase' 'admin' '--gzip'\n"
        );
        assert_eq!(
            restore(&mongo),
            "set -eo pipefail\nmongorestore '--username' 'mongo' '--password' 'pw' '--authenticationDatabase' 'admin' '--db' 'logs' '--archive' '--gzip' '--drop'\n"
        );
        assert_eq!(
            check(&mongo),
            "set -e\nmongosh '--quiet' '-u' 'mongo' '-p' 'pw' '--eval' 'db.runCommand({ping:1}).ok' '--authenticationDatabase' 'admin'\n"
        );

        let libsql = DatabaseDumper::LibSql {
            database: "main".into(),
            target: target(),
        };
        assert_eq!(
            dump(&libsql),
            "set -eo pipefail\ntar '-cf' '-' '-C' '/var/lib/sqld' 'main' | gzip\n"
        );
        assert_eq!(
            restore(&libsql),
            "set -eo pipefail\ngunzip | tar '-xf' '-' '-C' '/var/lib/sqld'\n"
        );
        assert_eq!(check(&libsql), "set -e\ntest '-d' '/var/lib/sqld'\n");

        let redis = DatabaseDumper::Redis { target: target() };
        assert_eq!(
            dump(&redis),
            "set -eo pipefail\nredis-cli 'BGSAVE'\nsleep '1'\ncat '/data/dump.rdb' | gzip\n"
        );
        assert_eq!(
            restore(&redis),
            "set -eo pipefail\ngunzip > '/data/dump.rdb'\n"
        );
        assert_eq!(check(&redis), "set -e\nredis-cli 'PING'\n");
    }
}
