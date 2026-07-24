import * as React from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {Plus, Search} from 'lucide-react';
import {toast} from 'sonner';

import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import {Input} from '#/components/ui/input';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {ProjectCard} from '#/components/projects/project-card';
import {EmptyState} from '#/components/projects/empty-state';
import {CreateProjectDialog} from '#/components/projects/create-project-dialog';
import {useOrganizationStore} from '#/stores/organization-store';
import {formatApiError, cn} from '#/api/utils';

export const Route = createFileRoute('/_app/projects')({
	component: ProjectsPage,
});

// Helper to extract hashtags (e.g. #prod, #api) from text
const getTagsFromDescription = (description?: string): string[] => {
	if (!description) return [];
	const matches = description.match(/#[\w-]+/g);
	return matches ? matches.map(m => m.toLowerCase()) : [];
};

function ProjectsPage() {
	const queryClient = useQueryClient();
	const [isCreateOpen, setIsCreateOpen] = React.useState(false);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

	// Filters & Sorting state
	const [searchQuery, setSearchQuery] = React.useState('');
	const [sortBy, setSortBy] = React.useState('newest');
	const [selectedTags, setSelectedTags] = React.useState<string[]>([]);

	// Get active organization from global layout switcher store
	const activeOrg = useOrganizationStore(state => state.activeOrg);

	// Fetch Projects for the active organization
	const {data: projects, isLoading: isLoadingProjects} = $api.useQuery(
		'get',
		'/projects/organization/{organization_id}',
		{
			params: {
				path: {
					organization_id: activeOrg?.id || 0,
				},
			},
		},
		{
			enabled: activeOrg !== null,
		},
	);

	// Create Project Mutation
	const createProjectMutation = $api.useMutation('post', '/projects');

	// Delete Project Mutation
	const deleteProjectMutation = $api.useMutation('delete', '/projects/{id}');

	// Extract all unique tags present across all projects
	const allTags = React.useMemo(() => {
		if (!projects) return [];
		const tagsSet = new Set<string>();
		projects.forEach(p => {
			getTagsFromDescription(p.description).forEach(t => tagsSet.add(t));
		});
		return Array.from(tagsSet);
	}, [projects]);

	// Filter and sort projects list
	const filteredAndSortedProjects = React.useMemo(() => {
		if (!projects) return [];

		// 1. Filter by search query and tags
		let result = projects.filter(project => {
			const matchesSearch =
				project.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				(project.description || '').toLowerCase().includes(searchQuery.toLowerCase());

			const projectTags = getTagsFromDescription(project.description);
			const matchesTags =
				selectedTags.length === 0 ||
				selectedTags.every(t => projectTags.includes(t));

			return matchesSearch && matchesTags;
		});

		// 2. Sort projects
		return [...result].sort((a, b) => {
			if (sortBy === 'newest') return b.created_at - a.created_at;
			if (sortBy === 'oldest') return a.created_at - b.created_at;
			if (sortBy === 'alphabetical-asc') return a.name.localeCompare(b.name);
			if (sortBy === 'alphabetical-desc') return b.name.localeCompare(a.name);
			return 0;
		});
	}, [projects, searchQuery, sortBy, selectedTags]);

	const handleTagClick = (tag: string) => {
		setSelectedTags(prev =>
			prev.includes(tag) ? prev.filter(t => t !== tag) : [...prev, tag]
		);
	};

	const handleCreateProjectSubmit = async (
		name: string,
		description: string,
		envVar: string,
	) => {
		if (!name.trim() || !activeOrg) {
			toast.error('Project name is required');
			return;
		}

		setIsSubmitting(true);
		try {
			await createProjectMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					env_var: envVar,
					organization_id: activeOrg.id,
				},
			});

			toast.success('Project created successfully!');
			setIsCreateOpen(false);
			// Refresh projects list
			queryClient.invalidateQueries({
				queryKey: [
					'get',
					'/projects/organization/{organization_id}',
					{params: {path: {organization_id: activeOrg.id}}},
				],
			});
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProject = async (projectId: number) => {
		if (!confirm('Are you sure you want to delete this project?')) return;
		if (!activeOrg) return;

		try {
			await deleteProjectMutation.mutateAsync({
				params: {
					path: {
						id: projectId,
					},
				},
			});

			toast.success('Project deleted successfully');
			queryClient.invalidateQueries({
				queryKey: [
					'get',
					'/projects/organization/{organization_id}',
					{params: {path: {organization_id: activeOrg.id}}},
				],
			});
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return (
		<div className="flex flex-col gap-8 w-full max-w-7xl mx-auto">
			{/* Page Header */}
			<div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/40 pb-6">
				<div>
					<h1 className="text-3xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/75 bg-clip-text">
						Projects
					</h1>
					<p className="text-muted-foreground mt-1.5 text-sm">
						Organize, manage and deploy applications inside{' '}
						<span className="font-semibold text-foreground">
							{activeOrg?.name || 'your environment'}
						</span>
						.
					</p>
				</div>

				<div className="flex items-center gap-3">
					<Button
						onClick={() => setIsCreateOpen(true)}
						disabled={!activeOrg}
						className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold h-10 px-4 rounded-lg flex items-center gap-2 shadow-lg shadow-primary/10">
						<Plus className="size-4" />
						Create Project
					</Button>
				</div>
			</div>

			{/* Filters & Sorting Control Bar */}
			{projects && projects.length > 0 && (
				<div className="flex flex-col gap-4 animate-in fade-in duration-255">
					<div className="flex flex-col sm:flex-row items-center gap-3">
						{/* Search Input */}
						<div className="relative w-full sm:grow">
							<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground/60" />
							<Input
								placeholder="Search projects by name or description..."
								value={searchQuery}
								onChange={e => setSearchQuery(e.target.value)}
								className="pl-9 bg-card/45 border-border/80 h-10 w-full"
							/>
						</div>

						{/* Sort Select */}
						<div className="flex items-center gap-2 shrink-0 w-full sm:w-auto">
							<span className="text-xs font-semibold text-muted-foreground whitespace-nowrap hidden sm:inline">
								Sort by:
							</span>
							<Select value={sortBy} onValueChange={setSortBy}>
								<SelectTrigger className="w-full sm:w-[160px] bg-card/45 border-border/80 h-10 shadow-sm font-medium">
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
			)}

			{/* Main Grid View */}
			{isLoadingProjects ? (
				<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
					{[1, 2, 3, 4].map(i => (
						<Card
							key={i}
							className="h-[160px] border-border bg-card/45 animate-pulse"
						/>
					))}
				</div>
			) : filteredAndSortedProjects.length > 0 ? (
				<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 animate-in fade-in duration-200">
					{filteredAndSortedProjects.map(project => (
						<ProjectCard
							key={project.id}
							project={project}
							onDelete={handleDeleteProject}
						/>
					))}
				</div>
			) : projects && projects.length > 0 ? (
				/* Filter Empty State */
				<div className="flex flex-col items-center justify-center border border-dashed border-border/80 rounded-2xl p-12 text-center bg-card/10 backdrop-blur-[2px]">
					<h3 className="text-md font-bold text-foreground">No matching projects</h3>
					<p className="text-muted-foreground mt-1 text-xs">
						Try adjusting your search keywords or clearing selected tags.
					</p>
					<Button
						variant="ghost"
						onClick={() => {
							setSearchQuery('');
							setSelectedTags([]);
						}}
						className="mt-4 text-xs font-semibold text-primary">
						Clear All Filters
					</Button>
				</div>
			) : (
				<EmptyState
					onCreateClick={() => setIsCreateOpen(true)}
					disabled={!activeOrg}
				/>
			)}

			{/* Create Project Modal */}
			<CreateProjectDialog
				isOpen={isCreateOpen}
				onOpenChange={setIsCreateOpen}
				onSubmit={handleCreateProjectSubmit}
				isSubmitting={isSubmitting}
			/>
		</div>
	);
}
