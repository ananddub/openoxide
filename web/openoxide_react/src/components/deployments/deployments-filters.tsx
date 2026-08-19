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
		<div className="flex animate-in flex-col gap-4 duration-200 fade-in">
			<div className="flex flex-col items-center gap-3 sm:flex-row">
				{/* Search */}
				<div className="relative w-full sm:grow">
					<Search className="absolute top-1/2 left-3.5 size-4 -translate-y-1/2 text-muted-foreground/70" />
					<Input
						placeholder="Search by title, description or logs..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="h-10 w-full rounded-lg border-border bg-card pr-9 pl-9.5 text-xs shadow-2xs focus-visible:ring-1 focus-visible:ring-primary"
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

				{/* Status Select */}
				<div className="flex w-full shrink-0 items-center gap-2 sm:w-auto">
					<Select
						value={statusFilter}
						onValueChange={val => setStatusFilter(val ?? 'all')}>
						<SelectTrigger className="!h-10 w-full rounded-lg border-border bg-card text-xs font-medium shadow-2xs sm:w-[150px]">
							<SelectValue placeholder="Status" />
						</SelectTrigger>
						<SelectContent className="border-border bg-popover">
							<SelectItem value="all" className="text-xs">
								All statuses
							</SelectItem>
							<SelectItem value="running" className="text-xs">
								Running
							</SelectItem>
							<SelectItem value="queued" className="text-xs">
								Queued
							</SelectItem>
							<SelectItem value="done" className="text-xs">
								Completed
							</SelectItem>
							<SelectItem value="error" className="text-xs">
								Failed
							</SelectItem>
						</SelectContent>
					</Select>
				</div>

				{/* Type Select */}
				<div className="flex w-full shrink-0 items-center gap-2 sm:w-auto">
					<Select
						value={typeFilter}
						onValueChange={val => setTypeFilter(val ?? 'all')}>
						<SelectTrigger className="!h-10 w-full rounded-lg border-border bg-card text-xs font-medium shadow-2xs sm:w-[150px]">
							<SelectValue placeholder="Type" />
						</SelectTrigger>
						<SelectContent className="border-border bg-popover">
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
				</div>

				{/* Sort Select */}
				<div className="flex w-full shrink-0 items-center gap-2 sm:w-auto">
					<Select
						value={`${sortBy}-${sortDir}`}
						onValueChange={val => {
							if (val) {
								const [key, dir] = val.split('-');
								setSortBy(key as SortKey);
								setSortDir(dir as 'asc' | 'desc');
							}
						}}>
						<SelectTrigger className="!h-10 w-full rounded-lg border-border bg-card text-xs font-medium shadow-2xs sm:w-[160px]">
							<SelectValue placeholder="Sort by" />
						</SelectTrigger>
						<SelectContent className="border-border bg-popover">
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
		</div>
	);
}
