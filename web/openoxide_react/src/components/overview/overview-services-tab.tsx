import {useState, useMemo} from 'react';
import {Loader2} from 'lucide-react';
import {
	useRemoteServerList,
	useProjectListByOrganization,
	useOverviewServices,
} from '#/api/live';
import {useOrganizations} from '#/stores/session-context';
import {
	OverviewServicesTable,
	type OverviewServiceItem,
} from './overview-services-table';
import {OverviewServicesToolbar} from './overview-services-toolbar';
import {TablePagination} from './table-pagination';
import {useServiceActions} from './use-service-actions';
import {toServiceItems} from './map-services';

export function OverviewServicesTab() {
	const [searchQuery, setSearchQuery] = useState('');
	const [selectedProjectId, setSelectedProjectId] = useState('all');
	const [selectedType, setSelectedType] = useState('all');
	const [selectedStatus, setSelectedStatus] = useState('all');
	const [sortBy, setSortBy] = useState('newest');
	const [pageSize, setPageSize] = useState(50);
	const [pageIndex, setPageIndex] = useState(0);

	// Live query keyed ['overview-services'] — the table-watch stream
	// refetches it on any services-table change while the tab is mounted.
	const servicesQuery = useOverviewServices();
	// Live projects query keyed ['projects','list',org] — table-watch
	// invalidates it while the overview page is mounted.
	const {activeOrg} = useOrganizations();
	const projectsQuery = useProjectListByOrganization(
		BigInt(activeOrg?.id || 1),
	);
	const projectsToUse = (projectsQuery.data ?? []) as any[];
	const {data: remoteServers = []} = useRemoteServerList();
	// Live servers query; the table-watch stream keeps it fresh, so no
	// zustand fallback is needed anymore.
	const rawServices = servicesQuery.data ?? [];

	const isLoading = servicesQuery.isLoading && rawServices.length === 0;

	const {handleDeploy, handleStart, handleStop} = useServiceActions();

	// Transform raw API data into OverviewServiceItem list
	const allServices = useMemo<OverviewServiceItem[]>(
		() => toServiceItems(rawServices, remoteServers as any[]),
		[rawServices, remoteServers],
	);

	// Filter and sort services
	const filteredServices = useMemo(() => {
		let list = allServices.filter(s => {
			const matchesSearch =
				!searchQuery ||
				s.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				s.projectName.toLowerCase().includes(searchQuery.toLowerCase());

			const matchesProject =
				selectedProjectId === 'all' ||
				String(s.projectId) === String(selectedProjectId);

			const matchesType =
				selectedType === 'all' ||
				(selectedType === 'application' && s.type === 'APP') ||
				(selectedType === 'compose' && s.type === 'COMPOSE') ||
				(selectedType === 'database' && s.type === 'DATABASE') ||
				(s.dbKind && s.dbKind === selectedType);

			const matchesStatus =
				selectedStatus === 'all' ||
				s.status.toLowerCase().includes(selectedStatus.toLowerCase());

			return (
				matchesSearch && matchesProject && matchesType && matchesStatus
			);
		});

		if (sortBy === 'newest')
			list.sort((a, b) => b.createdAt - a.createdAt);
		else if (sortBy === 'oldest')
			list.sort((a, b) => a.createdAt - b.createdAt);
		else if (sortBy === 'name-asc')
			list.sort((a, b) => a.name.localeCompare(b.name));
		else if (sortBy === 'name-desc')
			list.sort((a, b) => b.name.localeCompare(a.name));

		return list;
	}, [
		allServices,
		searchQuery,
		selectedProjectId,
		selectedType,
		selectedStatus,
		sortBy,
	]);

	// Pagination
	const pageCount = Math.max(
		1,
		Math.ceil(filteredServices.length / pageSize),
	);
	const currentPageIndex = Math.min(pageIndex, pageCount - 1);
	const pagedServices = useMemo(() => {
		const start = currentPageIndex * pageSize;
		return filteredServices.slice(start, start + pageSize);
	}, [filteredServices, currentPageIndex, pageSize]);

	return (
		<div className="flex w-full flex-col gap-5">
			<OverviewServicesToolbar
				count={filteredServices.length}
				search={searchQuery}
				onSearch={setSearchQuery}
				projectId={selectedProjectId}
				onProjectId={setSelectedProjectId}
				projects={projectsToUse}
				type={selectedType}
				onType={setSelectedType}
				status={selectedStatus}
				onStatus={setSelectedStatus}
				sort={sortBy}
				onSort={setSortBy}
			/>

			{/* Loading State */}
			{isLoading && (
				<div className="flex items-center justify-center gap-2 py-16 text-xs text-muted-foreground">
					<Loader2 className="size-4 animate-spin text-primary" />
					Loading services...
				</div>
			)}

			{/* Empty Filter State */}
			{!isLoading && filteredServices.length === 0 && (
				<div className="flex flex-col items-center justify-center gap-2 py-16 text-xs text-muted-foreground">
					<span>No services match the current filters.</span>
				</div>
			)}

			{/* Table & Pagination Footer */}
			{!isLoading && filteredServices.length > 0 && (
				<>
					<OverviewServicesTable
						services={pagedServices}
						onDeploy={handleDeploy}
						onStart={handleStart}
						onStop={handleStop}
					/>

					<TablePagination
						total={filteredServices.length}
						unit="service"
						pageSize={pageSize}
						onPageSize={setPageSize}
						pageIndex={currentPageIndex}
						pageCount={pageCount}
						onGoto={setPageIndex}
					/>
				</>
			)}
		</div>
	);
}
