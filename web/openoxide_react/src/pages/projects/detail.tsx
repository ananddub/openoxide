import {createFileRoute, Link} from '@tanstack/react-router';
import {Box, KeyRound, Search, Plus} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {EnvDropdown} from '#/components/projects/env/env-dropdown';
import {ServiceCard} from '#/components/projects/service/service-card';
import {ProjectModals} from '#/components/projects/dashboard/project-modals';
import {ProjectDetailFilters} from '#/components/projects/dashboard/project-detail-filters';
import {AddNewServiceMenu} from '#/components/projects/dashboard/add-new-service-menu';
import {useProjectDetails} from '#/hooks/projects/use-project-details';
import {SectionHeader} from '#/components/settings/list-card';
import {IconTooltip} from '#/components/shared/icon-tooltip';

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
		project,
		envs,
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
		<div className="mx-auto flex w-full max-w-[1920px] animate-in flex-col gap-6 px-4 pt-4 pb-6 duration-200 fade-in">
			<SectionHeader
				title={project?.name || `Project #${projectId}`}
				description={
					project?.description ||
					'Applications, compose stacks and databases deployed from this project.'
				}
				action={
					<>
						<IconTooltip label="Environment variables">
							<Button
								variant="outline"
								size="icon"
								aria-label="Environment Variables"
								onClick={() => setShowEnvVars(true)}
								className="size-9">
								<KeyRound className="size-4" />
							</Button>
						</IconTooltip>
						<AddNewServiceMenu
							projectId={projectId}
							envId={selectedEnvId ?? undefined}
							onCreated={handleRefresh}
						/>
					</>
				}>
				{selectedEnv?.name && (
					<span className="shrink-0 rounded-md bg-foreground/[0.05] px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
						{selectedEnv.name}
					</span>
				)}
			</SectionHeader>

			{/* One-line toolbar: env switcher · search + filters */}
			<div className="flex w-full flex-wrap items-center gap-3">
				<div className="flex shrink-0 items-center gap-1.5">
					<EnvDropdown
						envs={envs}
						selectedId={selectedEnvId}
						onSelect={setSelectedEnvId}
						onCreateNew={() => setShowCreateEnv(true)}
					/>
				</div>

				<ProjectDetailFilters
					totalServices={totalServices}
					searchQuery={searchQuery}
					setSearchQuery={setSearchQuery}
					typeFilter={typeFilter}
					setTypeFilter={setTypeFilter}
					statusFilter={statusFilter}
					setStatusFilter={setStatusFilter}
				/>
			</div>

			{/* Services */}
			{isLoading ? (
				<div className="flex justify-center py-24">
					<div className="size-6 animate-spin rounded-full border-2  border-t-primary" />
				</div>
			) : totalServices === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl border bg-card py-20 backdrop-blur-[2px]">
					<Box className="mb-3 size-12 text-muted-foreground/45" />
					<h3 className="text-sm font-bold text-foreground">
						No services configured
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Deploy an application, compose stack, or database in{' '}
						{selectedEnv?.name || 'this environment'}.
					</p>
					<Link
						to="/projects/$id/create"
						params={{id: String(projectId)}}
						search={selectedEnvId ? {env: selectedEnvId} : {}}
						className="mt-4 inline-flex h-9 items-center gap-1.5 rounded-lg bg-primary px-4 text-xs font-semibold text-primary-foreground shadow-lg shadow-primary/10 transition-colors hover:bg-primary/95">
						<Plus className="size-3.5" />
						Create Your First Service
					</Link>
				</div>
			) : filteredServices.length === 0 ? (
				<div className="flex flex-col items-center justify-center rounded-xl  bg-card/5 py-16">
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
							serverName={srv.serverName}
						/>
					))}
				</div>
			)}

			<ProjectModals
				projectId={projectId}
				project={project}
				selectedEnv={selectedEnv}
				showCreateEnv={showCreateEnv}
				setShowCreateEnv={setShowCreateEnv}
				showProjectEnv={showProjectEnv}
				setShowProjectEnv={setShowProjectEnv}
				showEnvVars={showEnvVars}
				setShowEnvVars={setShowEnvVars}
				handleRefresh={handleRefresh}
				envsRefetch={refetchEnvs}
			/>
		</div>
	);
}
