use crate::{
    core::config::Config,
    db::models::{container_metrics::ContainerMetric, server_metrics::ServerMetric},
    services::{monitoring::agent_auth::MonitoringAgentAuth, server::RemoteServerService},
};
use auto_di::singleton;
use dashmap::mapref::entry::Entry;
use os::exec::{AgentCommandRequest, AgentExecutor, AgentTransport, ExecError, ExecStreamEvent};
use std::path::Path;
use std::sync::Arc;
use tonic::transport::{Certificate, ClientTlsConfig};
use tonic::{Request, metadata::MetadataValue};

pub mod proto {
    tonic::include_proto!("agent");
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
                    crate::services::monitoring::lifecycle::MonitoringLifecycleService,
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
        let tls_ca = std::env::var("AGENT_TLS_CA_CERT")
            .ok()
            .filter(|v| !v.trim().is_empty());
        let use_tls = tls_ca.is_some();
        let scheme = if use_tls { "https" } else { "http" };
        let mut endpoint =
            tonic::transport::Endpoint::from_shared(format!("{scheme}://{host}:50051"))
                .map_err(|error| format!("invalid monitoring agent address: {error}"))?
                .http2_keep_alive_interval(std::time::Duration::from_secs(30));
        if is_local {
            tracing::debug!(server_id, host, "using local Agent endpoint");
        }
        if let Some(ca_path) = tls_ca {
            let ca =
                std::fs::read(ca_path).map_err(|error| format!("read agent TLS CA: {error}"))?;
            let domain = std::env::var("AGENT_TLS_DOMAIN").unwrap_or_else(|_| host.to_owned());
            endpoint = endpoint
                .tls_config(
                    ClientTlsConfig::new()
                        .ca_certificate(Certificate::from_pem(ca))
                        .domain_name(domain),
                )
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
        if capabilities.protocol_version != "1" {
            return Err("agent protocol version is unsupported".into());
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
        let checksum_for_stream = checksum.clone();
        let producer = tokio::spawn(async move {
            let mut file = tokio::fs::File::open(&source_path)
                .await
                .map_err(|error| format!("open upload source: {error}"))?;
            let mut cursor = 0u64;
            let mut buffer = vec![0u8; 64 * 1024];
            loop {
                let count = file
                    .read(&mut buffer)
                    .await
                    .map_err(|error| format!("read upload source: {error}"))?;
                if count == 0 {
                    break;
                }
                let count = count as u64;
                sender
                    .send(proto::FileUploadChunk {
                        server_id,
                        destination: destination.clone(),
                        data: buffer[..count as usize].to_vec(),
                        finish: cursor + count == total_size,
                        checksum_sha256: checksum_for_stream.clone(),
                        total_size,
                    })
                    .await
                    .map_err(|_| "upload stream cancelled".to_owned())?;
                cursor += count;
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
            .authenticated(server_id, proto::AgentInfoRequest { server_id })
            .await?;
        self.client(server_id)
            .await?
            .get_agent_info(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| format!("agent capability negotiation failed: {error}"))
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
            .into_inner()
            .metrics;
        let _ = self.agent_auth.touch_seen(server_id).await;
        Ok(metrics
            .into_iter()
            .map(|metric| ServerMetric {
                timestamp: chrono::DateTime::parse_from_rfc3339(&metric.timestamp)
                    .ok()
                    .map(|value| value.timestamp()),
                server_id,
                cpu: metric.cpu,
                cpu_model: metric.cpu_model,
                cpu_cores: i64::from(metric.cpu_cores),
                cpu_physical_cores: i64::from(metric.cpu_physical_cores),
                cpu_speed: metric.cpu_speed,
                os: metric.os,
                distro: metric.distro,
                kernel: metric.kernel,
                arch: metric.arch,
                mem_used: metric.mem_used,
                mem_used_gb: metric.mem_used_gb,
                mem_total: metric.mem_total,
                uptime: metric.uptime as i64,
                disk_used: metric.disk_used,
                total_disk: metric.total_disk,
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
                timestamp: chrono::DateTime::parse_from_rfc3339(&metric.timestamp)
                    .map(|value| value.timestamp())
                    .unwrap_or_default(),
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
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tx.send(proto::LifecycleControl {})
            .await
            .map_err(|_| "could not initialize lifecycle channel".to_string())?;
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                ticker.tick().await;
                if tx.send(proto::LifecycleControl {}).await.is_err() {
                    break;
                }
            }
        });
        let request = self
            .authenticated(server_id, tokio_stream::wrappers::ReceiverStream::new(rx))
            .await?;
        client
            .watch_container_states(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| format!("agent lifecycle stream failed: {error}"))
    }

    pub async fn server_ids(&self) -> Result<Vec<i64>, String> {
        self.server_service
            .list()
            .await
            .map(|servers| servers.into_iter().filter_map(|server| server.id).collect())
            .map_err(|error| error.to_string())
    }

    pub async fn remote_server_ids(&self) -> Result<Vec<i64>, String> {
        self.server_service
            .list()
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
                    .filter_map(|server| server.id)
                    .collect()
            })
            .map_err(|error| error.to_string())
    }

    pub async fn get_latest_metrics_per_server(&self) -> Result<Vec<ServerMetric>, String> {
        let servers = self
            .server_service
            .list()
            .await
            .map_err(|error| error.to_string())?;
        let mut latest = Vec::new();
        for server in servers {
            let Some(server_id) = server.id else { continue };
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
            .list()
            .await
            .map_err(|error| error.to_string())?;
        let mut latest = Vec::new();
        for server in servers {
            let Some(server_id) = server.id else { continue };
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
        while let Some(item) = stream
            .message()
            .await
            .map_err(|error| ExecError::Agent(error.to_string()))?
        {
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
