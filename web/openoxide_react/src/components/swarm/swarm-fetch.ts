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

export async function fetchServerSwarm(
	serverId: number | undefined,
): Promise<ServerSwarmResult> {
	try {
		const info = (await swarmInfo(serverId ?? null)) as SwarmInfo | null;
		if (
			!info ||
			String(info.local_node_state || '').toLowerCase() !== 'active'
		) {
			return {info: null, nodes: []};
		}
		// Worker nodes are "active" too but Docker refuses `node ls`/`join-token` on them (managers-only commands) — asking would just log a failed SSH round trip for nothing.
		if (!info.control_available) {
			return {info, nodes: []};
		}
		const rawNodes = await swarmNodes(serverId ?? null);
		// The Connect payload carries the same NodeDto fields as the generated type; the cast bridges the two type sources.
		const nodes = (rawNodes || []).map(n => ({
			...n,
			_serverId: serverId,
		})) as unknown as TaggedSwarmNode[];
		return {info, nodes};
	} catch {
		// An unreachable engine reads as "no swarm here" instead of failing the page.
		return {info: null, nodes: []};
	}
}
