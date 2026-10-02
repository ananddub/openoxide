use crate::db::queries::FlashQueries;
use crate::flash_gen::{health_report::HealthReportCreateParams, models::HealthReports};
use crate::{
    core::config::Config,
    services::{
        infrastructure::server::RemoteServerService,
        observability::monitoring::agent_auth::MonitoringAgentAuth,
    },
};
use auto_di::singleton;
use dashmap::mapref::entry::Entry;
use os::exec::{AgentCommandRequest, AgentExecutor, AgentTransport, ExecError, ExecStreamEvent};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tonic::transport::{Certificate, ClientTlsConfig};
use tonic::{Request, metadata::MetadataValue};

pub mod proto {
    tonic::include_proto!("agent");
}

// ── In-memory metric snapshots (moved from the retired db::models layer) ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerMetric {
    pub timestamp: Option<i64>,
    pub server_id: i64,
    pub cpu: f64,
    pub cpu_model: String,
    pub cpu_cores: i64,
    pub cpu_physical_cores: i64,
    pub cpu_speed: f64,
    pub os: String,
    pub distro: String,
    pub kernel: String,
    pub arch: String,
    pub mem_used: f64,
    pub mem_used_gb: f64,
    pub mem_total: f64,
    pub uptime: i64,
    pub disk_used: f64,
    pub total_disk: f64,
    pub network_in: f64,
    pub network_out: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerMetric {
    pub id: Option<i64>,
    pub timestamp: i64,
    pub container_id: String,
    pub container_name: String,
    pub metrics_json: String,
    pub server_id: i64,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
}

pub struct MonitoringService {
    server_service: Arc<RemoteServerService>,
    config: Arc<Config>,
    agent_auth: Arc<MonitoringAgentAuth>,
    channels: Arc<dashmap::DashMap<i64, tonic::transport::Channel>>,
    repairs: Arc<dashmap::DashSet<i64>>,
}

#[singleton]
impl MonitoringService {
    pub async fn open_agent_shell(
        self: &Arc<Self>,
        server_id: i64,
        open: proto::ShellMessage,
    ) -> Result<
        (
            tokio::sync::mpsc::Sender<proto::ShellMessage>,
            tonic::Streaming<proto::ShellMessage>,
        ),
        String,
    > {
        use tokio_stream::wrappers::ReceiverStream;
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        sender
            .send(open)
            .await
            .map_err(|_| "Agent shell channel closed before opening".to_owned())?;
        let request = self
            .authenticated(server_id, ReceiverStream::new(receiver))
            .await?;
        let response = match self.client(server_id).await?.shell(request).await {
            Ok(response) => response,
            Err(error) => {
                // A pre-Shell Agent returns UNIMPLEMENTED. A recreated Agent can
                // also temporarily have a stale token and return UNAUTHENTICATED.
                // Both cases require the same SSH bootstrap recovery, which
                // rotates the token and recreates the container atomically.
                if matches!(
                    error.code(),
                    tonic::Code::Unimplemented
                        | tonic::Code::Unauthenticated
                        | tonic::Code::Unavailable
                ) {
                    let reason = if error.code() == tonic::Code::Unimplemented {
                        "outdated"
                    } else if error.code() == tonic::Code::Unauthenticated {
                        "authentication is out of sync"
                    } else {
                        "unavailable"
                    };
                    return Err(self.trigger_agent_repair(server_id, reason));
                }
                return Err(format!("Agent shell failed to open: {error}"));
            }
        };
        Ok((sender, response.into_inner()))
    }

    /// Proxies live container stats from the Agent's in-process Docker API
    /// stream. Each frame is one `docker stats --format {{json .}}`-shaped
    /// JSON document; no CLI subprocess is spawned anywhere on the path.
    pub async fn stream_container_stats(
        &self,
        server_id: i64,
        container_ids: Vec<String>,
        stream: bool,
    ) -> Result<tonic::Streaming<proto::StatsFrame>, String> {
        let request = self
            .authenticated(
                server_id,
                proto::StreamStatsRequest {
                    server_id,
                    container_ids,
                    stream,
                },
            )
            .await?;
        let response = self
            .client(server_id)
            .await?
            .stream_container_stats(request)
            .await
            .map_err(|error| format!("Agent stats stream failed to open: {error}"))?;
        let _ = self.agent_auth.touch_seen(server_id).await;
        Ok(response.into_inner())
    }

