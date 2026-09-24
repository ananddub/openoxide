import {useState} from 'react';
import type {ReactNode} from 'react';
import {Calendar, Play, Trash2, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	ListCard,
	ListRow,
	ListEmpty,
	ListSkeleton,
	GridSkeleton,
	RowTile,
	ViewCard,
	ViewGrid,
	type ListView,
} from '#/components/settings/list-card';
import {CronChip} from '#/components/shared/cron-chip';

interface ComposeSchedulesTableProps {
	schedules: any[];
	isLoading: boolean;
	hasQuery: boolean;
	view: ListView;
	onRun: (id: number) => Promise<void>;
	onDelete: (id: number) => Promise<void>;
	createAction?: ReactNode;
}

function Chip({children, mono}: {children: ReactNode; mono?: boolean}) {
	return (
		<span
			className={`shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground ${
				mono ? 'font-mono' : ''
			}`}>
			{children}
		</span>
	);
}

function ScheduleTile() {
	return (
		<RowTile>
			<Calendar className="size-4" />
		</RowTile>
	);
}

/** Compose / database schedules surface — grid cards or one bordered list card, matching the SSH Keys page. */
export function ComposeSchedulesTable({
	schedules,
	isLoading,
	hasQuery,
	view,
	onRun,
	onDelete,
	createAction,
}: ComposeSchedulesTableProps) {
	const [activeRunningId, setActiveRunningId] = useState<number | null>(
		null,
	);
	const [activeDeletingId, setActiveDeletingId] = useState<number | null>(
		null,
	);
	const safeSchedules = Array.isArray(schedules) ? schedules : [];

	const handleRun = async (id: number) => {
		setActiveRunningId(id);
		try {
			await onRun(id);
		} finally {
			setActiveRunningId(null);
		}
	};

	const handleDelete = async (id: number) => {
		setActiveDeletingId(id);
		try {
			await onDelete(id);
		} finally {
			setActiveDeletingId(null);
		}
	};

	const actions = (s: any) => (
		<>
			<Button
				variant="ghost"
				size="sm"
				onClick={() => handleRun(s.id)}
				disabled={activeRunningId === s.id}
				title="Run now"
				className="h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground">
				<Play
					className={`size-3.5 ${activeRunningId === s.id ? 'animate-spin' : ''}`}
				/>
				Run
			</Button>
			<Button
				variant="ghost"
				size="icon"
				onClick={() => handleDelete(s.id)}
				disabled={activeDeletingId === s.id}
				title="Delete schedule"
				className="size-8 text-muted-foreground hover:bg-destructive/10 hover:text-destructive">
				<Trash2 className="size-4" />
			</Button>
		</>
	);

	if (isLoading && safeSchedules.length === 0)
		return view === 'grid' ? <GridSkeleton /> : <ListSkeleton />;

	if (safeSchedules.length === 0)
		return hasQuery ? (
			<ListEmpty
				icon={Search}
				title="No schedules match your search"
				description="Try a different task name, command or cron expression."
			/>
		) : (
			<ListEmpty
				icon={Calendar}
				title="No scheduled tasks yet"
				description="Configure scheduled tasks to run recurring commands inside your compose service container."
				action={createAction}
			/>
		);

	if (view === 'grid')
		return (
			<ViewGrid>
				{safeSchedules.map((s: any) => (
					<ViewCard key={s.id}>
						<div className="flex items-start justify-between gap-2.5">
							<div className="flex min-w-0 items-center gap-3">
								<ScheduleTile />
								<div className="flex min-w-0 flex-col">
									<h3 className="truncate text-sm font-semibold text-foreground">
										{s.name}
									</h3>
									<span className="mt-0.5 truncate text-[11px] text-muted-foreground">
										runs in {s.service_name || 'app'}
									</span>
								</div>
							</div>
							{actions(s)}
						</div>
						<div className="flex flex-col gap-2">
							<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-1 font-mono text-[11px] text-primary">
								{s.command}
							</code>
							<div className="flex flex-wrap items-center gap-1.5">
								<Chip>{s.service_name || 'app'}</Chip>
								<CronChip expression={s.cron_expression || '0 * * * *'} />
							</div>
						</div>
					</ViewCard>
				))}
			</ViewGrid>
		);

	return (
		<ListCard>
			{safeSchedules.map((s: any) => (
				<ListRow
					key={s.id}
					icon={<ScheduleTile />}
					title={s.name}
					subtitle={<code className="font-mono">{s.command}</code>}
					badges={
						<>
							<Chip>{s.service_name || 'app'}</Chip>
							<CronChip expression={s.cron_expression || '0 * * * *'} />
						</>
					}>
					{actions(s)}
				</ListRow>
			))}
		</ListCard>
	);
}
