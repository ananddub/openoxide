import {useState} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {useQuery, useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {useAppStore} from '#/stores/app-store';
import {client} from '#/api/client';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {SwarmHeader} from '#/components/swarm/swarm-header';
import {SwarmInfoCard} from '#/components/swarm/swarm-info-card';
import {SwarmNodesList} from '#/components/swarm/swarm-nodes-list';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';
import {JoinTokensModal} from '#/components/swarm/join-tokens-modal';
import type {
	RemoteServerResponse,
	SwarmInfo,
	SwarmTokens,
} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/swarm')({
	component: SwarmPage,
});

interface ServerSwarmResult {
	info: SwarmInfo | null;
	nodes: TaggedSwarmNode[];
}

// Backend SSH connect timeout is 15s per server — fine for one server, but
// "All Clusters" awaits every server together, so one unreachable box (bad
// key, dead host) drags the whole page down to its worst case. Bound each
// server's contribution so a single broken one can't block the rest.
const PER_SERVER_TIMEOUT_MS = 4000;

function withTimeout<T>(
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

interface SwarmOverview {
	info: SwarmInfo | null;
	nodes: TaggedSwarmNode[];
	activeServerId: number | undefined;
}

async function fetchServerSwarm(
	serverId: number | undefined,
	serverLabel: string,
): Promise<ServerSwarmResult> {
	const body = {server_id: serverId};
	try {
		const {data: info} = await client.POST('/swarm/info', {body});
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
		const {data: rawNodes} = await client.POST('/swarm/nodes', {body});
		const nodes: TaggedSwarmNode[] = (rawNodes || []).map(n => ({
			...n,
			_serverId: serverId,
			_serverName: serverLabel,
		}));
		return {info, nodes};
	} catch {
		// A single unreachable server shouldn't fail the whole "All Clusters" view.
		return {info: null, nodes: []};
	}
}

function SwarmPage() {
	const [selectedServerId, setSelectedServerId] = useState<string>('all');
	const [isTokensOpen, setIsTokensOpen] = useState(false);
	const queryClient = useQueryClient();

	const serversData = useAppStore(state => state.servers);
	const servers = Array.isArray(serversData)
		? (serversData as unknown as RemoteServerResponse[])
		: [];
	const serverIdsKey = servers.map(s => s.id).join(',');

	const promoteMutation = $api.useMutation('post', '/swarm/nodes/promote');
	const demoteMutation = $api.useMutation('post', '/swarm/nodes/demote');
	const availabilityMutation = $api.useMutation(
		'post',
		'/swarm/nodes/availability',
	);
	const removeNodeMutation = $api.useMutation(
		'post',
		'/swarm/nodes/remove',
	);
	const leaveMutation = $api.useMutation('post', '/swarm/leave');
	const joinMutation = $api.useMutation('post', '/swarm/join');

	// Cached for 15s so revisiting the page (or switching tabs back) shows the
	// last-known cluster state instantly instead of re-running an SSH round
	// trip per server before anything paints.
	const swarmQuery = useQuery({
		queryKey: ['swarm-overview', selectedServerId, serverIdsKey],
		queryFn: async (): Promise<SwarmOverview> => {
			if (selectedServerId === 'all') {
				const targets: {id: number | undefined; label: string}[] = [
					{id: undefined, label: 'Local Server'},
					...servers.map(s => ({id: s.id, label: s.name})),
				];
				const results = await Promise.all(
					targets.map(t =>
						withTimeout(
							fetchServerSwarm(t.id, t.label),
							PER_SERVER_TIMEOUT_MS,
							{info: null, nodes: []},
						),
					),
				);
				// Prefer an actual manager as the cluster reference — only managers
				// can issue join tokens or list nodes, so picking a worker here would
				// make every later action fail the same way `/swarm/nodes` just did.
				const managerIdx = results.findIndex(
					r => r.info?.control_available,
				);
				const activeIdx =
					managerIdx >= 0
						? managerIdx
						: results.findIndex(r => r.info !== null);

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
					activeServerId:
						activeIdx >= 0 ? targets[activeIdx].id : undefined,
				};
			}

			const sId =
				selectedServerId === 'local'
					? undefined
					: parseInt(selectedServerId, 10);
			const sName =
				selectedServerId === 'local'
					? 'Local Server'
					: servers.find(s => String(s.id) === selectedServerId)?.name ||
						'Server';
			const res = await fetchServerSwarm(sId, sName);
			return {
				info: res.info,
				nodes: res.nodes,
				activeServerId: res.info ? sId : undefined,
			};
		},
		staleTime: 15_000,
		refetchOnWindowFocus: false,
	});

	const info = swarmQuery.data?.info ?? null;
	const nodes = swarmQuery.data?.nodes ?? [];
	const activeServerId = swarmQuery.data?.activeServerId;
	const isLoading = swarmQuery.isLoading;
	const isRefreshing = swarmQuery.isFetching;

	// Tokens are only needed once the "Add Node" modal is opened — fetching
	// them on every page load was one of three parallel SSH round trips per
	// server that nobody was looking at yet.
	const tokensQuery = useQuery({
		queryKey: ['swarm-tokens', activeServerId],
		queryFn: async (): Promise<SwarmTokens | null> => {
			const {data} = await client.POST('/swarm/tokens', {
				body: {server_id: activeServerId},
			});
			return data ?? null;
		},
		enabled: isTokensOpen && !!info?.control_available,
		staleTime: 60_000,
	});

	const refreshAll = () => {
		queryClient.invalidateQueries({queryKey: ['swarm-overview']});
		queryClient.invalidateQueries({queryKey: ['swarm-tokens']});
		queryClient.invalidateQueries({queryKey: ['swarm-node-identity']});
	};

	const getServerIdNumber = (
		node?: TaggedSwarmNode,
	): number | undefined => {
		if (node && node._serverId !== undefined) return node._serverId;
		if (selectedServerId === 'all') return activeServerId;
		if (selectedServerId === 'local') return undefined;
		return parseInt(selectedServerId, 10);
	};

	const handlePromote = async (nodeId: string, node?: TaggedSwarmNode) => {
		try {
			await promoteMutation.mutateAsync({
				body: {server_id: getServerIdNumber(node), node_id: nodeId},
			});
			toast.success('Node promoted to Manager');
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDemote = async (nodeId: string, node?: TaggedSwarmNode) => {
		try {
			await demoteMutation.mutateAsync({
				body: {server_id: getServerIdNumber(node), node_id: nodeId},
			});
			toast.success('Node demoted to Worker');
			refreshAll();
		} catch (err: unknown) {
			const rawErr = formatApiError(err);
			if (rawErr.toLowerCase().includes('last manager')) {
				toast.error(
					'Promote another node to Manager first — a swarm always needs at least one.',
				);
			} else {
				toast.error(rawErr);
			}
		}
	};

	const handleSetAvailability = async (
		nodeId: string,
		availability: string,
		node?: TaggedSwarmNode,
	) => {
		try {
			await availabilityMutation.mutateAsync({
				body: {
					server_id: getServerIdNumber(node),
					node_id: nodeId,
					availability,
				},
			});
			toast.success(`Node availability set to ${availability}`);
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleRemoveNode = async (
		nodeId: string,
		node?: TaggedSwarmNode,
	) => {
		try {
			await removeNodeMutation.mutateAsync({
				body: {server_id: getServerIdNumber(node), node_id: nodeId},
			});
			toast.success('Node removed from Swarm cluster');
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleLeaveSwarm = async () => {
		try {
			await leaveMutation.mutateAsync({
				body: {server_id: getServerIdNumber()},
			});
			toast.success('Left Swarm cluster successfully');
			refreshAll();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const isSwarmActive =
		nodes.length > 0 ||
		String(info?.local_node_state || '').toLowerCase() === 'active';

	const handleLeaveRemoteSwarm = async (serverId: number) => {
		try {
			await leaveMutation.mutateAsync({
				body: {server_id: serverId, force: true},
			});
			toast.success('Force disconnected Swarm on target server');
			refreshAll();
		} catch (err: unknown) {
			const srvName =
				servers.find(s => s.id === serverId)?.name || 'target server';
			const rawErr = formatApiError(err);
			if (rawErr.toLowerCase().includes('not part of a swarm')) {
				toast.success(
					`Server "${srvName}" is not in any Swarm and is ready to join!`,
				);
				refreshAll();
				return;
			}
			toast.error(`Server "${srvName}" error: ${rawErr}`);
		}
	};

	const handleJoinServer = async (
		serverId: number,
		role: 'worker' | 'manager',
	) => {
		try {
			await joinMutation.mutateAsync({
				body: {
					target_server_id: serverId,
					manager_server_id: activeServerId,
					role,
				},
			});
			const srvName =
				servers.find(s => s.id === serverId)?.name || 'Server';
			toast.success(
				`"${srvName}" joined the cluster as ${role === 'manager' ? 'a Manager' : 'a Worker'}!`,
			);
			refreshAll();
		} catch (err: unknown) {
			const srvName =
				servers.find(s => s.id === serverId)?.name || 'target server';
			toast.error(`Failed to join "${srvName}": ${formatApiError(err)}`);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-7xl flex-col gap-6 p-6">
			<SwarmHeader
				servers={servers}
				selectedServerId={selectedServerId}
				onSelectServer={setSelectedServerId}
				onRefresh={refreshAll}
				onOpenTokens={() => setIsTokensOpen(true)}
				isRefreshing={isRefreshing}
				isSwarmActive={isSwarmActive}
			/>

			<SwarmInfoCard
				info={info}
				isLoading={isLoading}
				onLeaveSwarm={handleLeaveSwarm}
			/>

			<SwarmNodesList
				nodes={nodes}
				isLoading={isLoading}
				onPromote={(id, n) => handlePromote(id, n)}
				onDemote={(id, n) => handleDemote(id, n)}
				onSetAvailability={(id, av, n) => handleSetAvailability(id, av, n)}
				onRemoveNode={(id, n) => handleRemoveNode(id, n)}
			/>

			{isTokensOpen && (
				<JoinTokensModal
					isOpen={isTokensOpen}
					onClose={() => setIsTokensOpen(false)}
					tokens={tokensQuery.data ?? null}
					isLoading={tokensQuery.isLoading}
					servers={servers}
					nodes={nodes}
					onLeaveRemoteSwarm={handleLeaveRemoteSwarm}
					onJoinServer={handleJoinServer}
				/>
			)}
		</div>
	);
}
