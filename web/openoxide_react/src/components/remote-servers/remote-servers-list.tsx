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
} from 'lucide-react';
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from '#/components/ui/select';
import type {RemoteServerResponse, SshKeyResponse} from '#/types/api-helpers';

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
	const testConnMutation = $api.useMutation('post', '/servers/{id}/test-connection');

	useEffect(() => {
		const unsubscribe = globalServerConnStore.subscribe(() => setStoreVersion((v) => v + 1));
		return unsubscribe;
	}, []);

	const filtered = useMemo(() => {
		return servers.filter((s) => {
			const matchName =
				s.name.toLowerCase().includes(search.toLowerCase()) ||
				s.ip_address?.toLowerCase().includes(search.toLowerCase());
			const connSt = globalServerConnStore.getStatus(s.id);
			const isConnected = connSt === 'success';
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
				{[1, 2, 3].map((i) => (
					<div key={i} className="flex items-center gap-4 px-4 py-4 border border-border rounded-lg">
						<Skeleton className="w-9 h-9 rounded-lg shrink-0" />
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
					<Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
					<Input
						placeholder="Search by name or IP…"
						value={search}
						onChange={(e) => setSearch(e.target.value)}
						className="pl-8 h-8 text-xs"
					/>
					{search && (
						<button
							onClick={() => setSearch('')}
							className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
						>
							<X className="w-3 h-3" />
						</button>
					)}
				</div>

				{/* Status filter dropdown */}
				<Select value={statusFilter} onValueChange={(v) => setStatusFilter(v as StatusFilter)}>
					<SelectTrigger size="sm" className="h-8 w-40 text-xs">
						<SelectValue placeholder="Status" />
					</SelectTrigger>
					<SelectContent>
						{STATUS_FILTERS.map((f) => (
							<SelectItem key={f} value={f}>
								{f === 'All' ? 'All Servers' : f}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty — no servers at all ── */}
			{servers.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 py-20 text-center border border-dashed border-border rounded-lg">
					<div className="w-12 h-12 rounded-full bg-muted flex items-center justify-center">
						<Server className="w-5 h-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">No remote servers yet</p>
						<p className="text-xs text-muted-foreground mt-0.5">
							Add a Linux node via SSH to deploy applications remotely.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 py-14 text-center border border-dashed border-border rounded-lg">
					<Search className="w-5 h-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">No servers match your filter</p>
					<Button variant="ghost" size="sm" onClick={clearFilters} className="text-xs h-7">
						Clear filters
					</Button>
				</div>
			) : (
				/* ── List ── */
				<div className="border border-border rounded-lg overflow-hidden divide-y divide-border">
					{filtered.map((item) => {
						const connStatus = globalServerConnStore.getStatus(item.id);
						const isTesting = connStatus === 'testing';
						const isServerActive = (item.server_status || 'ACTIVE').toUpperCase() === 'ACTIVE';

						const dotCls = isTesting
							? 'bg-amber-400 animate-pulse'
							: connStatus === 'success'
								? 'bg-emerald-500'
								: connStatus === 'failed'
									? 'bg-rose-500'
									: 'bg-zinc-500/60';

						const connIcon = isTesting ? (
							<RefreshCw className="w-3.5 h-3.5 animate-spin text-primary" />
						) : connStatus === 'success' ? (
							<Check className="w-3.5 h-3.5 text-emerald-500" />
						) : connStatus === 'failed' ? (
							<X className="w-3.5 h-3.5 text-rose-500" />
						) : (
							<Plug className="w-3.5 h-3.5" />
						);

						return (
							<div
								key={item.id}
								className="group flex items-center gap-4 px-4 py-3.5 bg-card hover:bg-accent/30 transition-colors"
							>
								{/* Icon + status dot */}
								<div className="relative shrink-0">
									<div className="w-8 h-8 rounded-full bg-muted flex items-center justify-center">
										<Server className="w-4 h-4 text-foreground/70" />
									</div>
									<span
										className={`absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-background ${dotCls}`}
									/>
								</div>

								{/* Info */}
								<div className="flex-1 min-w-0">
									<div className="flex items-center gap-2 mb-0.5">
										<span className="text-sm font-medium text-foreground truncate">{item.name}</span>
									</div>
									<p className="text-[11px] font-mono text-muted-foreground truncate">
										{item.username || 'root'}@{item.ip_address}:{item.port || 22}
									</p>
								</div>

								{/* Hover actions */}
								<div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
									<button
										onClick={() => handleTestConnection(item)}
										disabled={isTesting}
										title={isTesting ? 'Testing…' : 'Test SSH connection'}
										className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
									>
										{connIcon}
									</button>
									{onOpenTerminal && (
										<button
											onClick={() => onOpenTerminal(item)}
											title="Open terminal"
											className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
										>
											<Terminal className="w-4 h-4" />
										</button>
									)}
								</div>

								<Separator
									orientation="vertical"
									className="h-5 opacity-0 group-hover:opacity-100 transition-opacity"
								/>

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<button className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer" />
										}
									>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onSetupServer(item)}>
											<ShieldCheck className="w-3.5 h-3.5" /> Audit Server
										</DropdownMenuItem>
										{onOpenTerminal && (
											<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onOpenTerminal(item)}>
												<Terminal className="w-3.5 h-3.5" /> Open Terminal
											</DropdownMenuItem>
										)}
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => handleTestConnection(item)}>
											<Plug className="w-3.5 h-3.5" /> Test Connection
										</DropdownMenuItem>
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onPrivateNetwork(item)}>
											<Shield className="w-3.5 h-3.5" /> Private Network
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onToggleStatus(item)}>
											{isServerActive ? (
												<>
													<AlertCircle className="w-3.5 h-3.5" /> Deactivate
												</>
											) : (
												<>
													<Check className="w-3.5 h-3.5" /> Activate
												</>
											)}
										</DropdownMenuItem>
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onEditServer(item)}>
											<Pencil className="w-3.5 h-3.5" /> Edit Server
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="gap-2 cursor-pointer text-destructive focus:text-destructive"
											onClick={() => onDeleteServer(item)}
										>
											<Trash2 className="w-3.5 h-3.5" /> Delete Server
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
				<p className="text-xs text-muted-foreground px-1">
					Showing {filtered.length} of {servers.length} servers
				</p>
			)}
		</div>
	);
}
