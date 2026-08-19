import {createFileRoute, Link} from '@tanstack/react-router';
import {FolderOpen, Box, Settings2, RefreshCw, Search} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {EnvDropdown} from '#/components/projects/env/env-dropdown';
import {CreateServiceDropdown} from '#/components/projects/service/create-service-dropdown';
import {ServiceCard} from '#/components/projects/service/service-card';
import {ProjectModals} from '#/components/projects/dashboard/project-modals';
import {ProjectDetailFilters} from '#/components/projects/dashboard/project-detail-filters';
import {useProjectDetails} from '#/hooks/projects/use-project-details';

export const Route = createFileRoute('/_app/projects/$id')({
	component: ProjectDetailPage,
});

function ProjectDetailPage() {
	const {id} = Route.useParams();
	const projectId = Number(id);

	const {
		showCreateEnv,
		setShowCreateEnv,
		showProjectEnv,
		setShowProjectEnv,
		showEnvVars,
		setShowEnvVars,
		showCreateApp,
		setShowCreateApp,
		showCreateCompose,
		setShowCreateCompose,
		showCreateDatabase,
		setShowCreateDatabase,
		project,
		envs,
		servers,
		selectedEnvId,
		setSelectedEnvId,
		selectedEnv,
		filteredServices,
		handleRefresh,
		isLoading,
		totalServices,
		searchQuery,
		setSearchQuery,
		typeFilter,
		setTypeFilter,
		statusFilter,
		setStatusFilter,
		refetchEnvs,
	} = useProjectDetails(projectId);

	return (
		<div className="flex w-full animate-in flex-col gap-6 pb-10 duration-200 fade-in">
			{/* Header */}
			<div className="flex flex-col justify-between gap-4 border-b border-border/40 pb-6 sm:flex-row sm:items-center">
				<div className="flex flex-wrap items-center gap-2 text-sm font-semibold">
					<Link
						to="/projects"
						className="flex items-center gap-1.5 text-muted-foreground transition-colors hover:text-foreground">
						<FolderOpen className="size-4" />
						Projects
					</Link>
					<span className="text-muted-foreground/35">/</span>
					<span className="font-bold text-foreground">
						{project?.name || 'Loading...'}
					</span>
					<span className="text-muted-foreground/35">/</span>
					<EnvDropdown
						envs={envs}
						selectedId={selectedEnvId}
						onSelect={setSelectedEnvId}
						onCreateNew={() => setShowCreateEnv(true)}
					/>
					<Settings2
						onClick={() => setShowEnvVars(true)}
						className="ml-1.5 size-4 shrink-0 cursor-pointer text-muted-foreground transition-colors hover:text-foreground"
					/>
				</div>

				<div className="flex items-center gap-3">
					<Button
						variant="outline"
						size="icon"
						onClick={handleRefresh}
						className="size-8 rounded-lg border-border">
						<RefreshCw className="size-3.5" />
					</Button>
					<Button
						variant="outline"
						onClick={() => setShowProjectEnv(true)}
						className="flex h-8 items-center gap-1.5 border-border text-xs font-semibold text-foreground hover:bg-muted">
						<Settings2 className="size-3.5" />
						Project Env
					</Button>
					<CreateServiceDropdown
						disabled={!selectedEnvId}
						onSelect={type => {
							if (type === 'application') setShowCreateApp(true);
							else if (type === 'compose') setShowCreateCompose(true);
							else if (type === 'database') setShowCreateDatabase(true);
						}}
					/>
				</div>
			</div>

			{/* Filters */}
			<ProjectDetailFilters
				totalServices={totalServices}
				searchQuery={searchQuery}
				setSearchQuery={setSearchQuery}
				typeFilter={typeFilter}
				setTypeFilter={setTypeFilter}
				statusFilter={statusFilter}
				setStatusFilter={setStatusFilter}
			/>

			{/* Services */}
			{isLoading ? (
				<div className="flex justify-center py-24">
					<div className="size-6 animate-spin rounded-full border-2 border-primary/30 border-t-primary" />
				</div>
			) : totalServices === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/70 bg-card/10 py-20 backdrop-blur-[2px]">
					<Box className="mb-3 size-12 text-muted-foreground/45" />
					<h3 className="text-sm font-bold text-foreground">
						No services configured
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Deploy an application, compose stack, or database in{' '}
						{selectedEnv?.name || 'this environment'}.
					</p>
					<Button
						onClick={() => setShowCreateApp(true)}
						className="mt-4 h-9 px-4 text-xs font-semibold">
						Create Your First Service
					</Button>
				</div>
			) : filteredServices.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/60 bg-card/5 py-16">
					<Search className="mb-2.5 size-8 text-muted-foreground/40" />
					<p className="text-xs font-semibold text-muted-foreground">
						No services match your filters
					</p>
				</div>
			) : (
				<div className="grid animate-in grid-cols-1 gap-4 duration-200 fade-in sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
					{filteredServices.map((srv: any) => (
						<ServiceCard
							key={`${srv.type}-${srv.id}`}
							projectId={srv.projectId}
							type={srv.type}
							id={srv.id}
							name={srv.name}
							subtitle={srv.subtitle}
							status={srv.status}
							createdAt={srv.createdAt}
							dbKind={srv.dbKind}
						/>
					))}
				</div>
			)}

			<ProjectModals
				projectId={projectId}
				project={project}
				selectedEnvId={selectedEnvId}
				selectedEnv={selectedEnv}
				servers={servers}
				showCreateEnv={showCreateEnv}
				setShowCreateEnv={setShowCreateEnv}
				showProjectEnv={showProjectEnv}
				setShowProjectEnv={setShowProjectEnv}
				showEnvVars={showEnvVars}
				setShowEnvVars={setShowEnvVars}
				showCreateApp={showCreateApp}
				setShowCreateApp={setShowCreateApp}
				showCreateCompose={showCreateCompose}
				setShowCreateCompose={setShowCreateCompose}
				showCreateDatabase={showCreateDatabase}
				setShowCreateDatabase={setShowCreateDatabase}
				handleRefresh={handleRefresh}
				envsRefetch={refetchEnvs}
			/>
		</div>
	);
}
