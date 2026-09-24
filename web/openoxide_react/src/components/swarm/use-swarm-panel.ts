import {useState} from 'react';
import {toast} from 'sonner';
import {useQuery, useQueryClient} from '@tanstack/react-query';
import {
	swarmJoinTokens,
	useSwarmJoin,
	useSwarmLeave,
	useSwarmNodeAvailabilityUpdate,
	useSwarmNodeDemote,
	useSwarmNodePromote,
	useSwarmNodeRemove,
	useRemoteServerList,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import type {RemoteServerResponse, SwarmTokens} from '#/types/api-helpers';
import type {TaggedSwarmNode} from '#/components/swarm/swarm-nodes-list';
import {
	fetchServerSwarm,
	type SwarmOverview,
} from '#/components/swarm/swarm-fetch';
import {useSwarmNodeActions} from '#/components/swarm/use-swarm-node-actions';

/** State, queries and node/cluster handlers for the Swarm tab. */
export function useSwarmPanel() {
	const [selectedServerId, setSelectedServerId] =
		useState<string>('local');
	const [isTokensOpen, setIsTokensOpen] = useState(false);
	const queryClient = useQueryClient();

	// Live servers query; the table-watch stream keeps it fresh.
	const {data: serversData} = useRemoteServerList();
	const servers = Array.isArray(serversData)
		? (serversData as unknown as RemoteServerResponse[])
		: [];

	const promoteMutation = useSwarmNodePromote();
	const demoteMutation = useSwarmNodeDemote();
	const availabilityMutation = useSwarmNodeAvailabilityUpdate();
	const removeNodeMutation = useSwarmNodeRemove();
	const leaveMutation = useSwarmLeave();
	const joinMutation = useSwarmJoin();

	// Cached for 15s so revisiting the page (or switching tabs back) shows the last-known cluster state instantly instead of re-running an SSH round trip before anything paints.
	const swarmQuery = useQuery({
		queryKey: ['swarm-overview', selectedServerId],
		queryFn: async (): Promise<SwarmOverview> => {
			const isLocal = selectedServerId === 'local';
			const sId = isLocal ? undefined : parseInt(selectedServerId, 10);
			const res = await fetchServerSwarm(sId);
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

	// Tokens are only needed once the "New Node" sheet is opened — fetching them on every page load would be an SSH round trip nobody is looking at yet.
	const tokensQuery = useQuery({
		queryKey: ['swarm-tokens', activeServerId],
		queryFn: async (): Promise<SwarmTokens | null> => {
			const data = await swarmJoinTokens(activeServerId ?? null);
			return (data as SwarmTokens | null) ?? null;
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
		if (selectedServerId === 'local') return undefined;
		return parseInt(selectedServerId, 10);
	};

	const nodeActions = useSwarmNodeActions({
		promoteMutation,
		demoteMutation,
		availabilityMutation,
		removeNodeMutation,
		getServerIdNumber,
		refreshAll,
	});

	const isSwarmActive =
		nodes.length > 0 ||
		String(info?.local_node_state || '').toLowerCase() === 'active';

	const handleLeaveRemoteSwarm = async (serverId: number) => {
		try {
			await leaveMutation.mutateAsync({server_id: serverId, force: true});
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
				target_server_id: serverId,
				manager_server_id: activeServerId,
				role,
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

	return {
		servers,
		nodes,
		info,
		isLoading,
		isRefreshing,
		isSwarmActive,
		selectedServerId,
		setSelectedServerId,
		isTokensOpen,
		setIsTokensOpen,
		tokensQuery,
		refreshAll,
		...nodeActions,
		handleLeaveRemoteSwarm,
		handleJoinServer,
	};
}
