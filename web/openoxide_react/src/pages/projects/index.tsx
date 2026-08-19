import {createFileRoute} from '@tanstack/react-router';
import {Plus, FolderInput, Search} from 'lucide-react';
import {useState} from 'react';

import {Button} from '#/components/ui/button';
import {ProjectCard} from '#/components/projects/dashboard/project-card';
import {EmptyState} from '#/components/projects/dashboard/empty-state';
import {HandleProjectDialog} from '#/components/projects/dashboard/handle-project-dialog';
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
		activeOrg,
		handleTagClick,
		handleDeleteProject,
	} = useProjectsList();

	const confirmDelete = async () => {
		if (deletingId !== null) {
			await handleDeleteProject(deletingId);
			setDeletingId(null);
		}
	};

	return (
		<div className="flex w-full max-w-full animate-in flex-col gap-6 duration-200 fade-in">
			{/* Page Header Row */}
			<div className="flex w-full flex-wrap items-center justify-between gap-4 border-b border-border/40 pb-6">
				<div className="flex flex-1 flex-col gap-1">
					<div className="flex items-center gap-2.5">
						<FolderInput className="size-6 shrink-0 text-muted-foreground" />
						<h1 className="text-xl font-bold tracking-tight text-foreground">
							Projects
						</h1>
					</div>
					<p className="text-xs text-muted-foreground">
						Create and manage your projects
					</p>
				</div>

				<Button
					onClick={() => setIsCreateOpen(true)}
					disabled={!activeOrg}
					className="h-9 cursor-pointer gap-2 bg-primary px-4 text-xs font-bold text-primary-foreground shadow-xs transition-all hover:bg-primary/90 active:scale-95">
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

			{/* Content Cards Grid */}
			{isLoadingProjects ? (
				<div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
					{[1, 2, 3].map(i => (
						<div
							key={i}
							className="h-44 max-w-[340px] animate-pulse rounded-xl border border-border/60 bg-card/60"
						/>
					))}
				</div>
			) : filteredAndSortedProjects.length > 0 ? (
				<div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
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
				<div className="my-auto flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/80 bg-card/10 p-12 text-center backdrop-blur-xs">
					<Search className="mb-2 size-8 text-muted-foreground/40" />
					<h3 className="text-md font-bold text-foreground">
						No matching projects found
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
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
			<HandleProjectDialog
				isOpen={isCreateOpen}
				onOpenChange={setIsCreateOpen}
				activeOrgId={activeOrg?.id}
			/>

			{/* Delete Confirmation Alert Dialog */}
			<AlertDialog
				open={deletingId !== null}
				onOpenChange={open => !open && setDeletingId(null)}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Delete Project</AlertDialogTitle>
						<AlertDialogDescription>
							Are you sure you want to delete this project? All associated
							applications, databases, and environments will be permanently
							removed.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel onClick={() => setDeletingId(null)}>
							Cancel
						</AlertDialogCancel>
						<AlertDialogAction
							className="text-destructive-foreground bg-destructive font-bold hover:bg-destructive/90"
							onClick={confirmDelete}>
							Delete Project
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
