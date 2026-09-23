import {Calendar, Pause, Play, Power, Trash2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {ListRow, RowTile, ViewCard} from '#/components/settings/list-card';
import {CronChip} from '#/components/shared/cron-chip';
import {formatTimestamp} from '#/components/projects/detail/deployments/build-status';

interface ScheduleJobActionsProps {
	job: any;
	onRun: (id: number) => void;
	onToggle: (job: any) => void;
	onDelete: (id: number) => void;
}

function StatusChip({enabled}: {enabled: boolean}) {
	return (
		<span
			className={`shrink-0 rounded-md px-1.5 py-0.5 text-[10px] font-medium ${
				enabled
					? 'bg-emerald-500/10 text-emerald-500'
					: 'bg-foreground/[0.05] text-muted-foreground'
			}`}>
			{enabled ? 'Active' : 'Paused'}
		</span>
	);
}

function JobTile() {
	return (
		<RowTile>
			<Calendar className="size-4" />
		</RowTile>
	);
}

function JobActions({
	job,
	enabled,
	onRun,
	onToggle,
	onDelete,
}: ScheduleJobActionsProps & {enabled: boolean}) {
	return (
		<>
			<Button
				variant="ghost"
				size="sm"
				onClick={() => onRun(job.id)}
				title="Run now"
				className="h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground">
				<Play className="size-3.5" /> Run
			</Button>
			<Button
				variant="ghost"
				size="icon"
				onClick={() => onToggle(job)}
				title={enabled ? 'Pause job' : 'Resume job'}
				className="size-8 text-muted-foreground hover:text-foreground">
				{enabled ? (
					<Pause className="size-4" />
				) : (
					<Power className="size-4" />
				)}
			</Button>
			<Button
				variant="ghost"
				size="icon"
				onClick={() => onDelete(job.id)}
				title="Delete job"
				className="size-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-4" />
			</Button>
		</>
	);
}

/** List row — tile, name + cron chips, mono command. */
export function ScheduleJobRow(props: ScheduleJobActionsProps) {
	const {job} = props;
	const enabled = !!job.enabled;

	return (
		<ListRow
			icon={<JobTile />}
			title={job.name}
			subtitle={
				<>
					<code className="font-mono">{job.command}</code>
					<span className="text-muted-foreground/70">
						{' '}
						· created {formatTimestamp(job.created_at)}
					</span>
				</>
			}
			badges={
				<>
					<StatusChip enabled={enabled} />
					<CronChip expression={job.cron_expression} />
				</>
			}>
			<JobActions {...props} enabled={enabled} />
		</ListRow>
	);
}

/** Grid card — the row's counterpart. */
export function ScheduleJobCard(props: ScheduleJobActionsProps) {
	const {job} = props;
	const enabled = !!job.enabled;

	return (
		<ViewCard className={enabled ? undefined : 'opacity-70'}>
			<div className="flex items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<JobTile />
					<div className="flex min-w-0 flex-col">
						<h3 className="truncate text-sm font-semibold text-foreground">
							{job.name}
						</h3>
						<span className="mt-0.5 truncate text-[11px] text-muted-foreground">
							created {formatTimestamp(job.created_at)}
						</span>
					</div>
				</div>
				<JobActions {...props} enabled={enabled} />
			</div>

			<div className="flex flex-col gap-2">
				<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-1 font-mono text-[11px] text-primary">
					{job.command}
				</code>
				<div className="flex flex-wrap items-center gap-1.5">
					<StatusChip enabled={enabled} />
					<CronChip expression={job.cron_expression} />
				</div>
			</div>
		</ViewCard>
	);
}
