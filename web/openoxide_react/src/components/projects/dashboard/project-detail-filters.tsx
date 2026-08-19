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
		<div className="flex animate-in flex-col items-center gap-3 duration-200 fade-in sm:flex-row">
			{/* Search Input */}
			<div className="relative w-full sm:grow">
				<Search className="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/60" />
				<Input
					placeholder="Search services by name..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="h-10 w-full border-border/80 bg-card/45 pl-9 text-xs shadow-sm"
				/>
			</div>

			<div className="flex w-full shrink-0 flex-col items-center justify-end gap-3 sm:w-auto sm:flex-row">
				{/* Type Filter */}
				<div className="flex w-full shrink-0 items-center gap-2 sm:w-auto">
					<span className="hidden text-xs font-semibold whitespace-nowrap text-muted-foreground sm:inline">
						Type:
					</span>
					<Select
						value={typeFilter}
						onValueChange={val => setTypeFilter(val ?? 'all')}>
						<SelectTrigger className="!h-10 w-full border-border/80 bg-card/45 text-xs font-medium shadow-sm sm:w-[150px]">
							<span className="text-left text-foreground">
								{typeFilter === 'all'
									? 'All Types'
									: typeFilter === 'app'
										? 'Applications'
										: typeFilter === 'compose'
											? 'Compose Stacks'
											: 'Databases'}
							</span>
						</SelectTrigger>
						<SelectContent className="border border-border bg-card">
							<SelectItem value="all" className="text-xs">
								All Types
							</SelectItem>
							<SelectItem value="app" className="text-xs">
								Applications
							</SelectItem>
							<SelectItem value="compose" className="text-xs">
								Compose Stacks
							</SelectItem>
							<SelectItem value="database" className="text-xs">
								Databases
							</SelectItem>
						</SelectContent>
					</Select>
				</div>

				{/* Status Filter */}
				<div className="flex w-full shrink-0 items-center gap-2 sm:w-auto">
					<span className="hidden text-xs font-semibold whitespace-nowrap text-muted-foreground sm:inline">
						Status:
					</span>
					<Select
						value={statusFilter}
						onValueChange={val => setStatusFilter(val ?? 'all')}>
						<SelectTrigger className="!h-10 w-full border-border/80 bg-card/45 text-xs font-medium shadow-sm sm:w-[150px]">
							<span className="text-left text-foreground">
								{statusFilter === 'all'
									? 'All Status'
									: statusFilter === 'running'
										? 'Running'
										: 'Stopped'}
							</span>
						</SelectTrigger>
						<SelectContent className="border border-border bg-card">
							<SelectItem value="all" className="text-xs">
								All Status
							</SelectItem>
							<SelectItem value="running" className="text-xs">
								Running
							</SelectItem>
							<SelectItem value="stopped" className="text-xs">
								Stopped
							</SelectItem>
						</SelectContent>
					</Select>
				</div>
			</div>
		</div>
	);
}
