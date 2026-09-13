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
