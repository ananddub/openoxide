import { createFileRoute } from '@tanstack/react-router';
import { useState, useMemo } from 'react';
import { Box, Layers, Database, Globe, Clock, Rocket, Search, Filter } from 'lucide-react';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '#/components/ui/tabs';
import { Input } from '#/components/ui/input';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
import { useAppStore } from '#/stores/app-store';
import { ServiceCard } from '#/components/projects/service/service-card';
import { useDeployments } from '#/hooks/deployments/use-deployments';
import { DeploymentItem } from '#/components/deployments/deployment-item';
import { DeploymentLogsDialog } from '#/components/deployments/deployment-logs-dialog';
import { $api } from '#/api/query';

export const Route = createFileRoute('/_app/overview')({
	component: OverviewPage,
});

function OverviewPage() {
	const [activeTab, setActiveTab] = useState('services');
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');

	// RAM Store Readers (0ms Local-First Reads)
	const projects = useAppStore((state) => state.projects);
	const storeApps = useAppStore((state) => state.applications);
	const storeDbs = useAppStore((state) => state.databases);
	const storeComposes = useAppStore((state) => state.composes);
	const storeDomains = useAppStore((state) => state.domains);
	const storeBackups = useAppStore((state) => state.backups);

	// Direct API Fallbacks (in case RAM store is hydrating)
	const { data: apiApps } = $api.useQuery('get', '/applications' as any, {} as any);
	const { data: apiDbs } = $api.useQuery('get', '/databases' as any, {} as any);
	const { data: apiComposes } = $api.useQuery('get', '/composes' as any, {} as any);

	const {
		filteredAndSorted: deploymentsList,
		isLoading: isDeploymentsLoading,
		selectedDeployment,
		setSelectedDeployment,
		logs,
		isLogsLoading,
		copied,
		handleCopyLogs,
	} = useDeployments();

	// Map project ID to Project Name for subtitle display
	const projectNameMap = useMemo(() => {
		const map = new Map<number, string>();
		projects.forEach((p) => {
			if (p.id) map.set(Number(p.id), p.name || `Project #${p.id}`);
		});
		return map;
	}, [projects]);

	// Extract all services across flat arrays AND nested project environments
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

		// 1. Process Flat Applications
		const appsList = storeApps?.length ? storeApps : (Array.isArray(apiApps) ? apiApps : []);
		appsList.forEach((app: any) => {
			const id = Number(app.id);
			if (!id) return;
			const pId = Number(app.project_id || app.projectId || 1);
			const projName = projectNameMap.get(pId) || 'Default Project';
			serviceMap.set(`APP-${id}`, {
				projectId: pId,
				type: 'APP',
				id,
				name: app.name || app.app_name || `App #${id}`,
				subtitle: `${projName} / production`,
				status: app.application_status || app.status || 'idle',
				createdAt: app.created_at || Date.now() / 1000,
			});
		});

		// 2. Process Flat Compose Stacks
		const composesList = storeComposes?.length ? storeComposes : (Array.isArray(apiComposes) ? apiComposes : []);
		composesList.forEach((comp: any) => {
			const id = Number(comp.id);
			if (!id) return;
			const pId = Number(comp.project_id || comp.projectId || 1);
			const projName = projectNameMap.get(pId) || 'Default Project';
			serviceMap.set(`COMPOSE-${id}`, {
				projectId: pId,
				type: 'COMPOSE',
				id,
				name: comp.name || comp.app_name || `Compose #${id}`,
				subtitle: `${projName} / production`,
				status: comp.compose_status || comp.status || 'idle',
				createdAt: comp.created_at || Date.now() / 1000,
			});
		});

		// 3. Process Flat Databases
		const dbsList = storeDbs?.length ? storeDbs : (Array.isArray(apiDbs) ? apiDbs : []);
		dbsList.forEach((db: any) => {
			const id = Number(db.id);
			if (!id) return;
			const pId = Number(db.project_id || db.projectId || 1);
			const projName = projectNameMap.get(pId) || 'Default Project';
			const dbKind = (db.db_type || db.type || db.database_type || 'postgres').toLowerCase();
			serviceMap.set(`DATABASE-${id}`, {
				projectId: pId,
				type: 'DATABASE',
				id,
				name: db.name || db.database_name || `DB #${id}`,
				subtitle: `${projName} / production`,
				status: db.database_status || db.status || 'idle',
				dbKind,
				createdAt: db.created_at || Date.now() / 1000,
			});
		});

		// 4. Process Nested Projects -> Environments
		projects.forEach((proj: any) => {
			const envs = proj.environments || [];
			const pId = Number(proj.id);
			const projName = proj.name || `Project #${pId}`;

			envs.forEach((env: any) => {
				const envName = env.name || 'production';
				// Applications
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
				// Compose
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
				// Postgres
				(env.postgreses || []).forEach((db: any) => {
					const id = Number(db.id);
					if (id && !serviceMap.has(`DATABASE-${id}`)) {
						serviceMap.set(`DATABASE-${id}`, {
							projectId: pId,
							type: 'DATABASE',
							id,
							name: db.name || `Postgres #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: db.database_status || db.status || 'idle',
							dbKind: 'postgres',
							createdAt: db.created_at || Date.now() / 1000,
						});
					}
				});
				// MySQL
				(env.mysqls || []).forEach((db: any) => {
					const id = Number(db.id);
					if (id && !serviceMap.has(`DATABASE-${id}`)) {
						serviceMap.set(`DATABASE-${id}`, {
							projectId: pId,
							type: 'DATABASE',
							id,
							name: db.name || `MySQL #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: db.database_status || db.status || 'idle',
							dbKind: 'mysql',
							createdAt: db.created_at || Date.now() / 1000,
						});
					}
				});
				// MariaDB
				(env.mariadbs || []).forEach((db: any) => {
					const id = Number(db.id);
					if (id && !serviceMap.has(`DATABASE-${id}`)) {
						serviceMap.set(`DATABASE-${id}`, {
							projectId: pId,
							type: 'DATABASE',
							id,
							name: db.name || `MariaDB #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: db.database_status || db.status || 'idle',
							dbKind: 'mariadb',
							createdAt: db.created_at || Date.now() / 1000,
						});
					}
				});
				// Mongo
				(env.mongos || []).forEach((db: any) => {
					const id = Number(db.id);
					if (id && !serviceMap.has(`DATABASE-${id}`)) {
						serviceMap.set(`DATABASE-${id}`, {
							projectId: pId,
							type: 'DATABASE',
							id,
							name: db.name || `Mongo #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: db.database_status || db.status || 'idle',
							dbKind: 'mongo',
							createdAt: db.created_at || Date.now() / 1000,
						});
					}
				});
				// Redis
				(env.redises || []).forEach((db: any) => {
					const id = Number(db.id);
					if (id && !serviceMap.has(`DATABASE-${id}`)) {
						serviceMap.set(`DATABASE-${id}`, {
							projectId: pId,
							type: 'DATABASE',
							id,
							name: db.name || `Redis #${id}`,
							subtitle: `${projName} / ${envName}`,
							status: db.database_status || db.status || 'idle',
							dbKind: 'redis',
							createdAt: db.created_at || Date.now() / 1000,
						});
					}
				});
			});
		});

		return Array.from(serviceMap.values());
	}, [projects, storeApps, storeDbs, storeComposes, apiApps, apiDbs, apiComposes, projectNameMap]);

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
			{/* Page Title */}
			<div className="flex flex-col gap-1">
				<h1 className="text-2xl font-bold text-foreground tracking-tight">Platform Overview</h1>
				<p className="text-xs text-muted-foreground">
					Centralized management of all services, deployments, domains, and backups across your organization
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
							{storeDomains.length}
						</Badge>
					</TabsTrigger>
					<TabsTrigger value="backups" className="pb-2.5 text-xs font-semibold flex items-center gap-2">
						<Clock className="size-4" />
						Volume Backups
						<Badge variant="secondary" className="text-[10px] font-mono px-1.5 py-0">
							{storeBackups.length}
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
					{storeDomains.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Globe className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No active domains configured</p>
							<p className="text-xs text-muted-foreground">Configure custom domain routes in application settings</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{storeDomains.map((dom) => (
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
					{storeBackups.length === 0 ? (
						<div className="flex flex-col items-center justify-center p-12 border border-dashed rounded-xl text-center">
							<Clock className="size-8 text-muted-foreground/40 mb-2" />
							<p className="text-sm font-semibold text-foreground">No volume backups found</p>
							<p className="text-xs text-muted-foreground">Scheduled volume backups will appear here</p>
						</div>
					) : (
						<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
							{storeBackups.map((b) => (
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
				isLogsLoading={isLogsLoading}
				copied={copied}
				onCopyLogs={handleCopyLogs}
			/>
		</div>
	);
}
