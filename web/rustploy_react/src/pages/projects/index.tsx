import {createFileRoute} from '@tanstack/react-router';
import {Plus} from 'lucide-react';

import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import {ProjectCard} from '#/components/projects/dashboard/project-card';
import {EmptyState} from '#/components/projects/dashboard/empty-state';
import {CreateProjectDialog} from '#/components/projects/dashboard/create-project-dialog';
import {ProjectDashboardFilters} from '#/components/projects/dashboard/project-dashboard-filters';
import {useProjectsList} from '#/hooks/projects/use-projects-list';

export const Route = createFileRoute('/_app/projects')({
	component: ProjectsPage,
});

function ProjectsPage() {
	const {
		projects,
		isLoadingProjects,
		filteredAndSortedProjects,
		allTags,
		searchQuery,
		setSearchQuery,
		sortBy,
		setSortBy,
		selectedTags,
		setSelectedTags,
		isCreateOpen,
		setIsCreateOpen,
		isSubmitting,
		activeOrg,
		handleTagClick,
		handleCreateProjectSubmit,
		handleDeleteProject,
	} = useProjectsList();

	return (
		<div className="flex flex-col gap-6 w-full">
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
			<ProjectDashboardFilters
				projects={projects}
				searchQuery={searchQuery}
				setSearchQuery={setSearchQuery}
				sortBy={sortBy}
				setSortBy={setSortBy}
				allTags={allTags}
				selectedTags={selectedTags}
				handleTagClick={handleTagClick}
				setSelectedTags={setSelectedTags}
			/>

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
