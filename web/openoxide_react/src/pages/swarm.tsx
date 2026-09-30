import {useState} from 'react';
import {Plus, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {SectionHeader} from '#/components/settings/list-card';
import {cn} from '#/api/utils';
import {
	SwarmNodesList,
	type TaggedSwarmNode,
} from '#/components/swarm/swarm-nodes-list';
import {JoinTokensSheet} from '#/components/swarm/join-tokens-sheet';
import {SwarmDangerDialog} from '#/components/swarm/swarm-danger-dialog';
import {useSwarmPanel} from '#/components/swarm/use-swarm-panel';

// Rendered as the "Swarm" tab of pages/infra.tsx — no standalone route.
export function SwarmPanel() {
	const p = useSwarmPanel();
	const [pendingNode, setPendingNode] = useState<TaggedSwarmNode | null>(
		null,
	);
	const [isConfirming, setIsConfirming] = useState(false);

	const closeConfirm = () => setPendingNode(null);

	const runConfirm = async () => {
		if (!pendingNode) return;
		setIsConfirming(true);
		try {
			await p.handleRemoveNode(pendingNode.id, pendingNode);
		} finally {
			setIsConfirming(false);
			setPendingNode(null);
		}
	};

	return (
		<div className="flex h-full w-full flex-col gap-6">
			<SectionHeader
				title="Docker Swarm"
				description="Cluster nodes, roles, and join tokens across your engines."
				action={
					<>
						<Button
							variant="outline"
							onClick={p.refreshAll}
							disabled={p.isRefreshing}
							className="h-9 gap-1.5">
							<RefreshCw
								className={cn(
									'size-3.5',
									p.isRefreshing && 'animate-spin',
								)}
							/>
							Refresh
						</Button>
						{p.isSwarmActive && (
							<Button
								onClick={() => p.setIsTokensOpen(true)}
								className="h-9 gap-1.5">
								<Plus className="size-3.5" />
								New Node
							</Button>
						)}
					</>
				}
			/>

			<SwarmNodesList
				nodes={p.nodes}
				isLoading={p.isLoading}
				servers={p.servers}
				selectedServerId={p.selectedServerId}
				onSelectServer={p.setSelectedServerId}
				isSwarmActive={p.isSwarmActive}
				onOpenTokens={() => p.setIsTokensOpen(true)}
				onPromote={(id, n) => p.handlePromote(id, n)}
				onDemote={(id, n) => p.handleDemote(id, n)}
				onSetAvailability={(id, av, n) =>
					p.handleSetAvailability(id, av, n)
				}
				onRemoveNode={(_id, n) => n && setPendingNode(n)}
			/>

			{p.isTokensOpen && (
				<JoinTokensSheet
					isOpen={p.isTokensOpen}
					onClose={() => p.setIsTokensOpen(false)}
					tokens={p.tokensQuery.data ?? null}
					isLoading={p.tokensQuery.isLoading}
					servers={p.servers}
					nodes={p.nodes}
					onLeaveRemoteSwarm={p.handleLeaveRemoteSwarm}
					onJoinServer={p.handleJoinServer}
				/>
			)}

			{pendingNode && (
				<SwarmDangerDialog
					title="Remove Swarm Node"
					description={
						<>
							Are you sure you want to remove{' '}
							<strong className="text-foreground">
								{pendingNode.hostname || pendingNode.id}
							</strong>
							? Workloads still scheduled on it can fail — drain or promote
							the node first if it is serving traffic.
						</>
					}
					confirmLabel="Remove Node"
					busyLabel="Removing..."
					busy={isConfirming}
					onCancel={closeConfirm}
					onConfirm={() => void runConfirm()}
				/>
			)}
		</div>
	);
}
