use std::sync::Arc;

use crate::services::notification::{
    NotificationLevel, NotificationMessage, NotificationScope, NotificationService,
    NotificationTrigger,
};

pub fn dispatch_startup(service: Arc<NotificationService>) {
    let message = NotificationMessage::new(
        "Rustploy Instance Started",
        "Rustploy control panel server has successfully initialized and is operational.",
    )
    .level(NotificationLevel::Info);

    tokio::spawn(async move {
        service
            .notify(
                NotificationScope::AllOrganizations,
                NotificationTrigger::PanelRestart,
                &message,
            )
            .await;
    });
}
