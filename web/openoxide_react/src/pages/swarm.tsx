import {useState} from 'react';
import {SwarmInfoCard} from '#/components/swarm/swarm-info-card';
import {
	SwarmNodesList,
	type TaggedSwarmNode,
} from '#/components/swarm/swarm-nodes-list';
import {JoinTokensSheet} from '#/components/swarm/join-tokens-sheet';
import {SwarmDangerDialog} from '#/components/swarm/swarm-danger-dialog';
import {useSwarmPanel} from '#/components/swarm/use-swarm-panel';

type SwarmConfirm =
	| {kind: 'remove'; node: TaggedSwarmNode}
	| {kind: 'leave'};

// Rendered as the "Swarm" tab of pages/infra.tsx — no standalone route.
export function SwarmPanel() {
	const p = useSwarmPanel();
	const [pending, setPending] = useState<SwarmConfirm | null>(null);
	const [isConfirming, setIsConfirming] = useState(false);

	const closeConfirm = () => setPending(null);

	const runConfirm = async () => {
		if (!pending) return;
		setIsConfirming(true);
		try {
			if (pending.kind === 'remove') {
				await p.handleRemoveNode(pending.node.id, pending.node);
			} else {
				await p.handleLeaveSwarm();
			}
		} finally {
			setIsConfirming(false);
			setPending(null);
		}
	};

	return (
		<div className="flex w-full flex-col gap-6">
			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					<SwarmNodesList
						nodes={p.nodes}
						isLoading={p.isLoading}
						servers={p.servers}
						selectedServerId={p.selectedServerId}
						onSelectServer={p.setSelectedServerId}
						isRefreshing={p.isRefreshing}
						onRefresh={p.refreshAll}
						isSwarmActive={p.isSwarmActive}
						onOpenTokens={() => p.setIsTokensOpen(true)}
						onPromote={(id, n) => p.handlePromote(id, n)}
						onDemote={(id, n) => p.handleDemote(id, n)}
						onSetAvailability={(id, av, n) =>
							p.handleSetAvailability(id, av, n)
						}
						onRemoveNode={(_id, n) =>
							n && setPending({kind: 'remove', node: n})
						}
					/>
				</div>

				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<SwarmInfoCard
						info={p.info}
						isLoading={p.isLoading}
						onLeaveSwarm={() => setPending({kind: 'leave'})}
					/>
				</div>
			</div>

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

			{pending?.kind === 'remove' && (
				<SwarmDangerDialog
					title="Remove Swarm Node"
					description={
						<>
							Are you sure you want to remove{' '}
							<strong className="text-foreground">
								{pending.node.hostname || pending.node.id}
							</strong>
							? Workloads still scheduled on it can fail — drain or
							promote the node first if it is serving traffic.
						</>
					}
					confirmLabel="Remove Node"
					busyLabel="Removing..."
					busy={isConfirming}
					onCancel={closeConfirm}
					onConfirm={() => void runConfirm()}
				/>
			)}

			{pending?.kind === 'leave' && (
				<SwarmDangerDialog
					title="Leave Swarm"
					description="This engine leaves the cluster and stops serving as a node. Deployments already running keep going, but the panel loses cluster control until it rejoins."
					confirmLabel="Leave Swarm"
					busyLabel="Leaving..."
					busy={isConfirming}
					onCancel={closeConfirm}
					onConfirm={() => void runConfirm()}
				/>
			)}
		</div>
	);
}
