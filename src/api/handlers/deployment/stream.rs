use crate::utils::{
    builder::custom_type::{DeployEvent, DeployState, DeploySubscription},
    docker::DockerStreamEvent,
};
use axum::response::sse::Event;
use futures::Stream;
use serde_json::json;
use std::{convert::Infallible, pin::Pin};
use tokio::sync::{broadcast, mpsc};
use tokio::time::{Duration, MissedTickBehavior};

pub type DeploymentEventStream = Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>;

pub fn deployment_log_stream(receiver: mpsc::Receiver<String>) -> DeploymentEventStream {
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (receiver, keep_alive),
        |(mut receiver, mut keep_alive)| async move {
            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (receiver, keep_alive)));
                    }
                    received = receiver.recv() => {
                        match received {
                            Some(line) => {
                                let event = Event::default().event("log").data(json_payload(json!({
                                    "type": "log",
                                    "line": line,
                                })));
                                return Some((Ok(event), (receiver, keep_alive)));
                            }
                            None => return None,
                        }
                    }
                }
            }
        },
    ))
}

pub fn deployment_event_stream(subscription: DeploySubscription) -> DeploymentEventStream {
    let initial_state = subscription.state.borrow().clone();
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (subscription, Some(initial_state), keep_alive),
        |(mut subscription, initial_state, mut keep_alive)| async move {
            if let Some(state) = initial_state {
                return Some((Ok(state_event(state)), (subscription, None, keep_alive)));
            }

            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (subscription, None, keep_alive)));
                    }
                    changed = subscription.state.changed() => {
                        if changed.is_err() {
                            return None;
                        }

                        let state = subscription.state.borrow().clone();
                        return Some((Ok(state_event(state)), (subscription, None, keep_alive)));
                    }
                    received = subscription.events.recv() => {
                        match received {
                            Ok(event) => {
                                return Some((Ok(deploy_event(event)), (subscription, None, keep_alive)));
                            }
                            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                                return Some((Ok(lagged_event(skipped)), (subscription, None, keep_alive)));
                            }
                            Err(broadcast::error::RecvError::Closed) => {
                                return None;
                            }
                        }
                    }
                }
            }
        },
    ))
}

pub fn docker_stream(receiver: mpsc::Receiver<DockerStreamEvent>) -> DeploymentEventStream {
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (receiver, keep_alive),
        |(mut receiver, mut keep_alive)| async move {
            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (receiver, keep_alive)));
                    }
                    received = receiver.recv() => {
                        let event = match received {
                            Some(DockerStreamEvent::Stdout(bytes)) => docker_log_event("stdout", bytes),
                            Some(DockerStreamEvent::Stderr(bytes)) => docker_log_event("stderr", bytes),
                            None => return None,
                        };
                        return Some((Ok(event), (receiver, keep_alive)));
                    }
                }
            }
        },
    ))
}

pub fn docker_stats_stream(receiver: mpsc::Receiver<DockerStreamEvent>) -> DeploymentEventStream {
    let mut keep_alive = tokio::time::interval(Duration::from_secs(15));
    keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

    Box::pin(futures::stream::unfold(
        (receiver, keep_alive),
        |(mut receiver, mut keep_alive)| async move {
            loop {
                tokio::select! {
                    _ = keep_alive.tick() => {
                        return Some((Ok(keep_alive_event()), (receiver, keep_alive)));
                    }
                    received = receiver.recv() => {
                        let event = match received {
                            Some(DockerStreamEvent::Stdout(bytes)) => docker_stats_event(bytes),
                            Some(DockerStreamEvent::Stderr(bytes)) => docker_log_event("stderr", bytes),
                            None => return None,
                        };
                        return Some((Ok(event), (receiver, keep_alive)));
                    }
                }
            }
        },
    ))
}

fn keep_alive_event() -> Event {
    Event::default()
        .event("keep-alive")
        .data(json_payload(json!({
            "type": "keep-alive",
        })))
}

fn state_event(state: DeployState) -> Event {
    Event::default().event("state").data(json_payload(json!({
        "type": "state",
        "state": format!("{state:?}"),
    })))
}

fn deploy_event(event: DeployEvent) -> Event {
    match event {
        DeployEvent::StateChanged(state) => state_event(state),
        DeployEvent::Log(line) => Event::default().event("log").data(json_payload(json!({
            "type": "log",
            "line": line,
        }))),
        DeployEvent::Message(message) => {
            Event::default().event("message").data(json_payload(json!({
                "type": "message",
                "message": message,
            })))
        }
    }
}

fn lagged_event(skipped: u64) -> Event {
    Event::default().event("lagged").data(json_payload(json!({
        "type": "lagged",
        "skipped": skipped,
    })))
}

fn docker_log_event(stream: &str, bytes: Vec<u8>) -> Event {
    Event::default().event(stream).data(json_payload(json!({
        "type": stream,
        "line": String::from_utf8_lossy(&bytes),
    })))
}

fn docker_stats_event(bytes: Vec<u8>) -> Event {
    let line = String::from_utf8_lossy(&bytes);
    let value = serde_json::from_str::<serde_json::Value>(line.trim()).unwrap_or_else(|_| {
        json!({
            "raw": line,
        })
    });

    Event::default().event("stats").data(json_payload(json!({
        "type": "stats",
        "stats": value,
    })))
}

fn json_payload(value: serde_json::Value) -> String {
    serde_json::to_string(&value).unwrap_or_else(|_| "{}".into())
}
