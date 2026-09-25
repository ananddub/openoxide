import {useMemo, useState} from 'react';
import {CalendarDays, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	GridSkeleton,
	ListCard,
	ListEmpty,
	ListRegion,
	ListSkeleton,
	Toolbar,
	ToolbarSearch,
	ViewGrid,
	ViewToggle,
	useListView,
} from '#/components/settings/list-card';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {Schedule} from '#/hooks/schedules/use-schedules';
import {ScheduleCard, ScheduleListItem} from './schedule-list-item';

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

/** Automated schedules surface — the SSH Keys grammar: toolbar (search + status filter + grid/list) above grid cards or one bordered list card. */
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
	const [view, setView] = useListView('schedules-view');

	const q = search.trim().toLowerCase();
	const filtered = useMemo(
		() =>
			schedules.filter(s => {
				const isEnabled = s.enabled === 1;
				if (statusFilter === 'Enabled' && !isEnabled) return false;
				if (statusFilter === 'Disabled' && isEnabled) return false;
				if (!q) return true;
				return [s.name, s.command, s.description]
					.filter(Boolean)
					.join(' ')
					.toLowerCase()
					.includes(q);
			}),
		[schedules, q, statusFilter],
	);

	const hasFilters = q !== '' || statusFilter !== 'All';
	const serverName = (s: Schedule) =>
		servers.find(srv => srv.id === s.server_id)?.name ||
		(s.server_id ? `Server #${s.server_id}` : null);

	const shared = (s: Schedule) => ({
		schedule: s,
		serverName: serverName(s),
		onEdit,
		onDelete,
		onToggle,
		onRun,
	});

	const body =
		schedules.length === 0 ? (
			<ListEmpty
				icon={CalendarDays}
				title="No automated schedules yet"
				description="Create cron jobs, database backups and maintenance scripts that run against your services."
			/>
		) : filtered.length === 0 ? (
			<ListEmpty
				icon={Search}
				title="No schedules match your filters"
				description="Try a different name, command or description — or clear the filters."
				action={
					hasFilters && (
						<Button
							variant="ghost"
							onClick={() => {
								setSearch('');
								setStatusFilter('All');
							}}
							className="text-xs font-semibold text-primary">
							Clear All Filters
						</Button>
					)
				}
			/>
		) : view === 'grid' ? (
			<ViewGrid>
				{filtered.map(s => (
					<ScheduleCard key={s.id} {...shared(s)} />
				))}
			</ViewGrid>
		) : (
			<ListCard>
				{filtered.map(s => (
					<ScheduleListItem key={s.id} {...shared(s)} />
				))}
			</ListCard>
		);

	return (
		<div className="flex h-full min-h-0 w-full flex-col gap-4">
			<Toolbar
				action={
					<>
						<Select
							value={statusFilter}
							onValueChange={val =>
								val && setStatusFilter(val as StatusFilter)
							}>
							<SelectTrigger className="h-9 w-auto rounded-lg bg-card">
								<SelectValue>
									{statusFilter === 'All'
										? 'All Schedules'
										: statusFilter === 'Enabled'
											? 'Enabled'
											: 'Disabled'}
								</SelectValue>
							</SelectTrigger>
							<SelectContent className="bg-card">
								{STATUS_FILTERS.map(f => (
									<SelectItem key={f} value={f}>
										{f === 'All' ? 'All Schedules' : f}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
						<ViewToggle view={view} onChange={setView} />
					</>
				}>
				<ToolbarSearch
					value={search}
					onChange={setSearch}
					placeholder="Search schedules…"
					className="min-w-[200px] flex-1 sm:max-w-none"
				/>
			</Toolbar>
			<ListRegion>
				{isLoading ? (
					view === 'grid' ? (
						<GridSkeleton />
					) : (
						<ListSkeleton />
					)
				) : (
					body
				)}
			</ListRegion>
		</div>
	);
}
