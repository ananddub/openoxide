import {useState} from 'react';
import {Server, TerminalSquare} from 'lucide-react';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {cn, isLocalServerRow, serverHostLabel} from '#/api/utils';
import {TerminalCard} from '#/components/shared/terminal-card';
import {
	useServerMetrics,
	type ServerMetricPayload,
} from '#/api/live/monitoring';
import {UsageRing} from '#/components/shared/usage-ring';
import {RowTile} from '#/components/settings/list-card';
import {
	ContextMenu,
	ContextMenuContent,
	ContextMenuItem,
	ContextMenuTrigger,
} from '#/components/ui/context-menu';
import {OsGlyph} from './os-icon';
import {formatAdded} from './server-detail-parts';

/** Resolve the reachability dot from live conn + server status. */
function useServerState(item: RemoteServerResponse, connStatus: string) {
	const isTesting = connStatus === 'testing';
	const serverStatus = (item.server_status || 'INACTIVE').toUpperCase();
	const isServerActive = serverStatus === 'ACTIVE';
	const online = isTesting
		? null
		: connStatus === 'failed'
			? false
			: isServerActive;
	return {
		isTesting,
		online: online === true,
		serverStatus,
		label: isTesting ? 'Checking' : online ? 'Online' : 'Offline',
		text: isTesting
			? 'text-amber-500'
			: online
				? 'text-emerald-500'
				: 'text-rose-500',
		dot: isTesting
			? 'bg-amber-400'
			: online
				? 'bg-emerald-500'
				: 'bg-rose-500',
	};
}

/** Live CPU and memory rings for one server. Mounted only for online servers — an offline one renders nothing rather than a pair of dead rings. */
function ServerUsageRings({m}: {m?: ServerMetricPayload}) {
	return (
		<div className="flex shrink-0 items-start gap-2">
			<span className="flex flex-col items-center gap-0.5">
				<UsageRing
					percent={m?.cpu ?? 0}
					ready={!!m}
					title={m ? `Live CPU ${m.cpu.toFixed(1)}%` : 'CPU — no stats'}
				/>
				<span className="font-mono text-[8px] tracking-wider text-muted-foreground uppercase">
					CPU
				</span>
			</span>
			<span className="flex flex-col items-center gap-0.5">
				<UsageRing
					percent={m?.mem_used ?? 0}
					ready={!!m}
					title={
						m
							? `Memory ${m.mem_used_gb} of ${m.mem_total} GB`
							: 'Memory — no stats'
					}
				/>
				<span className="font-mono text-[8px] tracking-wider text-muted-foreground uppercase">
					MEM
				</span>
			</span>
		</div>
	);
}

/** Right-click menu for a server card/row — opens the remote terminal as an
 *  in-place overlay modal (no page jump). */
function ServerContextMenu({
	item,
	children,
}: {
	item: RemoteServerResponse;
	/** The card/row element — rendered as the menu trigger. */
	children: React.ReactElement;
}) {
	const [terminalOpen, setTerminalOpen] = useState(false);
	return (
		<>
			<ContextMenu>
				<ContextMenuTrigger render={children} />
				<ContextMenuContent>
					<ContextMenuItem
						onClick={() => setTerminalOpen(true)}
						className="cursor-pointer gap-2 text-xs">
						<TerminalSquare className="size-3.5" />
						Open terminal
					</ContextMenuItem>
				</ContextMenuContent>
			</ContextMenu>

			{terminalOpen && (
				<TerminalCard
					app={{
						app_name: serverHostLabel(item.ip_address),
						name: item.name || 'Server',
						server_id: Number(item.id),
						isRemoteServer: !isLocalServerRow(item.ip_address),
					}}
					enabled={terminalOpen}
					onClose={() => setTerminalOpen(false)}
				/>
			)}
		</>
	);
}

