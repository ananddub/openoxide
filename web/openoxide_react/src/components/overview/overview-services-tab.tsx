import React, { useState, useEffect } from 'react';
import { Box, Search, Folder, Building2 } from 'lucide-react';
import { Input } from '#/components/ui/input';
import { Button } from '#/components/ui/button';
import { ServiceCard } from '#/components/projects/service/service-card';
import { useOrganizationStore } from '#/stores/organization-store';
import {
	useProjectListByOrganization,
	useEnvironmentListByProject,
	useApplicationListByEnvironment,
	useComposeListByEnvironment,
	usePostgresListByEnvironment,
	useMysqlListByEnvironment,
	useMariadbListByEnvironment,
	useMongoListByEnvironment,
	useRedisListByEnvironment,
} from 'virtual:openoxide-live';

export type AggregatedService = {
	key: string;
	projectId: number;
	type: 'APP' | 'COMPOSE' | 'DATABASE';
	id: number;
	name: string;
	subtitle: string;
	status: string;
	createdAt: number;
	dbKind?: string;
};

// Component for a single Environment that queries all services reactively
function EnvironmentServicesFetcher({
	projectId,
	projName,
	envId,
	envName,
	onServicesUpdate,
}: {
	projectId: number;
	projName: string;
	envId: number;
	envName: string;
	onServicesUpdate: (envId: number, services: AggregatedService[]) => void;
}) {
	const { data: apps = [] } = useApplicationListByEnvironment(BigInt(envId));
	const { data: composes = [] } = useComposeListByEnvironment(BigInt(envId));
	const { data: pgDbs = [] } = usePostgresListByEnvironment(BigInt(envId));
	const { data: myDbs = [] } = useMysqlListByEnvironment(BigInt(envId));
	const { data: mariaDbs = [] } = useMariadbListByEnvironment(BigInt(envId));
	const { data: mongoDbs = [] } = useMongoListByEnvironment(BigInt(envId));
	const { data: redisDbs = [] } = useRedisListByEnvironment(BigInt(envId));

	useEffect(() => {
		const list: AggregatedService[] = [];

		(apps || []).forEach((app: any) => {
			const id = Number(app.id);
			if (id) {
				list.push({
					key: `APP-${id}`,
					projectId,
					type: 'APP',
					id,
					name: app.name || app.app_name || `App #${id}`,
					subtitle: `${projName} / ${envName}`,
					status: app.application_status || app.status || 'idle',
					createdAt: app.created_at || Date.now() / 1000,
				});
			}
		});

		(composes || []).forEach((comp: any) => {
			const id = Number(comp.id);
			if (id) {
				list.push({
					key: `COMPOSE-${id}`,
					projectId,
					type: 'COMPOSE',
					id,
					name: comp.name || comp.app_name || `Compose #${id}`,
					subtitle: `${projName} / ${envName}`,
					status: comp.compose_status || comp.status || 'idle',
					createdAt: comp.created_at || Date.now() / 1000,
				});
			}
		});

		const dbsWithKind = [
			...(pgDbs || []).map((d: any) => ({ ...d, kind: 'postgres' })),
			...(myDbs || []).map((d: any) => ({ ...d, kind: 'mysql' })),
			...(mariaDbs || []).map((d: any) => ({ ...d, kind: 'mariadb' })),
			...(mongoDbs || []).map((d: any) => ({ ...d, kind: 'mongo' })),
			...(redisDbs || []).map((d: any) => ({ ...d, kind: 'redis' })),
		];

		dbsWithKind.forEach((db: any) => {
			const id = Number(db.id);
			if (id) {
				list.push({
					key: `DATABASE-${id}`,
					projectId,
					type: 'DATABASE',
					id,
					name: db.name || db.database_name || `DB #${id}`,
					subtitle: `${projName} / ${envName}`,
					status: db.database_status || db.status || 'idle',
					dbKind: db.kind,
					createdAt: db.created_at || Date.now() / 1000,
				});
			}
		});

		onServicesUpdate(envId, list);
	}, [apps, composes, pgDbs, myDbs, mariaDbs, mongoDbs, redisDbs, projectId, projName, envId, envName]);

	return null;
}

