import {createFileRoute} from '@tanstack/react-router';
import {RefreshCw, Rocket, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {DeploymentActivityList} from '#/components/deployments/deployment-activity-list';
import {DeploymentsFilters} from '#/components/deployments/deployments-filters';
import {EmptyState} from '#/components/activity/empty-state';
import {cn} from '#/api/utils';
import {Titlebar} from '#/components/shared/titlebar';
import {useDeploymentsPage} from '#/hooks/deployments/use-deployments-page';
import {
	TABS,
	type DeploymentsTabId,
} from '#/components/deployments/deployments-tabs';
import {DeploymentsRail} from '#/components/deployments/deployments-rail';

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
	const {dp, active, select, stats} = useDeploymentsPage();

	return (
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			{/* Vercel-style titlebar — quiet title + live count line */}
			<Titlebar
				breadcrumbs={[{label: 'Deployments', icon: Rocket}]}
				actions={
					<>
						<span className="select-none text-xs text-muted-foreground">
							{dp.isLoading
								? 'Loading...'
								: `${stats.total} deployments`}
						</span>
						<Button
							variant="ghost"
							onClick={dp.handleRefresh}
							disabled={dp.refreshing}
							className="flex h-8 items-center gap-1.5 text-xs font-medium text-muted-foreground hover:text-foreground">
							<RefreshCw
								className={cn(
									'size-3.5',
									dp.refreshing && 'animate-spin',
								)}
							/>
							Refresh
						</Button>
						<Button
							variant="ghost"
							onClick={dp.handleClearAllDeployments}
							className="flex h-8 items-center gap-1.5 text-xs font-medium text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
							<Trash2 className="size-3.5" />
							Clear History
						</Button>
					</>
				}>
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
								<t.icon
									className="size-[15px]"
									strokeWidth={1.7}
								/>
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

			{/* Main grid: content left, sticky overview rail right */}
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
										Loading deployments
										history...
									</p>
								</div>
							) : dp.filteredAndSorted.length > 0 ? (
								<DeploymentActivityList
									deployments={dp.filteredAndSorted}
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
					) : dp.activeQueue.length > 0 ? (
						<DeploymentActivityList
							deployments={dp.activeQueue}
						/>
					) : (
						<EmptyState
							hasFilters={false}
							onClearFilters={dp.clearFilters}
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
		</div>
	);
}
