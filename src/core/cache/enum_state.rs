use crate::services::{
    application::ApplicationRecord, compose::ComposeRecord, database::DatabaseRecord,
};
use crate::utils::docker::{NodeInspect, SwarmInfo};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Application(i64),
    Compose(i64),
    Database(i64),
    ProjectsList(i64),
    EnvironmentsProject(i64),
    ServersList,
    SshKeysList,
    DestinationsList,
    RegistriesList,
    VolumeBackups,
    SchedulesCompose(i64),
    SchedulesApp(i64),
    DomainsCompose(i64),
    DomainsApp(i64),
    /// Keyed by server id — None means the local engine.
    SwarmInfo(Option<i64>),
    SwarmNodes(Option<i64>),
}

#[derive(Debug, Clone)]
pub enum CacheEnum {
    Application(ApplicationRecord),
    Compose(ComposeRecord),
    Database(DatabaseRecord),
    ProjectsList(Vec<crate::db::models::projects::Project>),
    Environments(Vec<crate::db::models::environments::Environment>),
    ServersList(Vec<crate::db::models::servers::Server>),
    SshKeysList(Vec<crate::db::models::ssh_keys::SshKey>),
    DestinationsList(Vec<crate::db::models::destinations::Destination>),
    RegistriesList(Vec<crate::db::models::registries::Registry>),
    VolumeBackups(Vec<crate::db::models::volume_backups::VolumeBackup>),
    SwarmInfo(SwarmInfo),
    SwarmNodes(Vec<NodeInspect>),
}
