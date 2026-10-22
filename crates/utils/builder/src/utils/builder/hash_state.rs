use crate::utils::builder::custom_type::{
    ActiveDeploySnapshot, AppDeploy, DeployEvent, DeployState, DeploySubscription, IdType,
};
use auto_di::singleton;
use dashmap::DashMap;
use tokio::sync::{broadcast, watch};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct ApplicationState {
    pub hashmap: DashMap<IdType, AppDeploy>,
}
#[singleton]
impl ApplicationState {
    pub fn new() -> Self {
        Self {
            hashmap: DashMap::new(),
        }
    }
    pub fn set_state(&self, app_id: IdType, state: AppDeploy) {
        self.hashmap.insert(app_id, state);
    }

    pub fn add_default(&self, app_id: IdType, env_id: i64, project_id: i64) {
        let app_deploy = AppDeploy {
            app_id,
            project_id,
            env_id,
            state: watch::channel(DeployState::Queue).0,
            broadcast: broadcast::channel(5).0,
            cancellation_token: CancellationToken::new(),
        };
        self.hashmap.insert(app_id, app_deploy);
    }

    pub fn ensure_default(&self, app_id: IdType, env_id: i64, project_id: i64) {
        self.hashmap.entry(app_id).or_insert_with(|| AppDeploy {
            app_id,
            project_id,
            env_id,
            state: watch::channel(DeployState::Queue).0,
            broadcast: broadcast::channel(5).0,
            cancellation_token: CancellationToken::new(),
        });
    }

    pub fn reset_default(&self, app_id: IdType, env_id: i64, project_id: i64) {
        let app_deploy = AppDeploy {
            app_id,
            project_id,
            env_id,
            state: watch::channel(DeployState::Queue).0,
            broadcast: broadcast::channel(5).0,
            cancellation_token: CancellationToken::new(),
        };
        self.hashmap.insert(app_id, app_deploy);
    }

    pub fn remove_state(&self, app_id: IdType) {
        self.hashmap.remove(&app_id);
    }

    pub fn stop(&self, app_id: AppDeploy) -> Result<(), String> {
        self.hashmap
            .get(&app_id.app_id)
            .map(|entry| {
                entry.cancellation_token.cancel();
            })
            .ok_or_else(|| "AppDeploy not found".to_string())
    }

    pub fn cancel_by_id(&self, app_id: IdType) -> bool {
        self.hashmap
            .get(&app_id)
            .map(|entry| {
                entry.cancellation_token.cancel();
            })
            .is_some()
    }

    pub fn cancellation_token(&self, app_id: IdType) -> Option<CancellationToken> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.cancellation_token.clone())
    }

    pub fn state_recv(&self, app_id: IdType) -> Option<watch::Receiver<DeployState>> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.state.subscribe())
    }

    pub fn broadcast_recv(&self, app_id: IdType) -> Option<broadcast::Receiver<DeployEvent>> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.broadcast.subscribe())
    }

    pub fn subscribe(&self, app_id: IdType) -> Option<DeploySubscription> {
        self.hashmap.get(&app_id).map(|entry| DeploySubscription {
            state: entry.state.subscribe(),
            events: entry.broadcast.subscribe(),
        })
    }

    pub fn active_deployments(&self) -> Vec<ActiveDeploySnapshot> {
        self.hashmap
            .iter()
            .map(|entry| {
                let deploy = entry.value();
                ActiveDeploySnapshot {
                    id: deploy.app_id,
                    project_id: deploy.project_id,
                    env_id: deploy.env_id,
                    state: deploy.state.subscribe().borrow().clone(),
                }
            })
            .collect()
    }

    pub fn get_broadcast_send(&self, app_id: IdType) -> Option<broadcast::Sender<DeployEvent>> {
        self.hashmap
            .get(&app_id)
            .map(|entry| entry.broadcast.clone())
    }
    pub fn get_state_send(&self, app_id: IdType) -> Option<watch::Sender<DeployState>> {
        self.hashmap.get(&app_id).map(|entry| entry.state.clone())
    }

    pub fn send_state(&self, app_id: IdType, state: DeployState) -> Result<(), String> {
        let Some(entry) = self.hashmap.get(&app_id) else {
            return Err("AppDeploy not found".to_string());
        };
        let _ = entry.state.send(state.clone());
        let _ = entry.broadcast.send(DeployEvent::StateChanged(state));
        Ok(())
    }

    pub fn send_event(&self, app_id: IdType, event: DeployEvent) -> Result<(), String> {
        let Some(entry) = self.hashmap.get(&app_id) else {
            return Err("AppDeploy not found".to_string());
        };
        let _ = entry.broadcast.send(event);
        Ok(())
    }

    pub fn send_log(&self, app_id: IdType, line: impl Into<String>) -> Result<(), String> {
        self.send_event(app_id, DeployEvent::Log(line.into()))
    }

    pub fn send_broadcast(&self, app_id: IdType, event: String) -> Result<(), String> {
        self.send_event(app_id, DeployEvent::Message(event))
    }
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self::new()
    }
}
