import { createFileRoute } from '@tanstack/react-router';
import { Plus, FolderInput, Search } from 'lucide-react';
import { useState } from 'react';

import { Button } from '#/components/ui/button';
import { ProjectCard } from '#/components/projects/dashboard/project-card';
import { EmptyState } from '#/components/projects/dashboard/empty-state';
import { CreateProjectDialog } from '#/components/projects/dashboard/create-project-dialog';
import { ProjectDashboardFilters } from '#/components/projects/dashboard/project-dashboard-filters';
import { useProjectsList } from '#/hooks/projects/use-projects-list';
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
		<div className="w-full max-w-full flex flex-col gap-6 animate-in fade-in duration-200">
			{/* Page Header Row */}
			<div className="flex justify-between gap-4 w-full items-center flex-wrap pb-6 border-b border-border/40">
				<div className="flex-1 flex flex-col gap-1">
					<div className="flex items-center gap-2.5">
						<FolderInput className="size-6 text-muted-foreground shrink-0" />
						<h1 className="text-xl font-bold tracking-tight text-foreground">Projects</h1>
					</div>
					<p className="text-xs text-muted-foreground">
						Create and manage your projects
					</p>
				</div>

				<Button
					onClick={() => setIsCreateOpen(true)}
					disabled={!activeOrg}
					className="h-9 text-xs font-bold px-4 gap-2 bg-primary hover:bg-primary/90 text-primary-foreground shadow-xs transition-all active:scale-95 cursor-pointer"
				>
					<Plus className="size-4" />
					<span>Create Project</span>
				</Button>
			</div>

			{/* Filter Toolbar */}
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

			{/* Content Cards Grid (Compact dokploy layout) */}
			{isLoadingProjects ? (
				<div className="w-full grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
					{[1, 2, 3].map((i) => (
						<div key={i} className="h-44 max-w-[340px] animate-pulse bg-card/60 border border-border/60 rounded-xl" />
					))}
				</div>
			) : filteredAndSortedProjects.length > 0 ? (
				<div className="w-full grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
					{filteredAndSortedProjects.map((project) => (
						<ProjectCard
							key={String(project.id)}
							project={project as unknown as any}
							onDelete={(id) => setDeletingId(id)}
						/>
					))}
				</div>
			) : projects && projects.length > 0 ? (
				/* Filter Empty State */
				<div className="flex flex-col items-center justify-center border border-dashed border-border/80 rounded-2xl p-12 text-center bg-card/10 backdrop-blur-xs my-auto">
					<Search className="size-8 text-muted-foreground/40 mb-2" />
					<h3 className="text-md font-bold text-foreground">No matching projects found</h3>
					<p className="text-muted-foreground mt-1 text-xs">
						Try adjusting your search keywords or clearing selected tags.
					</p>
					<Button
						variant="ghost"
						onClick={() => {
							setSearchQuery('');
							setSelectedTags([]);
						}}
						className="mt-4 text-xs font-semibold text-primary"
					>
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
			<AlertDialog open={deletingId !== null} onOpenChange={(open) => !open && setDeletingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete Project</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete this project? All associated applications, databases, and environments will be permanently reduced.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeletingId(null)}>Cancel</AlertDialogCancel>
						<AlertDialogAction
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90 font-bold"
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
