import {createFileRoute, Link} from '@tanstack/react-router';
import {Rocket, SearchX, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListEmpty,
	ListRegion,
	ListSkeleton,
	SectionHeader,
} from '#/components/settings/list-card';
import {DeploymentActivityList} from '#/components/deployments/deployment-activity-list';
import {DeploymentsFilters} from '#/components/deployments/deployments-filters';
import {useDeploymentsPage} from '#/hooks/deployments/use-deployments-page';

export const Route = createFileRoute('/_app/Deployments')({
	component: DeploymentsPage,
	validateSearch: (search: Record<string, unknown>): {id?: string} => ({
		id: typeof search.id === 'string' && search.id ? search.id : undefined,
	}),
});

/** Logs & History — its own page (no tab switch): the same SectionHeader grammar as the rest of the rail, the filters toolbar and ONE bordered table card below. The live stream lives at /Deployments/queue. */
function DeploymentsPage() {
	const dp = useDeploymentsPage();

	const historyEmpty = dp.filteredAndSorted.length === 0;
	const hasFilters = Boolean(
		dp.searchQuery || dp.statusFilter !== 'all' || dp.typeFilter !== 'all',
	);

	return (
		<div className="mx-auto flex h-full w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Logs & History"
				description="Every build and deploy stream across your projects."
				action={
					<Button
						variant="ghost"
						onClick={dp.handleClearAllDeployments}
						className="h-8 gap-1.5 text-xs font-medium text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
						<Trash2 className="size-3.5" />
						Clear History
					</Button>
				}
			/>

			<div className="flex min-h-0 flex-1 flex-col gap-4">
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

				<ListRegion>
					{dp.isLoading ? (
						<ListSkeleton rows={6} />
					) : historyEmpty ? (
						hasFilters ? (
							<ListEmpty
								icon={SearchX}
								title="No matching deployments"
								description="Nothing matches the current filters. Try a different search or clear the filters to see everything."
								action={
									<Button
										variant="ghost"
										onClick={dp.clearFilters}
										className="text-xs font-semibold text-primary">
										Clear All Filters
									</Button>
								}
							/>
						) : (
							<ListEmpty
								icon={Rocket}
								title="No deployments yet"
								description="Deploy your first app or database and its build history will appear here."
								action={
									<Link to="/projects" search={{create: 1} as never}>
										<Button className="h-9 gap-1.5 text-xs font-semibold">
											New Project
										</Button>
									</Link>
								}
							/>
						)
					) : (
						<DeploymentActivityList deployments={dp.filteredAndSorted} />
					)}
				</ListRegion>
			</div>
		</div>
	);
}
