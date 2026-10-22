use crate::services::{
    compute::application::ApplicationRecord, compute::compose::ComposeRecord,
    compute::database::DatabaseRecord,
};
use crate::utils::docker::{NodeInspect, SwarmInfo};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Application(i64),
    Compose(i64),
    Database(i64),
    ProjectsList(i64),
    EnvironmentsProject(i64),
    /// Organization-scoped list caches: the active organization is part of
    /// the key so switching workspaces never serves another org's rows.
    DestinationsList(i64),
    RegistriesList(i64),
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
    Application(Box<ApplicationRecord>),
    Compose(Box<ComposeRecord>),
    Database(Box<DatabaseRecord>),
    ProjectsList(Vec<crate::flash_gen::models::Projects>),
    Environments(Vec<crate::flash_gen::models::Environments>),
    DestinationsList(Vec<crate::flash_gen::models::Destinations>),
    RegistriesList(Vec<crate::flash_gen::models::Registries>),
    VolumeBackups(Vec<crate::flash_gen::models::VolumeBackups>),
    SwarmInfo(SwarmInfo),
    SwarmNodes(Vec<NodeInspect>),
}
