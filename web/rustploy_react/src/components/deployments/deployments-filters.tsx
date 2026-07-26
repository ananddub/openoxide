import {Search, X} from 'lucide-react';
import {Input} from '#/components/ui/input';
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
		<div className="flex flex-col gap-4 animate-in fade-in duration-200">
			<div className="flex flex-col sm:flex-row items-center gap-3">
				{/* Search */}
				<div className="relative w-full sm:grow">
					<Search className="absolute left-3.5 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/70" />
					<Input
						placeholder="Search by title, description or logs..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="pl-9.5 pr-9 bg-card/60 border-border/80 h-10 w-full text-xs rounded-lg focus-visible:ring-1 focus-visible:ring-primary shadow-sm"
					/>
					{searchQuery && (
						<button
							type="button"
							onClick={() => setSearchQuery('')}
							className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground p-0.5 rounded-md hover:bg-muted/50 transition-colors">
							<X className="size-3.5" />
						</button>
					)}
				</div>

				{/* Status Select */}
				<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
					<Select value={statusFilter} onValueChange={val => setStatusFilter(val ?? 'all')}>
						<SelectTrigger className="w-full sm:w-[150px] bg-card/60 border-border/80 !h-10 text-xs shadow-sm font-medium rounded-lg">
							<SelectValue placeholder="Status" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="all" className="text-xs">All statuses</SelectItem>
							<SelectItem value="running" className="text-xs">Running</SelectItem>
							<SelectItem value="queued" className="text-xs">Queued</SelectItem>
							<SelectItem value="done" className="text-xs">Completed</SelectItem>
							<SelectItem value="error" className="text-xs">Failed</SelectItem>
						</SelectContent>
					</Select>
				</div>

				{/* Type Select */}
				<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
					<Select value={typeFilter} onValueChange={val => setTypeFilter(val ?? 'all')}>
						<SelectTrigger className="w-full sm:w-[150px] bg-card/60 border-border/80 !h-10 text-xs shadow-sm font-medium rounded-lg">
							<SelectValue placeholder="Type" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="all" className="text-xs">All types</SelectItem>
							<SelectItem value="application" className="text-xs">Application</SelectItem>
							<SelectItem value="compose" className="text-xs">Compose</SelectItem>
							<SelectItem value="database" className="text-xs">Database</SelectItem>
						</SelectContent>
					</Select>
				</div>

				{/* Sort Select */}
				<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
					<Select
						value={`${sortBy}-${sortDir}`}
						onValueChange={val => {
							if (val) {
								const [key, dir] = val.split('-');
								setSortBy(key as SortKey);
								setSortDir(dir as 'asc' | 'desc');
							}
						}}>
						<SelectTrigger className="w-full sm:w-[160px] bg-card/60 border-border/80 !h-10 text-xs shadow-sm font-medium rounded-lg">
							<SelectValue placeholder="Sort by" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="created_at-desc" className="text-xs">Newest First</SelectItem>
							<SelectItem value="created_at-asc" className="text-xs">Oldest First</SelectItem>
							<SelectItem value="title-asc" className="text-xs">Title (A-Z)</SelectItem>
							<SelectItem value="title-desc" className="text-xs">Title (Z-A)</SelectItem>
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
