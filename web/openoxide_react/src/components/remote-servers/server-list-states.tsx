import {Button} from '#/components/ui/button';
import {Input} from '#/components/ui/input';
import {Skeleton} from '#/components/ui/skeleton';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {LayoutGrid, List, Plus, Search, Server, X} from 'lucide-react';
import {cn} from '#/api/utils';

const STATUS_FILTERS = ['All', 'Connected', 'Not Connected'] as const;
export type StatusFilter = (typeof STATUS_FILTERS)[number];
export type ServersView = 'grid' | 'list';

/** Loading placeholder rows. */
export function ServerListSkeleton() {
	return (
		<div className="flex flex-col gap-2">
			{[1, 2, 3].map(i => (
				<div
					key={i}
					className="flex items-center gap-4 rounded-lg px-4 py-4">
					<Skeleton className="h-9 w-9 shrink-0 rounded-lg" />
					<div className="flex-1 space-y-2">
						<Skeleton className="h-3.5 w-40" />
						<Skeleton className="h-3 w-56" />
					</div>
					<Skeleton className="h-5 w-14 rounded-full" />
				</div>
			))}
		</div>
	);
}

/** First-run empty state with the add CTA. */
export function ServerListEmpty({onAdd}: {onAdd: () => void}) {
	return (
		<div className="flex flex-col items-center justify-center gap-4 rounded-xl border bg-card px-6 py-20 text-center">
			<div className="flex size-12 items-center justify-center rounded-full bg-muted">
				<Server className="size-5 text-muted-foreground" />
			</div>
			<div>
				<p className="text-sm font-medium text-foreground">
					No remote servers yet
				</p>
				<p className="mx-auto mt-1 max-w-sm text-xs text-muted-foreground">
					Connect a Linux node over SSH to deploy applications, containers
					and databases onto it from this panel.
				</p>
			</div>
			<Button
				onClick={onAdd}
				className="h-9 cursor-pointer gap-1.5 px-4 text-sm font-medium">
				<Plus className="size-3.5" />
				Add Server
			</Button>
		</div>
	);
}

/** One-line Geist toolbar: search + status filter + grid/list toggle. */
export function ServersToolbar({
	search,
	status,
	onSearch,
	onStatus,
	view,
	onViewChange,
}: {
	search: string;
	status: StatusFilter;
	onSearch: (v: string) => void;
	onStatus: (v: StatusFilter) => void;
	view: ServersView;
	onViewChange: (v: ServersView) => void;
}) {
	return (
		<div className="flex w-full flex-wrap items-center gap-3">
			<div className="relative min-w-0 flex-1">
				<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/60" />
				<Input
					placeholder="Search servers…"
					value={search}
					onChange={e => onSearch(e.target.value)}
					className="h-9 rounded-lg pr-10 pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
				/>
				{search && (
					<button
						onClick={() => onSearch('')}
						className="absolute top-1/2 right-3 -translate-y-1/2 cursor-pointer text-muted-foreground hover:text-foreground">
						<X className="size-3.5" />
					</button>
				)}
			</div>

			<Select
				value={status}
				onValueChange={v => onStatus(v as StatusFilter)}>
				<SelectTrigger className="h-9 w-auto min-w-[150px] rounded-lg bg-card text-sm">
					<SelectValue placeholder="Status" />
				</SelectTrigger>
				<SelectContent className="bg-card">
					{STATUS_FILTERS.map(f => (
						<SelectItem key={f} value={f}>
							{f === 'All' ? 'All Servers' : f}
						</SelectItem>
					))}
				</SelectContent>
			</Select>

			{/* Grid / list toggle */}
			<div className="ml-auto flex shrink-0 items-center rounded-lg border bg-card p-1">
				<button
					type="button"
					aria-label="Grid view"
					title="Grid view"
					onClick={() => onViewChange('grid')}
					className={cn(
						'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
						view === 'grid' && 'bg-accent text-foreground',
					)}>
					<LayoutGrid className="size-3.5" />
				</button>
				<button
					type="button"
					aria-label="List view"
					title="List view"
					onClick={() => onViewChange('list')}
					className={cn(
						'flex size-7 cursor-pointer items-center justify-center rounded-md text-muted-foreground transition-colors hover:text-foreground',
						view === 'list' && 'bg-accent text-foreground',
					)}>
					<List className="size-3.5" />
				</button>
			</div>
		</div>
	);
}

/** No-results state shown when filters exclude everything. */
export function ServerFilterEmpty({onClear}: {onClear: () => void}) {
	return (
		<div className="flex flex-col items-center justify-center gap-2 rounded-xl border bg-card py-14 text-center">
			<Search className="size-5 text-muted-foreground" />
			<p className="text-sm text-muted-foreground">
				No servers match your filter
			</p>
			<Button
				variant="ghost"
				size="sm"
				onClick={onClear}
				className="h-7 cursor-pointer text-xs">
				Clear filters
			</Button>
		</div>
	);
}
