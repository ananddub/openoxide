import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
import {readResourceList} from './resource';
// Only SwarmOperationsService is served by the backend (SwarmService has no
// controller), so the live layer targets the operations variant: cluster
// info, nodes and join/leave flows. Settings and network RPCs from
// swarm.proto are not wired up server-side.
import {SwarmOperationsService} from '#/gen/connect/openoxide/api/v1/swarm_operations_pb';

/** Swarm node summary (backend NodeDto JSON). */
export type SwarmNodePayload = {
	id: string;
	hostname: string;
	status: string;
	availability: string;
	role: string;
	is_leader: boolean;
	reachability: string;
	ip_address: string;
	engine_version: string;
	[key: string]: unknown;
};

const swarmClient = createServiceClient(SwarmOperationsService);

/**
 * Swarm payloads carry an optional server id; null/undefined targets the
 * panel's local Docker engine.
 */
function toServerId(value: number | bigint | null | undefined): number | null {
	return value === null || value === undefined ? null : Number(value);
}

/**
 * Swarm nodes for a server (legacy REST: GET /swarm/nodes). The backend
 * takes `{server_id}` in the request payload; null lists the local cluster.
 */
export function useSwarmNodeList(serverId: number | bigint | null = null) {
	return useResourceListQuery<SwarmNodePayload>(
		['swarm', 'nodes', String(serverId)],
		() => swarmClient.listNodes({payload: JSON.stringify({server_id: toServerId(serverId)})}),
	);
}

/** Update a swarm node's availability (`active` | `pause` | `drain`). */
export function useSwarmNodeAvailabilityUpdate() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string; availability: string}) =>
			swarmClient.updateNodeAvailability({
				payload: JSON.stringify({
					server_id: toServerId(input.server_id),
					node_id: input.node_id,
					availability: input.availability,
				}),
			}),
		// Docker swarm state is agent-side — the CDC stream never sees it —
		// so these invalidate the swarm queries directly.
		{invalidate: [['swarm']]},
	);
}

/** Force-remove a swarm node from the cluster by node id. */
export function useSwarmNodeRemove() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string}) =>
			swarmClient.removeNode({
				payload: JSON.stringify({
					server_id: toServerId(input.server_id),
					node_id: input.node_id,
				}),
			}),
		{invalidate: [['swarm']]},
	);
}

/** Swarm nodes for a server as a plain promise (null server = local engine). */
export async function swarmNodes(
	serverId: number | bigint | null = null,
): Promise<SwarmNodePayload[]> {
	const response = await swarmClient.listNodes({
		payload: JSON.stringify({server_id: toServerId(serverId)}),
	});
	return readResourceList<SwarmNodePayload>(response);
}

/** Swarm cluster info for a server (null = local Docker engine). */
export async function swarmInfo(
	serverId: number | bigint | null = null,
): Promise<Record<string, unknown>> {
	const response = await swarmClient.info({
		payload: JSON.stringify({server_id: toServerId(serverId)}),
	});
	return JSON.parse(response.payload || '{}');
}

/** Join tokens (manager/worker) for a server's swarm cluster. */
export async function swarmJoinTokens(
	serverId: number | bigint | null = null,
): Promise<{manager?: string; worker?: string; [key: string]: unknown}> {
	const response = await swarmClient.getJoinTokens({
		payload: JSON.stringify({server_id: toServerId(serverId)}),
	});
	return JSON.parse(response.payload || '{}');
}

/** Promote a worker node to manager. */
export function useSwarmNodePromote() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string}) =>
			swarmClient.promoteNode({
				payload: JSON.stringify({
					server_id: toServerId(input.server_id),
					node_id: input.node_id,
				}),
			}),
		{invalidate: [['swarm']]},
	);
}

/** Demote a manager node to worker. */
export function useSwarmNodeDemote() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string}) =>
			swarmClient.demoteNode({
				payload: JSON.stringify({
					server_id: toServerId(input.server_id),
					node_id: input.node_id,
				}),
			}),
		{invalidate: [['swarm']]},
	);
}

/**
 * Leave the swarm cluster on a server. `force` is required when leaving as a
 * manager node (it resets the node so it can join another cluster).
 */
export function useSwarmLeave() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; force?: boolean}) =>
			swarmClient.leave({
				payload: JSON.stringify({
					server_id: toServerId(input.server_id),
					force: input.force ?? false,
				}),
			}),
		{invalidate: [['swarm']]},
	);
}

/**
 * Join one server into another server's swarm cluster in one shot: the
 * target force-leaves any standalone swarm first, then joins the manager's
 * cluster with a VPN-aware advertise address.
 */
export function useSwarmJoin() {
	return useConnectMutation(
		(input: {target_server_id: number | bigint; manager_server_id?: number | bigint | null; role: string}) =>
			swarmClient.join({
				payload: JSON.stringify({
					target_server_id: Number(input.target_server_id),
					manager_server_id: toServerId(input.manager_server_id),
					role: input.role,
				}),
			}),
		{invalidate: [['swarm']]},
	);
}
