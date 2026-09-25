import {
	createFileRoute,
	useNavigate,
	useSearch,
} from '@tanstack/react-router';
import {Plus, Search} from 'lucide-react';
import {useEffect, useState} from 'react';

import {Button} from '#/components/ui/button';
import {ProjectCard} from '#/components/projects/dashboard/project-card';
import {ProjectListRow} from '#/components/projects/dashboard/project-list-row';
import {
	ProjectsToolbar,
	type ProjectsView,
} from '#/components/projects/dashboard/projects-toolbar';
import {EmptyState} from '#/components/projects/dashboard/empty-state';
import {
	ListRegion,
	SectionHeader,
} from '#/components/settings/list-card';
import {HandleProjectDialog} from '#/components/projects/dashboard/handle-project-dialog';
import {useProjectsList} from '#/hooks/projects/list/use-projects-list';
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

const VIEW_STORAGE_KEY = 'projects-view';

function ProjectsPage() {
	const [deletingId, setDeletingId] = useState<number | null>(null);
	const [view, setView] = useState<ProjectsView>(() => {
		if (typeof window === 'undefined') return 'grid';
		return localStorage.getItem(VIEW_STORAGE_KEY) === 'list'
			? 'list'
			: 'grid';
	});
	const navigate = useNavigate();
	const {create} = useSearch({from: '/_app/projects'});

	const {
		projects,
		isLoadingProjects,
		filteredAndSortedProjects,
		searchQuery,
		setSearchQuery,
		sortBy,
		setSortBy,
		selectedTags,
		setSelectedTags,
		isCreateOpen,
		setIsCreateOpen,
		activeOrg,
		handleDeleteProject,
	} = useProjectsList();

	// The rail's "New Project" CTA links here with ?create=1; open the dialog and drop the param so a refresh doesn't re-trigger it.
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

	const handleViewChange = (next: ProjectsView) => {
		setView(next);
		localStorage.setItem(VIEW_STORAGE_KEY, next);
	};

	return (
		<div className="mx-auto flex h-full w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title="Projects"
				description="Every application, database and service you manage, grouped by project."
				action={
					<Button
						onClick={() => setIsCreateOpen(true)}
						disabled={!activeOrg}
						className="h-9 gap-1.5">
						<Plus className="size-3.5" />
						New Project
					</Button>
				}
			/>

			{/* Search + filters + view toggle */}
			<ProjectsToolbar
				searchQuery={searchQuery}
				onSearchChange={setSearchQuery}
				sortBy={sortBy}
				onSortChange={setSortBy}
				selectedTags={selectedTags}
				onTagsChange={setSelectedTags}
				view={view}
				onViewChange={handleViewChange}
			/>

			<ListRegion>
				{/* Content: grid cards or compact list */}
				{isLoadingProjects ? (
					<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
						{[1, 2, 3, 4].map(i => (
							<div
								key={i}
								className="h-32 animate-pulse rounded-lg border bg-card"
							/>
						))}
					</div>
				) : filteredAndSortedProjects.length > 0 ? (
					view === 'grid' ? (
						<div className="grid w-full grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
							{filteredAndSortedProjects.map(project => (
								<ProjectCard
									key={String(project.id)}
									project={project as unknown as any}
									onDelete={id => setDeletingId(id)}
								/>
							))}
						</div>
					) : (
						<div className="w-full divide-y overflow-hidden rounded-xl border bg-card">
							{filteredAndSortedProjects.map(project => (
								<ProjectListRow
									key={String(project.id)}
									project={project as unknown as any}
									onDelete={id => setDeletingId(id)}
								/>
							))}
						</div>
					)
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
			</ListRegion>

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
