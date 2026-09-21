import {Calendar, Clock, Play, Power, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';

export function ScheduleJobRow({
	job,
	onRun,
	onToggle,
	onDelete,
}: {
	job: any;
	onRun: (id: number) => void;
	onToggle: (job: any) => void;
	onDelete: (id: number) => void;
}) {
	return (
		<div className="flex flex-col justify-between gap-4 p-4 transition-colors hover:bg-muted/10 sm:flex-row sm:items-center">
			<div className="flex min-w-0 items-start gap-3">
				<div className="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-secondary text-muted-foreground">
					<Calendar className="h-4 w-4" />
				</div>
				<div className="min-w-0">
					<div className="flex items-center gap-2">
						<span className="text-xs font-semibold text-foreground">
							{job.name}
						</span>
						<span
							className={`rounded px-1.5 py-0.5 text-[9px] font-bold tracking-wider uppercase ${
								job.enabled
									? 'border-emerald-500/20 bg-emerald-500/10 text-emerald-500'
									: 'border- bg-muted/40 text-muted-foreground'
							}`}>
							{job.enabled ? 'Active' : 'Paused'}
						</span>
					</div>
					<div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-[10px] font-medium text-muted-foreground">
						<span className="rounded bg-muted/30 px-1.5 py-0.5 font-mono text-foreground">
							{job.cron_expression}
						</span>
						<span className="max-w-xs truncate font-mono text-muted-foreground">
							{job.command}
						</span>
						<span className="flex items-center gap-1">
							<Clock className="h-3 w-3" />
							{new Date(
								Number(job.created_at) * 1000,
							).toLocaleDateString()}
						</span>
					</div>
				</div>
			</div>

			<div className="flex shrink-0 items-center gap-2 self-end sm:self-auto">
				<Button
					variant="outline"
					size="sm"
					onClick={() => onRun(job.id)}
					className="flex h-7 items-center gap-1 rounded-md text-[10px] font-semibold text-foreground hover:bg-muted">
					<Play className="h-3 w-3" /> Run Now
				</Button>
				<Button
					variant="ghost"
					size="icon"
					onClick={() => onToggle(job)}
					title={job.enabled ? 'Pause Job' : 'Activate Job'}
					className="h-7 w-7 rounded-md text-muted-foreground hover:bg-muted hover:text-foreground">
					<Power className="h-3.5 w-3.5" />
				</Button>
				<Button
					variant="ghost"
					size="icon"
					onClick={() => onDelete(job.id)}
					title="Delete Job"
					className="h-7 w-7 rounded-md text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
					<Trash2 className="h-3.5 w-3.5" />
				</Button>
			</div>
		</div>
	);
}
