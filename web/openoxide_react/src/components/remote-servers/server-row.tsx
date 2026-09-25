import {Server} from 'lucide-react';
import type {RemoteServerResponse} from '#/types/api-helpers';
import {cn, serverHostLabel} from '#/api/utils';
import {RowTile} from '#/components/settings/list-card';

/** Resolve the reachability dot from live conn + server status. */
function useServerState(item: RemoteServerResponse, connStatus: string) {
	const isTesting = connStatus === 'testing';
	const serverStatus = (item.server_status || 'INACTIVE').toUpperCase();
	const isServerActive = serverStatus === 'ACTIVE';
	const online = isTesting ? null : connStatus === 'failed' ? false : isServerActive;
	return {
		isTesting,
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

/** Grid card — the whole card opens the server settings panel. */
export function ServerCard({
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

	return (
		<button
			type="button"
			onClick={() => onOpen(item)}
			className="group flex cursor-pointer flex-col gap-3 rounded-xl border bg-card p-4 text-start transition-colors hover:bg-accent/30">
			<div className="flex min-w-0 items-start gap-3">
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
					title={`${label} · server ${serverStatus.toLowerCase()}`}
					className={cn(
						'inline-flex shrink-0 items-center gap-1.5 text-xs font-medium',
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
			</div>
			{item.description && (
				<p className="line-clamp-2 text-xs text-muted-foreground">
					{item.description}
				</p>
			)}
		</button>
	);
}

/** List row — the whole row opens the server settings panel. */
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

	return (
		<button
			type="button"
			onClick={() => onOpen(item)}
			className="group flex w-full cursor-pointer items-center gap-3 px-4 py-2.5 text-start transition-colors hover:bg-muted/40">
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
	);
}
