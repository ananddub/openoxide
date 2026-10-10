import {Code, ConnectError} from '@connectrpc/connect';
import {
	createServiceClient,
	useActiveOrgId,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	MonitoringAgentState,
	MonitoringService,
	type MonitoringContainerMetric,
	type MonitoringContainerState,
	type MonitoringMaintenanceWindow,
	type MonitoringServerMetric,
} from '#/gen/connect/openoxide/api/v1/monitoring_pb';

/** Container state point from the monitoring agent (ContainerStatePoint JSON). */
export type ContainerStatePayload = {
	container_id?: string;
	name?: string;
	state?: string;
	status?: string;
	application_id?: bigint;
	compose_id?: bigint;
	compose_project?: string;
	compose_service?: string;
	[key: string]: unknown;
};

const monitoringClient = createServiceClient(MonitoringService);

function toContainerStatePayload(
	state: MonitoringContainerState,
): ContainerStatePayload {
	return {
		container_id: state.containerId,
		name: state.name,
		state: state.state,
		status: state.status,
		application_id: state.applicationId,
		compose_id: state.composeId,
		compose_project: state.composeProject,
		compose_service: state.composeService,
	};
}

/** Container states for a server (legacy live: useMonitoringContainerStates). `null` (server not picked yet) disables the query — firing it with id 0 only produces a not-found round-trip. */
export function useMonitoringContainerStates(serverId: bigint | null) {
	return useConnectQuery<ContainerStatePayload[]>(
		['monitoring', 'container-states', (serverId ?? 0n).toString()],
		async () => {
			const response = await monitoringClient.getContainerStates({
				serverId: serverId ?? 0n,
			});
			return response.states.map(toContainerStatePayload);
		},
		{enabled: serverId !== null},
	);
}

/** Server host metric point from the monitoring agent (panel ServerMetric JSON).
 *  Units: `cpu`/`mem_used`/`disk_used` are percentages; `mem_used_gb`,
 *  `mem_total`, and `total_disk` are gigabytes; `uptime` is seconds since boot. */
export type ServerMetricPayload = {
	timestamp?: string;
	server_id: number;
	cpu: number;
	cpu_model: string;
	cpu_cores: number;
	cpu_physical_cores: number;
	cpu_speed: number;
	os: string;
	distro: string;
	kernel: string;
	arch: string;
	mem_used: number;
	mem_used_gb: number;
	mem_total: number;
	uptime: number;
	disk_used: number;
	total_disk: number;
	network_in: number;
	network_out: number;
};

function toServerMetricPayload(
	metric: MonitoringServerMetric,
): ServerMetricPayload {
	return {
		timestamp:
			metric.timestamp === undefined
				? undefined
				: String(metric.timestamp),
		server_id: Number(metric.serverId),
		cpu: metric.cpu,
		cpu_model: metric.cpuModel,
		cpu_cores: Number(metric.cpuCores),
		cpu_physical_cores: Number(metric.cpuPhysicalCores),
		cpu_speed: metric.cpuSpeed,
		os: metric.os,
		distro: metric.distro,
		kernel: metric.kernel,
		arch: metric.arch,
		mem_used: metric.memUsed,
		mem_used_gb: metric.memUsedGb,
		mem_total: metric.memTotal,
		uptime: Number(metric.uptime),
		disk_used: metric.diskUsed,
		total_disk: metric.totalDisk,
		network_in: metric.networkIn,
		network_out: metric.networkOut,
	};
}

/** Newest-first server metric samples for one server (agent history via the panel's MonitoringService.GetServerMetrics — the agent's get_last_n returns the newest point FIRST). The typed request has no limit field, so the page is trimmed client-side. `null` server id disables the query. */
export function useServerMetrics(
	serverId: bigint | null,
	limit = 50,
	refetchMs = 15_000,
) {
	return useConnectQuery<ServerMetricPayload[]>(
		['monitoring', 'server-metrics', (serverId ?? 0n).toString(), limit],
		async () => {
			const response = await monitoringClient.getServerMetrics({
				serverId: serverId ?? 0n,
			});
			return response.metrics.slice(0, limit).map(toServerMetricPayload);
		},
		{enabled: serverId !== null, refetchInterval: refetchMs},
	);
}

/** Message returned by an agent lifecycle action (start/stop/restart/reinstall). */
export type AgentActionResult = {
	action: string;
	success: boolean;
	message: string;
};

