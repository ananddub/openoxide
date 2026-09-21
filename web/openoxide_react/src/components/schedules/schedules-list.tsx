import {useState, useMemo} from 'react';
import {CalendarDays, Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {Schedule} from '#/hooks/use-schedules';
import {ScheduleListItem, SchedulesSkeleton} from './schedule-list-item';

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
				(s.description &&
					s.description.toLowerCase().includes(search.toLowerCase()));

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
	if (isLoading) return <SchedulesSkeleton />;

	return (
		<div className="flex flex-col gap-3">
			{/* ── Search + Filter bar ── */}
			<div className="flex items-center gap-2">
				<div className="relative flex-1">
					<Search className="pointer-events-none absolute top-1/2 left-2.5 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
					<Input
						placeholder="Search by name, command or description…"
						value={search}
						onChange={e => setSearch(e.target.value)}
						className="h-8 pl-8 text-xs"
					/>
				</div>

				<Select
					value={statusFilter}
					onValueChange={v => setStatusFilter(v as StatusFilter)}>
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
				<div className="flex flex-col items-center justify-center gap-3 rounded-lg py-20 text-center">
					<div className="flex h-12 w-12 items-center justify-center rounded-full bg-muted">
						<CalendarDays className="h-5 w-5 text-muted-foreground" />
					</div>
					<div>
						<p className="text-sm font-medium text-foreground">
							No automated schedules yet
						</p>
						<p className="mt-0.5 text-xs text-muted-foreground">
							Create automated cron jobs, backups, or maintenance scripts.
						</p>
					</div>
				</div>
			) : filtered.length === 0 ? (
				<div className="flex flex-col items-center justify-center gap-2 rounded-lg py-14 text-center">
					<Search className="h-5 w-5 text-muted-foreground" />
					<p className="text-sm text-muted-foreground">
						No schedules match your filter
					</p>
				</div>
			) : (
				/* ── List ── */
				<div className="divide-y divide-border overflow-hidden rounded-lg">
					{filtered.map(s => (
						<ScheduleListItem
							key={s.id}
							schedule={s}
							servers={servers}
							onEdit={onEdit}
							onDelete={onDelete}
							onToggle={onToggle}
							onRun={onRun}
						/>
					))}
				</div>
			)}

			{/* Result count */}
			{hasFilters && filtered.length > 0 && (
				<p className="px-1 text-xs text-muted-foreground">
					Showing {filtered.length} of {schedules.length} schedules
				</p>
			)}
		</div>
	);
}
