//! MongoDB client tools: `mongodump`, `mongorestore`, `mongosh`.

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

client_builder!(MongodumpBuilder, "mongodump");
client_builder!(MongorestoreBuilder, "mongorestore");
client_builder!(MongoshBuilder, "mongosh");

impl MongodumpBuilder {
    /// `-d <db>` — database to dump.
    pub fn database(self, db: impl Into<String>) -> Self {
        self.pair("-d", db)
    }
    /// `-u <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-u", user)
    }
    /// `-p <pw>`.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.pair("-p", pw)
    }
    /// `--archive` — read/write the dump as a single stream on stdin/stdout.
    pub fn archive(self) -> Self {
        self.flag("--archive")
    }
    /// `--authenticationDatabase <db>`.
    pub fn authentication_database(self, db: impl Into<String>) -> Self {
        self.pair("--authenticationDatabase", db)
    }
    /// `--gzip` — compress the archive stream.
    pub fn gzip(self) -> Self {
        self.flag("--gzip")
    }
}

impl MongorestoreBuilder {
    /// `--username <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("--username", user)
    }
    /// `--password <pw>`.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.pair("--password", pw)
    }
    /// `--authenticationDatabase <db>`.
    pub fn authentication_database(self, db: impl Into<String>) -> Self {
        self.pair("--authenticationDatabase", db)
    }
    /// `--db <db>` — target database.
    pub fn database(self, db: impl Into<String>) -> Self {
        self.pair("--db", db)
    }
    /// `--archive` — read the dump as a single stream from stdin.
    pub fn archive(self) -> Self {
        self.flag("--archive")
    }
    /// `--gzip` — decompress the archive stream.
    pub fn gzip(self) -> Self {
        self.flag("--gzip")
    }
    /// `--drop` — drop existing collections before restoring.
    pub fn drop(self) -> Self {
        self.flag("--drop")
    }
}

impl MongoshBuilder {
    /// `--quiet` — suppress the connection banner.
    pub fn quiet(self) -> Self {
        self.flag("--quiet")
    }
    /// `-u <user>`.
    pub fn username(self, user: impl Into<String>) -> Self {
        self.pair("-u", user)
    }
    /// `-p <pw>`.
    pub fn password(self, pw: impl Into<String>) -> Self {
        self.pair("-p", pw)
    }
    /// `--authenticationDatabase <db>`.
    pub fn authentication_database(self, db: impl Into<String>) -> Self {
        self.pair("--authenticationDatabase", db)
    }
    /// `--eval '<js>'` — JavaScript to run against the connected database.
    pub fn eval_script(self, js: impl Into<String>) -> Self {
        self.pair("--eval", js)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mongodump_stream_shape() {
        let cmd = MongodumpBuilder::new()
            .database("orders")
            .username("admin")
            .archive()
            .authentication_database("admin")
            .gzip()
            .build_str();
        assert_eq!(
            cmd,
            "mongodump '-d' 'orders' '-u' 'admin' '--archive' '--authenticationDatabase' 'admin' '--gzip'"
        );
    }

    #[test]
    fn eval_script_is_one_quoted_token() {
        // The JS body contains `{}`, `(`, `)` — all must stay inert.
        let cmd = MongoshBuilder::new()
            .quiet()
            .eval_script("db.runCommand({ping:1}).ok")
            .build_str();
        assert_eq!(
            cmd,
            "mongosh '--quiet' '--eval' 'db.runCommand({ping:1}).ok'"
        );
    }
}
