use super::{
    email::send_email,
    loader::{ChannelConfig, NotificationConfigLoader},
    message::NotificationMessage,
    senders,
    types::{NotificationScope, NotificationTrigger},
};
use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    models::Notifications as Notification, notification_delivery::NotificationDeliveryBeginParams,
};
use auto_di::singleton;
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct NotificationService {
    queries: Arc<FlashQueries>,
    client: Client,
    send_limit: Arc<Semaphore>,
    loader: Arc<NotificationConfigLoader>,
}

#[singleton]
impl NotificationService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .user_agent(concat!("OpenOxide/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap_or_default();

        let loader = Arc::new(NotificationConfigLoader);

        Self {
            queries,
            client,
            send_limit: Arc::new(Semaphore::new(5)),
            loader,
        }
    }

    /// Dispatches to every channel in `scope` that subscribed to `trigger`.
    pub async fn notify(
        &self,
        scope: NotificationScope,
        trigger: NotificationTrigger,
        msg: &NotificationMessage,
    ) {
        let loaded = match scope {
            NotificationScope::Organization(id) => {
                self.queries.list_notifications_by_organization(id).await
            }
            NotificationScope::AllOrganizations => self.queries.list_notifications().await,
        };

        let notifications = match loaded {
            Ok(items) => items,
            Err(error) => {
                tracing::error!(error = %error, "could not load notifications for dispatch");
                return;
            }
        };

        let targets: Vec<_> = notifications
            .into_iter()
            .filter(|n| trigger.is_enabled_for(n))
            .collect();

        if targets.is_empty() {
            return;
        }

        tracing::debug!(
            trigger = ?trigger,
            scope = ?scope,
            count = targets.len(),
            "dispatching notification"
        );

        for notification in targets {
            let limit = self.send_limit.clone();
            let loader = self.loader.clone();
            let client = self.client.clone();
            let msg = msg.clone();
            let queries = self.queries.clone();
            let trigger_name = trigger.as_str();
            let correlation_id = uuid::Uuid::new_v4().to_string();

            tokio::spawn(async move {
                let permit = match limit.acquire_owned().await {
                    Ok(permit) => permit,
                    Err(_) => return,
                };

                let name = notification.name.clone();
                let kind = notification.notification_type.clone();

                for attempt in 1..=3 {
                    let delivery_id = match queries
                        .notification_delivery_begin(&NotificationDeliveryBeginParams {
                            notification_id: notification.id,
                            organization_id: notification.organization_id,
                            trigger_name: trigger_name.to_string(),
                            correlation_id: correlation_id.clone(),
                            status: "PENDING".to_string(),
                            attempt,
                            title: msg.title.clone(),
                            body: msg.body.clone(),
                        })
                        .await
                    {
                        Ok(id) => id,
                        Err(error) => {
                            tracing::error!(%error, "could not persist notification delivery attempt");
                            break;
                        }
                    };
                    match Self::dispatch_one_static(&client, &loader, &notification, &msg).await {
                        Ok(()) => {
                            let _ = queries
                                .notification_delivery_finish_success(delivery_id)
                                .await;
                            break;
                        }
                        Err(error) => {
                            let _ = queries
                                .notification_delivery_finish_failure(&error, delivery_id)
                                .await;
                            tracing::warn!(notification = %name, provider = %kind, attempt, error = %error, "notification dispatch failed");
                            if attempt < 3 {
                                tokio::time::sleep(Duration::from_secs(1_u64 << (attempt - 1)))
                                    .await;
                            }
                        }
                    }
                }

                drop(permit);
            });
        }
    }

    pub async fn send_test(&self, id: i64, msg: &NotificationMessage) -> Result<(), String> {
        let notification = self
            .queries
            .get_notification_by_i_d(id)
            .await
            .map_err(|e| format!("could not load notification {id}: {e}"))?;

        Self::dispatch_one_static(&self.client, &self.loader, &notification, msg).await
    }

    async fn dispatch_one_static(
        client: &Client,
        loader: &NotificationConfigLoader,
        notification: &Notification,
        msg: &NotificationMessage,
    ) -> Result<(), String> {
        match loader.load(notification)? {
            ChannelConfig::Slack(cfg) => senders::send_slack(client, &cfg, msg).await,
            ChannelConfig::Telegram(cfg) => senders::send_telegram(client, &cfg, msg).await,
            ChannelConfig::Discord(cfg) => senders::send_discord(client, &cfg, msg).await,
            ChannelConfig::Email(cfg) => send_email(&cfg, msg).await,
            ChannelConfig::Resend(cfg) => senders::send_resend(client, &cfg, msg).await,
            ChannelConfig::Gotify(cfg) => senders::send_gotify(client, &cfg, msg).await,
            ChannelConfig::Ntfy(cfg) => senders::send_ntfy(client, &cfg, msg).await,
            ChannelConfig::Mattermost(cfg) => senders::send_mattermost(client, &cfg, msg).await,
            ChannelConfig::Teams(cfg) => senders::send_teams(client, &cfg, msg).await,
            ChannelConfig::Lark(cfg) => senders::send_lark(client, &cfg, msg).await,
            ChannelConfig::Pushover(cfg) => senders::send_pushover(client, &cfg, msg).await,
            ChannelConfig::Custom(cfg) => senders::send_custom(client, &cfg, msg).await,
        }
    }
}
