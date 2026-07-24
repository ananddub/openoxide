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
    pub fn inner_dump_command(&self) -> String {
        match self {
            Self::Postgres { creds: c, .. } => format!(
                "set -o pipefail; pg_dump -Fc --no-acl --no-owner -h localhost -U '{user}' --no-password '{db}' | gzip",
                user = c.user,
                db = c.database,
            ),
            Self::Mysql { creds: c, .. } => format!(
                "set -o pipefail; mysqldump --default-character-set=utf8mb4 -u 'root' --password='{password}' --single-transaction --no-tablespaces --quick '{db}' | gzip",
                password = c.password,
                db = c.database,
            ),
            Self::MariaDb { creds: c, .. } => format!(
                "set -o pipefail; mariadb-dump --user='{user}' --password='{password}' --single-transaction --quick --databases {db} | gzip",
                user = c.user,
                password = c.password,
                db = c.database,
            ),
            Self::Mongo { creds: c, .. } => format!(
                "set -o pipefail; mongodump -d '{db}' -u '{user}' -p '{password}' --archive --authenticationDatabase admin --gzip",
                db = c.database,
                user = c.user,
                password = c.password,
            ),
            Self::LibSql { database, .. } => {
                format!("tar cf - -C /var/lib/sqld {db} | gzip", db = database,)
            }
            Self::Redis { .. } => {
                "redis-cli BGSAVE && sleep 1 && cat /data/dump.rdb | gzip".to_string()
            }
        }
    }

    pub fn inner_restore_command(&self) -> String {
        match self {
            Self::Postgres { creds: c, .. } => format!(
                "set -o pipefail; gunzip | pg_restore -U '{user}' -d '{db}' -O --clean --if-exists",
                user = c.user,
                db = c.database,
            ),
            Self::Mysql { creds: c, .. } => format!(
                "set -o pipefail; gunzip | mysql -u 'root' --password='{password}' '{db}'",
                password = c.password,
                db = c.database,
            ),
            Self::MariaDb { creds: c, .. } => format!(
                "set -o pipefail; gunzip | mariadb -u '{user}' --password='{password}' '{db}'",
                user = c.user,
                password = c.password,
                db = c.database,
            ),
            Self::Mongo { creds: c, .. } => format!(
                "set -o pipefail; mongorestore --username '{user}' --password '{password}' --authenticationDatabase admin --db '{db}' --archive --gzip --drop",
                user = c.user,
                password = c.password,
                db = c.database,
            ),
            Self::LibSql { .. } => {
                "set -o pipefail; gunzip | tar xf - -C /var/lib/sqld".to_string()
            }
            Self::Redis { .. } => "gunzip > /data/dump.rdb".to_string(),
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
        match self {
            Self::Postgres { creds: c, .. } => format!(
                "pg_isready -h localhost -U '{user}' -d '{db}'",
                user = c.user,
                db = c.database,
            ),
            Self::Mysql { creds: c, .. } => format!(
                "mysqladmin -u root --password='{password}' ping",
                password = c.password,
            ),
            Self::MariaDb { creds: c, .. } => format!(
                "mariadb-admin --user='{user}' --password='{password}' ping",
                user = c.user,
                password = c.password,
            ),
            Self::Mongo { creds: c, .. } => format!(
                "mongosh --quiet -u '{user}' -p '{password}' --eval 'db.runCommand({{ping:1}}).ok' --authenticationDatabase admin",
                user = c.user,
                password = c.password,
            ),
            Self::LibSql { .. } => "test -d /var/lib/sqld".to_string(),
            Self::Redis { .. } => "redis-cli PING".to_string(),
        }
    }
}
