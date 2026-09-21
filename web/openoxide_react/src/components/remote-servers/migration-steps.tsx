import {
	ArrowLeftRight,
	ArrowRight,
	ChevronLeft,
	RefreshCw,
	RotateCcw,
	ShieldCheck,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {serverHostLabel} from '#/api/utils';
import type {RemoteServerResponse} from '#/types/api-helpers';
import type {ServerMigrationPayload} from '#/api/live';
import {TERMINAL, ProgressGrid, statusMeta} from './migration-progress';
import {MigrationSelect} from './migration-select';

/** Step 1: pick a destination node and start the metadata migration. */
export function MigrationStartStep({
	server,
	candidates,
	targetId,
	onTarget,
	starting,
	onStart,
	onCancel,
}: {
	server: RemoteServerResponse;
	candidates: RemoteServerResponse[];
	targetId: string;
	onTarget: (id: string) => void;
	starting: boolean;
	onStart: () => void;
	onCancel: () => void;
}) {
	const target = candidates.find(s => String(s.id) === targetId);
	return (
		<div className="flex flex-col gap-4 py-3">
			<MigrationSelect
				label="Target server"
				value={targetId}
				placeholder="Select a destination node…"
				options={candidates.map(s => ({
					value: String(s.id),
					label: `${s.name} · ${serverHostLabel(s.ip_address)}`,
				}))}
				emptyHint="No other servers available — add a destination node first."
				onChange={onTarget}
			/>

			<div className="flex items-center justify-center gap-3 rounded-xl bg-primary/10 p-3.5 text-xs">
				<span className="font-mono text-foreground">
					{serverHostLabel(server.ip_address)}
				</span>
				<ArrowRight className="size-4 text-primary" />
				<span className="font-mono text-foreground">
					{target ? serverHostLabel(target.ip_address) : '—'}
				</span>
			</div>

			<div className="flex items-center gap-2.5 rounded-xl bg-muted/20 p-3.5">
				<ShieldCheck className="size-5 shrink-0 text-primary" />
				<p className="text-[11px] text-muted-foreground">
					Applications, compose projects, databases, certificates and
					schedules owned by this node are reassigned, then redeployed on
					the target.
				</p>
			</div>

			<div className="flex justify-end gap-2">
				<Button
					variant="ghost"
					size="sm"
					onClick={onCancel}
					className="h-8 text-xs">
					Cancel
				</Button>
				<Button
					size="sm"
					onClick={onStart}
					disabled={!targetId || starting}
					className="h-8 gap-1.5 text-xs font-semibold">
					{starting ? (
						<RefreshCw className="h-3.5 w-3.5 animate-spin" />
					) : (
						<ArrowLeftRight className="h-3.5 w-3.5" />
					)}
					{starting ? 'Starting…' : 'Start Migration'}
				</Button>
			</div>
		</div>
	);
}

/** Step 2: live progress, error, rollback and done affordances. */
export function MigrationProgressStep({
	migration,
	isLoading,
	onNew,
	rollbackPending,
	onRollback,
	onClose,
}: {
	migration: ServerMigrationPayload | undefined;
	isLoading: boolean;
	onNew: () => void;
	rollbackPending: boolean;
	onRollback: () => void;
	onClose: () => void;
}) {
	const meta = migration ? statusMeta(migration.status) : null;
	const isRunning = !!migration && !TERMINAL.has(migration.status);
	return (
		<div className="flex flex-col gap-4 py-3">
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-2">
					<button
						onClick={onNew}
						className="flex cursor-pointer items-center gap-1 text-xs text-muted-foreground hover:text-foreground">
						<ChevronLeft className="size-3.5" /> New migration
					</button>
				</div>
				{meta && (
					<span
						className={`flex items-center gap-1.5 text-xs font-semibold ${meta.cls}`}>
						{meta.icon} {migration?.status}
					</span>
				)}
			</div>

			{isLoading && (
				<p className="text-xs text-muted-foreground">
					Loading migration status…
				</p>
			)}

			{migration && <ProgressGrid migration={migration} />}

			{migration?.error && (
				<div className="rounded-xl bg-rose-500/10 p-3.5 text-[11px] text-rose-500">
					{migration.error}
				</div>
			)}

			{isRunning && (
				<p className="flex items-center gap-1.5 text-[11px] text-muted-foreground">
					<RefreshCw className="size-3 animate-spin" /> Running — you can
					close this dialog; it keeps going in the background.
				</p>
			)}

			<div className="flex justify-end gap-2">
				{migration &&
					TERMINAL.has(migration.status) &&
					migration.status !== 'ROLLED_BACK' && (
						<Button
							variant="outline"
							size="sm"
							onClick={onRollback}
							disabled={rollbackPending}
							className="h-8 gap-1.5 text-xs font-semibold">
							{rollbackPending ? (
								<RefreshCw className="h-3.5 w-3.5 animate-spin" />
							) : (
								<RotateCcw className="h-3.5 w-3.5" />
							)}
							Roll Back
						</Button>
					)}
				<Button
					size="sm"
					onClick={onClose}
					className="h-8 text-xs font-semibold">
					Done
				</Button>
			</div>
		</div>
	);
}
