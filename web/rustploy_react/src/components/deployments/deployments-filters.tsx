import {Search} from 'lucide-react';
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
					<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
					<Input
						placeholder="Search by title, description or logs..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="pl-9 bg-card/45 border-border/80 h-10 w-full"
					/>
				</div>

				{/* Status Select */}
				<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
					<Select value={statusFilter} onValueChange={val => setStatusFilter(val ?? 'all')}>
						<SelectTrigger className="w-full sm:w-[150px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
							<SelectValue placeholder="Status" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="all">All statuses</SelectItem>
							<SelectItem value="running">Running</SelectItem>
							<SelectItem value="queued">Queued</SelectItem>
							<SelectItem value="done">Completed</SelectItem>
							<SelectItem value="error">Failed</SelectItem>
						</SelectContent>
					</Select>
				</div>

				{/* Type Select */}
				<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
					<Select value={typeFilter} onValueChange={val => setTypeFilter(val ?? 'all')}>
						<SelectTrigger className="w-full sm:w-[150px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
							<SelectValue placeholder="Type" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="all">All types</SelectItem>
							<SelectItem value="application">Application</SelectItem>
							<SelectItem value="compose">Compose</SelectItem>
							<SelectItem value="database">Database</SelectItem>
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
						<SelectTrigger className="w-full sm:w-[160px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
							<SelectValue placeholder="Sort by" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="created_at-desc">Newest First</SelectItem>
							<SelectItem value="created_at-asc">Oldest First</SelectItem>
							<SelectItem value="title-asc">Title (A-Z)</SelectItem>
							<SelectItem value="title-desc">Title (Z-A)</SelectItem>
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
