import {createServiceClient, useConnectQuery} from './query';
import {MonitoringOperationsService} from '#/gen/connect/openoxide/api/v1/monitoring_operations_pb';

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

const monitoringClient = createServiceClient(MonitoringOperationsService);

/**
 * Container states for a server (legacy live: useMonitoringContainerStates).
 * GetContainerStates wraps the whole state list inside a single
 * `ResourceListResponse` item payload, so the parsed result is an array.
 * `null` (server not picked yet) disables the query — firing it with id 0
 * only produces a not-found round-trip.
 */
export function useMonitoringContainerStates(serverId: bigint | null) {
	return useConnectQuery<ContainerStatePayload[]>(
		['monitoring', 'container-states', (serverId ?? 0n).toString()],
		async () => {
			const response = await monitoringClient.getContainerStates({
				id: serverId ?? 0n,
				payload: '',
			});
			const raw = response.items[0]?.payload;
			return raw ? (JSON.parse(raw) as ContainerStatePayload[]) : [];
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

/**
 * Newest-first server metric samples for one server (agent history via the
 * panel's MonitoringOperations.GetServerMetrics — the agent's get_last_n
 * returns the newest point FIRST). `null` server id disables the query.
 */
export function useServerMetrics(serverId: bigint | null, limit = 50) {
	return useConnectQuery<ServerMetricPayload[]>(
		['monitoring', 'server-metrics', (serverId ?? 0n).toString(), limit],
		async () => {
			const response = await monitoringClient.getServerMetrics({
				id: serverId ?? 0n,
				payload: JSON.stringify({limit}),
			});
			return response.payload
				? (JSON.parse(response.payload) as ServerMetricPayload[])
				: [];
		},
		{enabled: serverId !== null, refetchInterval: 15_000},
	);
}
