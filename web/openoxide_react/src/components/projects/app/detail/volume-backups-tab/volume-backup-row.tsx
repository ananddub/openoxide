import {
	HardDrive,
	Play,
	RotateCcw,
	Trash2,
	Clock,
	ShieldCheck,
	RefreshCw,
} from 'lucide-react';
import {Button} from '#/components/ui/button';

/** A single volume-backup rule row with run / restore / delete actions. */
export function VolumeBackupRow({
	backup,
	activeAction,
	onRun,
	onRestore,
	onDelete,
}: {
	backup: any;
	activeAction: 'run' | 'restore' | null;
	onRun: () => void;
	onRestore: () => void;
	onDelete: () => void;
}) {
	return (
		<div className="flex flex-col justify-between gap-4 p-4 transition-colors hover:bg-muted/10 sm:flex-row sm:items-center">
			<div className="flex min-w-0 items-start gap-3">
				<div className="mt-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
					<HardDrive className="h-4 w-4" />
				</div>
				<div className="min-w-0">
					<div className="flex items-center gap-2">
						<span className="truncate text-xs font-bold text-foreground">
							{backup.name}
						</span>
						{backup.turn_off === 1 && (
							<span className="rounded bg-amber-500/10 px-1.5 py-0.5 font-mono text-[10px] text-amber-600 dark:text-amber-400">
								Pause container on backup
							</span>
						)}
					</div>
					<div className="mt-1.5 flex flex-wrap items-center gap-x-3 gap-y-1 text-[11px] font-medium text-muted-foreground">
						<span className="rounded bg-muted px-1.5 py-0.5 font-mono text-foreground/80">
							Volume: {backup.volume_name}
						</span>
						<span className="flex items-center gap-1">
							<Clock className="h-3 w-3 text-muted-foreground" /> Schedule:{' '}
							{backup.cron_expression}
						</span>
						<span className="flex items-center gap-1">
							<ShieldCheck className="h-3 w-3 text-emerald-500" /> S3
							Prefix: {backup.prefix}
						</span>
					</div>
				</div>
			</div>

			<div className="flex shrink-0 items-center gap-2 self-end sm:self-auto">
				<Button
					onClick={onRun}
					disabled={activeAction !== null}
					variant="outline"
					size="sm"
					className="flex h-8 items-center gap-1.5 rounded-lg  text-xs font-semibold text-foreground hover:bg-muted">
					{activeAction === 'run' ? (
						<RefreshCw className="h-3.5 w-3.5 animate-spin" />
					) : (
						<Play className="h-3.5 w-3.5" />
					)}
					Run Now
				</Button>
				<Button
					onClick={onRestore}
					disabled={activeAction !== null}
					variant="outline"
					size="sm"
					className="flex h-8 items-center gap-1.5 rounded-lg  text-xs font-semibold text-foreground hover:bg-muted">
					{activeAction === 'restore' ? (
						<RefreshCw className="h-3.5 w-3.5 animate-spin" />
					) : (
						<RotateCcw className="h-3.5 w-3.5" />
					)}
					Restore
				</Button>
				<Button
					onClick={onDelete}
					variant="ghost"
					size="icon"
					className="h-8 w-8 rounded-lg text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
					<Trash2 className="h-4 w-4" />
				</Button>
			</div>
		</div>
	);
}
