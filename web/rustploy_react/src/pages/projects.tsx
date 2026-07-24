import * as React from 'react';
import {createFileRoute} from '@tanstack/react-router';
import {$api} from '#/api/query';
import {useQueryClient} from '@tanstack/react-query';
import {
	FolderOpen,
	Plus,
	Trash2,
	Calendar,
	Terminal,
	Loader2,
	FolderClosed,
} from 'lucide-react';
import {toast} from 'sonner';

import {Button} from '#/components/ui/button';
import {
	Card,
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from '#/components/ui/card';
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from '#/components/ui/select';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from '#/components/ui/dialog';
import {Input} from '#/components/ui/input';
import {Label} from '#/components/ui/label';

export const Route = createFileRoute('/_app/projects')({
	component: ProjectsPage,
});

function ProjectsPage() {
	const queryClient = useQueryClient();
	const [selectedOrgId, setSelectedOrgId] = React.useState<number | null>(null);
	const [isCreateOpen, setIsCreateOpen] = React.useState(false);

	// Form states
	const [name, setName] = React.useState('');
	const [description, setDescription] = React.useState('');
	const [envVar, setEnvVar] = React.useState('');
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

	const handleCreateProject = async (e: React.FormEvent) => {
		e.preventDefault();
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
				// Reset form
				setName('');
				setDescription('');
				setEnvVar('');
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
						<Card
							key={project.id}
							className="group overflow-hidden border-border bg-card/40 shadow-sm backdrop-blur-sm transition-all duration-300 hover:shadow-lg hover:border-primary/20 hover:bg-card/75">
							<CardHeader className="pb-3">
								<div className="flex items-start justify-between">
									<div className="flex items-center gap-3">
										<div className="p-2.5 rounded-lg bg-primary/5 text-primary border border-primary/10 group-hover:bg-primary/10 group-hover:border-primary/20 transition-all">
											<FolderOpen className="size-5" />
										</div>
										<div>
											<CardTitle className="text-lg font-bold group-hover:text-primary transition-colors">
												{project.name}
											</CardTitle>
											<span className="text-[10px] uppercase tracking-wider text-muted-foreground font-semibold flex items-center gap-1.5 mt-1">
												<Calendar className="size-3" />
												{new Date(project.created_at * 1000).toLocaleDateString()}
											</span>
										</div>
									</div>
								</div>
							</CardHeader>
							<CardContent className="pb-4 min-h-[72px]">
								<p className="text-sm text-muted-foreground line-clamp-3 leading-relaxed">
									{project.description || 'No description provided.'}
								</p>
							</CardContent>
							<CardFooter className="pt-3 border-t border-border/30 bg-muted/15 flex items-center justify-between">
								<div className="flex items-center gap-2">
									{project.env_var && (
										<span className="inline-flex items-center gap-1 text-[11px] font-semibold text-primary/95 bg-primary/5 border border-primary/10 px-2 py-0.5 rounded">
											<Terminal className="size-3" />
											Envs Configured
										</span>
									)}
								</div>

								<div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity">
									<Button
										variant="ghost"
										size="icon"
										onClick={() => handleDeleteProject(project.id)}
										className="size-8 text-muted-foreground hover:text-destructive hover:bg-destructive/5 rounded-md">
										<Trash2 className="size-4" />
									</Button>
								</div>
							</CardFooter>
						</Card>
					))}
				</div>
			) : (
				/* Empty State */
				<div className="flex flex-col items-center justify-center border border-dashed border-border/80 rounded-2xl p-16 text-center bg-card/10 backdrop-blur-[2px] animate-in fade-in duration-200">
					<div className="p-4 rounded-full bg-muted/40 border border-border/30 text-muted-foreground mb-4">
						<FolderClosed className="size-8" />
					</div>
					<h3 className="text-xl font-bold text-foreground">No Projects Yet</h3>
					<p className="text-muted-foreground max-w-sm mt-1.5 text-sm leading-relaxed">
						Projects group your environments, applications and services. Create your first project to get started.
					</p>
					<Button
						onClick={() => setIsCreateOpen(true)}
						disabled={selectedOrgId === null}
						className="mt-6 bg-primary hover:bg-primary/95 text-primary-foreground font-semibold px-4 rounded-lg flex items-center gap-2 shadow-lg shadow-primary/5">
						<Plus className="size-4" />
						Create Project
					</Button>
				</div>
			)}

			{/* Create Project Modal */}
			<Dialog
				open={isCreateOpen}
				onOpenChange={setIsCreateOpen}>
				<DialogContent className="sm:max-w-md bg-card border-border">
					<DialogHeader>
						<DialogTitle className="text-xl font-bold">Create New Project</DialogTitle>
						<DialogDescription className="text-muted-foreground text-sm">
							Provide a name, optional description, and environment variables for your new project.
						</DialogDescription>
					</DialogHeader>

					<form onSubmit={handleCreateProject}>
						<div className="grid gap-5 py-4">
							<div className="flex flex-col gap-2">
								<Label
									htmlFor="name"
									className="text-sm font-semibold text-foreground">
									Project Name
								</Label>
								<Input
									id="name"
									placeholder="e.g. My Production API"
									value={name}
									onChange={e => setName(e.target.value)}
									disabled={isSubmitting}
									required
									className="bg-card border-border"
								/>
							</div>

							<div className="flex flex-col gap-2">
								<Label
									htmlFor="description"
									className="text-sm font-semibold text-foreground">
									Description
								</Label>
								<textarea
									id="description"
									placeholder="Describe the purpose of this project..."
									value={description}
									onChange={e => setDescription(e.target.value)}
									disabled={isSubmitting}
									rows={3}
									className="flex w-full rounded-md border border-input bg-card px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-border resize-none"
								/>
							</div>

							<div className="flex flex-col gap-2">
								<Label
									htmlFor="envVar"
									className="text-sm font-semibold text-foreground">
									Environment Variables
								</Label>
								<textarea
									id="envVar"
									placeholder="KEY=VALUE&#10;PORT=8080"
									value={envVar}
									onChange={e => setEnvVar(e.target.value)}
									disabled={isSubmitting}
									rows={4}
									className="flex w-full rounded-md border border-input bg-card px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 border-border font-mono resize-none"
								/>
							</div>
						</div>

						<DialogFooter className="gap-2 sm:gap-0 mt-2 border-t border-border/30 pt-4">
							<Button
								type="button"
								variant="ghost"
								onClick={() => setIsCreateOpen(false)}
								disabled={isSubmitting}
								className="text-muted-foreground hover:bg-muted font-medium">
								Cancel
							</Button>
							<Button
								type="submit"
								disabled={isSubmitting}
								className="bg-primary hover:bg-primary/95 text-primary-foreground font-semibold flex items-center gap-2">
								{isSubmitting ? (
									<>
										<Loader2 className="animate-spin size-4" />
										Creating...
									</>
								) : (
									'Create Project'
								)}
							</Button>
						</DialogFooter>
					</form>
				</DialogContent>
			</Dialog>
		</div>
	);
}
