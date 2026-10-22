use super::{sync::ProviderSyncBuilder, types::CloneProtocol};

pub struct CustomClient {
    url: String,
}

impl CustomClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
    pub fn repository(&self) -> CustomRepoBuilder<'_> {
        CustomRepoBuilder { url: &self.url }
    }
}

pub struct CustomRepoBuilder<'a> {
    url: &'a str,
}

impl<'a> CustomRepoBuilder<'a> {
    pub fn clone_url(&self, _protocol: CloneProtocol) -> String {
        self.url.to_string()
    }

    pub fn sync_into(
        &self,
        destination: &'a str,
        protocol: CloneProtocol,
    ) -> ProviderSyncBuilder<'a> {
        ProviderSyncBuilder::new(self.clone_url(protocol), destination)
    }
}
