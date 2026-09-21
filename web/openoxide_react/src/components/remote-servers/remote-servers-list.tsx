import {useState, useEffect, useMemo} from 'react';
import {Link, useNavigate} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {toast} from 'sonner';
import {formatApiError, serverHostLabel} from '#/api/utils';
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
	Search,
	Power,
	KeyRound,
	Lock,
	ArrowRight,
	ArrowLeftRight,
	Plus,
} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {useRemoteServerTestConnection} from '#/api/live';
import type {
	RemoteServerResponse,
	SshKeyResponse,
} from '#/types/api-helpers';

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

const STATUS_FILTERS = ['All', 'Connected', 'Not Connected'] as const;
type StatusFilter = (typeof STATUS_FILTERS)[number];

/** Reachability word + ring-dot colors (openship list pattern). */
const STATE_STYLE = {
	online: {dot: 'border-emerald-500', text: 'text-emerald-500', label: 'Online'},
	offline: {dot: 'border-rose-500', text: 'text-rose-500', label: 'Offline'},
	testing: {dot: 'border-amber-400', text: 'text-amber-500', label: 'Checking'},
} as const;

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
	const testConnMutation = useRemoteServerTestConnection();

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

	/* ── Loading ─ */
	if (isLoading) {
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div
						key={i}
						className="flex items-center gap-4 rounded-lg px-4 py-4">
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

	/* ── Empty — no servers at all ── */
	if (servers.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center gap-4 rounded-2xl bg-muted/10 px-6 py-20 text-center">
				<div className="flex size-12 items-center justify-center rounded-full bg-muted">
					<Server className="size-5 text-muted-foreground" />
				</div>
				<div>
					<p className="text-sm font-medium text-foreground">
						No remote servers yet
					</p>
					<p className="mx-auto mt-1 max-w-sm text-xs text-muted-foreground">
						Connect a Linux node over SSH to deploy applications,
						containers and databases onto it from this panel.
					</p>
				</div>
				<Button
					size="sm"
					onClick={onAddServer}
					className="h-9 cursor-pointer gap-1.5 px-4 text-xs font-semibold">
					<Plus className="size-3.5" />
					Add Server
				</Button>
			</div>
		);
	}

	return (
		<div className="flex min-w-0 flex-col gap-3">
			{/* ── Search + Filter bar ── */}
			<div className="flex items-center gap-2">
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
							className="absolute top-1/2 right-2 -translate-y-1/2 cursor-pointer text-muted-foreground hover:text-foreground">
							<X className="h-3 w-3" />
						</button>
					)}
				</div>

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

			{filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-2xl bg-card py-14 text-center ring-1 ring-border/50">
					<Search className="size-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No servers match your filter
					</p>
					<Button
						variant="ghost"
						size="sm"
						onClick={clearFilters}
						className="h-7 cursor-pointer text-xs">
						Clear filters
					</Button>
				</div>
			) : (
				/* ── Rows — one bordered card, divided rows (openship) ── */
				<div className="divide-y divide-border/50 overflow-hidden rounded-2xl bg-card ring-1 ring-border/50">
					{filtered.map(item => {
						const connStatus = globalServerConnStore.getStatus(item.id);
						const isTesting = connStatus === 'testing';
						const serverStatus = (
							item.server_status || 'INACTIVE'
						).toUpperCase();
						const isServerActive = serverStatus === 'ACTIVE';

						const state = isTesting
							? 'testing'
							: connStatus === 'failed'
								? 'offline'
								: isServerActive
									? 'online'
									: 'offline';
						const sm = STATE_STYLE[state];

						const hasKey = Boolean(item.ssh_key_id);
						const AuthIcon = hasKey ? KeyRound : Lock;

						return (
							<div
								key={item.id}
								className="group flex items-center gap-3.5 px-5 py-3 transition-colors hover:bg-muted/40">
								{/* Row body links to the server overview */}
								<Link
									to="/remote-servers/$id"
									params={{id: String(item.id)}}
									className="flex min-w-0 flex-1 items-center gap-3.5 text-start">
									{/* Avatar */}
									<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted/60 transition-colors group-hover:bg-muted">
										<Server className="size-[18px] text-foreground/70" />
									</div>

									{/* Name + host (fixed column keeps chips aligned) */}
									<div className="w-44 min-w-0 shrink-0 text-start lg:w-56">
										<p className="truncate text-sm font-medium text-foreground">
											{item.name}
										</p>
										<p className="mt-0.5 truncate font-mono text-xs text-muted-foreground">
											{serverHostLabel(item.ip_address)}
										</p>
									</div>

									{/* Meta chips */}
									<div className="flex min-w-0 flex-1 items-center gap-2 overflow-hidden">
										<span className="inline-flex shrink-0 items-center gap-1.5 rounded-md bg-muted/60 px-2 py-0.5 text-xs text-muted-foreground">
											<AuthIcon className="size-3.5" />
											{hasKey ? 'SSH Key' : 'Password'}
										</span>
										<span className="hidden shrink-0 items-center rounded-md bg-muted/60 px-2 py-0.5 text-xs text-foreground/80 md:inline-flex">
											{item.username || 'root'} · port{' '}
											{item.port || 22}
										</span>
									</div>
								</Link>

								{/* Status word + ring dot */}
								<span
									title={
										isTesting
											? 'Testing SSH connection…'
											: `${sm.label} · server ${serverStatus.toLowerCase()}`
									}
									className={`inline-flex shrink-0 items-center gap-1.5 text-xs font-medium ${sm.text}`}>
									<span
										className={`size-2.5 rounded-full border-2 ${sm.dot} ${isTesting ? 'animate-pulse' : ''}`}
									/>
									<span className="hidden sm:inline">{sm.label}</span>
								</span>

								{/* Hover: quick actions */}
								<div className="flex shrink-0 items-center gap-2.5 opacity-0 transition-opacity group-hover:opacity-100">
									<button
										onClick={() => handleTestConnection(item)}
										disabled={isTesting}
										title={isTesting ? 'Testing…' : 'Test SSH connection'}
										className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
										{isTesting ? (
											<RefreshCw className="size-4 animate-spin text-primary" />
										) : connStatus === 'success' ? (
											<Check className="size-4 text-emerald-500" />
										) : connStatus === 'failed' ? (
											<X className="size-4 text-rose-500" />
										) : (
											<Plug className="size-4" />
										)}
									</button>
									{onOpenTerminal && (
										<button
											onClick={() => onOpenTerminal(item)}
											title="Open terminal"
											className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
											<Terminal className="size-4" />
										</button>
									)}
								</div>

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger
										render={
											<button className="flex size-6 shrink-0 cursor-pointer items-center justify-center text-muted-foreground transition-colors hover:text-foreground" />
										}>
										<MoreVertical className="size-4" />
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
										{onMigrateServer && (
											<DropdownMenuItem
												className="cursor-pointer gap-2"
												onClick={() => onMigrateServer(item)}>
												<ArrowLeftRight className="h-3.5 w-3.5" /> Migrate Services
											</DropdownMenuItem>
										)}
										<DropdownMenuItem
											className="cursor-pointer gap-2"
											onClick={() =>
												void navigate({
													to: '/remote-servers/$id',
													params: {id: String(item.id)},
													search: {tab: 'network'},
												})
											}>
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

								{/* Arrow */}
								<ArrowRight className="size-4 shrink-0 text-muted-foreground/40 transition-colors group-hover:text-muted-foreground" />
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