/** Grid card — the whole card opens the server settings panel; right-click opens the terminal jump. */
export function ServerCard({
	item,
	connStatus,
	onOpen,
}: {
	item: RemoteServerResponse;
	connStatus: string;
	onOpen: (s: RemoteServerResponse) => void;
}) {
	const {isTesting, online, serverStatus, label, dot} = useServerState(
		item,
		connStatus,
	);
	const {data} = useServerMetrics(BigInt(item.id), 1);
	const metric = data?.[0];
	const createdDate = formatAdded(item.created_at);

	return (
		<ServerContextMenu item={item}>
			<button
				type="button"
				onClick={() => onOpen(item)}
				title={
					item.description
						? `${label} · ${item.description}`
						: `${label} · server ${serverStatus.toLowerCase()}`
				}
				className="group relative flex w-full cursor-pointer flex-col justify-between gap-2.5 overflow-hidden rounded-xl border bg-card p-4 text-left transition-colors duration-200 hover:border-primary/30">
				<div className="flex min-w-0 items-center justify-between gap-2">
					<div className="flex min-w-0 flex-1 items-center gap-3">
						<div className="relative flex size-9 shrink-0 items-center justify-center rounded-lg bg-muted/40 text-foreground">
							{metric ? (
								<OsGlyph
									distro={metric.distro}
									os={metric.os}
									kernel={metric.kernel}
									className="size-5"
								/>
							) : (
								<Server className="size-4" />
							)}
							<span
								className={cn(
									'absolute -top-0.5 -right-0.5 size-2 rounded-full border-2 border-card',
									dot,
									isTesting && 'animate-pulse',
								)}
							/>
						</div>
						<div className="min-w-0 flex-1">
							<p
								title={item.name}
								className="truncate text-xs leading-snug font-bold text-foreground transition-colors group-hover:text-primary">
								{item.name}
							</p>
							<p className="truncate font-mono text-[10px] tracking-wider text-muted-foreground uppercase">
								{serverHostLabel(item.ip_address)}
							</p>
						</div>
					</div>
					{online && <ServerUsageRings m={metric} />}
				</div>
				<div className="flex items-center justify-between border-t border-border pt-2">
					<span className="flex items-center gap-1.5 text-[10px] font-bold tracking-wider text-muted-foreground uppercase">
						<span className={cn('size-1.5 rounded-full', dot)} />
						{label}
					</span>
					{createdDate && (
						<span className="font-mono text-[10px] text-muted-foreground/70">
							{createdDate}
						</span>
					)}
				</div>
			</button>
		</ServerContextMenu>
	);
}

/** List row — the whole row opens the server settings panel; right-click opens the terminal jump. */
export function ServerRow({
	item,
	connStatus,
	onOpen,
}: {
	item: RemoteServerResponse;
	connStatus: string;
	onOpen: (s: RemoteServerResponse) => void;
}) {
	const {isTesting, serverStatus, label, text, dot} = useServerState(
		item,
		connStatus,
	);
	const {data} = useServerMetrics(BigInt(item.id), 1);
	const metric = data?.[0];

	return (
		<ServerContextMenu item={item}>
			<button
				type="button"
				onClick={() => onOpen(item)}
				className="group flex w-full cursor-pointer items-center gap-3 px-4 py-2.5 text-start transition-colors hover:bg-muted/40">
				<RowTile>
					{metric ? (
						<OsGlyph
							distro={metric.distro}
							os={metric.os}
							kernel={metric.kernel}
							className="size-4"
						/>
					) : (
						<Server className="size-4" />
					)}
				</RowTile>
				<div className="min-w-0 flex-1">
					<p className="truncate text-sm font-medium text-foreground">
						{item.name}
					</p>
					<p className="truncate font-mono text-xs text-muted-foreground">
						{serverHostLabel(item.ip_address)}
					</p>
				</div>
				<span
					title={`${label} · server ${serverStatus.toLowerCase()}`}
					className={cn(
						'hidden shrink-0 items-center gap-1.5 text-xs font-medium sm:inline-flex',
						text,
					)}>
					<span
						className={cn(
							'size-2 rounded-full',
							dot,
							isTesting && 'animate-pulse',
						)}
					/>
					{label}
				</span>
			</button>
		</ServerContextMenu>
	);
}
