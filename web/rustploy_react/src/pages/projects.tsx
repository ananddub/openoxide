import * as React from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {Plus} from 'lucide-react';
import {toast} from 'sonner';

import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
import {ProjectCard} from '#/components/projects/project-card';
import {EmptyState} from '#/components/projects/empty-state';
import {CreateProjectDialog} from '#/components/projects/create-project-dialog';
import {useOrganizationStore} from '#/stores/organization-store';
import {formatApiError} from '#/api/utils';

export const Route = createFileRoute('/_app/projects')({
	component: ProjectsPage,
});

function ProjectsPage() {
	const queryClient = useQueryClient();
	const [isCreateOpen, setIsCreateOpen] = React.useState(false);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

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

			{/* Main Grid View */}
			{isLoadingProjects ? (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					{[1, 2, 3].map(i => (
						<Card
							key={i}
							className="h-48 border-border bg-card/45 animate-pulse"
						/>
					))}
				</div>
			) : projects && projects.length > 0 ? (
				<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 animate-in fade-in duration-200">
					{projects.map(project => (
						<ProjectCard
							key={project.id}
							project={project}
							onDelete={handleDeleteProject}
						/>
					))}
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
