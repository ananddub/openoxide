import {Search, X} from 'lucide-react';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import type {SortKey} from '#/hooks/deployments/use-deployments';

interface DeploymentsFiltersProps {
	searchQuery: string;
	setSearchQuery: (val: string) => void;
	statusFilter: string;
	setStatusFilter: (val: string) => void;
	typeFilter: string;
	setTypeFilter: (val: string) => void;
	sortBy: SortKey;
	sortDir: 'asc' | 'desc';
	setSortBy: (val: SortKey) => void;
	setSortDir: (val: 'asc' | 'desc') => void;
}

/** OpenShip's status switch: quiet segmented pills, not a select. */
const FILTERS = [
	{value: 'all', label: 'All'},
	{value: 'done', label: 'Success'},
	{value: 'error', label: 'Failed'},
	{value: 'running', label: 'Building'},
	{value: 'queued', label: 'Queued'},
	{value: 'cancelled', label: 'Canceled'},
] as const;

export function DeploymentsFilters({
	searchQuery,
	setSearchQuery,
	statusFilter,
	setStatusFilter,
	typeFilter,
	setTypeFilter,
	sortBy,
	sortDir,
	setSortBy,
	setSortDir,
}: DeploymentsFiltersProps) {
	return (
		<div className="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-center">
			{/* Search — grows, shares the row with the status switch */}
			<div className="relative w-full sm:min-w-[220px] sm:flex-1">
				<Search className="pointer-events-none absolute top-1/2 start-3.5 size-4 -translate-y-1/2 text-muted-foreground" />
				<input
					type="text"
					placeholder="Search by title, description or logs..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="h-10 w-full rounded-xl border border-border/50 bg-card ps-10 pe-9 text-sm text-foreground transition-all placeholder:text-muted-foreground focus:border-primary/20 focus:ring-2 focus:ring-primary/20 focus:outline-none"
				/>
				{searchQuery && (
					<button
						type="button"
						onClick={() => setSearchQuery('')}
						className="absolute top-1/2 right-3 -translate-y-1/2 rounded-md p-0.5 text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground">
						<X className="size-3.5" />
					</button>
				)}
			</div>

			{/* Status switch — same line as the search (shrink-0 so input yields) */}
			<div className="inline-flex max-w-full shrink-0 flex-wrap items-center gap-1 rounded-xl bg-muted/35 p-1">
				{FILTERS.map(f => (
					<button
						key={f.value}
						type="button"
						onClick={() => setStatusFilter(f.value)}
						className={`inline-flex h-8 items-center rounded-lg px-3.5 text-[12px] font-medium transition-colors ${
							statusFilter === f.value
								? 'border border-border/60 bg-card text-foreground'
								: 'text-muted-foreground hover:bg-background/60 hover:text-foreground'
						}`}>
						{f.label}
					</button>
				))}
			</div>

			{/* Type + sort — compact, wrap below on tight viewports */}
			<div className="flex shrink-0 items-center gap-2">
				<Select
					value={typeFilter}
					onValueChange={val => setTypeFilter(val ?? 'all')}>
					<SelectTrigger className="!h-10 w-[130px] rounded-xl border-border/50 bg-card text-xs font-medium shadow-none">
						<SelectValue placeholder="Type" />
					</SelectTrigger>
					<SelectContent className="border- bg-popover">
						<SelectItem value="all" className="text-xs">
							All types
						</SelectItem>
						<SelectItem value="application" className="text-xs">
							Application
						</SelectItem>
						<SelectItem value="compose" className="text-xs">
							Compose
						</SelectItem>
						<SelectItem value="database" className="text-xs">
							Database
						</SelectItem>
					</SelectContent>
				</Select>
				<Select
					value={`${sortBy}-${sortDir}`}
					onValueChange={val => {
						if (val) {
							const [key, dir] = val.split('-');
							setSortBy(key as SortKey);
							setSortDir(dir as 'asc' | 'desc');
						}
					}}>
					<SelectTrigger className="!h-10 w-[145px] rounded-xl border-border/50 bg-card text-xs font-medium shadow-none">
						<SelectValue placeholder="Sort by" />
					</SelectTrigger>
					<SelectContent className="border- bg-popover">
						<SelectItem value="created_at-desc" className="text-xs">
							Newest First
						</SelectItem>
						<SelectItem value="created_at-asc" className="text-xs">
							Oldest First
						</SelectItem>
						<SelectItem value="title-asc" className="text-xs">
							Title (A-Z)
						</SelectItem>
						<SelectItem value="title-desc" className="text-xs">
							Title (Z-A)
						</SelectItem>
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
