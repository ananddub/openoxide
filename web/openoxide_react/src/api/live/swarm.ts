import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	SwarmService,
	SwarmNodeAvailability,
	SwarmNodeRole,
	SwarmRole,
	type SwarmNode,
} from '#/gen/connect/openoxide/api/v1/swarm_pb';

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

const swarmClient = createServiceClient(SwarmService);

/** Swarm payloads carry an optional server id; null/undefined/0 targets the panel's local Docker engine (proto field is `optional int64`). */
function toServerId(
	value: number | bigint | null | undefined,
): bigint | undefined {
	if (value === null || value === undefined) return undefined;
	const id = BigInt(value);
	return id > 0n ? id : undefined;
}

/** Enum value → member name ('WORKER', 'MANAGER', …). */
function enumName<E extends Record<number, string>>(
	enumeration: E,
	value: number,
): string {
	return enumeration[value] ?? '';
}

/** Enum name in arbitrary casing ('active', 'DRAIN', 'manager') → numeric enum. */
function parseEnum<E extends Record<string, number | string>>(
	enumeration: E,
	name: unknown,
): number {
	if (typeof name === 'number') return name;
	const key = String(name ?? '').toUpperCase();
	const value = enumeration[key as keyof E];
	return typeof value === 'number' ? value : 0;
}

function toSwarmNodePayload(node: SwarmNode): SwarmNodePayload {
	return {
		id: node.id,
		hostname: node.hostname,
		status: node.status,
		availability: node.availability,
		// The legacy NodeDto exposed the docker role lower-cased ('manager' | 'worker'); consumers compare lowercase strings.
		role: enumName(SwarmNodeRole, node.role).toLowerCase(),
		is_leader: node.isLeader,
		reachability: node.reachability,
		ip_address: node.ipAddress,
		engine_version: node.engineVersion,
	};
}

/** Swarm nodes for a server (legacy REST: GET /swarm/nodes). null lists the local cluster. */
export function useSwarmNodeList(serverId: number | bigint | null = null) {
	return useConnectQuery<SwarmNodePayload[]>(
		['swarm', 'nodes', String(serverId)],
		async () => {
			const response = await swarmClient.listNodes({
				serverId: toServerId(serverId),
			});
			return response.nodes.map(toSwarmNodePayload);
		},
	);
}

/** Update a swarm node's availability (`active` | `pause` | `drain`). */
export function useSwarmNodeAvailabilityUpdate() {
	return useConnectMutation(
		(input: {
			server_id?: number | bigint | null;
			node_id: string;
			availability: string;
		}) =>
			swarmClient.updateNodeAvailability({
				serverId: toServerId(input.server_id),
				nodeId: input.node_id,
				availability: parseEnum(
					SwarmNodeAvailability,
					input.availability,
				) as SwarmNodeAvailability,
			}),
		// Docker swarm state is agent-side — the CDC stream never sees it — so these invalidate the swarm queries directly.
		{invalidate: [['swarm']]},
	);
}

/** Force-remove a swarm node from the cluster by node id. */
export function useSwarmNodeRemove() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string}) =>
			swarmClient.removeNode({
				serverId: toServerId(input.server_id),
				nodeId: input.node_id,
			}),
		{invalidate: [['swarm']]},
	);
}

/** Swarm nodes for a server as a plain promise (null server = local engine). */
export async function swarmNodes(
	serverId: number | bigint | null = null,
): Promise<SwarmNodePayload[]> {
	const response = await swarmClient.listNodes({
		serverId: toServerId(serverId),
	});
	return response.nodes.map(toSwarmNodePayload);
}

/** Swarm cluster info for a server (null = local Docker engine). Keeps the legacy snake_case Record shape the swarm cards read. */
export async function swarmInfo(
	serverId: number | bigint | null = null,
): Promise<Record<string, unknown>> {
	const response = await swarmClient.info({
		serverId: toServerId(serverId),
	});
	const info = response.info;
	if (!info) return {};
	return {
		node_id: info.nodeId,
		node_addr: info.nodeAddr,
		local_node_state: info.localNodeState,
		control_available: info.controlAvailable,
		nodes: Number(info.nodes),
		managers: Number(info.managers),
	};
}

/** Join tokens (manager/worker) for a server's swarm cluster. */
export async function swarmJoinTokens(
	serverId: number | bigint | null = null,
): Promise<{manager?: string; worker?: string; [key: string]: unknown}> {
	const response = await swarmClient.getJoinTokens({
		serverId: toServerId(serverId),
	});
	const tokens = response.tokens;
	if (!tokens) return {};
	return {manager: tokens.manager, worker: tokens.worker};
}

/** Promote a worker node to manager. */
export function useSwarmNodePromote() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string}) =>
			swarmClient.promoteNode({
				serverId: toServerId(input.server_id),
				nodeId: input.node_id,
			}),
		{invalidate: [['swarm']]},
	);
}

/** Demote a manager node to worker. */
export function useSwarmNodeDemote() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; node_id: string}) =>
			swarmClient.demoteNode({
				serverId: toServerId(input.server_id),
				nodeId: input.node_id,
			}),
		{invalidate: [['swarm']]},
	);
}

/** Leave the swarm cluster on a server. `force` is required when leaving as a manager node (it resets the node so it can join another cluster). */
export function useSwarmLeave() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; force?: boolean}) =>
			swarmClient.leave({
				serverId: toServerId(input.server_id),
				force: input.force ?? false,
			}),
		{invalidate: [['swarm']]},
	);
}

/** Join one server into another server's swarm cluster in one shot: the target force-leaves any standalone swarm first, then joins the manager's cluster with a VPN-aware advertise address. The role arrives as the lower-case 'worker' | 'manager' string the panels use. */
export function useSwarmJoin() {
	return useConnectMutation(
		(input: {
			target_server_id: number | bigint;
			manager_server_id?: number | bigint | null;
			role: string;
		}) =>
			swarmClient.join({
				targetServerId: BigInt(input.target_server_id),
				managerServerId: toServerId(input.manager_server_id),
				role: parseEnum(SwarmRole, input.role) as SwarmRole,
			}),
		{invalidate: [['swarm']]},
	);
}
