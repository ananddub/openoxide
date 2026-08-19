import React from 'react';
import {Search, ArrowUpDown} from 'lucide-react';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {TagSelector} from '#/components/shared/tag-selector';

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
	selectedTags,
	setSelectedTags,
}: ProjectDashboardFiltersProps) {
	if (!projects || projects.length === 0) return null;

	const getSortLabel = (val: string) => {
		switch (val) {
			case 'alphabetical-asc':
			case 'name-asc':
				return 'Sort: Name (A-Z)';
			case 'alphabetical-desc':
			case 'name-desc':
				return 'Sort: Name (Z-A)';
			case 'oldest':
			case 'createdAt-asc':
				return 'Sort: Oldest First';
			case 'newest':
			case 'createdAt-desc':
			default:
				return 'Sort: Newest First';
		}
	};

	return (
		<div className="flex w-full animate-in flex-col items-center gap-3 duration-200 fade-in sm:flex-row">
			{/* Search Input */}
			<div className="relative w-full sm:grow">
				<Search className="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground/60" />
				<Input
					placeholder="Filter projects..."
					value={searchQuery}
					onChange={e => setSearchQuery(e.target.value)}
					className="h-10 w-full rounded-lg border-border/80 bg-card pr-4 pl-9 text-xs shadow-2xs"
				/>
			</div>

			{/* Dokploy TagFilter Dropdown */}
			<div className="w-full shrink-0 sm:w-[150px]">
				<TagSelector
					selectedTags={selectedTags}
					onTagsChange={setSelectedTags}
					placeholder="Tags"
					variant="filter"
				/>
			</div>

			{/* Sort Select with ArrowUpDown Icon */}
			<div className="flex w-full shrink-0 items-center gap-2 sm:w-auto">
				<Select
					value={sortBy}
					onValueChange={val => val && setSortBy(val)}>
					<SelectTrigger className="h-10 w-full rounded-lg border-border/80 bg-card text-xs font-semibold shadow-2xs sm:w-[185px]">
						<div className="flex min-w-0 items-center gap-2">
							<ArrowUpDown className="size-3.5 shrink-0 text-muted-foreground" />
							<SelectValue>{getSortLabel(sortBy)}</SelectValue>
						</div>
					</SelectTrigger>
					<SelectContent className="border-border bg-card">
						<SelectItem value="newest">Sort: Newest First</SelectItem>
						<SelectItem value="oldest">Sort: Oldest First</SelectItem>
						<SelectItem value="alphabetical-asc">
							Sort: Name (A-Z)
						</SelectItem>
						<SelectItem value="alphabetical-desc">
							Sort: Name (Z-A)
						</SelectItem>
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
