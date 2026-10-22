use crate::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RcloneCommand {
        default = Rcat;

        Rcat => "rcat",
        Cat => "cat",
        Lsf => "lsf",
        Copyto => "copyto",
        Sync => "sync",
        Delete => "delete",
        Purge => "purge",
        Check => "check",
    }
}
