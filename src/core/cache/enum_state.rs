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
    ProjectsList(Vec<crate::flash_gen::models::Projects>),
    Environments(Vec<crate::flash_gen::models::Environments>),
    ServersList(Vec<crate::flash_gen::models::Servers>),
    SshKeysList(Vec<crate::flash_gen::models::SshKeys>),
    DestinationsList(Vec<crate::flash_gen::models::Destinations>),
    RegistriesList(Vec<crate::flash_gen::registries::RegistriesGetAllRow>),
    VolumeBackups(Vec<crate::flash_gen::models::VolumeBackups>),
    SwarmInfo(SwarmInfo),
    SwarmNodes(Vec<NodeInspect>),
}