    pub fn new(
        server_service: Arc<RemoteServerService>,
        config: Arc<Config>,
        agent_auth: Arc<MonitoringAgentAuth>,
    ) -> Self {
        Self {
            server_service,
            config,
            agent_auth,
            channels: Arc::new(dashmap::DashMap::new()),
            repairs: Arc::new(dashmap::DashSet::new()),
        }
    }

    /// Drops any cached gRPC channel (and stale repair marker) for a server so
    /// a removed server stops occupying memory and a re-added one reconnects
    /// with a fresh lazy channel. Cheap and idempotent.
    pub fn forget_agent(&self, server_id: i64) {
        self.channels.remove(&server_id);
        self.repairs.remove(&server_id);
    }

    fn trigger_agent_repair(self: &Arc<Self>, server_id: i64, reason: &str) -> String {
        self.channels.remove(&server_id);
        let started = self.repairs.insert(server_id);
        if started {
            tracing::warn!(
                server_id,
                reason,
                "remote Agent requires repair; queuing automatic reinstall"
            );
            let service = self.clone();
            tokio::spawn(async move {
                match auto_di::resolve::<
                    crate::services::observability::monitoring::lifecycle::MonitoringLifecycleService,
                >()
                .await
                {
                    Ok(lifecycle) => {
                        if let Err(error) = lifecycle.reinstall_agent(server_id).await {
                            tracing::error!(server_id, %error, "automatic Agent reinstall failed");
                        }
                    }
                    Err(error) => tracing::error!(
                        server_id,
                        %error,
                        "could not resolve Agent lifecycle for automatic repair"
                    ),
                }
                service.channels.remove(&server_id);
                service.repairs.remove(&server_id);
            });
        }
        if started {
            format!("remote Agent {reason}; automatic repair started, retry shortly")
        } else {
            format!("remote Agent {reason}; automatic repair is already running, retry shortly")
        }
    }

