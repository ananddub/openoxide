mod builder;
pub mod tar;
pub mod zip;

pub use builder::ArchiveBuilder;
pub use tar::{ArchiveCreateBuilder, ArchiveExtractBuilder, ArchiveListBuilder, TarBuilder};
pub use zip::{CompressionLevel, ZipBuilder, ZipError, ZipPathMode};

#[cfg(test)]
mod tests {
    use crate::utils::exec::script::IntoCommand;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};
    use crate::utils::os::OsCli;
    use crate::utils::os::archive::{CompressionLevel, ZipPathMode};

    #[test]
    fn archive_actions_build_reusable_commands() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let os = OsCli::new(&executor);

        assert_eq!(
            os.archive("/tmp/panel.tar.gz")
                .tar()
                .create()
                .entry_from("/tmp/staging", "db.sqlite3")
                .ignore_failed_reads()
                .build_str(),
            "tar '-czf' '/tmp/panel.tar.gz' '--ignore-failed-read' '-C' '/tmp/staging' 'db.sqlite3'"
        );
        assert_eq!(
            os.archive("/tmp/panel.tar.gz").tar().list().build_str(),
            "tar '-tzf' '/tmp/panel.tar.gz'"
        );
        assert_eq!(
            os.archive("/tmp/panel.tar.gz")
                .tar()
                .extract_to("/tmp/restore")
                .build_str(),
            "tar '-xzf' '/tmp/panel.tar.gz' '-C' '/tmp/restore'"
        );
        assert_eq!(
            os.archive("/tmp/panel.zip")
                .zip()
                .create_from("/tmp/source")
                .recursive()
                .path_mode(ZipPathMode::FileNamesOnly)
                .compression(CompressionLevel::Best)
                .exclude("*.log")
                .build_str(),
            "zip '-r' '-j' '-9' '-x' '*.log' '/tmp/panel.zip' '/tmp/source'"
        );
        assert_eq!(
            os.archive("/tmp/panel.zip")
                .zip()
                .extract_to("/tmp/restore")
                .overwrite()
                .quiet()
                .entry("config/app.toml")
                .exclude("*.tmp")
                .build_str(),
            "unzip '-o' '-q' '/tmp/panel.zip' 'config/app.toml' '-x' '*.tmp' '-d' '/tmp/restore'"
        );
        assert_eq!(
            os.archive("/tmp/panel.zip")
                .zip()
                .list()
                .names_only()
                .build_str(),
            "unzip '-Z1' '/tmp/panel.zip'"
        );
    }
}
