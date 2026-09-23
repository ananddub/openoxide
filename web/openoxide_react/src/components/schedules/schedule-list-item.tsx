import {
	CalendarDays,
	Play,
	Pause,
	Pencil,
	Trash2,
	MoreHorizontal,
	Terminal,
	Server,
} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import {ListRow, RowTile, ViewCard} from '#/components/settings/list-card';
import {CronChip} from '#/components/shared/cron-chip';
import {cn} from '#/api/utils';
import type {Schedule} from '#/hooks/use-schedules';

interface ScheduleItemProps {
	schedule: Schedule;
	serverName?: string | null;
	onEdit: (s: Schedule) => void;
	onDelete: (id: number) => void;
	onToggle: (s: Schedule) => void;
	onRun: (id: number) => void;
}

function ScheduleTile() {
	return (
		<RowTile>
			<CalendarDays className="size-4" />
		</RowTile>
	);
}

function Chips({
	schedule: s,
	serverName,
}: {
	schedule: Schedule;
	serverName?: string | null;
}) {
	return (
		<>
			<CronChip expression={s.cron_expression} />
			{s.shell_type && (
				<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
					{s.shell_type}
				</span>
			)}
			{serverName && (
				<span className="flex min-w-0 shrink-0 items-center gap-1 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
					<Server className="size-3" />
					<span className="max-w-[120px] truncate">{serverName}</span>
				</span>
			)}
		</>
	);
}

function Menu({
	schedule: s,
	onEdit,
	onDelete,
	onToggle,
	onRun,
}: ScheduleItemProps) {
	const isEnabled = s.enabled === 1;
	return (
		<DropdownMenu>
			<DropdownMenuTrigger
				render={
					<Button
						variant="ghost"
						size="icon"
						aria-label={`Schedule ${s.name} actions`}
						className="size-8 rounded-md text-muted-foreground hover:bg-muted/80 hover:text-foreground focus-visible:ring-0"
					/>
				}>
				<MoreHorizontal className="size-4" />
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" className="w-44">
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					disabled={!isEnabled}
					onClick={() => onRun(s.id!)}>
					<Play className="size-3.5" /> Trigger run
				</DropdownMenuItem>
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={() => onToggle(s)}>
					{isEnabled ? (
						<Pause className="size-3.5" />
					) : (
						<Play className="size-3.5" />
					)}
					{isEnabled ? 'Pause schedule' : 'Resume schedule'}
				</DropdownMenuItem>
				<DropdownMenuItem
					className="cursor-pointer gap-2"
					onClick={() => onEdit(s)}>
					<Pencil className="size-3.5" /> Edit schedule
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem
					className="cursor-pointer gap-2 text-destructive focus:text-destructive"
					onClick={() => onDelete(s.id!)}>
					<Trash2 className="size-3.5" /> Delete schedule
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	);
}

/** List row — tile, name + cron/server chips, mono command. */
export function ScheduleListItem(props: ScheduleItemProps) {
	const {schedule: s, serverName} = props;
	const isEnabled = s.enabled === 1;

	return (
		<ListRow
			className={isEnabled ? undefined : 'opacity-70'}
			icon={<ScheduleTile />}
			title={s.name}
			subtitle={
				<span className="flex items-center gap-1.5 font-mono">
					<Terminal className="size-3 shrink-0 text-muted-foreground/70" />
					<span className="truncate">{s.command}</span>
					{s.description && (
						<span className="truncate font-sans text-muted-foreground/70">
							· {s.description}
						</span>
					)}
				</span>
			}
			badges={<Chips schedule={s} serverName={serverName} />}>
			<Button
				variant="ghost"
				size="sm"
				onClick={() => props.onRun(s.id!)}
				disabled={!isEnabled}
				title="Trigger run"
				className="h-8 gap-1.5 text-xs text-muted-foreground hover:text-foreground">
				<Play className="size-3.5" /> Run
			</Button>
			<Menu {...props} />
		</ListRow>
	);
}

/** Grid card — the row's counterpart. */
export function ScheduleCard(props: ScheduleItemProps) {
	const {schedule: s, serverName} = props;
	const isEnabled = s.enabled === 1;

	return (
		<ViewCard className={cn(!isEnabled && 'opacity-70')}>
			<div className="flex items-start justify-between gap-2.5">
				<div className="flex min-w-0 items-center gap-3">
					<ScheduleTile />
					<div className="flex min-w-0 flex-col">
						<h3 className="truncate text-sm font-semibold text-foreground">
							{s.name}
						</h3>
						<span className="mt-0.5 truncate text-[11px] text-muted-foreground">
							{s.description ||
								(isEnabled ? 'Runs on schedule' : 'Paused')}
						</span>
					</div>
				</div>
				<Menu {...props} />
			</div>

			<div className="flex flex-col gap-2">
				<code className="truncate rounded-md bg-foreground/[0.04] px-2 py-1 font-mono text-[11px] text-primary">
					{s.command}
				</code>
				<div className="flex flex-wrap items-center gap-1.5">
					<Chips schedule={s} serverName={serverName} />
				</div>
			</div>
		</ViewCard>
	);
}
