use super::{
    Alert, Application, CanCreate, CanDelete, CanDeploy, CanMonitor, CanRead, CanUpdate, CanWrite,
    Database, Environment, Organization, PermissionOperation, PermissionResource, Project, Server,
    Traefik, Users,
};

pub trait Allows<O: PermissionOperation>: PermissionResource {}

macro_rules! allow {
    ($resource:ty: $($operation:ty),+ $(,)?) => {
        $(impl Allows<$operation> for $resource {})+
    };
}

allow!(Project: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Server: CanRead, CanCreate, CanUpdate, CanDelete, CanMonitor);
allow!(Application: CanRead, CanCreate, CanUpdate, CanDelete, CanDeploy, CanMonitor);
allow!(Database: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Environment: CanRead, CanWrite);
allow!(Organization: CanRead, CanWrite);
allow!(Users: CanRead, CanWrite);
allow!(Traefik: CanRead, CanWrite);
allow!(Alert: CanWrite);
