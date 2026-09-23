import {Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '#/components/ui/select';

interface ProjectDetailFiltersProps {
	totalServices: number;
	searchQuery: string;
	setSearchQuery: (val: string) => void;
	typeFilter: string;
	setTypeFilter: (val: string) => void;
	statusFilter: string;
	setStatusFilter: (val: string) => void;
}

export function ProjectDetailFilters({
	totalServices,
	searchQuery,
	setSearchQuery,
	typeFilter,
	setTypeFilter,
	statusFilter,
	setStatusFilter,
}: ProjectDetailFiltersProps) {
	if (totalServices === 0) return null;

	return (
		<div className="flex min-w-0 flex-1 animate-in flex-col items-stretch gap-3 duration-200 fade-in sm:flex-row sm:items-center">
			{/* Search — Geist command-style, matches the projects toolbar */}
			<div className="relative w-full sm:grow">
				<Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/70" />
				<Input
					placeholder="Search services..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="h-9 rounded-lg pr-4 pl-9 text-sm transition-colors duration-150 hover:border-foreground/25"
				/>
			</div>

			<div className="flex shrink-0 items-center gap-2">
				{/* Type Filter */}
				<Select
					value={typeFilter}
					onValueChange={val => setTypeFilter(val ?? 'all')}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card sm:w-[150px]">
						<span className="text-left text-sm text-foreground">
							{typeFilter === 'all'
								? 'All Types'
								: typeFilter === 'app'
									? 'Applications'
									: typeFilter === 'compose'
										? 'Compose Stacks'
										: 'Databases'}
						</span>
					</SelectTrigger>
					<SelectContent className="bg-card">
						<SelectItem value="all">All Types</SelectItem>
						<SelectItem value="app">Applications</SelectItem>
						<SelectItem value="compose">Compose Stacks</SelectItem>
						<SelectItem value="database">Databases</SelectItem>
					</SelectContent>
				</Select>

				{/* Status Filter */}
				<Select
					value={statusFilter}
					onValueChange={val => setStatusFilter(val ?? 'all')}>
					<SelectTrigger className="h-9 w-auto rounded-lg bg-card sm:w-[150px]">
						<span className="text-left text-sm text-foreground">
							{statusFilter === 'all'
								? 'All Status'
								: statusFilter === 'running'
									? 'Running'
									: 'Stopped'}
						</span>
					</SelectTrigger>
					<SelectContent className="bg-card">
						<SelectItem value="all">All Status</SelectItem>
						<SelectItem value="running">Running</SelectItem>
						<SelectItem value="stopped">Stopped</SelectItem>
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
