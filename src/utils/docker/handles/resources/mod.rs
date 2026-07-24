use crate::utils::docker::{DockerCli, DockerResult};

pub use network::{
    NetworkConnectBuilder, NetworkCreate, NetworkDisconnectBuilder, NetworkPrune, NetworkQuery,
    NetworkRmBuilder,
};
pub use volume::{VolumeCreate, VolumePrune, VolumeQuery, VolumeRmBuilder};

pub struct NetworkHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> NetworkHandle<'a> {
    pub fn list(&self) -> NetworkQuery<'_> {
        NetworkQuery::new(self.0)
    }
    pub fn create(&self, name: impl Into<String>) -> NetworkCreate<'_> {
        NetworkCreate::new(self.0, name)
    }
    pub fn prune(&self) -> NetworkPrune<'_> {
        NetworkPrune::new(self.0)
    }
    pub fn rm(&self, name: impl Into<String>) -> NetworkRmBuilder<'_> {
        NetworkRmBuilder::new(self.0, name)
    }
    pub fn connect(
        &self,
        network: impl Into<String>,
        container: impl Into<String>,
    ) -> NetworkConnectBuilder<'_> {
        NetworkConnectBuilder::new(self.0, network, container)
    }
    pub fn disconnect(
        &self,
        network: impl Into<String>,
        container: impl Into<String>,
    ) -> NetworkDisconnectBuilder<'_> {
        NetworkDisconnectBuilder::new(self.0, network, container)
    }
    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.0.run(["network", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub struct VolumeHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> VolumeHandle<'a> {
    pub fn list(&self) -> VolumeQuery<'_> {
        VolumeQuery::new(self.0)
    }
    pub fn create(&self, name: impl Into<String>) -> VolumeCreate<'_> {
        VolumeCreate::new(self.0, name)
    }
    pub fn prune(&self) -> VolumePrune<'_> {
        VolumePrune::new(self.0)
    }
    pub fn rm(&self, name: impl Into<String>) -> VolumeRmBuilder<'_> {
        VolumeRmBuilder::new(self.0, name)
    }
    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.0.run(["volume", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod network;
pub mod volume;
