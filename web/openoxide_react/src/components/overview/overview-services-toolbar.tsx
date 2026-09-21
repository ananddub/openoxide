import {
	TabSearchInput,
	TabFilterSelect,
	type FilterOption,
} from './overview-tab-parts';
import {
	Select,
	SelectTrigger,
	SelectValue,
	SelectContent,
	SelectItem,
} from '#/components/ui/select';

const TYPE_OPTIONS: FilterOption[] = [
	{value: 'all', label: 'All Types'},
	{value: 'application', label: 'Application'},
	{value: 'compose', label: 'Compose'},
	{value: 'database', label: 'Database'},
	{value: 'postgres', label: 'PostgreSQL'},
	{value: 'mysql', label: 'MySQL'},
	{value: 'mariadb', label: 'MariaDB'},
	{value: 'mongo', label: 'MongoDB'},
	{value: 'redis', label: 'Redis'},
];

const STATUS_OPTIONS: FilterOption[] = [
	{value: 'all', label: 'All Statuses'},
	{value: 'done', label: 'Running'},
	{value: 'deploying', label: 'Deploying'},
	{value: 'idle', label: 'Idle'},
	{value: 'error', label: 'Error'},
];

const SORT_OPTIONS: FilterOption[] = [
	{value: 'newest', label: 'Newest First'},
	{value: 'oldest', label: 'Oldest First'},
	{value: 'name-asc', label: 'Name (A-Z)'},
	{value: 'name-desc', label: 'Name (Z-A)'},
];

/** Search + project/type/status/sort filter bar for the services table. */
export function OverviewServicesToolbar({
	count,
	search,
	onSearch,
	projectId,
	onProjectId,
	projects,
	type,
	onType,
	status,
	onStatus,
	sort,
	onSort,
}: {
	count: number;
	search: string;
	onSearch: (v: string) => void;
	projectId: string;
	onProjectId: (v: string) => void;
	projects: any[];
	type: string;
	onType: (v: string) => void;
	status: string;
	onStatus: (v: string) => void;
	sort: string;
	onSort: (v: string) => void;
}) {
	const projectLabel =
		projectId === 'all'
			? 'All Projects'
			: `Project: ${projects.find((p: any) => String(p.id) === String(projectId))?.name || 'Selected'}`;

	return (
		<div className="flex flex-wrap items-center justify-between gap-4">
			<h3 className="text-lg font-bold tracking-tight text-foreground">
				Services{' '}
				<span className="text-sm font-normal text-muted-foreground">
					({count})
				</span>
			</h3>

			<div className="flex flex-wrap items-center gap-2.5">
				<TabSearchInput
					placeholder="Filter services..."
					value={search}
					onChange={onSearch}
				/>

				{/* Project Filter (dynamic options) */}
				<Select
					value={projectId}
					onValueChange={val => val && onProjectId(val)}>
					<SelectTrigger
						size="sm"
						className="h-9 w-[160px] text-xs font-semibold">
						<SelectValue>
							{projectId === 'all'
								? 'Project: All Projects'
								: projectLabel}
						</SelectValue>
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="all">All Projects</SelectItem>
						{projects.map((p: any) => (
							<SelectItem key={p.id} value={String(p.id)}>
								{p.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>

				<TabFilterSelect
					title="Type"
					value={type}
					onChange={onType}
					options={TYPE_OPTIONS}
					width="w-[150px]"
				/>
				<TabFilterSelect
					title="Status"
					value={status}
					onChange={onStatus}
					options={STATUS_OPTIONS}
					width="w-[150px]"
				/>
				<TabFilterSelect
					title="Sort"
					value={sort}
					onChange={onSort}
					options={SORT_OPTIONS}
					width="w-[160px]"
				/>
			</div>
		</div>
	);
}
