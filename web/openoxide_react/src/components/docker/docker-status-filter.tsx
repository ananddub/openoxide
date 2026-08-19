import {Search, Filter} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
} from '#/components/ui/select';

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
	const getDisplayLabel = (filter: 'all' | 'running' | 'stopped') => {
		switch (filter) {
			case 'all':
				return 'All Status';
			case 'running':
				return 'Running Only';
			case 'stopped':
				return 'Stopped / Exited';
		}
	};

	return (
		<div className="flex shrink-0 flex-col items-center justify-between gap-3 sm:flex-row">
			{/* Search Input */}
			<div className="relative w-full max-w-sm">
				<Search className="absolute top-3 left-3 size-3.5 text-muted-foreground" />
				<Input
					value={searchQuery}
					onChange={e => onSearchChange(e.target.value)}
					placeholder="Search containers by name, id, image..."
					className="h-9 border-border/60 bg-card pl-9 font-mono text-xs shadow-2xs"
				/>
			</div>

			{/* Status Filter Dropdown */}
			<div className="flex w-full items-center gap-2 sm:w-auto">
				<Select
					value={statusFilter}
					onValueChange={v =>
						v && onStatusFilterChange(v as 'all' | 'running' | 'stopped')
					}>
					<SelectTrigger className="h-9 w-[180px] shrink-0 gap-2 border-border/60 bg-card text-xs font-medium shadow-2xs">
						<Filter className="size-3.5 shrink-0 text-muted-foreground" />
						<span className="truncate">
							{getDisplayLabel(statusFilter)}
						</span>
					</SelectTrigger>
					<SelectContent className="w-[200px] border-border bg-card p-1 text-xs shadow-md">
						<SelectItem
							value="all"
							className="cursor-pointer text-xs font-medium">
							All Status{' '}
							<span className="font-mono text-[10px] text-muted-foreground">
								({totalCount})
							</span>
						</SelectItem>
						<SelectItem
							value="running"
							className="cursor-pointer text-xs font-medium text-emerald-500">
							Running Only{' '}
							<span className="font-mono text-[10px] text-emerald-500/80">
								({runningCount})
							</span>
						</SelectItem>
						<SelectItem
							value="stopped"
							className="cursor-pointer text-xs font-medium text-rose-500">
							Stopped / Exited{' '}
							<span className="font-mono text-[10px] text-rose-500/80">
								({stoppedCount})
							</span>
						</SelectItem>
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
