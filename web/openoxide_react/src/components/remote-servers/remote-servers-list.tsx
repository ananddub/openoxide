import {useState, useEffect, useMemo} from 'react';
import {useNavigate} from '@tanstack/react-router';
import {toast} from 'sonner';
import {formatApiError, serverHostLabel} from '#/api/utils';
import {globalServerConnStore} from '#/hooks/use-server-connection-store';
import {useRemoteServerTestConnection} from '#/api/live';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';
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
	sshKeys?: SshKeyResponse[];
	isLoading: boolean;
	onAddServer: () => void;
	onEditServer: (server: RemoteServerResponse) => void;
	onDeleteServer: (server: RemoteServerResponse) => void;
	onSetupServer: (server: RemoteServerResponse) => void;
	onToggleStatus: (server: RemoteServerResponse) => void;
	onOpenTerminal?: (server: RemoteServerResponse) => void;
	onMigrateServer?: (server: RemoteServerResponse) => void;
}

const VIEW_STORAGE_KEY = 'remote-servers-view';

export function RemoteServersList({
	servers,
	isLoading,
	onAddServer,
	onEditServer,
	onDeleteServer,
	onSetupServer,
	onToggleStatus,
	onOpenTerminal,
	onMigrateServer,
}: RemoteServersListProps) {
	const navigate = useNavigate();
	const [storeVersion, setStoreVersion] = useState(0);
	const [search, setSearch] = useState('');
	const [statusFilter, setStatusFilter] = useState<StatusFilter>('All');
	const [view, setView] = useState<ServersView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const testConnMutation = useRemoteServerTestConnection();

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

	const handleTestConnection = async (server: RemoteServerResponse) => {
		globalServerConnStore.setStatus(server.id, 'testing');
		try {
			await testConnMutation.mutateAsync(server.id);
			globalServerConnStore.setStatus(server.id, 'success');
			toast.success(`SSH connection verified for "${server.name}"`);
		} catch (err: unknown) {
			globalServerConnStore.setStatus(server.id, 'failed');
			toast.error(formatApiError(err));
		}
	};

	const actions = {
		onTest: handleTestConnection,
		onSetup: onSetupServer,
		onToggleStatus,
		onEdit: onEditServer,
		onDelete: onDeleteServer,
		onOpenTerminal,
		onMigrate: onMigrateServer,
		onNetwork: (s: RemoteServerResponse) =>
			void navigate({
				to: '/remote-servers/$id',
				params: {id: String(s.id)},
				search: {tab: 'network'},
			}),
	};

	/* ── Loading ─ */
	if (isLoading) return <ServerListSkeleton />;

	return (
		<div className="flex min-w-0 flex-col gap-3">
			<ServersToolbar
				search={search}
				status={statusFilter}
				onSearch={setSearch}
				onStatus={setStatusFilter}
				view={view}
				onViewChange={handleViewChange}
			/>

			{servers.length === 0 ? (
				<ServerListEmpty onAdd={onAddServer} />
			) : filtered.length === 0 ? (
				<ServerFilterEmpty onClear={clearFilters} />
			) : view === 'grid' ? (
				/* ── Grid cards ── */
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
					{filtered.map(item => (
						<ServerCard
							key={item.id}
							item={item}
							connStatus={globalServerConnStore.getStatus(item.id)}
							actions={actions}
						/>
					))}
				</div>
			) : (
				/* ── Rows — one bordered card, divided rows (openship) ── */
				<div className="divide-y divide-border/60 overflow-hidden rounded-xl border bg-card">
					{filtered.map(item => (
						<ServerRow
							key={item.id}
							item={item}
							connStatus={globalServerConnStore.getStatus(item.id)}
							actions={actions}
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
		</div>
	);
}
