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
		<div className="flex flex-col sm:flex-row items-center gap-3 animate-in fade-in duration-200">
			{/* Search Input */}
			<div className="relative w-full sm:grow">
				<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
				<Input
					placeholder="Search services by name..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="pl-9 bg-card/45 border-border/80 h-10 w-full text-xs shadow-sm"
				/>
			</div>

			<div className="flex flex-col sm:flex-row items-center gap-3 w-full sm:w-auto shrink-0 justify-end">
				{/* Type Filter */}
				<div className="flex items-center gap-2 w-full sm:w-auto shrink-0">
					<span className="text-xs font-semibold text-muted-foreground whitespace-nowrap hidden sm:inline">Type:</span>
					<Select value={typeFilter} onValueChange={val => setTypeFilter(val ?? 'all')}>
						<SelectTrigger className="w-full sm:w-[150px] bg-card/45 border-border/80 h-10 shadow-sm text-xs font-medium">
							<span className="text-foreground text-left">
								{typeFilter === 'all' ? 'All Types' : typeFilter === 'app' ? 'Applications' : typeFilter === 'compose' ? 'Compose Stacks' : 'Databases'}
							</span>
						</SelectTrigger>
						<SelectContent className="bg-card border border-border">
							<SelectItem value="all" className="text-xs">All Types</SelectItem>
							<SelectItem value="app" className="text-xs">Applications</SelectItem>
							<SelectItem value="compose" className="text-xs">Compose Stacks</SelectItem>
							<SelectItem value="database" className="text-xs">Databases</SelectItem>
						</SelectContent>
					</Select>
				</div>

				{/* Status Filter */}
				<div className="flex items-center gap-2 w-full sm:w-auto shrink-0">
					<span className="text-xs font-semibold text-muted-foreground whitespace-nowrap hidden sm:inline">Status:</span>
					<Select value={statusFilter} onValueChange={val => setStatusFilter(val ?? 'all')}>
						<SelectTrigger className="w-full sm:w-[150px] bg-card/45 border-border/80 h-10 shadow-sm text-xs font-medium">
							<span className="text-foreground text-left">
								{statusFilter === 'all' ? 'All Status' : statusFilter === 'running' ? 'Running' : 'Stopped'}
							</span>
						</SelectTrigger>
						<SelectContent className="bg-card border border-border">
							<SelectItem value="all" className="text-xs">All Status</SelectItem>
							<SelectItem value="running" className="text-xs">Running</SelectItem>
							<SelectItem value="stopped" className="text-xs">Stopped</SelectItem>
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
