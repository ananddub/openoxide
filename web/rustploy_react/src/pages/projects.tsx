import * as React from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {Plus} from 'lucide-react';
import {toast} from 'sonner';

import {Button} from '#/components/ui/button';
import {Card} from '#/components/ui/card';
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

export const Route = createFileRoute('/_app/projects')({
	component: ProjectsPage,
});

function ProjectsPage() {
	const queryClient = useQueryClient();
	const [selectedOrgId, setSelectedOrgId] = React.useState<number | null>(null);
	const [isCreateOpen, setIsCreateOpen] = React.useState(false);
	const [isSubmitting, setIsSubmitting] = React.useState(false);

	// Fetch Organizations
	const {data: organizations, isLoading: isLoadingOrgs} = $api.useQuery(
		'get',
		'/organizations',
	);

	// Auto-select first organization if available
	React.useEffect(() => {
		if (organizations && organizations.length > 0 && selectedOrgId === null) {
			setSelectedOrgId(organizations[0].id);
		}
	}, [organizations, selectedOrgId]);

	// Fetch Projects
	const {data: projects, isLoading: isLoadingProjects} = $api.useQuery(
		'get',
		'/projects/organization/{organization_id}',
		{
			params: {
				path: {
					organization_id: selectedOrgId || 0,
				},
			},
		},
		{
			enabled: selectedOrgId !== null,
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
		if (!name.trim() || selectedOrgId === null) {
			toast.error('Project name is required');
			return;
		}

		setIsSubmitting(true);
		try {
			const {error} = await createProjectMutation.mutateAsync({
				body: {
					name,
					description: description || undefined,
					env_var: envVar,
					organization_id: selectedOrgId,
				},
			});

			if (error) {
				const errBody = error as any;
				toast.error(errBody?.message || 'Failed to create project');
			} else {
				toast.success('Project created successfully!');
				setIsCreateOpen(false);
				// Refresh projects list
				queryClient.invalidateQueries({
					queryKey: [
						'get',
						'/projects/organization/{organization_id}',
						{params: {path: {organization_id: selectedOrgId}}},
					],
				});
			}
		} catch {
			toast.error('An unexpected error occurred');
		} finally {
			setIsSubmitting(false);
		}
	};

	const handleDeleteProject = async (projectId: number) => {
		if (!confirm('Are you sure you want to delete this project?')) return;

		try {
			const {error} = await deleteProjectMutation.mutateAsync({
				params: {
					path: {
						id: projectId,
					},
				},
			});

			if (error) {
				const errBody = error as any;
				toast.error(errBody?.message || 'Failed to delete project');
			} else {
				toast.success('Project deleted successfully');
				queryClient.invalidateQueries({
					queryKey: [
						'get',
						'/projects/organization/{organization_id}',
						{params: {path: {organization_id: selectedOrgId}}},
					],
				});
			}
		} catch {
			toast.error('An unexpected error occurred');
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
						Organize, manage and deploy applications inside your environments.
					</p>
				</div>

				<div className="flex items-center gap-3">
					{/* Org Select Dropdown */}
					{isLoadingOrgs ? (
						<div className="h-10 w-44 rounded-lg bg-muted animate-pulse" />
					) : (
						organizations &&
						organizations.length > 0 && (
							<Select
								value={selectedOrgId?.toString()}
								onValueChange={val => setSelectedOrgId(Number(val))}>
								<SelectTrigger className="w-[180px] bg-card border-border/80 h-10 shadow-sm font-medium">
									<SelectValue placeholder="Select Organization" />
								</SelectTrigger>
								<SelectContent className="bg-card border-border">
									{organizations.map(org => (
										<SelectItem
											key={org.id}
											value={org.id.toString()}>
											{org.name}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						)
					)}

					<Button
						onClick={() => setIsCreateOpen(true)}
						disabled={selectedOrgId === null}
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
					disabled={selectedOrgId === null}
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
