import {createServiceClient, useResourceQuery} from './query';
import {MonitoringService} from '#/gen/connect/openoxide/api/v1/monitoring_pb';

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

/**
 * Container states for a server (legacy live: useMonitoringContainerStates).
 * GetContainerState wraps the whole state list inside a single
 * ResourceResponse payload, so the parsed result is an array. The optional
 * serverId keeps the legacy hook signature (consumers pass it) and scopes
 * the query cache per server; it maps to ResourceRequest.id.
 */
export function useMonitoringContainerStates(serverId: bigint = 0n) {
	return useResourceQuery<ContainerStatePayload[]>(
		['monitoring', 'container-states', serverId.toString()],
		() => monitoringClient.getContainerState({id: serverId, payload: ''}),
	);
}
