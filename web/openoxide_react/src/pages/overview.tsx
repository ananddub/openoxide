import { createFileRoute } from '@tanstack/react-router';
import { useState, useMemo } from 'react';
import { Box, Globe, Clock, Rocket, Search } from 'lucide-react';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '#/components/ui/tabs';
import { Input } from '#/components/ui/input';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
import { useAppStore } from '#/stores/app-store';
import { useOrganizationStore } from '#/stores/organization-store';
import { ServiceCard } from '#/components/projects/service/service-card';
import { useDeployments } from '#/hooks/deployments/use-deployments';
import { DeploymentItem } from '#/components/deployments/deployment-item';
import { DeploymentLogsDialog } from '#/components/deployments/deployment-logs-dialog';
import { useProjectListByOrganization } from 'virtual:openoxide-live';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState('services');
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');

	const activeOrg = useOrganizationStore((state) => state.activeOrg);
	const orgId = activeOrg?.id || 1;

	// Live Stream + RAM Store Readers
	const { data: liveProjects } = useProjectListByOrganization(BigInt(orgId));
	const storeProjects = useAppStore((state) => state.projects);
	const domains = useAppStore((state) => state.domains);
	const backups = useAppStore((state) => state.backups);

	const {
		filteredAndSorted: deploymentsList,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		copied,
		handleCopyLogs,
	} = useDeployments();

	const projectsToUse = useMemo(() => {
		if (Array.isArray(liveProjects) && liveProjects.length > 0) return liveProjects;
		if (Array.isArray(storeProjects) && storeProjects.length > 0) return storeProjects;
		return [];
	}, [liveProjects, storeProjects]);

	// Aggregate all Applications, Compose Stacks, and Databases across all Projects, Environments, & Deployments
	const allServices = useMemo(() => {
		const serviceMap = new Map<string, {
			projectId: number;
			type: 'APP' | 'COMPOSE' | 'DATABASE';
			id: number;
			name: string;
			subtitle: string;
			status: string;
			createdAt: number;
			dbKind?: string;
		}>();

		// 1. Extract from Projects array
		projectsToUse.forEach((proj: any) => {
			const pId = Number(proj.id) || 1;
			const projName = proj.name || `Project #${pId}`;

			// Top-level service arrays
			(proj.applications || []).forEach((app: any) => {
				const id = Number(app.id);
				if (id) {
					serviceMap.set(`APP-${id}`, {
						projectId: pId,
						type: 'APP',
						id,
						name: app.name || app.app_name || `App #${id}`,
						subtitle: `${projName} / production`,
						status: app.application_status || app.status || 'idle',
						createdAt: app.created_at || Date.now() / 1000,
					});
				}
			});

			(proj.composes || []).forEach((comp: any) => {
				const id = Number(comp.id);
				if (id) {
					serviceMap.set(`COMPOSE-${id}`, {
						projectId: pId,
						type: 'COMPOSE',
						id,
						name: comp.name || comp.app_name || `Compose #${id}`,
						subtitle: `${projName} / production`,
						status: comp.compose_status || comp.status || 'idle',
						createdAt: comp.created_at || Date.now() / 1000,
					});
				}
			});

			const topDbs = [
				...(proj.postgresDbs || []).map((db: any) => ({ ...db, kind: 'postgres' })),
				...(proj.mysqlDbs || []).map((db: any) => ({ ...db, kind: 'mysql' })),
				...(proj.mariadbDbs || []).map((db: any) => ({ ...db, kind: 'mariadb' })),
				...(proj.mongoDbs || []).map((db: any) => ({ ...db, kind: 'mongo' })),
				...(proj.redisDbs || []).map((db: any) => ({ ...db, kind: 'redis' })),
			];

			topDbs.forEach((db: any) => {
				const id = Number(db.id);
				if (id) {
					serviceMap.set(`DATABASE-${id}`, {
						projectId: pId,
						type: 'DATABASE',
						id,
						name: db.name || db.database_name || `DB #${id}`,
						subtitle: `${projName} / production`,
						status: db.database_status || db.status || 'idle',
						dbKind: db.kind,
						createdAt: db.created_at || Date.now() / 1000,
					});
				}
			});

			// Nested Environments array
			const envs = proj.environments || [];
			envs.forEach((env: any) => {
				const envName = env.name || 'production';

				(env.applications || []).forEach((app: any) => {
					const id = Number(app.id);
					if (id && !serviceMap.has(`APP-${id}`)) {
						serviceMap.set(`APP-${id}`, {
							projectId: pId,
							type: 'APP',
							id,
							name: app.name || `App #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: app.application_status || app.status || 'idle',
							createdAt: app.created_at || Date.now() / 1000,
						});
					}
				});

				(env.composes || []).forEach((comp: any) => {
					const id = Number(comp.id);
					if (id && !serviceMap.has(`COMPOSE-${id}`)) {
						serviceMap.set(`COMPOSE-${id}`, {
							projectId: pId,
							type: 'COMPOSE',
							id,
							name: comp.name || `Compose #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: comp.compose_status || comp.status || 'idle',
							createdAt: comp.created_at || Date.now() / 1000,
						});
					}
				});

				const envDbs = [
					...(env.postgreses || env.postgresDbs || []).map((db: any) => ({ ...db, kind: 'postgres' })),
					...(env.mysqls || env.mysqlDbs || []).map((db: any) => ({ ...db, kind: 'mysql' })),
					...(env.mariadbs || env.mariadbDbs || []).map((db: any) => ({ ...db, kind: 'mariadb' })),
					...(env.mongos || env.mongoDbs || []).map((db: any) => ({ ...db, kind: 'mongo' })),
					...(env.redises || env.redisDbs || []).map((db: any) => ({ ...db, kind: 'redis' })),
				];

				envDbs.forEach((db: any) => {
					const id = Number(db.id);
					if (id && !serviceMap.has(`DATABASE-${id}`)) {
						serviceMap.set(`DATABASE-${id}`, {
							projectId: pId,
							type: 'DATABASE',
							id,
							name: db.name || `DB #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: db.database_status || db.status || 'idle',
							dbKind: db.kind,
							createdAt: db.created_at || Date.now() / 1000,
						});
					}
				});
			});
		});

		// 2. Fallback: If serviceMap is empty but deployments exist, infer services from deployments list
		if (serviceMap.size === 0 && deploymentsList.length > 0) {
			deploymentsList.forEach((dep: any) => {
				const id = Number(dep.app_id || dep.id || 1);
				const isCompose = !!dep.compose_id;
				const isDb = !!dep.database_id;
				const type: 'APP' | 'COMPOSE' | 'DATABASE' = isCompose ? 'COMPOSE' : isDb ? 'DATABASE' : 'APP';
				const key = `${type}-${id}`;

				if (!serviceMap.has(key)) {
					serviceMap.set(key, {
						projectId: Number(dep.project_id || 1),
						type,
						id,
						name: dep.title || dep.app_name || dep.name || `Service #${id}`,
						subtitle: `${dep.project_name || 'Production'} / production`,
						status: dep.status || 'running',
						createdAt: dep.created_at || Date.now() / 1000,
					});
				}
			});
		}

		return Array.from(serviceMap.values());
	}, [projectsToUse, deploymentsList]);

	// Filtered services
	const filteredServices = useMemo(() => {
		return allServices.filter((s) => {
			const matchesSearch =
				!searchQuery ||
				s.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				s.subtitle.toLowerCase().includes(searchQuery.toLowerCase());

			const matchesType =
				typeFilter === 'all' ||
				(typeFilter === 'app' && s.type === 'APP') ||
				(typeFilter === 'compose' && s.type === 'COMPOSE') ||
				(typeFilter === 'database' && s.type === 'DATABASE');

			return matchesSearch && matchesType;
		});
	}, [allServices, searchQuery, typeFilter]);

	return (
		<div className="p-6 flex flex-col gap-6 max-w-7xl mx-auto w-full animate-in fade-in duration-200">
			{/* Page Header */}
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Overview</h1>
				<p className="text-xs text-muted-foreground">
					Centralized platform overview of all applications, compose stacks, databases, deployments, domains, and backups
				</p>
			</div>

			{/* Main Overview Tabs */}
			<Tabs value={activeTab} onValueChange={setActiveTab} className="w-full space-y-6">
				<TabsList variant="line" className="border-b border-border w-full justify-start gap-6 rounded-none pb-0">
					<TabsTrigger value="services" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Box className="size-4" />
						Services
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{allServices.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="deployments" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Rocket className="size-4" />
						Deployments
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{deploymentsList.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="domains" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Globe className="size-4" />
						Domains
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{domains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="backups" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Clock className="size-4" />
						Volume Backups
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{backups.length}
						</Badge>
					</TabsTrigger>
				</TabsList>

				{/* Services Tab Content */}
				<TabsContent value="services" className="space-y-4">
					{/* Search & Filter Toolbar */}
					<div className="flex items-center justify-between gap-4">
						<div className="relative flex-1 max-w-md">
							<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
							<Input
								placeholder="Search services by name or project..."
								value={searchQuery}
								onChange={(e) => setSearchQuery(e.target.value)}
								className="pl-9 h-9 text-xs"
							/>
						</div>
						<div className="flex items-center gap-2">
							<Button
								variant={typeFilter === 'all' ? 'secondary' : 'ghost'}
								size="sm"
								onClick={() => setTypeFilter('all')}
								className="h-8 text-xs"
							>
								All ({allServices.length})
							</Button>
							<Button
								variant={typeFilter === 'app' ? 'secondary' : 'ghost'}
								size="sm"
								onClick={() => setTypeFilter('app')}
								className="h-8 text-xs"
							>
								Apps ({allServices.filter((s) => s.type === 'APP').length})
							</Button>
							<Button
								variant={typeFilter === 'compose' ? 'secondary' : 'ghost'}
								size="sm"
								onClick={() => setTypeFilter('compose')}
								className="h-8 text-xs"
							>
								Compose ({allServices.filter((s) => s.type === 'COMPOSE').length})
							</Button>
							<Button
								variant={typeFilter === 'database' ? 'secondary' : 'ghost'}
								size="sm"
								onClick={() => setTypeFilter('database')}
								className="h-8 text-xs"
							>
								Databases ({allServices.filter((s) => s.type === 'DATABASE').length})
							</Button>
						</div>
					</div>

					{/* Services Grid */}
					{filteredServices.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Box className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No services found</p>
							<p className="text-xs text-muted-foreground">
								{searchQuery ? 'Try clearing your search query' : 'Create an application, compose stack, or database in a project'}
							</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{filteredServices.map((svc) => (
								<ServiceCard
									key={`${svc.type}-${svc.id}`}
									projectId={svc.projectId}
									type={svc.type}
									id={svc.id}
									name={svc.name}
									subtitle={svc.subtitle}
									status={svc.status}
									createdAt={svc.createdAt}
									dbKind={svc.dbKind}
								/>
							))}
						</div>
					)}
				</TabsContent>

				{/* Deployments Tab Content */}
				<TabsContent value="deployments" className="space-y-4">
					{deploymentsList.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Rocket className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No deployment history</p>
							<p className="text-xs text-muted-foreground">Deployments will appear here once triggered</p>
						</div>
					) : (
						<div className="flex flex-col gap-2">
							{deploymentsList.map((dep) => (
								<DeploymentItem
									key={dep.id}
									deployment={dep}
									onViewLogs={() => setSelectedDeployment(dep)}
									onViewError={() => setSelectedDeployment(dep)}
								/>
							))}
						</div>
					)}
				</TabsContent>

				{/* Domains Tab Content */}
				<TabsContent value="domains" className="space-y-4">
					{domains.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Globe className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No active domains configured</p>
							<p className="text-xs text-muted-foreground">Configure custom domain routes in application settings</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{domains.map((dom) => (
								<div key={dom.id} className="p-4 border rounded-xl bg-card flex flex-col gap-2">
									<div className="flex items-center justify-between">
										<p className="font-bold text-xs text-foreground font-mono">{dom.domain || dom.host}</p>
										<Badge variant="outline" className="text-[10px]">
											{dom.https ? 'HTTPS' : 'HTTP'}
										</Badge>
									</div>
									<p className="text-[10px] text-muted-foreground font-mono">
										Port: {dom.port || 80} · Path: {dom.path || '/'}
									</p>
								</div>
							))}
						</div>
					)}
				</TabsContent>

				{/* Backups Tab Content */}
				<TabsContent value="backups" className="space-y-4">
					{backups.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Clock className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No volume backups found</p>
							<p className="text-xs text-muted-foreground">Scheduled volume backups will appear here</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{backups.map((b) => (
								<div key={b.id} className="p-4 border rounded-xl bg-card flex flex-col gap-2">
									<div className="flex items-center justify-between">
										<p className="font-bold text-xs text-foreground font-mono">{b.name || `Backup #${b.id}`}</p>
										<Badge variant="secondary" className="text-[10px]">
											{b.status || 'DONE'}
										</Badge>
									</div>
									<p className="text-[10px] text-muted-foreground font-mono">
										Size: {b.size || 'N/A'} · Dest: {b.destination || 'Local'}
									</p>
								</div>
							))}
						</div>
					)}
				</TabsContent>
			</Tabs>

			{/* Logs Dialog */}
			<DeploymentLogsDialog
				selectedDeployment={selectedDeployment}
				onClose={() => setSelectedDeployment(null)}
				logs={logs}
				isLogsLoading={false}
				copied={copied}
				onCopyLogs={handleCopyLogs}
			/>
		</div>
	);
}
