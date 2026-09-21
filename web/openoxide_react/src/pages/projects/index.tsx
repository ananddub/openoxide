import {createFileRoute, useNavigate, useSearch} from '@tanstack/react-router';
import {Plus, Search} from 'lucide-react';
import {useEffect, useState} from 'react';

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
	validateSearch: (search: Record<string, unknown>): {create?: number} => ({
		create: search.create === '1' || search.create === 1 ? 1 : undefined,
	}),
});

function ProjectsPage() {
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const navigate = useNavigate();
	const {create} = useSearch({from: '/_app/projects'});

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

	// The rail's "New Project" CTA links here with ?create=1; open the
	// dialog and drop the param so a refresh doesn't re-trigger it.
	useEffect(() => {
		if (create !== 1) return;
		setIsCreateOpen(true);
		navigate({to: '/projects', replace: true});
	}, [create, setIsCreateOpen, navigate]);

	const confirmDelete = async () => {
		if (deletingId !== null) {
			await handleDeleteProject(deletingId);
			setDeletingId(null);
		}
	};

	return (
		<div className="mx-auto flex w-full max-w-[1600px] animate-in flex-col gap-6 px-4 py-6 duration-200 fade-in sm:px-6 lg:px-8">
			{/* Page Header Row */}
			<div className="flex w-full flex-wrap items-start justify-between gap-4 sm:flex-row sm:items-center">
				<div className="flex flex-1 flex-col">
					<h1
						className="text-2xl font-medium text-foreground/80"
						style={{letterSpacing: '-0.2px'}}>
						Projects
					</h1>
					<p className="mt-1 text-sm text-muted-foreground/70">
						Create and manage your projects
					</p>
				</div>

				<Button
					onClick={() => setIsCreateOpen(true)}
					disabled={!activeOrg}
					className="h-auto gap-2 bg-primary px-5 py-2.5 font-medium transition-all hover:-translate-y-0.5 hover:bg-primary/90 hover:shadow-lg hover:shadow-primary/25">
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
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 2xl:grid-cols-3">
					{[1, 2, 3].map(i => (
						<div
							key={i}
							className="h-44 animate-pulse rounded-2xl bg-card/60"
						/>
					))}
				</div>
			) : filteredAndSortedProjects.length > 0 ? (
				<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 2xl:grid-cols-3">
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
				<div className="my-auto flex flex-col items-center justify-center rounded-2xl  bg-card p-12 text-center">
					<Search className="mb-4 size-10 text-muted-foreground/40" />
					<h3
						className="mb-2 text-xl font-medium text-foreground/80"
						style={{letterSpacing: '-0.2px'}}>
						No matching projects found
					</h3>
					<p className="mx-auto max-w-sm text-sm leading-relaxed text-muted-foreground/70">
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
