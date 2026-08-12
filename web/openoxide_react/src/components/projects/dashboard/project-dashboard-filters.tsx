import {Search} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {cn} from '#/api/utils';

interface ProjectDashboardFiltersProps {
	projects: any[] | undefined;
	searchQuery: string;
	setSearchQuery: (val: string) => void;
	sortBy: string;
	setSortBy: (val: string) => void;
	allTags: string[];
	selectedTags: string[];
	handleTagClick: (tag: string) => void;
	setSelectedTags: (tags: string[]) => void;
}

export function ProjectDashboardFilters({
	projects,
	searchQuery,
	setSearchQuery,
	sortBy,
	setSortBy,
	allTags,
	selectedTags,
	handleTagClick,
	setSelectedTags,
}: ProjectDashboardFiltersProps) {
	if (!projects || projects.length === 0) return null;

	return (
		<div className="flex flex-col gap-4 animate-in fade-in duration-255">
			<div className="flex flex-col sm:flex-row items-center gap-3">
				{/* Search Input */}
				<div className="relative w-full sm:grow">
					<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
					<Input
						placeholder="Search projects by name or description..."
						value={searchQuery}
						onChange={e => setSearchQuery(e.target.value)}
						className="pl-9 bg-card border-border h-10 w-full text-xs shadow-2xs rounded-lg"
					/>
				</div>

				{/* Sort Select */}
				<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
					<span className="text-xs font-semibold text-muted-foreground whitespace-nowrap hidden sm:inline">
						Sort by:
					</span>
					<Select value={sortBy} onValueChange={val => setSortBy(val ?? 'newest')}>
						<SelectTrigger className="w-full sm:w-[160px] bg-card border-border !h-10 text-xs shadow-2xs font-medium rounded-lg">
							<SelectValue placeholder="Sort" />
						</SelectTrigger>
						<SelectContent className="bg-card border-border">
							<SelectItem value="newest">Newest First</SelectItem>
							<SelectItem value="oldest">Oldest First</SelectItem>
							<SelectItem value="alphabetical-asc">Name (A-Z)</SelectItem>
							<SelectItem value="alphabetical-desc">Name (Z-A)</SelectItem>
						</SelectContent>
					</Select>
				</div>
			</div>

			{/* Tag Filter Pills */}
			{allTags.length > 0 && (
				<div className="flex flex-wrap items-center gap-1.5 pt-1 animate-in fade-in duration-100">
					<span className="text-xs font-semibold text-muted-foreground mr-1">
						Tags:
					</span>
					{allTags.map(tag => {
						const isSelected = selectedTags.includes(tag);
						return (
							<button
								key={tag}
								onClick={() => handleTagClick(tag)}
								className={cn(
									'text-xs px-2.5 py-1 rounded-full font-medium transition-all border',
									isSelected
										? 'bg-primary text-primary-foreground border-primary shadow-sm'
										: 'bg-card/45 text-muted-foreground border-border/80 hover:text-foreground hover:bg-card/75',
								)}>
								{tag}
							</button>
						);
					})}

					{selectedTags.length > 0 && (
						<button
							onClick={() => setSelectedTags([])}
							className="text-xs text-primary hover:text-primary/80 font-bold ml-1 transition-colors">
							Clear Filters
						</button>
					)}
				</div>
			)}
		</div>
	);
}
