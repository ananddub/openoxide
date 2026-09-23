import {HardDrive, Play, RotateCcw, Trash2, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {ListRow, RowTile, ViewCard} from '#/components/settings/list-card';
import {CronChip} from '#/components/shared/cron-chip';

interface VolumeBackupActionProps {
	backup: any;
	activeAction: 'run' | 'restore' | null;
	onRun: () => void;
	onRestore: () => void;
	onDelete: () => void;
}

function BackupTile() {
	return (
		<RowTile>
			<HardDrive className="size-4" />
		</RowTile>
	);
}

function BackupChips({backup}: {backup: any}) {
	return (
		<>
			<CronChip expression={backup.cron_expression} />
			{backup.turn_off === 1 && (
				<span className="shrink-0 rounded-md bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-medium text-amber-600 dark:text-amber-400">
					Pause on backup
				</span>
			)}
		</>
	);
}

function BackupPath({backup}: {backup: any}) {
	return (
		<span className="font-mono">
			{backup.volume_name} → {backup.prefix || 'volume-backups/'}
		</span>
	);
}

function BackupActions({
	activeAction,
	onRun,
	onRestore,
	onDelete,
	compact,
}: Omit<VolumeBackupActionProps, 'backup'> & {compact?: boolean}) {
	return (
		<>
			<Button
				onClick={onRun}
				disabled={activeAction !== null}
				variant="ghost"
				size={compact ? 'icon' : 'sm'}
				title="Run backup now"
				className={
					compact
						? 'size-8 text-muted-foreground hover:text-foreground'
						: 'h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground'
				}>
				{activeAction === 'run' ? (
					<RefreshCw
						className={`${compact ? 'size-4' : 'size-3.5'} animate-spin`}
					/>
				) : (
					<Play className={compact ? 'size-4' : 'size-3.5'} />
				)}
				{!compact && 'Run'}
			</Button>
			<Button
				onClick={onRestore}
				disabled={activeAction !== null}
				variant="ghost"
				size={compact ? 'icon' : 'sm'}
				title="Restore from the latest snapshot"
				className={
					compact
						? 'size-8 text-muted-foreground hover:text-foreground'
						: 'h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground'
				}>
				{activeAction === 'restore' ? (
					<RefreshCw
						className={`${compact ? 'size-4' : 'size-3.5'} animate-spin`}
					/>
				) : (
					<RotateCcw className={compact ? 'size-4' : 'size-3.5'} />
				)}
				{!compact && 'Restore'}
			</Button>
			<Button
				onClick={onDelete}
				variant="ghost"
				size="icon"
				title="Delete backup rule"
				className="size-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-4" />
			</Button>
		</>
	);
}

/** List row — tile, rule name, mono volume → prefix path. */
export function VolumeBackupRow(props: VolumeBackupActionProps) {
	const {backup} = props;
	return (
		<ListRow
			icon={<BackupTile />}
			title={backup.name}
			subtitle={<BackupPath backup={backup} />}
			badges={<BackupChips backup={backup} />}>
			<BackupActions {...props} />
		</ListRow>
	);
}

/** Grid card — the row's counterpart: icon-only actions, path as code line. */
export function VolumeBackupCard(props: VolumeBackupActionProps) {
	const {backup} = props;
	return (
		<ViewCard>
			<div className="flex items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<BackupTile />
					<div className="flex min-w-0 flex-col">
						<h3 className="truncate text-sm font-semibold text-foreground">
							{backup.name}
						</h3>
						<span className="mt-0.5 truncate text-[11px] text-muted-foreground">
							snapshot every run
						</span>
					</div>
				</div>
				<BackupActions {...props} compact />
			</div>

			<div className="flex flex-col gap-2">
				<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-1 font-mono text-[11px] text-primary">
					<BackupPath backup={backup} />
				</code>
				<div className="flex flex-wrap items-center gap-1.5">
					<BackupChips backup={backup} />
				</div>
			</div>
		</ViewCard>
	);
}
