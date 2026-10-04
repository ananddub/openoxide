use std::sync::Arc;

use auto_di::singleton;

use crate::services::integrations::git::webhooks::{WebhookCredentialStore, WebhookDispatcher};

pub struct WebhookController {
    pub(super) credentials: Arc<WebhookCredentialStore>,
    pub(super) dispatcher: Arc<WebhookDispatcher>,
}

#[singleton]
impl WebhookController {
    fn new(credentials: Arc<WebhookCredentialStore>, dispatcher: Arc<WebhookDispatcher>) -> Self {
        Self {
            credentials,
            dispatcher,
        }
    }
}