    async fn client(
        &self,
        server_id: i64,
    ) -> Result<proto::agent_service_client::AgentServiceClient<tonic::transport::Channel>, String>
    {
        if let Some(channel) = self.channels.get(&server_id) {
            return Ok(proto::agent_service_client::AgentServiceClient::new(
                channel.clone(),
            ));
        }
        let server = self
            .server_service
            .get_by_id(server_id)
            .await
            .map_err(|_| format!("monitoring server {server_id} not found"))?;
        let is_local = matches!(
            server.ip_address.as_str(),
            "" | "0.0.0.0" | "localhost" | "127.0.0.1" | "openoxide-agent" | "openoxide-monitor"
        );
        let host = match server.ip_address.as_str() {
            "" | "0.0.0.0" | "localhost" | "127.0.0.1" | "openoxide-agent"
            | "openoxide-monitor" => {
                if std::path::Path::new("/.dockerenv").exists() {
                    "openoxide-agent"
                } else {
                    "127.0.0.1"
                }
            }
            other => other,
        };
        let mtls_base =
            std::env::var("OPENOXIDE_MTLS_DIR").unwrap_or_else(|_| "/app/data/mtls".to_string());
        let server_dir = std::path::Path::new(&mtls_base).join(format!("server-{server_id}"));
        let ca_file = server_dir.join("ca.crt");
        // Provision this server's isolated mTLS material on first use. Without
        // it the panel would fall back to plain HTTP against a TLS-only agent
        // and the handshake fails with an opaque "h2 protocol error". The DB
        // holds the CA, so this is idempotent (reuse) after the first call.
        if !ca_file.exists() && !is_local {
            if let Ok(store) = auto_di::resolve::<crate::utils::mtls::MtlsCaStore>().await {
                let sans = vec![
                    "openoxide-agent".to_string(),
                    "localhost".to_string(),
                    "127.0.0.1".to_string(),
                    host.to_string(),
                ];
                if let Err(error) = store
                    .ensure_server(server_id, std::path::Path::new(&mtls_base), &sans)
                    .await
                {
                    tracing::warn!(server_id, %error, "could not provision per-server mTLS");
                }
            }
        }
        let use_tls = ca_file.exists();
        let scheme = if use_tls { "https" } else { "http" };
        let mut endpoint =
            tonic::transport::Endpoint::from_shared(format!("{scheme}://{host}:50051"))
                .map_err(|error| format!("invalid monitoring agent address: {error}"))?
                .http2_keep_alive_interval(std::time::Duration::from_secs(30));
        if is_local {
            tracing::debug!(server_id, host, "using local Agent endpoint");
        }
        if use_tls {
            let ca = std::fs::read(&ca_file)
                .map_err(|error| format!("read server {server_id} TLS CA: {error}"))?;
            let domain = std::env::var("AGENT_TLS_DOMAIN").unwrap_or_else(|_| host.to_owned());
            let mut tls = ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(ca))
                .domain_name(domain);
            // mTLS: present this server's own panel-client identity. Each
            // server has an isolated CA, so its cert is only valid against it.
            let cert_file = server_dir.join("panel-client.crt");
            let key_file = server_dir.join("panel-client.key");
            if cert_file.exists() && key_file.exists() {
                let cert = std::fs::read(&cert_file)
                    .map_err(|error| format!("read server {server_id} panel cert: {error}"))?;
                let key = std::fs::read(&key_file)
                    .map_err(|error| format!("read server {server_id} panel key: {error}"))?;
                tls = tls.identity(tonic::transport::Identity::from_pem(cert, key));
            }
            endpoint = endpoint
                .tls_config(tls)
                .map_err(|error| format!("configure agent TLS: {error}"))?;
        }
        let channel = endpoint.connect_lazy();
        let channel = match self.channels.entry(server_id) {
            Entry::Occupied(entry) => entry.get().clone(),
            Entry::Vacant(entry) => entry.insert(channel).clone(),
        };
        Ok(proto::agent_service_client::AgentServiceClient::new(
            channel,
        ))
    }

    pub fn agent_executor(self: &Arc<Self>, server_id: i64) -> AgentExecutor {
        AgentExecutor::new(Arc::new(GrpcAgentTransport {
            service: Arc::clone(self),
            server_id,
        }))
    }

    pub async fn upload_file(
        &self,
        server_id: i64,
        source: &Path,
        destination: &str,
    ) -> Result<u64, String> {
        use sha2::Digest;
        use tokio::io::AsyncReadExt;
        use tokio_stream::wrappers::ReceiverStream;

        let total_size = tokio::fs::metadata(source)
            .await
            .map_err(|error| format!("stat upload source: {error}"))?
            .len();
        if total_size == 0 {
            return Err("cannot upload an empty file".into());
        }
        let capabilities = self.agent_info(server_id).await?;
        if capabilities.protocol_version != "3" {
            return Err("agent protocol version is unsupported; the agent must be upgraded to the latest build".into());
        }
        if !capabilities
            .capabilities
            .iter()
            .any(|value| value == "file_upload")
        {
            return Err("agent does not support file uploads".into());
        }
        if total_size > capabilities.max_upload_bytes {
            return Err("file exceeds the Agent upload limit".into());
        }
        let mut source_file = tokio::fs::File::open(source)
            .await
            .map_err(|error| format!("open upload source: {error}"))?;
        let mut hasher = sha2::Sha256::new();
        let mut hash_buffer = vec![0u8; 64 * 1024];
        loop {
            let count = source_file
                .read(&mut hash_buffer)
                .await
                .map_err(|error| format!("hash upload source: {error}"))?;
            if count == 0 {
                break;
            }
            hasher.update(&hash_buffer[..count]);
        }
        let checksum = format!("{:x}", hasher.finalize());
        let (sender, receiver) = tokio::sync::mpsc::channel(8);
        let source_path = source.to_path_buf();
        let destination = destination.to_owned();
        let producer = tokio::spawn(async move {
            let mut file = tokio::fs::File::open(&source_path)
                .await
                .map_err(|error| format!("open upload source: {error}"))?;
            // All metadata rides once in the header; data chunks stay raw.
            sender
                .send(proto::FileUploadMessage {
                    message: Some(proto::file_upload_message::Message::Header(
                        proto::FileUploadHeader {
                            server_id,
                            destination: destination.clone(),
                            total_size,
                            checksum_sha256: checksum.clone(),
                        },
                    )),
                })
                .await
                .map_err(|_| "upload stream cancelled".to_owned())?;
            let mut buffer = vec![0u8; 64 * 1024];
            loop {
                let count = file
                    .read(&mut buffer)
                    .await
                    .map_err(|error| format!("read upload source: {error}"))?;
                if count == 0 {
                    break;
                }
                sender
                    .send(proto::FileUploadMessage {
                        message: Some(proto::file_upload_message::Message::Data(
                            buffer[..count].to_vec(),
                        )),
                    })
                    .await
                    .map_err(|_| "upload stream cancelled".to_owned())?;
            }
            Ok::<(), String>(())
        });
        let request = self
            .authenticated(server_id, ReceiverStream::new(receiver))
            .await?;
        let result = self.client(server_id).await?.upload_file(request).await;
        let producer_result = producer.await.map_err(|error| {
            format!("Upload failed; please retry: producer task failed: {error}")
        })?;
        producer_result.map_err(|error| format!("Upload failed; please retry: {error}"))?;
        result
            .map(|response| response.into_inner().bytes_written)
            .map_err(|error| format!("Upload failed; please retry: {error}"))
    }

    pub async fn agent_info(&self, server_id: i64) -> Result<proto::AgentInfoResponse, String> {
        let request = self
            .authenticated(
                server_id,
                proto::AgentInfoRequest {
                    server_id,
                    update_image: String::new(),
                },
            )
            .await?;
        self.client(server_id)
            .await?
            .get_agent_info(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| format!("agent capability negotiation failed: {error}"))
    }

    /// Asks the agent to self-inspect and pull `image` through its Docker
    /// socket, so an up-to-date host is confirmed without spawning any CLI
    /// process on the remote machine.
    pub async fn agent_update_probe(
        &self,
        server_id: i64,
        image: &str,
    ) -> Result<proto::AgentInfoResponse, String> {
        let request = self
            .authenticated(
                server_id,
                proto::AgentInfoRequest {
                    server_id,
                    update_image: image.to_string(),
                },
            )
            .await?;
        self.client(server_id)
            .await?
            .get_agent_info(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| format!("agent update probe failed: {error}"))
    }

    async fn authenticated<T>(&self, server_id: i64, message: T) -> Result<Request<T>, String> {
        let raw_token = self
            .agent_auth
            .query_token(server_id)
            .await
            .map_err(|error| error.to_string())?
            .unwrap_or_else(|| self.config.metrics_token.clone());
        let token: MetadataValue<_> = raw_token
            .parse()
            .map_err(|_| "METRICS_TOKEN is not valid gRPC metadata".to_string())?;
        let mut request = Request::new(message);
        request.metadata_mut().insert("x-metrics-token", token);
        Ok(request)
    }

    pub async fn server_history(
        &self,
        server_id: i64,
        limit: i64,
    ) -> Result<Vec<ServerMetric>, String> {
        let mut client = self.client(server_id).await?;
        let request = self
            .authenticated(
                server_id,
                proto::GetMetricsRequest {
                    server_id,
                    limit: limit.clamp(1, 1000) as i32,
                },
            )
            .await?;
        let metrics = client
            .get_server_metrics(request)
            .await
            .map_err(|error| format!("agent metric query failed: {error}"))?
            .into_inner();
        let _ = self.agent_auth.touch_seen(server_id).await;
        let host = metrics.host;
        Ok(metrics
            .metrics
            .into_iter()
            .map(|metric| ServerMetric {
                timestamp: (metric.timestamp > 0).then_some(metric.timestamp),
                server_id,
                cpu: metric.cpu,
                cpu_model: host.as_ref().map_or_default(|host| host.cpu_model.clone()),
                cpu_cores: host.as_ref().map_or(0, |host| i64::from(host.cpu_cores)),
                cpu_physical_cores: host
                    .as_ref()
                    .map_or(0, |host| i64::from(host.cpu_physical_cores)),
                cpu_speed: host.as_ref().map_or_default(|host| host.cpu_speed),
                os: host.as_ref().map_or_default(|host| host.os.clone()),
                distro: host.as_ref().map_or_default(|host| host.distro.clone()),
                kernel: host.as_ref().map_or_default(|host| host.kernel.clone()),
                arch: host.as_ref().map_or_default(|host| host.arch.clone()),
                mem_used: if metric.mem_total_gb > 0.0 {
                    metric.mem_used_gb / metric.mem_total_gb * 100.0
                } else {
                    0.0
                },
                mem_used_gb: metric.mem_used_gb,
                mem_total: metric.mem_total_gb,
                uptime: metric.uptime as i64,
                disk_used: if metric.disk_total_gb > 0.0 {
                    metric.disk_used_gb / metric.disk_total_gb * 100.0
                } else {
                    0.0
                },
                total_disk: metric.disk_total_gb,
                network_in: metric.network_in,
                network_out: metric.network_out,
            })
            .collect())
    }

    pub async fn container_history(
        &self,
        server_id: i64,
        app_name: &str,
        limit: i64,
    ) -> Result<Vec<ContainerMetric>, String> {
        let mut client = self.client(server_id).await?;
        let request = self
            .authenticated(
                server_id,
                proto::GetContainerMetricsRequest {
                    server_id,
                    app_name: app_name.to_owned(),
                    limit: limit.clamp(1, 1000) as i32,
                },
            )
            .await?;
        let metrics = client
            .get_container_metrics(request)
            .await
            .map_err(|error| format!("agent container query failed: {error}"))?
            .into_inner()
            .metrics;
        let _ = self.agent_auth.touch_seen(server_id).await;
        Ok(metrics
            .into_iter()
            .map(|metric| ContainerMetric {
                id: (metric.id > 0).then_some(metric.id),
                timestamp: metric.timestamp,
                container_id: metric.container_id,
                container_name: metric.name,
                metrics_json: serde_json::json!({
                    "cpu_percent": metric.cpu_perc,
                    "memory_percent": metric.mem_perc,
                    "memory_used_mb": metric.mem_used_mb,
                    "memory_limit_mb": metric.mem_total_mb,
                    "net_rx_kbps": metric.net_in_mb * 1024.0,
                    "net_tx_kbps": metric.net_out_mb * 1024.0,
                    "block_read_mb": metric.block_read_mb,
                    "block_write_mb": metric.block_write_mb
                })
                .to_string(),
                server_id,
                application_id: (metric.application_id > 0).then_some(metric.application_id),
                compose_id: (metric.compose_id > 0).then_some(metric.compose_id),
            })
            .collect())
    }

    pub async fn container_states(
        &self,
        server_id: i64,
    ) -> Result<Vec<proto::ContainerStatePoint>, String> {
        let mut client = self.client(server_id).await?;
        let request = self
            .authenticated(
                server_id,
                proto::GetMetricsRequest {
                    server_id,
                    limit: 0,
                },
            )
            .await?;
        let states = client
            .get_container_states(request)
            .await
            .map_err(|error| format!("agent container state query failed: {error}"))?
            .into_inner()
            .containers;
        let _ = self.agent_auth.touch_seen(server_id).await;
        Ok(states)
    }

    pub async fn watch_container_states(
        &self,
        server_id: i64,
    ) -> Result<tonic::Streaming<proto::ContainerStatesResponse>, String> {
        let mut client = self.client(server_id).await?;
        let request = self
            .authenticated(server_id, proto::WatchRequest { server_id })
            .await?;
        client
            .watch_container_states(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| format!("agent lifecycle stream failed: {error}"))
    }

    pub async fn server_ids(&self) -> Result<Vec<i64>, String> {
        self.server_service
            .list_all_servers()
            .await
            .map(|servers| {
                servers
                    .into_iter()
                    .filter(|server| {
                        !matches!(
                            server.ip_address.as_str(),
                            "" | "0.0.0.0"
                                | "localhost"
                                | "127.0.0.1"
                                | "openoxide-agent"
                                | "openoxide-monitor"
                        )
                    })
                    .map(|server| server.id)
                    .collect()
            })
            .map_err(|error| error.to_string())
    }

    pub async fn remote_server_ids(&self) -> Result<Vec<i64>, String> {
        self.server_service
            .list_all_servers()
            .await
            .map(|servers| {
                servers
                    .into_iter()
                    .filter(|server| {
                        !matches!(
                            server.ip_address.as_str(),
                            "" | "0.0.0.0" | "localhost" | "127.0.0.1" | "openoxide-agent"
                        )
                    })
                    .map(|server| server.id)
                    .collect()
            })
            .map_err(|error| error.to_string())
    }

    pub async fn get_latest_metrics_per_server(&self) -> Result<Vec<ServerMetric>, String> {
        let servers = self
            .server_service
            .list_all_servers()
            .await
            .map_err(|error| error.to_string())?;
        let mut latest = Vec::new();
        for server in servers {
            let server_id = server.id;
            match self.server_history(server_id, 1).await {
                Ok(mut metrics) => latest.append(&mut metrics),
                Err(error) => tracing::debug!(server_id, %error, "monitoring agent unavailable"),
            }
        }
        Ok(latest)
    }

    pub async fn get_latest_container_metrics(&self) -> Result<Vec<ContainerMetric>, String> {
        let servers = self
            .server_service
            .list_all_servers()
            .await
            .map_err(|error| error.to_string())?;
        let mut latest = Vec::new();
        for server in servers {
            let server_id = server.id;
            match self.container_history(server_id, "", 1000).await {
                Ok(metrics) => {
                    let mut newest = std::collections::HashMap::new();
                    for metric in metrics {
                        newest.entry(metric.container_id.clone()).or_insert(metric);
                    }
                    latest.extend(newest.into_values());
                }
                Err(error) => tracing::debug!(server_id, %error, "monitoring agent unavailable"),
            }
        }
        Ok(latest)
    }
}

