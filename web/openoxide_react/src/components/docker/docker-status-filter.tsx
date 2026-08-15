import { Search, Filter } from 'lucide-react';
import { Input } from '#/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '#/components/ui/select';

interface DockerStatusFilterProps {
	searchQuery: string;
	onSearchChange: (query: string) => void;
	statusFilter: 'all' | 'running' | 'stopped';
	onStatusFilterChange: (status: 'all' | 'running' | 'stopped') => void;
	totalCount: number;
	runningCount: number;
	stoppedCount: number;
}

export function DockerStatusFilter({
	searchQuery,
	onSearchChange,
	statusFilter,
	onStatusFilterChange,
	totalCount,
	runningCount,
	stoppedCount,
}: DockerStatusFilterProps) {
	return (
		<div className="flex flex-col sm:flex-row items-center justify-between gap-3 shrink-0">
			{/* Search Input */}
			<div className="relative max-w-sm w-full">
				<Search className="size-3.5 text-muted-foreground absolute left-3 top-3" />
				<Input
					value={searchQuery}
					onChange={(e) => onSearchChange(e.target.value)}
					placeholder="Search containers by name, id, image..."
					className="h-9 text-xs font-mono pl-9 bg-card border-border/60 shadow-2xs"
				/>
			</div>

			{/* Status Filter Dropdown */}
			<div className="flex items-center gap-2 w-full sm:w-auto">
				<Select value={statusFilter} onValueChange={(v) => v && onStatusFilterChange(v as 'all' | 'running' | 'stopped')}>
					<SelectTrigger className="w-[180px] h-9 text-xs font-medium bg-card border-border/60 gap-2 shrink-0 shadow-2xs">
						<Filter className="size-3.5 text-muted-foreground shrink-0" />
						<SelectValue placeholder="All Status" />
					</SelectTrigger>
					<SelectContent className="bg-card border-border text-xs w-[200px] p-1 shadow-md">
						<SelectItem value="all" className="text-xs font-medium cursor-pointer">
							All Status <span className="text-[10px] text-muted-foreground font-mono">({totalCount})</span>
						</SelectItem>
						<SelectItem value="running" className="text-xs font-medium cursor-pointer text-emerald-500">
							Running Only <span className="text-[10px] text-emerald-500/80 font-mono">({runningCount})</span>
						</SelectItem>
						<SelectItem value="stopped" className="text-xs font-medium cursor-pointer text-rose-500">
							Stopped / Exited <span className="text-[10px] text-rose-500/80 font-mono">({stoppedCount})</span>
						</SelectItem>
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
