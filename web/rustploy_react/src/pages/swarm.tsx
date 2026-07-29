import {useState, useEffect, useCallback} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {SwarmHeader} from '#/components/swarm/swarm-header';
import {SwarmInfoCard} from '#/components/swarm/swarm-info-card';
import {SwarmNodesList} from '#/components/swarm/swarm-nodes-list';
import {JoinTokensModal} from '#/components/swarm/join-tokens-modal';
import type {RemoteServerResponse} from '#/types/api-helpers';

export const Route = createFileRoute('/_app/swarm')({
	component: SwarmPage,
});

function SwarmPage() {
	const [selectedServerId, setSelectedServerId] = useState<string>('all');
	const [isTokensOpen, setIsTokensOpen] = useState(false);
	const [info, setInfo] = useState<Record<string, unknown> | null>(null);
	const [tokens, setTokens] = useState<Record<string, unknown> | null>(null);
	const [nodes, setNodes] = useState<Record<string, unknown>[]>([]);
	const [isLoading, setIsLoading] = useState(false);

	const {data: serversData} = $api.useQuery('get', '/remote-servers');
	const servers = Array.isArray(serversData) ? (serversData as RemoteServerResponse[]) : [];

	const infoMutation = $api.useMutation('post', '/swarm/info');
	const tokensMutation = $api.useMutation('post', '/swarm/tokens');
	const nodesMutation = $api.useMutation('post', '/swarm/nodes');
	const promoteMutation = $api.useMutation('post', '/swarm/nodes/promote');
	const demoteMutation = $api.useMutation('post', '/swarm/nodes/demote');
	const availabilityMutation = $api.useMutation('post', '/swarm/nodes/availability');
	const removeNodeMutation = $api.useMutation('post', '/swarm/nodes/remove');
	const leaveMutation = $api.useMutation('post', '/swarm/leave');

	const fetchServerNodes = async (serverId?: number, serverLabel: string = 'Local Server') => {
		const payload = {server_id: serverId};
		try {
			const infoRes = await infoMutation.mutateAsync({body: payload as unknown as {server_id?: number}}).catch(() => null);
			if (infoRes && String((infoRes as any).local_node_state || '').toLowerCase() === 'active') {
				const [nodesRes, tokensRes] = await Promise.all([
					nodesMutation.mutateAsync({body: payload as unknown as {server_id?: number}}).catch(() => []),
					tokensMutation.mutateAsync({body: payload as unknown as {server_id?: number}}).catch(() => null),
				]);
				const rawNodes = Array.isArray(nodesRes) ? (nodesRes as Record<string, unknown>[]) : [];
				const taggedNodes = rawNodes.map(n => ({
					...n,
					_serverId: serverId,
					_serverName: serverLabel,
				}));
				return {info: infoRes as Record<string, unknown>, nodes: taggedNodes, tokens: tokensRes as Record<string, unknown>};
			}
		} catch {
			// ignore single server error in multi fetch
		}
		return {info: null, nodes: [], tokens: null};
	};

	const loadSwarmData = useCallback(async () => {
		setIsLoading(true);
		try {
			if (selectedServerId === 'all') {
				const targets = [
					{id: undefined, label: 'Local Server'},
					...servers.map(s => ({id: s.id, label: s.name})),
				];

				const results = await Promise.all(targets.map(t => fetchServerNodes(t.id, t.label)));

				const activeResult = results.find(r => r.info !== null);
				setInfo(activeResult?.info || results[0]?.info || null);
				setTokens(activeResult?.tokens || null);

				// Deduplicate nodes by node ID across cluster queries
				const allNodesMap = new Map<string, Record<string, unknown>>();
				results.forEach(r => {
					r.nodes.forEach(n => {
						const nodeId = String((n as any).id || (n as any).ID || Math.random());
						if (!allNodesMap.has(nodeId)) {
							allNodesMap.set(nodeId, n);
						}
					});
				});

				setNodes(Array.from(allNodesMap.values()));
			} else {
				const sId = selectedServerId === 'local' ? undefined : parseInt(selectedServerId);
				const sName = selectedServerId === 'local' ? 'Local Server' : servers.find(s => String(s.id) === selectedServerId)?.name || 'Server';
				const res = await fetchServerNodes(sId, sName);
				setInfo(res.info);
				setNodes(res.nodes);
				setTokens(res.tokens);
			}
		} catch (err: unknown) {
			setInfo(null);
			setNodes([]);
			setTokens(null);
			toast.error(formatApiError(err));
		} finally {
			setIsLoading(false);
		}
	}, [selectedServerId, servers]);

	useEffect(() => {
		loadSwarmData();
	}, [loadSwarmData]);

	const getServerIdNumber = (node?: Record<string, unknown>) => {
		if (node && node._serverId !== undefined) return node._serverId as number | undefined;
		return selectedServerId === 'local' || selectedServerId === 'all' ? undefined : parseInt(selectedServerId);
	};

	const handlePromote = async (nodeId: string, node?: Record<string, unknown>) => {
		try {
			await promoteMutation.mutateAsync({
				body: {server_id: getServerIdNumber(node), node_id: nodeId} as unknown as {node_id: string; server_id?: number},
			});
			toast.success('Node promoted to Manager');
			loadSwarmData();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleDemote = async (nodeId: string, node?: Record<string, unknown>) => {
		try {
			await demoteMutation.mutateAsync({
				body: {server_id: getServerIdNumber(node), node_id: nodeId} as unknown as {node_id: string; server_id?: number},
			});
			toast.success('Node demoted to Worker');
			loadSwarmData();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleSetAvailability = async (nodeId: string, availability: string, node?: Record<string, unknown>) => {
		try {
			await availabilityMutation.mutateAsync({
				body: {
					server_id: getServerIdNumber(node),
					node_id: nodeId,
					availability,
				} as unknown as {availability: string; node_id: string; server_id?: number},
			});
			toast.success(`Node availability set to ${availability}`);
			loadSwarmData();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleRemoveNode = async (nodeId: string, node?: Record<string, unknown>) => {
		try {
			await removeNodeMutation.mutateAsync({
				body: {server_id: getServerIdNumber(node), node_id: nodeId} as unknown as {node_id: string; server_id?: number},
			});
			toast.success('Node removed from Swarm cluster');
			loadSwarmData();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const handleLeaveSwarm = async () => {
		try {
			await leaveMutation.mutateAsync({
				body: {server_id: getServerIdNumber()} as unknown as {server_id?: number},
			});
			toast.success('Left Swarm cluster successfully');
			loadSwarmData();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	const isSwarmActive = nodes.length > 0 || String(info?.local_node_state || '').toLowerCase() === 'active';

	const handleLeaveRemoteSwarm = async (serverId: number) => {
		try {
			await leaveMutation.mutateAsync({
				body: {server_id: serverId, force: true} as unknown as {server_id?: number; force?: boolean},
			});
			toast.success('Force disconnected Swarm on target server');
			loadSwarmData();
		} catch (err: unknown) {
			const srvName = servers.find(s => s.id === serverId)?.name || 'target server';
			const rawErr = formatApiError(err);
			if (rawErr.toLowerCase().includes('not part of a swarm')) {
				toast.success(`Server "${srvName}" is not in any Swarm and is ready to join!`);
				loadSwarmData();
				return;
			}
			toast.error(`Server "${srvName}" error: ${rawErr}`);
		}
	};

	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full">
			<SwarmHeader
				servers={servers}
				selectedServerId={selectedServerId}
				onSelectServer={setSelectedServerId}
				onRefresh={loadSwarmData}
				onOpenTokens={() => setIsTokensOpen(true)}
				isRefreshing={isLoading}
				isSwarmActive={isSwarmActive}
			/>

			<SwarmInfoCard
				info={info as any}
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

			{tokens && (
				<JoinTokensModal
					isOpen={isTokensOpen}
					onClose={() => setIsTokensOpen(false)}
					tokens={tokens as any}
					isLoading={isLoading}
					servers={servers}
					nodes={nodes}
					onLeaveRemoteSwarm={handleLeaveRemoteSwarm}
				/>
			)}
		</div>
	);
}
