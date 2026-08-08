use super::{
    Alert, Application, AuditLog, Backup, CanCancel, CanCreate, CanDelete, CanDeploy, CanMonitor,
    CanRead, CanRestore, CanUpdate, CanWrite, Certificate, Database, Deployment, Environment,
    Groups, Invitation, Logs, Members, Monitoring, Notification, Organization, PermissionOperation,
    PermissionResource, Project, Registry, Schedule, Server, SshKey, Traefik, Users, VolumeBackup,
    Workspace,
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
allow!(Groups: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Members: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Invitation: CanRead, CanCreate, CanDelete, CanCancel);
allow!(Deployment: CanRead, CanCreate, CanCancel);
allow!(Backup: CanRead, CanCreate, CanUpdate, CanDelete, CanRestore);
allow!(VolumeBackup: CanRead, CanCreate, CanUpdate, CanDelete, CanRestore);
allow!(Schedule: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Notification: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Certificate: CanRead, CanCreate, CanUpdate, CanDelete);
allow!(Registry: CanRead, CanCreate, CanDelete);
allow!(SshKey: CanRead, CanCreate, CanDelete);
allow!(Logs: CanRead);
allow!(Monitoring: CanRead);
allow!(AuditLog: CanRead);
allow!(Workspace: CanUpdate);
allow!(Traefik: CanRead, CanWrite);
allow!(Alert: CanWrite);
