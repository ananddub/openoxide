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
		showCreateEnv, setShowCreateEnv, showProjectEnv, setShowProjectEnv,
		showEnvVars, setShowEnvVars, showCreateApp, setShowCreateApp,
		showCreateCompose, setShowCreateCompose, showCreateDatabase, setShowCreateDatabase,
		project, envs, servers, selectedEnvId, setSelectedEnvId, selectedEnv,
		filteredServices, handleRefresh, isLoading, totalServices,
		searchQuery, setSearchQuery, typeFilter, setTypeFilter,
		statusFilter, setStatusFilter, refetchEnvs,
	} = useProjectDetails(projectId);

	return (
		<div className="flex flex-col gap-6 w-full pb-10 animate-in fade-in duration-200">
			{/* Breadcrumb & Navigation Top bar */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-6">
				<div className="flex flex-wrap items-center gap-2 text-sm font-semibold">
					<Link to="/projects" className="text-muted-foreground hover:text-foreground flex items-center gap-1.5 transition-colors">
						<FolderOpen className="size-4" />
						Projects
					</Link>
					<span className="text-muted-foreground/35">/</span>
					<span className="text-foreground font-bold">{project?.name || 'Loading...'}</span>
					<span className="text-muted-foreground/35">/</span>
					<EnvDropdown envs={envs} selectedId={selectedEnvId} onSelect={setSelectedEnvId} onCreateNew={() => setShowCreateEnv(true)} />
					<Settings2 onClick={() => setShowEnvVars(true)} className="size-4 text-muted-foreground hover:text-foreground cursor-pointer transition-colors ml-1.5 shrink-0" />
				</div>

				<div className="flex items-center gap-3">
					<Button variant="outline" size="icon" onClick={handleRefresh} className="size-8 border-border rounded-lg">
						<RefreshCw className="size-3.5" />
					</Button>
					<Button
						variant="outline"
						onClick={() => setShowProjectEnv(true)}
						className="border-border text-foreground hover:bg-muted font-semibold text-xs h-8 flex items-center gap-1.5">
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

			{/* Filters & Search Control Bar */}
			<ProjectDetailFilters
				totalServices={totalServices}
				searchQuery={searchQuery}
				setSearchQuery={setSearchQuery}
				typeFilter={typeFilter}
				setTypeFilter={setTypeFilter}
				statusFilter={statusFilter}
				setStatusFilter={setStatusFilter}
			/>

			{/* Service view grid */}
			{isLoading ? (
				<div className="flex justify-center py-24">
					<div className="size-6 border-2 border-primary/30 border-t-primary rounded-full animate-spin" />
				</div>
			) : totalServices === 0 ? (
				<div className="flex flex-col items-center justify-center py-20 border border-dashed border-border/70 rounded-2xl bg-card/10 backdrop-blur-[2px]">
					<Box className="size-12 mb-3 text-muted-foreground/45" />
					<h3 className="text-sm font-bold text-foreground">No services configured</h3>
					<p className="text-muted-foreground text-xs mt-1">Deploy an application, compose stack, or database in {selectedEnv?.name || 'this environment'}.</p>
					<Button onClick={() => setShowCreateApp(true)} className="mt-4 text-xs font-semibold h-9 px-4">
						Create Your First Service
					</Button>
				</div>
			) : filteredServices.length === 0 ? (
				<div className="flex flex-col items-center justify-center py-16 border border-dashed border-border/60 rounded-2xl bg-card/5">
					<Search className="size-8 mb-2.5 text-muted-foreground/40" />
					<p className="text-xs font-semibold text-muted-foreground">No services match your filters</p>
				</div>
			) : (
				<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 animate-in fade-in duration-200">
					{filteredServices.map(srv => (
						<ServiceCard
							key={srv.key}
							projectId={srv.projectId}
							type={srv.type}
							id={srv.id}
							name={srv.name}
							subtitle={srv.subtitle}
							status={srv.status}
							createdAt={srv.createdAt}
							dbKind={'dbKind' in srv ? srv.dbKind : undefined}
						/>
					))}
				</div>
			)}

			{/* Dialog Modals */}
			<ProjectModals
				projectId={projectId} project={project} selectedEnvId={selectedEnvId} selectedEnv={selectedEnv} servers={servers}
				showCreateEnv={showCreateEnv} setShowCreateEnv={setShowCreateEnv} showProjectEnv={showProjectEnv} setShowProjectEnv={setShowProjectEnv}
				showEnvVars={showEnvVars} setShowEnvVars={setShowEnvVars} showCreateApp={showCreateApp} setShowCreateApp={setShowCreateApp}
				showCreateCompose={showCreateCompose} setShowCreateCompose={setShowCreateCompose} showCreateDatabase={showCreateDatabase} setShowCreateDatabase={setShowCreateDatabase}
				handleRefresh={handleRefresh} envsRefetch={refetchEnvs}
			/>
		</div>
	);
}
