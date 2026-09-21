import type {SwarmNode} from '#/types/api-helpers';

export type TaggedSwarmNode = SwarmNode & {
	_serverId?: number;
	_serverName?: string;
};

export const ROLE_FILTERS = ['All', 'Managers', 'Workers'] as const;
export type RoleFilter = (typeof ROLE_FILTERS)[number];

/**
 * `role`/`is_leader`/`reachability` come from `docker node inspect`'s
 * `Spec.Role` and `ManagerStatus` — the authoritative config values, not a
 * string derived from reachability. A manager stays "manager" even when
 * Docker can't currently reach it (reachability: "unreachable").
 */
export function nodeRole(node: SwarmNode) {
	const isManager = node.role === 'manager';
	const isLeader = node.is_leader;
	const isUnreachable =
		isManager && (node.reachability || '').toLowerCase() === 'unreachable';
	return {isManager, isLeader, isUnreachable};
}
