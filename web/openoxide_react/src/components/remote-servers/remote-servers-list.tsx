import {useState, useEffect, useMemo} from 'react';
import {serverHostLabel} from '#/api/utils';
import {ListRegion} from '#/components/settings/list-card';
import {globalServerConnStore} from '#/stores/server-connection-store';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {
	ServerFilterEmpty,
	ServerListEmpty,
	ServerListSkeleton,
	ServersToolbar,
	type StatusFilter,
	type ServersView,
} from './server-list-states';
import {ServerRow, ServerCard} from './server-row';

interface RemoteServersListProps {
	servers: RemoteServerResponse[];
	isLoading: boolean;
	onAddServer: () => void;
	onOpenSettings: (server: RemoteServerResponse) => void;
}

const VIEW_STORAGE_KEY = 'remote-servers-view';

export function RemoteServersList({
	servers,
	isLoading,
	onAddServer,
	onOpenSettings,
}: RemoteServersListProps) {
	const [storeVersion, setStoreVersion] = useState(0);
	const [search, setSearch] = useState('');
	const [statusFilter, setStatusFilter] = useState<StatusFilter>('All');
	const [view, setView] = useState<ServersView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});

	const handleViewChange = (next: ServersView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	useEffect(() => {
		const unsubscribe = globalServerConnStore.subscribe(() =>
			setStoreVersion(v => v + 1),
		);
		return unsubscribe;
	}, []);

	const filtered = useMemo(() => {
		return servers.filter(s => {
			const matchName =
				s.name.toLowerCase().includes(search.toLowerCase()) ||
				serverHostLabel(s.ip_address)
					.toLowerCase()
					.includes(search.toLowerCase());
			const isConnected =
				(s.server_status || 'INACTIVE').toUpperCase() === 'ACTIVE';
			const matchStatus =
				statusFilter === 'All' ||
				(statusFilter === 'Connected' && isConnected) ||
				(statusFilter === 'Not Connected' && !isConnected);
			return matchName && matchStatus;
		});
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [servers, search, statusFilter, storeVersion]);

	const hasFilters = search !== '' || statusFilter !== 'All';
	const clearFilters = () => {
		setSearch('');
		setStatusFilter('All');
	};

	/* ── Loading ─ */
	if (isLoading) return <ServerListSkeleton />;

	return (
		<div className="flex h-full min-h-0 w-full flex-col gap-3">
			<ServersToolbar
				search={search}
				status={statusFilter}
				onSearch={setSearch}
				onStatus={setStatusFilter}
				view={view}
				onViewChange={handleViewChange}
			/>

			<ListRegion>
				{servers.length === 0 ? (
					<ServerListEmpty onAdd={onAddServer} />
				) : filtered.length === 0 ? (
					<ServerFilterEmpty onClear={clearFilters} />
				) : view === 'grid' ? (
					/* ── Grid cards ── */
					<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
						{filtered.map(item => (
							<ServerCard
								key={item.id}
								item={item}
								connStatus={globalServerConnStore.getStatus(item.id)}
								onOpen={onOpenSettings}
							/>
						))}
					</div>
				) : (
					/* ── Rows — one bordered card, divided rows (openship) ── */
					<div className="max-h-full divide-y divide-border/60 overflow-y-auto rounded-xl border bg-card">
						{filtered.map(item => (
							<ServerRow
								key={item.id}
								item={item}
								connStatus={globalServerConnStore.getStatus(item.id)}
								onOpen={onOpenSettings}
							/>
						))}
					</div>
				)}

				{/* Result count */}
				{hasFilters && filtered.length > 0 && (
					<p className="px-1 text-xs text-muted-foreground">
						Showing {filtered.length} of {servers.length} servers
					</p>
				)}
			</ListRegion>
		</div>
	);
}
