import {createFileRoute} from '@tanstack/react-router';
import {RefreshCw, Rocket, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentActivityList} from '#/components/deployments/deployment-activity-list';
import {DeploymentsFilters} from '#/components/deployments/deployments-filters';
import {DeploymentLogsDialog} from '#/components/deployments/deployment-logs-dialog';
import {DeploymentErrorDialog} from '#/components/deployments/deployment-error-dialog';
import {EmptyState} from '#/components/activity/empty-state';
import {cn} from '#/api/utils';
import {Titlebar} from '#/components/shared/titlebar';
import {useDeploymentsPage} from '#/hooks/deployments/use-deployments-page';
import {
	TABS,
	type DeploymentsTabId,
} from '#/components/deployments/deployments-tabs';
import {DeploymentQueueTable} from '#/components/deployments/deployment-queue-table';
import {DeploymentsRail} from '#/components/deployments/deployments-rail';
import {DeploymentCancelDialog} from '#/components/deployments/deployment-cancel-dialog';

export const Route = createFileRoute('/_app/Deployments')({
	component: DeploymentsPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {id?: string; tab?: DeploymentsTabId} => ({
		id: typeof search.id === 'string' && search.id ? search.id : undefined,
		tab: search.tab === 'queue' ? 'queue' : undefined,
	}),
});

function DeploymentsPage() {
	const {
		dp,
		active,
		select,
		stats,
		cancelingId,
		setCancelingId,
		confirmCancel,
	} = useDeploymentsPage();

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">			{/* Vercel-style titlebar — quiet title + live count line */}
			<Titlebar
				breadcrumbs={[{label: 'Deployments', icon: Rocket}]}
				title="Deployments"
				description={
					dp.isLoading
						? 'Loading deployment history...'
						: `${stats.total} deployments across your infrastructure`
				}
				actions={
					<>
						<Button
							variant="outline"
							onClick={dp.handleRefresh}
							disabled={dp.refreshing}
							className="h-8 gap-1.5 text-xs font-medium">
							<RefreshCw
								className={cn(
									'size-3.5 text-muted-foreground',
									dp.refreshing && 'animate-spin',
								)}
							/>
							Refresh
						</Button>
						<Button
							variant="outline"
							onClick={dp.handleClearAllDeployments}
							className="h-8 gap-1.5 border-destructive/30 bg-destructive/10 text-xs font-semibold text-destructive hover:bg-destructive/20">
							<Trash2 className="size-3.5" />
							Clear History
						</Button>
					</>
				}
			>
				{/* Mobile: pill strip (inside the header, above the hairline) */}
				<div className="overflow-x-auto lg:hidden">
					<div className="inline-flex items-center gap-1 rounded-xl bg-card p-1">
						{TABS.map(t => (
							<button
								key={t.id}
								type="button"
								onClick={() => select(t.id)}
								className={cn(
									'inline-flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium whitespace-nowrap transition-colors',
									t.id === active
										? 'bg-foreground/[0.07] text-foreground'
										: 'text-muted-foreground hover:bg-foreground/[0.04] hover:text-foreground',
								)}>
								<t.icon className="size-[15px]" strokeWidth={1.7} />
								{t.label}
								<span className="rounded-md bg-muted/60 px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">
									{t.id === 'history'
										? dp.filteredAndSorted.length
										: dp.activeQueue.length}
								</span>
							</button>
						))}
					</div>
				</div>
			</Titlebar>

			{/* Main Grid (OpenShip): content left, sticky overview rail right */}
			<div className="grid min-w-0 grid-cols-1 gap-6 lg:grid-cols-[1fr_340px]">
				{/* LEFT COLUMN */}
				<div className="min-w-0 space-y-4">
					{active === 'history' ? (
						<>
							<DeploymentsFilters
								searchQuery={dp.searchQuery}
								setSearchQuery={dp.setSearchQuery}
								statusFilter={dp.statusFilter}
								setStatusFilter={dp.setStatusFilter}
								typeFilter={dp.typeFilter}
								setTypeFilter={dp.setTypeFilter}
								sortBy={dp.sortBy}
								sortDir={dp.sortDir}
								setSortBy={dp.setSortBy}
								setSortDir={dp.setSortDir}
							/>

							{dp.isLoading ? (
								<div className="flex flex-col items-center justify-center gap-3 py-20">
									<RefreshCw className="size-8 animate-spin text-primary" />
									<p className="animate-pulse text-sm font-medium text-muted-foreground">
										Loading deployments history...
									</p>
								</div>
							) : dp.filteredAndSorted.length > 0 ? (
								<DeploymentActivityList
									deployments={dp.filteredAndSorted}
									onViewLogs={dp.setSelectedDeployment as never}
								/>
							) : (
								<EmptyState
									hasFilters={Boolean(
										dp.searchQuery ||
										dp.statusFilter !== 'all' ||
										dp.typeFilter !== 'all',
									)}
									onClearFilters={dp.clearFilters}
								/>
							)}
						</>
					) : (
						<DeploymentQueueTable
							queue={dp.activeQueue}
							onViewLogs={dp.setSelectedDeployment}
							onCancel={setCancelingId}
						/>
					)}
				</div>

				<DeploymentsRail
					active={active}
					onSelect={select}
					historyCount={dp.filteredAndSorted.length}
					queueCount={dp.activeQueue.length}
					stats={stats}
					isLoading={dp.isLoading}
				/>
			</div>

			{/* Logs Stream Dialog */}
			<DeploymentLogsDialog
				selectedDeployment={dp.selectedDeployment}
				onClose={() => dp.setSelectedDeployment(null)}
				logs={dp.logs}
				isLogsLoading={dp.isLogsLoading}
				copied={dp.copied}
				onCopyLogs={dp.handleCopyLogs}
			/>

			{/* Error Detail Dialog */}
			<DeploymentErrorDialog
				errorDetailDeployment={dp.errorDetailDeployment}
				onClose={() => dp.setErrorDetailDeployment(null)}
			/>

			<DeploymentCancelDialog
				cancelingId={cancelingId}
				onClose={() => setCancelingId(null)}
				onConfirm={confirmCancel}
			/>
		</div>
	);
}