// Component for a single Project that queries all environments reactively
function ProjectFetcher({
	projectId,
	projName,
	onServicesUpdate,
}: {
	projectId: number;
	projName: string;
	onServicesUpdate: (envId: number, services: AggregatedService[]) => void;
}) {
	const { data: envs = [] } = useEnvironmentListByProject(BigInt(projectId));

	return (
		<>
			{(envs || []).map((env: any) => (
				<EnvironmentServicesFetcher
					key={env.id}
					projectId={projectId}
					projName={projName}
					envId={Number(env.id)}
					envName={env.name || 'production'}
					onServicesUpdate={onServicesUpdate}
				/>
			))}
		</>
	);
}

export function OverviewServicesTab() {
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');
	const [selectedProjectId, setSelectedProjectId] = useState<string>('all');
	const [envServicesMap, setEnvServicesMap] = useState<Record<number, AggregatedService[]>>({});

	const { organizations, activeOrg, setActiveOrg } = useOrganizationStore();
	const orgId = activeOrg?.id || 1;

	const { data: projectsToUse = [] } = useProjectListByOrganization(BigInt(orgId));

	const handleServicesUpdate = (envId: number, services: AggregatedService[]) => {
		setEnvServicesMap((prev) => {
			if (JSON.stringify(prev[envId]) === JSON.stringify(services)) return prev;
			return { ...prev, [envId]: services };
		});
	};

	// Aggregate all services across all envs
	const allServices = React.useMemo(() => {
		const map = new Map<string, AggregatedService>();
		Object.values(envServicesMap).forEach((list) => {
			list.forEach((s) => map.set(s.key, s));
		});
		return Array.from(map.values());
	}, [envServicesMap]);

	// Filtered services
	const filteredServices = React.useMemo(() => {
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

			const matchesProject =
				selectedProjectId === 'all' || String(s.projectId) === String(selectedProjectId);

			return matchesSearch && matchesType && matchesProject;
		});
	}, [allServices, searchQuery, typeFilter, selectedProjectId]);

	return (
		<div className="space-y-4">
			{/* Mount Reactive Stream Fetchers for each Project */}
			{(projectsToUse || []).map((proj: any) => (
				<ProjectFetcher
					key={proj.id}
					projectId={Number(proj.id)}
					projName={proj.name || `Project #${proj.id}`}
					onServicesUpdate={handleServicesUpdate}
				/>
			))}

			{/* Organization Context & Project Selector Bar */}
			<div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 p-3 rounded-xl bg-card border border-border/60">
				<div className="flex items-center gap-2">
					<Building2 className="size-4 text-primary" />
					<span className="text-xs font-bold text-foreground">Organization:</span>
					<span className="text-xs font-mono font-medium text-muted-foreground">{activeOrg?.name || 'Default Organization'}</span>
					{organizations.length > 1 && (
						<select
							value={activeOrg?.id || 1}
							onChange={(e) => {
								const selected = organizations.find((o) => String(o.id) === e.target.value);
								if (selected) setActiveOrg(selected);
							}}
							className="h-7 text-xs bg-muted border border-border rounded px-2 text-foreground font-semibold cursor-pointer"
						>
							{organizations.map((org) => (
								<option key={org.id} value={org.id}>
									{org.name}
								</option>
							))}
						</select>
					)}
				</div>

				<div className="flex items-center gap-2">
					<Folder className="size-4 text-muted-foreground" />
					<span className="text-xs font-bold text-foreground">Filter by Project:</span>
					<select
						value={selectedProjectId}
						onChange={(e) => setSelectedProjectId(e.target.value)}
						className="h-7 text-xs bg-muted border border-border rounded px-2 text-foreground font-semibold cursor-pointer"
					>
						<option value="all">All Projects ({projectsToUse.length})</option>
						{projectsToUse.map((p: any) => (
							<option key={p.id} value={p.id}>
								{p.name}
							</option>
						))}
					</select>
				</div>
			</div>

			{/* Search & Filter Toolbar */}
			<div className="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-4">
				<div className="relative flex-1 max-w-md">
					<Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
					<Input
						placeholder="Search services by name or project..."
						value={searchQuery}
						onChange={(e) => setSearchQuery(e.target.value)}
						className="pl-9 h-9 text-xs"
					/>
				</div>
				<div className="flex items-center gap-2 flex-wrap">
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
						{searchQuery ? 'Try clearing your search query' : 'Create an application, compose stack, or database inside a project'}
					</p>
				</div>
			) : (
				<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
					{filteredServices.map((svc) => (
						<ServiceCard
							key={svc.key}
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
		</div>
	);
}
