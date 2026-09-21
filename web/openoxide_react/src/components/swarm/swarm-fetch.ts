import {swarmInfo, swarmNodes} from '#/api/live';
import type {SwarmInfo} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';

export interface ServerSwarmResult {
	info: SwarmInfo | null;
	nodes: TaggedSwarmNode[];
}

export interface SwarmOverview {
	info: SwarmInfo | null;
	nodes: TaggedSwarmNode[];
	activeServerId: number | undefined;
}

// Backend SSH connect timeout is 15s per server — fine for one server, but
// "All Clusters" awaits every server together, so one unreachable box (bad
// key, dead host) drags the whole page down to its worst case. Bound each
// server's contribution so a single broken one can't block the rest.
export const PER_SERVER_TIMEOUT_MS = 4000;

export function withTimeout<T>(
	promise: Promise<T>,
	ms: number,
	fallback: T,
): Promise<T> {
	return new Promise<T>(resolve => {
		const timer = setTimeout(() => resolve(fallback), ms);
		promise.then(
			value => {
				clearTimeout(timer);
				resolve(value);
			},
			() => {
				clearTimeout(timer);
				resolve(fallback);
			},
		);
	});
}

export async function fetchServerSwarm(
	serverId: number | undefined,
	serverLabel: string,
): Promise<ServerSwarmResult> {
	try {
		const info = (await swarmInfo(serverId ?? null)) as SwarmInfo | null;
		if (
			!info ||
			String(info.local_node_state || '').toLowerCase() !== 'active'
		) {
			return {info: null, nodes: []};
		}
		// Worker nodes are "active" too but Docker refuses `node ls`/`join-token`
		// on them (managers-only commands) — asking would just log a failed
		// SSH round trip for nothing.
		if (!info.control_available) {
			return {info, nodes: []};
		}
		const rawNodes = await swarmNodes(serverId ?? null);
		// The Connect payload carries the same NodeDto fields as the
		// generated type; the cast bridges the two type sources.
		const nodes = (rawNodes || []).map(n => ({
			...n,
			_serverId: serverId,
			_serverName: serverLabel,
		})) as unknown as TaggedSwarmNode[];
		return {info, nodes};
	} catch {
		// A single unreachable server shouldn't fail the whole "All Clusters" view.
		return {info: null, nodes: []};
	}
}

/** Aggregate every server's swarm (local + remote) into one overview. */
export async function fetchAllClustersSwarm(
	servers: {id: number; name: string}[] | any[],
): Promise<SwarmOverview> {
	const targets: {id: number | undefined; label: string}[] = [
		{id: undefined, label: 'Local Server'},
		...servers.map(s => ({id: s.id, label: s.name})),
	];
	const results = await Promise.all(
		targets.map(t =>
			withTimeout(fetchServerSwarm(t.id, t.label), PER_SERVER_TIMEOUT_MS, {
				info: null,
				nodes: [],
			}),
		),
	);
	// Prefer an actual manager as the cluster reference — only managers
	// can issue join tokens or list nodes, so picking a worker here would
	// make every later action fail the same way `/swarm/nodes` just did.
	const managerIdx = results.findIndex(r => r.info?.control_available);
	const activeIdx =
		managerIdx >= 0 ? managerIdx : results.findIndex(r => r.info !== null);

	const allNodesMap = new Map<string, TaggedSwarmNode>();
	for (const r of results) {
		for (const n of r.nodes) {
			if (n.id && !allNodesMap.has(n.id)) allNodesMap.set(n.id, n);
		}
	}

	return {
		info:
			activeIdx >= 0
				? results[activeIdx].info
				: (results[0]?.info ?? null),
		nodes: Array.from(allNodesMap.values()),
		activeServerId: activeIdx >= 0 ? targets[activeIdx].id : undefined,
	};
}
