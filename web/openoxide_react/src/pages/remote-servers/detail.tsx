import {
	createFileRoute,
	Link,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import {
	ArrowLeft,
	Clock,
	Cpu,
	Globe,
	HardDrive,
	LayoutGrid,
	MemoryStick,
	Network,
	RefreshCw,
	Server,
	Shield,
	Terminal as TerminalIcon,
	User,
} from 'lucide-react';
import {useState} from 'react';
import {Button} from '#/components/ui/button';
import {Skeleton} from '#/components/ui/skeleton';
import {useRemoteServerGet} from '#/api/live';
import {useServerMetrics} from '#/api/live/monitoring';
import {isLocalServerRow, serverHostLabel} from '#/api/utils';
import {TerminalCard} from '#/components/shared/terminal-card';
import {PrivateNetworkPanel} from '#/components/remote-servers/private-network';
import {UnderlineTabs} from '#/components/shared/underline-tabs';
import type {RemoteServerResponse} from '#/types/api-helpers';

type ServerTabId = 'overview' | 'network' | 'terminal';

export const Route = createFileRoute('/_app/remote-servers/$id')({
	component: ServerDetailPage,
	validateSearch: (
		search: Record<string, unknown>,
	): {tab?: ServerTabId} => ({
		tab:
			search.tab === 'network' || search.tab === 'terminal'
				? search.tab
				: undefined,
	}),
});

/** Compact GB formatter — mirrors the panel's other byte readouts. */
function formatGb(gb: number): string {
	if (!Number.isFinite(gb) || gb <= 0) return '0 GB';
	if (gb >= 1024) return `${(gb / 1024).toFixed(1)} TB`;
	return `${gb.toFixed(1)} GB`;
}

/** Seconds since boot → "241d 21h" / "3h 12m" / "7m". */
function formatUptime(seconds: number): string {
	const s = Math.max(0, Math.floor(seconds || 0));
	const days = Math.floor(s / 86400);
	const hours = Math.floor((s % 86400) / 3600);
	const mins = Math.floor((s % 3600) / 60);
	if (days > 0) return `${days}d ${hours}h`;
	if (hours > 0) return `${hours}h ${mins}m`;
	return `${mins}m`;
}

/**
 * Usage bar. Neutral foreground tone by default — amber past 70%, red past
 * 90%. Colour tracks the data, not per-metric branding (OpenShip pattern).
 */
function UsageBar({pct}: {pct: number}) {
	const tone =
		pct >= 90
			? 'bg-rose-500'
			: pct >= 70
				? 'bg-amber-500'
				: 'bg-foreground/60';
	return (
		<div className="mt-3 h-1.5 overflow-hidden rounded-full bg-muted">
			<div
				className={`h-full rounded-full transition-all duration-700 ease-out ${tone}`}
				style={{width: `${Math.min(Math.max(pct, 0), 100)}%`}}
			/>
		</div>
	);
}

function StatCard({
	icon: Icon,
	label,
	value,
	sub,
	pct,
}: {
	icon: typeof Cpu;
	label: string;
	value: string;
	sub?: string;
	pct?: number;
}) {
	return (
		<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
			<div className="mb-3 flex items-center gap-2">
				<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				<span className="text-xs font-medium tracking-wide text-muted-foreground uppercase">
					{label}
				</span>
			</div>
			<p className="text-2xl font-semibold tracking-tight text-foreground tabular-nums">
				{value}
			</p>
			{sub && (
				<p className="mt-1 text-xs text-muted-foreground tabular-nums">
					{sub}
				</p>
			)}
			{pct != null && <UsageBar pct={pct} />}
		</div>
	);
}

function ConnectionRow({
	icon: Icon,
	label,
	value,
	mono = false,
}: {
	icon: typeof Globe;
	label: string;
	value: React.ReactNode;
	mono?: boolean;
}) {
	return (
		<div className="flex items-center justify-between gap-3 py-1.5">
			<div className="flex min-w-0 items-center gap-2">
				<div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-muted/60">
					<Icon className="size-4 text-muted-foreground" strokeWidth={2} />
				</div>
				<span className="text-sm text-muted-foreground">{label}</span>
			</div>
			<div
				className={`min-w-0 truncate text-sm font-medium text-foreground ${mono ? 'font-mono text-[13px]' : ''}`}>
				{value}
			</div>
		</div>
	);
}

function ServerDetailPage() {
	const {id} = Route.useParams();
	const {tab: rawTab} = useSearch({from: '/_app/remote-servers/$id'});
	const tab: ServerTabId = rawTab ?? 'overview';
	const navigate = useNavigate();
	const serverId = BigInt(id);
	const [refreshing, setRefreshing] = useState(false);

	const serverQuery = useRemoteServerGet(serverId);
	const metricsQuery = useServerMetrics(serverId);

	const server = serverQuery.data as
		| {
				name?: string;
				ip_address?: string;
				ip?: string;
				port?: number;
				username?: string;
				server_status?: string;
		  }
		| undefined;
	const host = serverHostLabel(server?.ip_address || server?.ip);
	const status = (server?.server_status || 'INACTIVE').toUpperCase();
	const isHealthy = status === 'ACTIVE';

	const metrics = metricsQuery.data ?? [];
	// The agent serves samples newest-first (get_last_n → rev/take).
	const latest = metrics[0] ?? null;
	const memPct = latest ? Math.round(latest.mem_used) : null;
	const diskPct = latest ? Math.round(latest.disk_used) : null;
	const diskUsedGb = latest
		? (latest.total_disk * latest.disk_used) / 100
		: 0;

	const handleRefresh = async () => {
		setRefreshing(true);
		try {
			await Promise.all([serverQuery.refetch(), metricsQuery.refetch()]);
		} finally {
			setRefreshing(false);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* ── Header ─────────────────────────────────────────────── */}
			<div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
				<div className="flex min-w-0 items-center gap-3">
					<Link
						to="/remote-servers"
						title="Back to servers"
						className="flex size-8 shrink-0 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-foreground/[0.06] hover:text-foreground">
						<ArrowLeft className="size-4" />
					</Link>
					<div className="flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted">
						<Server className="size-4 text-foreground/70" />
					</div>
					<div className="min-w-0">
						<div className="flex flex-wrap items-center gap-2">
							<h1 className="truncate text-2xl font-medium tracking-tight text-foreground/80">
								{server?.name || 'Loading...'}
							</h1>
							<span
								className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
									isHealthy
										? 'bg-emerald-500/10 text-emerald-500'
										: 'bg-rose-500/10 text-rose-500'
								}`}>
								{isHealthy ? 'Healthy' : status.toLowerCase()}
							</span>
						</div>
						<p className="truncate font-mono text-xs text-muted-foreground/70">
							{server?.username || 'root'}@{host}
						</p>
					</div>
				</div>
				<Button
					variant="outline"
					size="sm"
					onClick={handleRefresh}
					disabled={refreshing}
					className="h-8 shrink-0 cursor-pointer gap-1.5 text-xs">
					<RefreshCw
						className={`size-3.5 ${refreshing ? 'animate-spin' : ''}`}
					/>
					Refresh
				</Button>
			</div>

			{/* ── Tabs (OpenShip server detail pattern) ──────────────── */}
			<UnderlineTabs
				tabs={[
					{key: 'overview', label: 'Overview', icon: LayoutGrid},
					{key: 'network', label: 'Private Network', icon: Network},
					{key: 'terminal', label: 'Terminal', icon: TerminalIcon},
				]}
				value={tab}
				onChange={key =>
					void navigate({
						to: '/remote-servers/$id',
						params: {id},
						search: {tab: key === 'overview' ? undefined : key},
						replace: true,
					})
				}
			/>

			{/* ── Content grid: tab body + persistent Connection sidebar ── */}
			<div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_340px]">
				<div className="flex min-w-0 flex-col gap-4">
					{/* ── Overview: stats + system ─────────────────────── */}
					{tab === 'overview' && (
						<div className="flex flex-col gap-4">
							{/* Stat cards — CPU / Memory / Disk / Uptime */}
							<div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
								{metricsQuery.isLoading ? (
									Array.from({length: 4}).map((_, i) => (
										<Skeleton key={i} className="h-[132px] rounded-xl" />
									))
								) : latest ? (
									<>
										<StatCard
											icon={Cpu}
											label="CPU"
											value={`${Math.round(latest.cpu)}%`}
											sub={
												latest.cpu_cores
													? `${latest.cpu_cores} cores · ${latest.cpu_model.slice(0, 24)}`
													: undefined
											}
											pct={Math.round(latest.cpu)}
										/>
										<StatCard
											icon={MemoryStick}
											label="Memory"
											value={`${memPct ?? 0}%`}
											sub={`${formatGb(latest.mem_used_gb)} of ${formatGb(latest.mem_total)}`}
											pct={memPct ?? undefined}
										/>
										<StatCard
											icon={HardDrive}
											label="Disk"
											value={`${diskPct ?? 0}%`}
											sub={`${formatGb(diskUsedGb)} of ${formatGb(latest.total_disk)}`}
											pct={diskPct ?? undefined}
										/>
										<StatCard
											icon={Clock}
											label="Uptime"
											value={formatUptime(latest.uptime)}
											sub="since last boot"
										/>
									</>
								) : (
									<div className="col-span-2 rounded-xl bg-card p-5 ring-1 ring-border/50 lg:col-span-4">
										<p className="text-sm font-medium text-foreground">
											No metrics yet
										</p>
										<p className="mt-1 text-xs text-muted-foreground">
											{metricsQuery.isError
												? 'The monitoring agent on this server is unreachable. Check that the agent is installed and running (Remote Servers → Setup).'
												: 'Waiting for the first sample from the agent on this server.'}
										</p>
									</div>
								)}
							</div>

							{/* System facts card (only when we have a sample) */}
							{latest && (
								<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
									<div className="mb-2 flex items-center gap-2">
										<Globe
											className="size-4 text-muted-foreground"
											strokeWidth={2}
										/>
										<h2 className="text-sm font-semibold text-foreground">
											System
										</h2>
									</div>
									<ConnectionRow
										icon={Cpu}
										label="Processor"
										value={`${latest.cpu_model || '-'} · ${latest.cpu_cores} vCPU`}
									/>
									<ConnectionRow
										icon={Server}
										label="OS"
										value={latest.distro || latest.os || '-'}
									/>
									<ConnectionRow
										icon={Shield}
										label="Kernel"
										value={latest.kernel || '-'}
										mono
									/>
								</div>
							)}
						</div>
					)}

					{/* ── Private Network ──────────────────────────────── */}
					{tab === 'network' && server && (
						<PrivateNetworkPanel
							server={server as unknown as RemoteServerResponse}
						/>
					)}

					{/* ── Terminal ─────────────────────────────────────── */}
					{tab === 'terminal' && server && (
						<TerminalCard
							app={{
								app_name: host,
								name: server.name || 'Server',
								server_id: Number(id),
								isRemoteServer: !isLocalServerRow(
									server.ip_address || server.ip,
								),
							}}
							enabled={true}
							hostModeDefault={isLocalServerRow(
								server.ip_address || server.ip,
							)}
						/>
					)}
				</div>

				{/* ── Right sidebar: connection summary on every tab ── */}
				<div className="lg:sticky lg:top-6 lg:self-start">
					<div className="rounded-xl bg-card p-5 ring-1 ring-border/50">
						<div className="mb-4 flex items-center gap-2">
							<Server
								className="size-4 text-muted-foreground"
								strokeWidth={2}
							/>
							<h3 className="text-sm font-semibold text-foreground">
								Connection
							</h3>
						</div>
						<ConnectionRow
							icon={Globe}
							label="Host"
							value={
								<span className="flex items-center gap-2">
									<span
										className={`size-2 shrink-0 rounded-full ${isHealthy ? 'bg-emerald-500' : 'bg-rose-500'}`}
									/>
									<span className="font-mono text-[13px]">{host}</span>
								</span>
							}
						/>
						<ConnectionRow
							icon={Server}
							label="Port"
							value={server?.port ?? '-'}
							mono
						/>
						<ConnectionRow
							icon={User}
							label="User"
							value={server?.username || 'root'}
							mono
						/>
					</div>
				</div>
			</div>

		</div>
	);
}
