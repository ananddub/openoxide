import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	MonitoringService,
	type MonitoringContainerState,
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
export function useServerMetrics(serverId: bigint | null, limit = 50) {
	return useConnectQuery<ServerMetricPayload[]>(
		['monitoring', 'server-metrics', (serverId ?? 0n).toString(), limit],
		async () => {
			const response = await monitoringClient.getServerMetrics({
				serverId: serverId ?? 0n,
			});
			return response.metrics.slice(0, limit).map(toServerMetricPayload);
		},
		{enabled: serverId !== null, refetchInterval: 15_000},
	);
}

/** Message returned by an agent lifecycle action (restart/reinstall). */
export type AgentActionResult = {
	action: string;
	success: boolean;
	message: string;
};

/**
 * Reinstall (update) the monitoring agent: pulls the latest agent image and
 * recreates the container. This is the "Update Agent" action — a running agent
 * on an old image is replaced with the current one.
 */
export function useReinstallAgent() {
	return useConnectMutation<bigint, AgentActionResult>(async serverId => {
		const res = await monitoringClient.reinstallAgent({serverId});
		const a = res.action;
		return {
			action: a?.action ?? 'reinstall',
			success: a?.success ?? true,
			message: a?.message ?? 'agent updated',
		};
	});
}

/** Restart the monitoring agent container (no image change). */
export function useRestartAgent() {
	return useConnectMutation<bigint, AgentActionResult>(async serverId => {
		const res = await monitoringClient.restartAgent({serverId});
		const a = res.action;
		return {
			action: a?.action ?? 'restart',
			success: a?.success ?? true,
			message: a?.message ?? 'agent restarted',
		};
	});
}
