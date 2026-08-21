import {useState, useEffect, useMemo} from 'react';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Separator} from '#/components/ui/separator';
import {Skeleton} from '#/components/ui/skeleton';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {globalServerConnStore} from '#/hooks/use-server-connection-store';
import {
	Server,
	Plug,
	RefreshCw,
	Check,
	X,
	MoreVertical,
	Terminal,
	Pencil,
	Trash2,
	ShieldCheck,
	Shield,
	AlertCircle,
	Search,
	Power,
} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';

interface RemoteServersListProps {
	servers: RemoteServerResponse[];
	sshKeys: SshKeyResponse[];
	isLoading: boolean;
	onEditServer: (server: RemoteServerResponse) => void;
	onDeleteServer: (server: RemoteServerResponse) => void;
	onSetupServer: (server: RemoteServerResponse) => void;
	onToggleStatus: (server: RemoteServerResponse) => void;
	onOpenTerminal?: (server: RemoteServerResponse) => void;
	onPrivateNetwork: (server: RemoteServerResponse) => void;
}

const STATUS_FILTERS = ['All', 'Connected', 'Not Connected'] as const;
type StatusFilter = (typeof STATUS_FILTERS)[number];

export function RemoteServersList({
	servers,
	isLoading,
	onEditServer,
	onDeleteServer,
	onSetupServer,
	onToggleStatus,
	onOpenTerminal,
	onPrivateNetwork,
}: RemoteServersListProps) {
	const [storeVersion, setStoreVersion] = useState(0);
	const [search, setSearch] = useState('');
	const [statusFilter, setStatusFilter] = useState<StatusFilter>('All');
	const testConnMutation = $api.useMutation(
		'post',
		'/servers/{id}/test-connection',
	);

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
				s.ip_address?.toLowerCase().includes(search.toLowerCase());
			const isConnected = (s.server_status || 'INACTIVE').toUpperCase() === 'ACTIVE';
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
			await testConnMutation.mutateAsync({
				params: {path: {id: server.id}},
				body: {host_key_fingerprint: ''},
			});
			globalServerConnStore.setStatus(server.id, 'success');
			toast.success(`SSH connection verified for "${server.name}"`);
		} catch (err: unknown) {
			globalServerConnStore.setStatus(server.id, 'failed');
			toast.error(formatApiError(err));
		}
	};

	/* ── Loading ── */
	if (isLoading) {
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div
						key={i}
						className="flex items-center gap-4 rounded-lg border border-border px-4 py-4">
						<Skeleton className="h-9 w-9 shrink-0 rounded-lg" />
						<div className="flex-1 space-y-2">
							<Skeleton className="h-3.5 w-40" />
							<Skeleton className="h-3 w-56" />
						</div>
						<Skeleton className="h-5 w-14 rounded-full" />
					</div>
				))}
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-3">
			{/* ── Search + Filter bar ── */}
			<div className="flex items-center gap-2">
				{/* Search */}
				<div className="relative flex-1">
					<Search className="pointer-events-none absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						placeholder="Search by name or IP…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-8 pl-8 text-xs"
					/>
					{search && (
						<button
							onClick={() => setSearch('')}
							className="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground">
							<X className="h-3 w-3" />
						</button>
					)}
				</div>

				{/* Status filter dropdown */}
				<Select
					value={statusFilter}
					onValueChange={v => setStatusFilter(v as StatusFilter)}>
					<SelectTrigger size="sm" className="h-8 w-40 text-xs">
						<SelectValue placeholder="Status" />
					</SelectTrigger>
					<SelectContent>
						{STATUS_FILTERS.map(f => (
							<SelectItem key={f} value={f}>
								{f === 'All' ? 'All Servers' : f}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty — no servers at all ── */}
			{servers.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-border py-20 text-center">
					<div className="flex h-12 w-12 items-center justify-center rounded-full bg-muted">
						<Server className="h-5 w-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">
							No remote servers yet
						</p>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Add a Linux node via SSH to deploy applications remotely.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-lg border border-dashed border-border py-14 text-center">
					<Search className="h-5 w-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No servers match your filter
					</p>
					<Button
						variant="ghost"
						size="sm"
						onClick={clearFilters}
						className="h-7 text-xs">
						Clear filters
					</Button>
				</div>
			) : (
				/* ── List ── */
				<div className="divide-y divide-border overflow-hidden rounded-lg border border-border">
					{filtered.map(item => {
						const connStatus = globalServerConnStore.getStatus(item.id);
						const isTesting = connStatus === 'testing';
						const serverStatus = (item.server_status || 'INACTIVE').toUpperCase();
						const isServerActive = serverStatus === 'ACTIVE';

						const dotCls = isTesting
							? 'bg-amber-400 animate-pulse'
							: isServerActive
								? 'bg-emerald-500'
								: serverStatus === 'INACTIVE'
									? 'bg-rose-500'
									: 'bg-zinc-500/60';

						const connIcon = isTesting ? (
							<RefreshCw className="h-3.5 w-3.5 animate-spin text-primary" />
						) : connStatus === 'success' ? (
							<Check className="h-3.5 w-3.5 text-emerald-500" />
						) : connStatus === 'failed' ? (
							<X className="h-3.5 w-3.5 text-rose-500" />
						) : (
							<Plug className="h-3.5 w-3.5" />
						);

						return (
							<div
								key={item.id}
								className="group flex items-center gap-4 bg-card px-4 py-3.5 transition-colors hover:bg-accent/30">
								{/* Icon + status dot */}
								<div className="relative shrink-0">
									<div className="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
										<Server className="h-4 w-4 text-foreground/70" />
									</div>
									<span
									title={`Server ${serverStatus.toLowerCase()}`}
										className={`absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-background ${dotCls}`}
									/>
								</div>

								{/* Info */}
								<div className="min-w-0 flex-1">
									<div className="mb-0.5 flex items-center gap-2">
										<span className="truncate text-sm font-medium text-foreground">
											{item.name}
										</span>
									</div>
									<p className="truncate font-mono text-[11px] text-muted-foreground">
										{item.username || 'root'}@{item.ip_address}:
										{item.port || 22}
									</p>
								</div>

								{/* Hover actions */}
								<div className="flex items-center gap-3 opacity-0 transition-opacity group-hover:opacity-100">
									<button
										onClick={() => handleTestConnection(item)}
										disabled={isTesting}
										title={isTesting ? 'Testing…' : 'Test SSH connection'}
										className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
										{connIcon}
									</button>
									{onOpenTerminal && (
										<button
											onClick={() => onOpenTerminal(item)}
											title="Open terminal"
											className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
											<Terminal className="h-4 w-4" />
										</button>
									)}
								</div>

								<Separator
									orientation="vertical"
									className="h-5 opacity-0 transition-opacity group-hover:opacity-100"
								/>

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<button className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground" />
										}>
										<MoreVertical className="h-4 w-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => onSetupServer(item)}>
											<ShieldCheck className="h-3.5 w-3.5" /> Audit Server
										</DropdownMenuItem>
										{onOpenTerminal && (
											<DropdownMenuItem
												className="cursor-pointer gap-2"
												onClick={() => onOpenTerminal(item)}>
												<Terminal className="h-3.5 w-3.5" /> Open Terminal
											</DropdownMenuItem>
										)}
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => handleTestConnection(item)}>
											<Plug className="h-3.5 w-3.5" /> Test Connection
										</DropdownMenuItem>
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => onPrivateNetwork(item)}>
											<Shield className="h-3.5 w-3.5" /> Private Network
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => onToggleStatus(item)}>
											{isServerActive ? (
												<>
													<Power className="h-3.5 w-3.5 text-amber-500" />{' '}
													Deactivate
												</>
											) : (
												<>
													<Power className="h-3.5 w-3.5 text-emerald-500" />{' '}
													Activate
												</>
											)}
										</DropdownMenuItem>
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() => onEditServer(item)}>
											<Pencil className="h-3.5 w-3.5" /> Edit Server
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="cursor-pointer gap-2 text-destructive focus:text-destructive"
											onClick={() => onDeleteServer(item)}>
											<Trash2 className="h-3.5 w-3.5" /> Delete Server
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						);
					})}
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
