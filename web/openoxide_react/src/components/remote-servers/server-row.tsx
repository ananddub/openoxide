import {Link} from '@tanstack/react-router';
import {
	ArrowRight,
	Check,
	KeyRound,
	Lock,
	Plug,
	RefreshCw,
	Server,
	Terminal,
	X,
} from 'lucide-react';
import {serverHostLabel} from '#/api/utils';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {ServerRowMenu} from './server-row-menu';

/** Reachability word + ring-dot colors (openship list pattern). */
const STATE_STYLE = {
	online: {
		dot: 'border-emerald-500',
		text: 'text-emerald-500',
		label: 'Online',
	},
	offline: {
		dot: 'border-rose-500',
		text: 'text-rose-500',
		label: 'Offline',
	},
	testing: {
		dot: 'border-amber-400',
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

/** One server row: avatar, chips, live status, quick actions, 3-dot menu. */
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

	const hasKey = Boolean(item.ssh_key_id);
	const AuthIcon = hasKey ? KeyRound : Lock;
	const {onOpenTerminal} = actions;

	return (
		<div className="group flex items-center gap-3.5 px-5 py-3 transition-colors hover:bg-muted/40">
			{/* Row body links to the server overview */}
			<Link
				to="/remote-servers/$id"
				params={{id: String(item.id)}}
				className="flex min-w-0 flex-1 items-center gap-3.5 text-start">
				<div className="flex size-9 shrink-0 items-center justify-center rounded-xl bg-muted/60 transition-colors group-hover:bg-muted">
					<Server className="size-[18px] text-foreground/70" />
				</div>
				<div className="w-44 min-w-0 shrink-0 text-start lg:w-56">
					<p className="truncate text-sm font-medium text-foreground">
						{item.name}
					</p>
					<p className="mt-0.5 truncate font-mono text-xs text-muted-foreground">
						{serverHostLabel(item.ip_address)}
					</p>
				</div>
				<div className="flex min-w-0 flex-1 items-center gap-2 overflow-hidden">
					<span className="inline-flex shrink-0 items-center gap-1.5 rounded-md bg-muted/60 px-2 py-0.5 text-xs text-muted-foreground">
						<AuthIcon className="size-3.5" />
						{hasKey ? 'SSH Key' : 'Password'}
					</span>
					<span className="hidden shrink-0 items-center rounded-md bg-muted/60 px-2 py-0.5 text-xs text-foreground/80 md:inline-flex">
						{item.username || 'root'} · port {item.port || 22}
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
					onClick={() => actions.onTest(item)}
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
			<ServerRowMenu
				item={item}
				isServerActive={isServerActive}
				actions={actions}
			/>

			<ArrowRight className="size-4 shrink-0 text-muted-foreground/40 transition-colors group-hover:text-muted-foreground" />
		</div>
	);
}
