import { useState, useMemo } from 'react';
import { Search, Loader2 } from 'lucide-react';
import { Input } from '#/components/ui/input';
import { Button } from '#/components/ui/button';
import {
	Select,
	SelectTrigger,
	SelectValue,
	SelectContent,
	SelectItem,
} from '#/components/ui/select';
import { useOrganizationStore } from '#/stores/organization-store';
import { useAppStore } from '#/stores/app-store';
import { $api } from '#/api/query';
import { client } from '#/api/client';
import {
	useOverviewServices,
	useProjectListByOrganization,
} from 'virtual:openoxide-live';
import { toast } from 'sonner';
import { formatApiError } from '#/api/utils';
import {
	OverviewServicesTable,
	type OverviewServiceItem,
} from './overview-services-table';

export function OverviewServicesTab() {
	const [searchQuery, setSearchQuery] = useState('');
	const [selectedProjectId, setSelectedProjectId] = useState('all');
	const [selectedType, setSelectedType] = useState('all');
	const [selectedStatus, setSelectedStatus] = useState('all');
	const [sortBy, setSortBy] = useState('newest');
	const [pageSize, setPageSize] = useState(50);
	const [pageIndex, setPageIndex] = useState(0);

	const { activeOrg } = useOrganizationStore();
	const orgId = activeOrg?.id || 1;

	// Zustand RAM store overviewServices
	const storeServices = useAppStore((state) => state.overviewServices);

	// Live Projects stream for Project Select filter
	const { data: projectsToUse = [] } = useProjectListByOrganization(BigInt(orgId));

	// LIVE REALTIME Socket.IO stream from virtual:openoxide-live
	const { data: liveServicesQuery, loading: isLiveLoading } = useOverviewServices(BigInt(orgId));

	// Prefer live Socket.IO stream data, fallback to Zustand RAM store
	const rawServices = useMemo(() => {
		if (liveServicesQuery && Array.isArray(liveServicesQuery) && liveServicesQuery.length > 0) {
			return liveServicesQuery;
		}
		return storeServices || [];
	}, [liveServicesQuery, storeServices]);

	const isLoading = isLiveLoading && rawServices.length === 0;

	// App Mutations
	const appStart = $api.useMutation('post', '/applications/{id}/start');
	const appStop = $api.useMutation('post', '/applications/{id}/stop');
	const appDeploy = $api.useMutation('post', '/applications/{id}/deploy');

	// Compose Mutations
	const composeStart = $api.useMutation('post', '/compose/{id}/start');
	const composeStop = $api.useMutation('post', '/compose/{id}/stop');
	const composeDeploy = $api.useMutation('post', '/compose/{id}/deploy');

	// Transform raw API data into OverviewServiceItem list
	const allServices = useMemo<OverviewServiceItem[]>(() => {
		if (!rawServices || !Array.isArray(rawServices)) return [];

		return rawServices.map((svc: any) => ({
			key: `${svc.service_type}-${svc.id}`,
			id: Number(svc.id),
			name: svc.name,
			type: (svc.service_type || 'APP') as 'APP' | 'COMPOSE' | 'DATABASE',
			status: svc.status || 'done',
			createdAt: Number(svc.created_at),
			projectId: Number(svc.project_id),
			projectName: svc.project_name || 'Project',
			environmentId: Number(svc.environment_id),
			environmentName: svc.environment_name || 'production',
			dbKind: svc.db_kind || undefined,
			serverName: 'Rustploy Server',
		}));
	}, [rawServices]);

	// Filter and sort services
	const filteredServices = useMemo(() => {
		let list = allServices.filter((s) => {
			const matchesSearch =
				!searchQuery ||
				s.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				s.projectName.toLowerCase().includes(searchQuery.toLowerCase());

			const matchesProject =
				selectedProjectId === 'all' || String(s.projectId) === String(selectedProjectId);

			const matchesType =
				selectedType === 'all' ||
				(selectedType === 'application' && s.type === 'APP') ||
				(selectedType === 'compose' && s.type === 'COMPOSE') ||
				(selectedType === 'database' && s.type === 'DATABASE') ||
				(s.dbKind && s.dbKind === selectedType);

			const matchesStatus =
				selectedStatus === 'all' || s.status.toLowerCase().includes(selectedStatus.toLowerCase());

			return matchesSearch && matchesProject && matchesType && matchesStatus;
		});

		if (sortBy === 'newest') list.sort((a, b) => b.createdAt - a.createdAt);
		else if (sortBy === 'oldest') list.sort((a, b) => a.createdAt - b.createdAt);
		else if (sortBy === 'name-asc') list.sort((a, b) => a.name.localeCompare(b.name));
		else if (sortBy === 'name-desc') list.sort((a, b) => b.name.localeCompare(a.name));

		return list;
	}, [allServices, searchQuery, selectedProjectId, selectedType, selectedStatus, sortBy]);

	// Pagination
	const pageCount = Math.max(1, Math.ceil(filteredServices.length / pageSize));
	const currentPageIndex = Math.min(pageIndex, pageCount - 1);
	const pagedServices = useMemo(() => {
		const start = currentPageIndex * pageSize;
		return filteredServices.slice(start, start + pageSize);
	}, [filteredServices, currentPageIndex, pageSize]);

	// Action Handlers
	const handleDeploy = async (svc: OverviewServiceItem) => {
		try {
			if (svc.type === 'APP') {
				await appDeploy.mutateAsync({ params: { path: { id: svc.id } } });
			} else if (svc.type === 'COMPOSE') {
				await composeDeploy.mutateAsync({ params: { path: { id: svc.id } } });
			} else if (svc.dbKind) {
				const k = svc.dbKind.toLowerCase();
				await client.POST(`/${k}/{id}/deploy` as any, { params: { path: { id: svc.id } } });
			}
			toast.success(`Queued deployment for ${svc.name}`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleStart = async (svc: OverviewServiceItem) => {
		try {
			if (svc.type === 'APP') {
				await appStart.mutateAsync({ params: { path: { id: svc.id } } });
			} else if (svc.type === 'COMPOSE') {
				await composeStart.mutateAsync({ params: { path: { id: svc.id } } });
			} else if (svc.dbKind) {
				const k = svc.dbKind.toLowerCase();
				await client.POST(`/${k}/{id}/start` as any, { params: { path: { id: svc.id } } });
			}
			toast.success(`Starting ${svc.name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	const handleStop = async (svc: OverviewServiceItem) => {
		try {
			if (svc.type === 'APP') {
				await appStop.mutateAsync({ params: { path: { id: svc.id } } });
			} else if (svc.type === 'COMPOSE') {
				await composeStop.mutateAsync({ params: { path: { id: svc.id } } });
			} else if (svc.dbKind) {
				const k = svc.dbKind.toLowerCase();
				await client.POST(`/${k}/{id}/stop` as any, { params: { path: { id: svc.id } } });
			}
			toast.info(`Stopping ${svc.name}...`);
		} catch (err) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-5 w-full">
			{/* Top Header & Filter Toolbar */}
			<div className="flex flex-wrap items-center justify-between gap-4">
				<h3 className="text-lg font-bold tracking-tight text-foreground">
					Services{' '}
					<span className="text-sm font-normal text-muted-foreground">
						({filteredServices.length})
					</span>
				</h3>

				<div className="flex flex-wrap items-center gap-2.5">
					{/* Search Input */}
					<div className="relative">
						<Input
							placeholder="Filter services..."
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
							className="pr-9 w-[190px] h-9 text-xs"
						/>
						<Search className="absolute right-3 top-1/2 -translate-y-1/2 size-3.5 text-muted-foreground" />
					</div>

					{/* Project Filter Select */}
					<Select value={selectedProjectId} onValueChange={(val) => val && setSelectedProjectId(val)}>
						<SelectTrigger size="sm" className="w-[145px] text-xs font-semibold h-9">
							<SelectValue placeholder="All Projects" />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="all">All Projects</SelectItem>
							{projectsToUse.map((p: any) => (
								<SelectItem key={p.id} value={String(p.id)}>
									{p.name}
								</SelectItem>
							))}
						</SelectContent>
					</Select>

					{/* Type Filter Select */}
					<Select value={selectedType} onValueChange={(val) => val && setSelectedType(val)}>
						<SelectTrigger size="sm" className="w-[135px] text-xs font-semibold h-9">
							<SelectValue placeholder="All Types" />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="all">All Types</SelectItem>
							<SelectItem value="application">Application</SelectItem>
							<SelectItem value="compose">Compose</SelectItem>
							<SelectItem value="database">Database</SelectItem>
							<SelectItem value="postgres">PostgreSQL</SelectItem>
							<SelectItem value="mysql">MySQL</SelectItem>
							<SelectItem value="mariadb">MariaDB</SelectItem>
							<SelectItem value="mongo">MongoDB</SelectItem>
							<SelectItem value="redis">Redis</SelectItem>
						</SelectContent>
					</Select>

					{/* Status Filter Select */}
					<Select value={selectedStatus} onValueChange={(val) => val && setSelectedStatus(val)}>
						<SelectTrigger size="sm" className="w-[135px] text-xs font-semibold h-9">
							<SelectValue placeholder="All Statuses" />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="all">All Statuses</SelectItem>
							<SelectItem value="done">Running</SelectItem>
							<SelectItem value="deploying">Deploying</SelectItem>
							<SelectItem value="idle">Idle</SelectItem>
							<SelectItem value="error">Error</SelectItem>
						</SelectContent>
					</Select>

					{/* Sort By Select */}
					<Select value={sortBy} onValueChange={(val) => val && setSortBy(val)}>
						<SelectTrigger size="sm" className="w-[155px] text-xs font-semibold h-9">
							<SelectValue placeholder="Sort: Newest First" />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value="newest">Sort: Newest First</SelectItem>
							<SelectItem value="oldest">Sort: Oldest First</SelectItem>
							<SelectItem value="name-asc">Sort: Name (A-Z)</SelectItem>
							<SelectItem value="name-desc">Sort: Name (Z-A)</SelectItem>
						</SelectContent>
					</Select>
				</div>
			</div>

			{/* Loading State */}
			{isLoading && (
				<div className="flex items-center justify-center gap-2 py-16 text-muted-foreground text-xs">
					<Loader2 className="size-4 animate-spin text-primary" />
					Loading services...
				</div>
			)}

			{/* Empty Filter State */}
			{!isLoading && filteredServices.length === 0 && (
				<div className="flex flex-col items-center justify-center gap-2 py-16 text-muted-foreground text-xs">
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

					{/* Pagination Controls */}
					<div className="flex flex-col sm:flex-row items-center justify-between gap-4 pt-3 text-xs text-muted-foreground border-t border-border/40">
						<span>
							{filteredServices.length} {filteredServices.length === 1 ? 'service' : 'services'} total
						</span>

						<div className="flex items-center gap-4">
							<div className="flex items-center gap-2">
								<span className="whitespace-nowrap">Rows per page</span>
								<Select
									value={String(pageSize)}
									onValueChange={(val) => {
										if (val) {
											setPageSize(Number(val));
											setPageIndex(0);
										}
									}}
								>
									<SelectTrigger size="sm" className="w-[75px] h-8 text-xs font-semibold">
										<SelectValue placeholder="50" />
									</SelectTrigger>
									<SelectContent>
										<SelectItem value="25">25</SelectItem>
										<SelectItem value="50">50</SelectItem>
										<SelectItem value="100">100</SelectItem>
										<SelectItem value="200">200</SelectItem>
									</SelectContent>
								</Select>
							</div>

							<span className="whitespace-nowrap font-mono">
								Page {currentPageIndex + 1} of {pageCount}
							</span>

							<div className="flex gap-2">
								<Button
									variant="outline"
									size="sm"
									onClick={() => setPageIndex(Math.max(0, currentPageIndex - 1))}
									disabled={currentPageIndex === 0}
									className="h-8 text-xs font-semibold px-3"
								>
									Previous
								</Button>
								<Button
									variant="outline"
									size="sm"
									onClick={() => setPageIndex(Math.min(pageCount - 1, currentPageIndex + 1))}
									disabled={currentPageIndex + 1 >= pageCount}
									className="h-8 text-xs font-semibold px-3"
								>
									Next
								</Button>
							</div>
						</div>
					</div>
				</>
			)}
		</div>
	);
}
