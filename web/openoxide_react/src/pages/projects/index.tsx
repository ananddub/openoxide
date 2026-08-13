import {createFileRoute} from '@tanstack/react-router';
import {Plus} from 'lucide-react';
import {useState} from 'react';

import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import {ProjectCard} from '#/components/projects/dashboard/project-card';
import {EmptyState} from '#/components/projects/dashboard/empty-state';
import {CreateProjectDialog} from '#/components/projects/dashboard/create-project-dialog';
import {ProjectDashboardFilters} from '#/components/projects/dashboard/project-dashboard-filters';
import {useProjectsList} from '#/hooks/projects/use-projects-list';
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from '#/components/ui/alert-dialog';

export const Route = createFileRoute('/_app/projects')({
	component: ProjectsPage,
});

function ProjectsPage() {
	const [deletingId, setDeletingId] = useState<number | null>(null);

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

	const confirmDelete = async () => {
		if (deletingId !== null) {
			await handleDeleteProject(deletingId);
			setDeletingId(null);
		}
	};

	return (
		<div className="flex flex-col gap-6 w-full">
			{/* Page Header */}
			<div className="flex flex-col md:flex-row md:items-center justify-between gap-4 border-b border-border/40 pb-6">
				<div>
					<h1 className="text-2xl font-bold tracking-tight text-foreground">Projects</h1>
					<p className="text-muted-foreground text-xs font-medium mt-1">
						Manage your deployment environments, applications, and infrastructure stacks
					</p>
				</div>
				<Button
					onClick={() => setIsCreateOpen(true)}
					disabled={!activeOrg}
					className="h-10 text-xs font-semibold px-4 gap-2 bg-primary hover:bg-primary/90 text-primary-foreground shadow-md transition-all active:scale-95 cursor-pointer">
					<Plus className="size-4" />
					<span>New Project</span>
				</Button>
			</div>

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

			{/* Content Area */}
			{isLoadingProjects ? (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
					{[1, 2, 3].map(i => (
						<Card key={i} className="h-44 animate-pulse bg-muted/40 border-border/40" />
					))}
				</div>
			) : filteredAndSortedProjects.length > 0 ? (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
					{filteredAndSortedProjects.map(project => (
						<ProjectCard
							key={String(project.id)}
							project={project as unknown as any}
							onDelete={id => setDeletingId(id)}
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

			{/* Delete Confirmation Alert Dialog */}
			<AlertDialog open={deletingId !== null} onOpenChange={open => !open && setDeletingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete Project</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete this project? All associated applications, databases, and environments will be permanently removed.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeletingId(null)}>Cancel</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
							onClick={confirmDelete}
						>
							Delete Project
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