/** All four lifecycle verbs answer with the same `action` envelope, so one factory covers them. */
function useAgentLifecycleAction(
	method: 'startAgent' | 'stopAgent' | 'restartAgent' | 'reinstallAgent',
	verb: string,
	fallbackMessage: string,
) {
	return useConnectMutation<bigint, AgentActionResult>(
		async serverId => {
			const res = await monitoringClient[method]({serverId});
			const a = res.action;
			return {
				action: a?.action ?? verb,
				success: a?.success ?? true,
				message: a?.message ?? fallbackMessage,
			};
		},
		{invalidate: [['monitoring', 'agent-status']]},
	);
}

/**
 * Reinstall (update) the monitoring agent: pulls the latest agent image and
 * recreates the container. This is the "Update Agent" action — a running agent
 * on an old image is replaced with the current one.
 */
export function useReinstallAgent() {
	return useAgentLifecycleAction(
		'reinstallAgent',
		'reinstall',
		'agent updated',
	);
}

/** Restart the monitoring agent container (no image change). */
export function useRestartAgent() {
	return useAgentLifecycleAction(
		'restartAgent',
		'restart',
		'agent restarted',
	);
}

/** Start the agent container after a stop, or bring up an agent the panel installed but never started. */
export function useStartAgent() {
	return useAgentLifecycleAction('startAgent', 'start', 'agent started');
}

/** Stop the agent container. Registration and its token survive — this is not an uninstall. */
export function useStopAgent() {
	return useAgentLifecycleAction('stopAgent', 'stop', 'agent stopped');
}

/**
 * Rotate the server's agent token and read the plaintext back once — the panel
 * stores a hash, so a rotation is the only moment the value exists to show.
 * Rotation invalidates the token the running agent holds, so it is a deliberate
 * click and never part of loading the tab.
 */
export function useCreateAgentToken() {
	const organizationId = useActiveOrgId();
	return useConnectMutation<bigint, string>(
		async serverId => {
			const res = await monitoringClient.createAgentToken({
				serverId,
				organizationId,
			});
			return res.token;
		},
		{invalidate: [['monitoring', 'agent-status']]},
	);
}

/**
 * Registration state derived from the agent heartbeat (ONLINE within 30s of the
 * last heartbeat, OFFLINE once seen, NEVER_SEEN with no heartbeat at all).
 * `unregistered` is the calm form of the backend's NotFound reply: an agent
 * that was never installed, which is a setup prompt rather than a failure.
 * `last_seen_at` is epoch seconds.
 */
export type AgentStatusPayload = {
	server_id: number;
	state: 'online' | 'offline' | 'never_seen' | 'unregistered';
	last_seen_at: number | null;
	agent_version: string;
};

function unregisteredAgent(serverId: bigint): AgentStatusPayload {
	return {
		server_id: Number(serverId),
		state: 'unregistered',
		last_seen_at: null,
		agent_version: '',
	};
}

/**
 * Agent status for one server. Deliberately no `refetchInterval` — the panel
 * must not keep polling a $5 VPS's agent; the tab reads it once on open and a
 * lifecycle action invalidates it.
 */
export function useAgentStatus(serverId: bigint | null) {
	return useConnectQuery<AgentStatusPayload>(
		['monitoring', 'agent-status', (serverId ?? 0n).toString()],
		async () => {
			const id = serverId ?? 0n;
			try {
				const response = await monitoringClient.getAgentStatus({
					serverId: id,
				});
				const status = response.status;
				if (!status) return unregisteredAgent(id);
				return {
					server_id: Number(status.serverId),
					state:
						status.state === MonitoringAgentState.ONLINE
							? 'online'
							: status.state === MonitoringAgentState.OFFLINE
								? 'offline'
								: 'never_seen',
					last_seen_at:
						status.lastSeenAt === undefined
							? null
							: Number(status.lastSeenAt),
					agent_version: status.agentVersion ?? '',
				};
			} catch (error) {
				if (
					error instanceof ConnectError &&
					error.code === Code.NotFound
				) {
					return unregisteredAgent(id);
				}
				throw error;
			}
		},
		{enabled: serverId !== null},
	);
}

/** A maintenance window that mutes agent alerting. `starts_at`/`ends_at` are epoch seconds; `server_id` null means organization-wide. */
export type MaintenanceWindowPayload = {
	id: number;
	server_id: number | null;
	starts_at: number;
	ends_at: number;
	reason: string;
	created_at: number;
};

function toMaintenanceWindowPayload(
	window: MonitoringMaintenanceWindow,
): MaintenanceWindowPayload {
	return {
		id: Number(window.id),
		server_id:
			window.serverId === undefined ? null : Number(window.serverId),
		starts_at: Number(window.startsAt),
		ends_at: Number(window.endsAt),
		reason: window.reason,
		created_at: Number(window.createdAt),
	};
}

