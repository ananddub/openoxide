import {Link} from '@tanstack/react-router';
import {Check, Plug, RefreshCw, Server, Terminal, X} from 'lucide-react';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {ServerRowMenu} from './server-row-menu';
import {cn, serverHostLabel} from '#/api/utils';
import {RowTile} from '#/components/settings/list-card';
import {IconTooltip} from '#/components/shared/icon-tooltip';

/** Compact ring-dot + label for the reachability state. */
const STATE_STYLE = {
	online: {
		dot: 'bg-emerald-500',
		text: 'text-emerald-500',
		label: 'Online',
	},
	offline: {dot: 'bg-rose-500', text: 'text-rose-500', label: 'Offline'},
	testing: {
		dot: 'bg-amber-400',
		text: 'text-amber-500',
		label: 'Checking',
	},
} as const;

export interface ServerRowActions {
	onTest: (s: RemoteServerResponse) => void;
	onSetup: (s: RemoteServerResponse) => void;
	onToggleStatus: (s: RemoteServerResponse) => void;
	onEdit: (s: RemoteServerResponse) => void;
	onDelete: (s: RemoteServerResponse) => void;
	onOpenTerminal?: (s: RemoteServerResponse) => void;
	onMigrate?: (s: RemoteServerResponse) => void;
	onNetwork: (s: RemoteServerResponse) => void;
}

/** Resolve the reachability state + style from live conn + server status. */
function useServerState(
	item: RemoteServerResponse,
	connStatus: string,
): {
	isTesting: boolean;
	serverStatus: string;
	isServerActive: boolean;
	sm: (typeof STATE_STYLE)[keyof typeof STATE_STYLE];
} {
	const isTesting = connStatus === 'testing';
	const serverStatus = (item.server_status || 'INACTIVE').toUpperCase();
	const isServerActive = serverStatus === 'ACTIVE';
	const state = isTesting
		? 'testing'
		: connStatus === 'failed'
			? 'offline'
			: isServerActive
				? 'online'
				: 'offline';
	return {isTesting, serverStatus, isServerActive, sm: STATE_STYLE[state]};
}

/** Grid card: header (icon · name/ip · status) + footer (actions). */
export function ServerCard({
	item,
	connStatus,
	actions,
}: {
	item: RemoteServerResponse;
	connStatus: string;
	actions: ServerRowActions;
}) {
	const {isTesting, serverStatus, isServerActive, sm} = useServerState(
		item,
		connStatus,
	);

	return (
		<div className="group flex flex-col gap-4 rounded-xl border bg-card p-4 transition-colors hover:bg-accent/30">
			<Link
				to="/remote-servers/$id"
				params={{id: String(item.id)}}
				className="flex min-w-0 items-start gap-3 text-start">
				<RowTile>
					<Server className="size-4" />
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
					title={`${sm.label} · server ${serverStatus.toLowerCase()}`}
					className={cn(
						'inline-flex shrink-0 items-center gap-1.5 text-xs font-medium',
						sm.text,
					)}>
					<span
						className={cn(
							'size-2 rounded-full',
							sm.dot,
							isTesting && 'animate-pulse',
						)}
					/>
					{sm.label}
				</span>
			</Link>

			{/* Footer: test + terminal + overflow menu */}
			<div className="flex items-center justify-between border-t border-border pt-3">
				<button
					onClick={() => actions.onTest(item)}
					disabled={isTesting}
					title={isTesting ? 'Testing…' : 'Test SSH connection'}
					className="flex cursor-pointer items-center gap-1.5 text-xs font-medium text-muted-foreground transition-colors hover:text-foreground disabled:cursor-not-allowed disabled:opacity-50">
					{isTesting ? (
						<RefreshCw className="size-4 animate-spin text-primary" />
					) : connStatus === 'success' ? (
						<Check className="size-4 text-emerald-500" />
					) : connStatus === 'failed' ? (
						<X className="size-4 text-rose-500" />
					) : (
						<Plug className="size-4" />
					)}
					Test
				</button>

				<div className="flex items-center gap-1">
					{actions.onOpenTerminal && (
						<IconTooltip label="Open terminal">
							<button
								onClick={() => actions.onOpenTerminal?.(item)}
								aria-label="Open terminal"
								className="cursor-pointer rounded-md p-1.5 text-muted-foreground transition-colors hover:text-foreground">
								<Terminal className="size-4" />
							</button>
						</IconTooltip>
					)}
					<ServerRowMenu
						item={item}
						isServerActive={isServerActive}
						actions={actions}
					/>
				</div>
			</div>
		</div>
	);
}

/** One server row: icon · name/ip · status · menu. */
export function ServerRow({
	item,
	connStatus,
	actions,
}: {
	item: RemoteServerResponse;
	connStatus: string;
	actions: ServerRowActions;
}) {
	const isTesting = connStatus === 'testing';
	const serverStatus = (item.server_status || 'INACTIVE').toUpperCase();
	const isServerActive = serverStatus === 'ACTIVE';

	const state = isTesting
		? 'testing'
		: connStatus === 'failed'
			? 'offline'
			: isServerActive
				? 'online'
				: 'offline';
	const sm = STATE_STYLE[state];

	return (
		<div className="group flex items-center gap-3 px-4 py-2.5 transition-colors hover:bg-muted/40">
			<Link
				to="/remote-servers/$id"
				params={{id: String(item.id)}}
				className="flex min-w-0 flex-1 items-center gap-3 text-start">
				<RowTile>
					<Server className="size-4" />
				</RowTile>
				<div className="min-w-0">
					<p className="truncate text-sm font-medium text-foreground">
						{item.name}
					</p>
					<p className="truncate font-mono text-xs text-muted-foreground">
						{serverHostLabel(item.ip_address)}
					</p>
				</div>
			</Link>

			{/* Right: status pill + actions */}
			<div className="flex shrink-0 items-center gap-3">
				{/* Status pill */}
				<span
					title={`${sm.label} · server ${serverStatus.toLowerCase()}`}
					className={`hidden items-center gap-1.5 text-xs font-medium sm:inline-flex ${sm.text}`}>
					<span
						className={`size-2 rounded-full ${sm.dot} ${isTesting ? 'animate-pulse' : ''}`}
					/>
					{sm.label}
				</span>

				{/* Test connection quick button */}
				<IconTooltip
					label={isTesting ? 'Testing…' : 'Test SSH connection'}>
					<button
						onClick={() => actions.onTest(item)}
						disabled={isTesting}
						aria-label="Test SSH connection"
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
				</IconTooltip>

				{actions.onOpenTerminal && (
					<IconTooltip label="Open terminal">
						<button
							onClick={() => actions.onOpenTerminal?.(item)}
							aria-label="Open terminal"
							className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
							<Terminal className="size-4" />
						</button>
					</IconTooltip>
				)}

				<ServerRowMenu
					item={item}
					isServerActive={isServerActive}
					actions={actions}
				/>
			</div>
		</div>
	);
}
