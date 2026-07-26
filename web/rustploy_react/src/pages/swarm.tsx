import {useState, useEffect, useCallback} from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {SwarmHeader} from '#/components/swarm/swarm-header';
import {SwarmInfoCard} from '#/components/swarm/swarm-info-card';
import {SwarmNodesList} from '#/components/swarm/swarm-nodes-list';
import {JoinTokensModal} from '#/components/swarm/join-tokens-modal';

export const Route = createFileRoute('/_app/swarm')({
	component: SwarmPage,
});

function SwarmPage() {
	const [selectedServerId, setSelectedServerId] = useState<string>('local');
	const [isTokensOpen, setIsTokensOpen] = useState(false);
	const [info, setInfo] = useState<any>(null);
	const [tokens, setTokens] = useState<any>(null);
	const [nodes, setNodes] = useState<any[]>([]);
	const [isLoading, setIsLoading] = useState(false);

	const {data: serversData} = $api.useQuery('get', '/remote-servers');
	const servers = Array.isArray(serversData) ? serversData : [];

	const infoMutation = $api.useMutation('post', '/swarm/info');
	const tokensMutation = $api.useMutation('post', '/swarm/tokens');
	const nodesMutation = $api.useMutation('post', '/swarm/nodes');
	const promoteMutation = $api.useMutation('post', '/swarm/nodes/promote');
	const demoteMutation = $api.useMutation('post', '/swarm/nodes/demote');
	const availabilityMutation = $api.useMutation('post', '/swarm/nodes/availability');
	const removeNodeMutation = $api.useMutation('post', '/swarm/nodes/remove');
	const leaveMutation = $api.useMutation('post', '/swarm/leave');

	const loadSwarmData = useCallback(async () => {
		setIsLoading(true);
		const payload = {
			server_id: selectedServerId === 'local' ? undefined : parseInt(selectedServerId),
		};

		try {
			const infoRes = await infoMutation.mutateAsync({body: payload as any});
			setInfo(infoRes);

			if (infoRes?.local_node_state?.toLowerCase() === 'active') {
				const [nodesRes, tokensRes] = await Promise.all([
					nodesMutation.mutateAsync({body: payload as any}).catch(() => []),
					tokensMutation.mutateAsync({body: payload as any}).catch(() => null),
				]);
				setNodes(Array.isArray(nodesRes) ? nodesRes : []);
				setTokens(tokensRes);
			} else {
				setNodes([]);
				setTokens(null);
			}
		} catch (err: any) {
			setInfo(null);
			setNodes([]);
			setTokens(null);
			toast.error(formatApiError(err));
		} finally {
			setIsLoading(false);
		}
	}, [selectedServerId]);

	useEffect(() => {
		loadSwarmData();
	}, [loadSwarmData]);

	const getServerIdNumber = () =>
		selectedServerId === 'local' ? undefined : parseInt(selectedServerId);

	const handlePromote = async (nodeId: string) => {
		try {
			await promoteMutation.mutateAsync({
				body: {server_id: getServerIdNumber(), node_id: nodeId} as any,
			});
			toast.success('Node promoted to Manager');
			loadSwarmData();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleDemote = async (nodeId: string) => {
		try {
			await demoteMutation.mutateAsync({
				body: {server_id: getServerIdNumber(), node_id: nodeId} as any,
			});
			toast.success('Node demoted to Worker');
			loadSwarmData();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleSetAvailability = async (nodeId: string, availability: string) => {
		try {
			await availabilityMutation.mutateAsync({
				body: {
					server_id: getServerIdNumber(),
					node_id: nodeId,
					availability,
				} as any,
			});
			toast.success(`Node availability set to ${availability}`);
			loadSwarmData();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleRemoveNode = async (nodeId: string) => {
		if (!confirm('Are you sure you want to force remove this Swarm node?')) return;
		try {
			await removeNodeMutation.mutateAsync({
				body: {server_id: getServerIdNumber(), node_id: nodeId} as any,
			});
			toast.success('Node removed from Swarm cluster');
			loadSwarmData();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleLeaveSwarm = async () => {
		if (!confirm('Are you sure you want to leave this Swarm cluster?')) return;
		try {
			await leaveMutation.mutateAsync({
				body: {server_id: getServerIdNumber()} as any,
			});
			toast.success('Left Swarm cluster successfully');
			loadSwarmData();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const isSwarmActive = info?.local_node_state?.toLowerCase() === 'active';

	return (
		<div className="flex flex-col gap-4 p-6 max-w-7xl mx-auto w-full">
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
				info={info}
				isLoading={isLoading}
				onLeaveSwarm={handleLeaveSwarm}
			/>

			{isSwarmActive && (
				<SwarmNodesList
					nodes={nodes}
					isLoading={isLoading}
					onPromote={handlePromote}
					onDemote={handleDemote}
					onSetAvailability={handleSetAvailability}
					onRemoveNode={handleRemoveNode}
				/>
			)}

			<JoinTokensModal
				isOpen={isTokensOpen}
				tokens={tokens}
				isLoading={isLoading}
				onClose={() => setIsTokensOpen(false)}
			/>
		</div>
	);
}