struct GrpcAgentTransport {
    service: Arc<MonitoringService>,
    server_id: i64,
}

impl std::fmt::Debug for GrpcAgentTransport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GrpcAgentTransport")
            .field("server_id", &self.server_id)
            .finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl AgentTransport for GrpcAgentTransport {
    async fn execute(
        &self,
        request: AgentCommandRequest,
        output: tokio::sync::mpsc::Sender<ExecStreamEvent>,
    ) -> os::exec::ExecResult<i32> {
        let mut command = os::exec::script::shell_single_quote(&request.program);
        for arg in request.args {
            command.push(' ');
            command.push_str(&os::exec::script::shell_single_quote(&arg));
        }
        let message = proto::ExecTaskRequest {
            server_id: self.server_id,
            container: String::new(),
            command,
            working_directory: String::new(),
            timeout_seconds: request.timeout_seconds,
            stdin: request.stdin,
        };
        let request = self
            .service
            .authenticated(self.server_id, message)
            .await
            .map_err(ExecError::Agent)?;
        let mut client = self
            .service
            .client(self.server_id)
            .await
            .map_err(ExecError::Agent)?;
        let mut stream = match client.exec_task(request).await {
            Ok(response) => response.into_inner(),
            Err(error)
                if matches!(
                    error.code(),
                    tonic::Code::Unimplemented
                        | tonic::Code::Unauthenticated
                        | tonic::Code::Unavailable
                ) =>
            {
                return Err(ExecError::Agent(self.service.trigger_agent_repair(
                    self.server_id,
                    match error.code() {
                        tonic::Code::Unimplemented => "outdated",
                        tonic::Code::Unauthenticated => "authentication is out of sync",
                        _ => "unavailable",
                    },
                )));
            }
            Err(error) => return Err(ExecError::Agent(error.to_string())),
        };
        let mut exit_code = None;
        loop {
            // An idle container emits no log lines, so awaiting the stream alone would park
            // here forever and keep the gRPC stream alive. The Agent only reaps its child
            // once the response stream is dropped, so bail out as soon as our consumer is
            // gone and let `stream` drop.
            let item = tokio::select! {
                biased;
                _ = output.closed() => return Err(ExecError::StreamCancelled),
                item = stream.message() => item
                    .map_err(|error| ExecError::Agent(error.to_string()))?,
            };
            let Some(item) = item else {
                break;
            };
            match proto::ExecTaskOutputKind::try_from(item.kind).unwrap_or_default() {
                proto::ExecTaskOutputKind::ExecTaskOutputStdout => {
                    output
                        .send(ExecStreamEvent::Stdout(item.data))
                        .await
                        .map_err(|_| ExecError::StreamCancelled)?;
                }
                proto::ExecTaskOutputKind::ExecTaskOutputStderr => {
                    output
                        .send(ExecStreamEvent::Stderr(item.data))
                        .await
                        .map_err(|_| ExecError::StreamCancelled)?;
                }
                proto::ExecTaskOutputKind::ExecTaskOutputExit => {
                    exit_code = Some(item.exit_code);
                }
                proto::ExecTaskOutputKind::ExecTaskOutputError => {
                    return Err(ExecError::Agent(item.error));
                }
                proto::ExecTaskOutputKind::ExecTaskOutputUnspecified => {}
            }
        }
        self.service
            .agent_auth
            .touch_seen(self.server_id)
            .await
            .ok();
        exit_code
            .ok_or_else(|| ExecError::Agent("agent closed task stream without exit code".into()))
    }
}

pub struct HealthService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl HealthService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn list_reports(&self, limit: i64) -> Result<Vec<HealthReports>, String> {
        self.queries
            .health_report_list(limit)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn record_report(&self, report: &HealthReports) -> Result<(), String> {
        self.queries
            .health_report_create(&HealthReportCreateParams {
                target_id: report.target_id,
                target_type: report.target_type.clone(),
                status: report.status.clone(),
                response_time_ms: report.response_time_ms,
                error_message: report.error_message.clone().unwrap_or_default(),
                created_at: chrono::Utc::now().timestamp(),
            })
            .await
            .map_err(|e| e.to_string())
    }
}
