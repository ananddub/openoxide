import {useState, useMemo} from 'react';
import {
	CalendarDays,
	Play,
	Pause,
	Pencil,
	Trash2,
	MoreVertical,
	Search,
	Terminal,
	Server,
} from 'lucide-react';
import {Badge} from '#/components/ui/badge';
import {Input} from '#/components/ui/input';
import {Separator} from '#/components/ui/separator';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from '#/components/ui/dropdown';
import type {Schedule} from '#/hooks/use-schedules';

interface SchedulesListProps {
	schedules: Schedule[];
	isLoading: boolean;
	servers: any[];
	onEdit: (s: Schedule) => void;
	onDelete: (id: number) => void;
	onToggle: (s: Schedule) => void;
	onRun: (id: number) => void;
}

const STATUS_FILTERS = ['All', 'Enabled', 'Disabled'] as const;
type StatusFilter = (typeof STATUS_FILTERS)[number];

export function SchedulesList({
	schedules,
	isLoading,
	servers,
	onEdit,
	onDelete,
	onToggle,
	onRun,
}: SchedulesListProps) {
	const [search, setSearch] = useState('');
	const [statusFilter, setStatusFilter] = useState<StatusFilter>('All');

	// Filtered schedules
	const filtered = useMemo(() => {
		return schedules.filter(s => {
			const matchSearch =
				s.name?.toLowerCase().includes(search.toLowerCase()) ||
				s.command?.toLowerCase().includes(search.toLowerCase()) ||
				(s.description && s.description.toLowerCase().includes(search.toLowerCase()));

			const isEnabled = s.enabled === 1;
			const matchStatus =
				statusFilter === 'All' ||
				(statusFilter === 'Enabled' && isEnabled) ||
				(statusFilter === 'Disabled' && !isEnabled);

			return matchSearch && matchStatus;
		});
	}, [schedules, search, statusFilter]);

	const hasFilters = search !== '' || statusFilter !== 'All';

	/* ── Loading ── */
	if (isLoading) {
		return (
			<div className="flex flex-col gap-2">
				{[1, 2, 3].map(i => (
					<div key={i} className="flex items-center gap-4 px-4 py-4 border border-border rounded-lg">
						<Skeleton className="w-8 h-8 rounded-full shrink-0" />
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

	return (
		<div className="flex flex-col gap-3">
			{/* ── Search + Filter bar ── */}
			<div className="flex items-center gap-2">
				<div className="relative flex-1">
					<Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground pointer-events-none" />
					<Input
						placeholder="Search by name, command or description…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="pl-8 h-8 text-xs"
					/>
				</div>

				<Select value={statusFilter} onValueChange={v => setStatusFilter(v as StatusFilter)}>
					<SelectTrigger size="sm" className="h-8 w-36 text-xs">
						<SelectValue placeholder="Status" />
					</SelectTrigger>
					<SelectContent>
						{STATUS_FILTERS.map(f => (
							<SelectItem key={f} value={f}>
								{f === 'All' ? 'All Schedules' : f}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>

			{/* ── Empty state ── */}
			{schedules.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-3 py-20 text-center border border-dashed border-border rounded-lg">
					<div className="w-12 h-12 rounded-full bg-muted flex items-center justify-center">
						<CalendarDays className="w-5 h-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">No automated schedules yet</p>
						<p className="text-xs text-muted-foreground mt-0.5">
							Create automated cron jobs, backups, or maintenance scripts.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 py-14 text-center border border-dashed border-border rounded-lg">
					<Search className="w-5 h-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">No schedules match your filter</p>
				</div>
			) : (
				/* ── List ── */
				<div className="border border-border rounded-lg overflow-hidden divide-y divide-border">
					{filtered.map(s => {
						const isEnabled = s.enabled === 1;
						const dotCls = isEnabled ? 'bg-emerald-500' : 'bg-zinc-500/60';

						// Linked server details
						const linkedServer = servers.find(srv => srv.id === s.server_id);
						const serverName = linkedServer?.name || (s.server_id ? `Server #${s.server_id}` : null);

						return (
							<div
								key={s.id}
								className={`group flex items-center gap-4 px-4 py-3.5 bg-card hover:bg-accent/30 transition-colors ${
									!isEnabled ? 'opacity-70' : ''
								}`}
							>
								{/* Icon + status dot */}
								<div className="relative shrink-0">
									<div className="w-8 h-8 rounded-full bg-muted flex items-center justify-center">
										<CalendarDays className="w-4 h-4 text-foreground/70" />
									</div>
									<span className={`absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-background ${dotCls}`} />
								</div>

								{/* Info */}
								<div className="flex-1 min-w-0">
									<div className="flex items-center gap-2 mb-0.5">
										<span className="text-sm font-medium text-foreground truncate">{s.name}</span>
										<Badge variant="secondary" className="shrink-0 text-[10px] font-mono py-0">
											{s.cron_expression}
										</Badge>
										{serverName && (
											<span className="text-[10px] text-muted-foreground bg-muted/50 px-1.5 py-0.5 rounded flex items-center gap-1">
												<Server className="w-3 h-3" />
												<span className="truncate max-w-[100px]">{serverName}</span>
											</span>
										)}
									</div>
									<div className="flex items-center gap-2 text-[11px] font-mono text-muted-foreground truncate">
										<span className="flex items-center gap-1 text-foreground/80 truncate">
											<Terminal className="w-3 h-3 text-primary shrink-0" />
											<code className="truncate">{s.command}</code>
										</span>
										{s.description && (
											<span className="text-muted-foreground font-sans truncate">· {s.description}</span>
										)}
									</div>
								</div>

								{/* Hover actions */}
								<div className="flex items-center gap-3 opacity-0 group-hover:opacity-100 transition-opacity">
									<button
										onClick={() => onRun(s.id!)}
										disabled={!isEnabled}
										title="Trigger run"
										className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
									>
										<Play className="w-4 h-4" />
									</button>
									<button
										onClick={() => onEdit(s)}
										title="Edit schedule"
										className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
									>
										<Pencil className="w-4 h-4" />
									</button>
								</div>

								<Separator orientation="vertical" className="h-5 opacity-0 group-hover:opacity-100 transition-opacity" />

								{/* 3-dot menu */}
								<DropdownMenu>
									<DropdownMenuTrigger render={<button className="text-muted-foreground hover:text-foreground transition-colors cursor-pointer" />}>
										<MoreVertical className="w-4 h-4" />
									</DropdownMenuTrigger>
									<DropdownMenuContent align="end" className="w-44">
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onRun(s.id!)} disabled={!isEnabled}>
											<Play className="w-3.5 h-3.5" /> Trigger Run
										</DropdownMenuItem>
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onToggle(s)}>
											{isEnabled ? (
												<><Pause className="w-3.5 h-3.5" /> Pause Schedule</>
											) : (
												<><Play className="w-3.5 h-3.5" /> Resume Schedule</>
											)}
										</DropdownMenuItem>
										<DropdownMenuItem className="gap-2 cursor-pointer" onClick={() => onEdit(s)}>
											<Pencil className="w-3.5 h-3.5" /> Edit Schedule
										</DropdownMenuItem>
										<DropdownMenuSeparator />
										<DropdownMenuItem
											className="gap-2 cursor-pointer text-destructive focus:text-destructive"
											onClick={() => onDelete(s.id!)}
										>
											<Trash2 className="w-3.5 h-3.5" /> Delete Schedule
										</DropdownMenuItem>
									</DropdownMenuContent>
								</DropdownMenu>
							</div>
						);
					})}
				</div>
			)}

			{/* Result count */}
			{hasFilters && filtered.length > 0 && (
				<p className="text-xs text-muted-foreground px-1">
					Showing {filtered.length} of {schedules.length} schedules
				</p>
			)}
		</div>
	);
}
