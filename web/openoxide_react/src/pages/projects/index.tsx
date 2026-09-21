import {
	createFileRoute,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import {Plus, Search, Folder} from 'lucide-react';
import {useEffect, useState} from 'react';

import {Button} from '#/components/ui/button';
import {Titlebar} from '#/components/shared/titlebar';
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
	validateSearch: (
		search: Record<string, unknown>,
	): {create?: number} => ({
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
			{/* Vercel-style titlebar */}
			<Titlebar
				breadcrumbs={[{label: 'Projects', icon: Folder}]}
				title="Projects"
				description="Create and manage your projects"
				actions={
					<Button
						onClick={() => setIsCreateOpen(true)}
						disabled={!activeOrg}
						className="gap-1.5">
						<Plus className="size-3.5" />
						Create Project
					</Button>
				}
			/>

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
							className="h-44 animate-pulse rounded-xl border bg-card"
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
				<div className="my-auto flex flex-col items-center justify-center rounded-xl border bg-card p-12 text-center">
					<Search className="mb-4 size-10 text-muted-foreground/40" />
					<h3 className="mb-2 text-xl font-semibold tracking-[-0.01em] text-foreground">
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
							className="bg-danger text-white hover:bg-danger/90"
							onClick={confirmDelete}>
							Delete Project
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</div>
	);
}