/**
 * Windows for one server. `ListMaintenanceWindows` is organization-scoped with
 * no server filter, so this keeps this server's windows plus the
 * organization-wide ones (`server_id` null) — the same rule the backend applies
 * when it decides whether alerting is muted. `null` server id disables it.
 */
export function useMaintenanceWindows(serverId: bigint | null) {
	return useConnectQuery<MaintenanceWindowPayload[]>(
		['monitoring', 'maintenance-windows', (serverId ?? 0n).toString()],
		async () => {
			const response = await monitoringClient.listMaintenanceWindows({});
			return response.windows
				.filter(
					window =>
						window.serverId === undefined || window.serverId === serverId,
				)
				.map(toMaintenanceWindowPayload);
		},
		{enabled: serverId !== null},
	);
}

/** New window over this server. Times are epoch seconds; the backend rejects `ends_at <= starts_at`. */
export function useCreateMaintenanceWindow() {
	return useConnectMutation<
		{
			serverId: bigint;
			startsAt: number;
			endsAt: number;
			reason: string;
		},
		number
	>(
		async ({serverId, startsAt, endsAt, reason}) => {
			const res = await monitoringClient.createMaintenanceWindow({
				serverId,
				startsAt: BigInt(startsAt),
				endsAt: BigInt(endsAt),
				reason,
			});
			return Number(res.id);
		},
		{invalidate: [['monitoring', 'maintenance-windows']]},
	);
}

/** Delete one window by id. */
export function useDeleteMaintenanceWindow() {
	return useConnectMutation<bigint, void>(
		async id => {
			await monitoringClient.deleteMaintenanceWindow({id});
		},
		{invalidate: [['monitoring', 'maintenance-windows']]},
	);
}

/**
 * CSV of up to 1000 stored samples, header first. A mutation rather than a
 * query: nothing polls this, the operator presses "Export" and gets a download.
 */
export function useExportServerMetrics() {
	return useConnectMutation<bigint, string>(async serverId => {
		const res = await monitoringClient.exportServerMetrics({serverId});
		return res.csv;
	});
}

/** One live Docker resource reading for a container, in the units the agent reports. */
export type ContainerStatsPayload = {
	timestamp: number;
	container_name: string;
	cpu_percent: number;
	memory_percent: number;
	memory_used_mb: number;
	memory_limit_mb: number;
	network_in_kbps: number;
	network_out_kbps: number;
	disk_read_mb: number;
	disk_write_mb: number;
};

function toContainerStatsPayload(
	metric: MonitoringContainerMetric,
): ContainerStatsPayload {
	return {
		timestamp: Number(metric.timestamp),
		container_name: metric.containerName,
		cpu_percent: metric.stats?.cpuPercent ?? 0,
		memory_percent: metric.stats?.memoryPercent ?? 0,
		memory_used_mb: metric.stats?.memoryUsedMb ?? 0,
		memory_limit_mb: metric.stats?.memoryLimitMb ?? 0,
		network_in_kbps: metric.stats?.netRxKbps ?? 0,
		network_out_kbps: metric.stats?.netTxKbps ?? 0,
		disk_read_mb: metric.stats?.blockReadMb ?? 0,
		disk_write_mb: metric.stats?.blockWriteMb ?? 0,
	};
}

/**
 * Live resource stats, read from the agent's Docker socket on demand. The agent
 * filters by name substring and answers one current sample per container, so
 * `''` is a snapshot of the whole box in a single round-trip. `null` disables
 * the query — this only runs while someone has the snapshot open.
 */
export function useContainerStats(
	serverId: bigint | null,
	appName: string | null,
) {
	const organizationId = useActiveOrgId();
	return useConnectQuery<ContainerStatsPayload[]>(
		[
			'monitoring',
			'container-stats',
			(serverId ?? 0n).toString(),
			appName ?? '',
		],
		async () => {
			const response = await monitoringClient.getContainerMetrics({
				serverId: serverId ?? 0n,
				organizationId,
				appName: appName ?? '',
				limit: 100n,
			});
			return response.metrics.map(toContainerStatsPayload);
		},
		{enabled: serverId !== null && appName !== null},
	);
}

/**
 * The latest sample in Prometheus exposition text, on demand. A server that has
 * never reported answers NotFound, which is an empty payload here — the panel
 * shows "no metrics yet" instead of an error banner.
 */
export function usePrometheusMetrics() {
	return useConnectMutation<bigint, string>(async serverId => {
		try {
			const res = await monitoringClient.getPrometheusMetrics({serverId});
			return res.prometheusText;
		} catch (error) {
			if (error instanceof ConnectError && error.code === Code.NotFound) {
				return '';
			}
			throw error;
		}
	});
}
