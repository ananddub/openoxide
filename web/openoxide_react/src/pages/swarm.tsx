import {SwarmHeader} from '#/components/swarm/swarm-header';
import {SwarmInfoCard} from '#/components/swarm/swarm-info-card';
import {SwarmNodesList} from '#/components/swarm/swarm-nodes-list';
import {JoinTokensModal} from '#/components/swarm/join-tokens-modal';
import {useSwarmPanel} from '#/components/swarm/use-swarm-panel';

// Rendered as the "Swarm" tab of pages/infra.tsx — no standalone route.
export function SwarmPanel() {
	const p = useSwarmPanel();

	return (
		<div className="flex w-full flex-col gap-6">
			<SwarmHeader
				servers={p.servers}
				selectedServerId={p.selectedServerId}
				onSelectServer={p.setSelectedServerId}
				onRefresh={p.refreshAll}
				onOpenTokens={() => p.setIsTokensOpen(true)}
				isRefreshing={p.isRefreshing}
				isSwarmActive={p.isSwarmActive}
			/>

			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-6">
					<SwarmNodesList
						nodes={p.nodes}
						isLoading={p.isLoading}
						onPromote={(id, n) => p.handlePromote(id, n)}
						onDemote={(id, n) => p.handleDemote(id, n)}
						onSetAvailability={(id, av, n) =>
							p.handleSetAvailability(id, av, n)
						}
						onRemoveNode={(id, n) => p.handleRemoveNode(id, n)}
					/>
				</div>

				{/* ── Right rail: cluster summary ── */}
				<div className="flex flex-col gap-4 lg:sticky lg:top-6 lg:self-start">
					<SwarmInfoCard
						info={p.info}
						isLoading={p.isLoading}
						onLeaveSwarm={p.handleLeaveSwarm}
					/>
				</div>
			</div>

			{p.isTokensOpen && (
				<JoinTokensModal
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
		</div>
	);
}
