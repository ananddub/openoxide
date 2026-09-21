import {
	CalendarDays,
	Play,
	Pause,
	Pencil,
	Trash2,
	MoreVertical,
	Terminal,
	Server,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Separator} from '#/components/ui/separator';
import {Skeleton} from '#/components/ui/skeleton';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {Schedule} from '#/hooks/use-schedules';

interface ScheduleListItemProps {
	schedule: Schedule;
	servers: any[];
	onEdit: (s: Schedule) => void;
	onDelete: (id: number) => void;
	onToggle: (s: Schedule) => void;
	onRun: (id: number) => void;
}

/** Loading placeholder rows for the schedules list. */
export function SchedulesSkeleton() {
	return (
		<div className="flex flex-col gap-2">
			{[1, 2, 3].map(i => (
				<div
					key={i}
					className="flex items-center gap-4 rounded-lg px-4 py-4">
					<Skeleton className="h-8 w-8 shrink-0 rounded-full" />
					<div className="flex-1 space-y-2">
						<Skeleton className="h-3.5 w-40" />
						<Skeleton className="h-3 w-56" />
					</div>
					<Skeleton className="h-5 w-16 rounded-full" />
				</div>
			))}
		</div>
	);
}

/** A single schedule row with hover actions and overflow menu. */
export function ScheduleListItem({
	schedule: s,
	servers,
	onEdit,
	onDelete,
	onToggle,
	onRun,
}: ScheduleListItemProps) {
	const isEnabled = s.enabled === 1;
	const dotCls = isEnabled ? 'bg-emerald-500' : 'bg-zinc-500/60';

	const linkedServer = servers.find(srv => srv.id === s.server_id);
	const serverName =
		linkedServer?.name || (s.server_id ? `Server #${s.server_id}` : null);

	return (
		<div
			className={`group flex items-center gap-4 bg-card px-4 py-3.5 transition-colors hover:bg-accent/30 ${
				!isEnabled ? 'opacity-70' : ''
			}`}>
			{/* Icon + status dot */}
			<div className="relative shrink-0">
				<div className="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
					<CalendarDays className="h-4 w-4 text-foreground/70" />
				</div>
				<span
					className={`absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-background ${dotCls}`}
				/>
			</div>

			{/* Info */}
			<div className="min-w-0 flex-1">
				<div className="mb-0.5 flex items-center gap-2">
					<span className="truncate text-sm font-medium text-foreground">
						{s.name}
					</span>
					<Badge
						variant="secondary"
						className="shrink-0 py-0 font-mono text-[10px]">
						{s.cron_expression}
					</Badge>
					{serverName && (
						<span className="flex items-center gap-1 rounded bg-muted/50 px-1.5 py-0.5 text-[10px] text-muted-foreground">
							<Server className="h-3 w-3" />
							<span className="max-w-[100px] truncate">{serverName}</span>
						</span>
					)}
				</div>
				<div className="flex items-center gap-2 truncate font-mono text-[11px] text-muted-foreground">
					<span className="flex items-center gap-1 truncate text-foreground/80">
						<Terminal className="h-3 w-3 shrink-0 text-primary" />
						<code className="truncate">{s.command}</code>
					</span>
					{s.description && (
						<span className="truncate font-sans text-muted-foreground">
							· {s.description}
						</span>
					)}
				</div>
			</div>

			{/* Hover actions */}
			<div className="flex items-center gap-3 opacity-0 transition-opacity group-hover:opacity-100">
				<button
					onClick={() => onRun(s.id!)}
					disabled={!isEnabled}
					title="Trigger run"
					className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground disabled:cursor-not-allowed disabled:opacity-40">
					<Play className="h-4 w-4" />
				</button>
				<button
					onClick={() => onEdit(s)}
					title="Edit schedule"
					className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground">
					<Pencil className="h-4 w-4" />
				</button>
			</div>

			<Separator
				orientation="vertical"
				className="h-5 opacity-0 transition-opacity group-hover:opacity-100"
			/>

			{/* 3-dot menu */}
			<DropdownMenu>
				<DropdownMenuTrigger
					render={
						<button className="cursor-pointer text-muted-foreground transition-colors hover:text-foreground" />
					}>
					<MoreVertical className="h-4 w-4" />
				</DropdownMenuTrigger>
				<DropdownMenuContent align="end" className="w-44">
					<DropdownMenuItem
						className="cursor-pointer gap-2"
						onClick={() => onRun(s.id!)}
						disabled={!isEnabled}>
						<Play className="h-3.5 w-3.5" /> Trigger Run
					</DropdownMenuItem>
					<DropdownMenuItem
						className="cursor-pointer gap-2"
						onClick={() => onToggle(s)}>
						{isEnabled ? (
							<>
								<Pause className="h-3.5 w-3.5" /> Pause Schedule
							</>
						) : (
							<>
								<Play className="h-3.5 w-3.5" /> Resume Schedule
							</>
						)}
					</DropdownMenuItem>
					<DropdownMenuItem
						className="cursor-pointer gap-2"
						onClick={() => onEdit(s)}>
						<Pencil className="h-3.5 w-3.5" /> Edit Schedule
					</DropdownMenuItem>
					<DropdownMenuSeparator />
					<DropdownMenuItem
						className="cursor-pointer gap-2 text-destructive focus:text-destructive"
						onClick={() => onDelete(s.id!)}>
						<Trash2 className="h-3.5 w-3.5" /> Delete Schedule
					</DropdownMenuItem>
				</DropdownMenuContent>
			</DropdownMenu>
		</div>
	);
}
