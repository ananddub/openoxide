import React from 'react';
import { Search } from 'lucide-react';
import { Input } from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import { TagSelector } from '#/components/shared/tag-selector';

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
		<div className="flex flex-col sm:flex-row items-center gap-3 w-full animate-in fade-in duration-200">
			{/* Search Input */}
			<div className="relative w-full sm:grow">
				<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
				<Input
					placeholder="Filter projects..."
					value={searchQuery}
					onChange={(e) => setSearchQuery(e.target.value)}
					className="pl-9 pr-4 bg-card border-border/80 h-10 w-full text-xs shadow-2xs rounded-lg"
				/>
			</div>

			{/* Dokploy TagFilter Dropdown with Selected Counter Badge */}
			<div className="w-full sm:w-[150px] shrink-0">
				<TagSelector
					selectedTags={selectedTags}
					onTagsChange={setSelectedTags}
					placeholder="Tags"
					variant="filter"
				/>
			</div>

			{/* Sort Select (Dokploy Style) */}
			<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
				<Select value={sortBy} onValueChange={(val) => val && setSortBy(val)}>
					<SelectTrigger className="w-full sm:w-[170px] bg-card border-border/80 h-10 text-xs font-semibold rounded-lg shadow-2xs">
						<SelectValue>{getSortLabel(sortBy)}</SelectValue>
					</SelectTrigger>
					<SelectContent className="bg-card border-border">
						<SelectItem value="newest">Sort: Newest First</SelectItem>
						<SelectItem value="oldest">Sort: Oldest First</SelectItem>
						<SelectItem value="alphabetical-asc">Sort: Name (A-Z)</SelectItem>
						<SelectItem value="alphabetical-desc">Sort: Name (Z-A)</SelectItem>
					</SelectContent>
				</Select>
			</div>
		</div>
	);
}
