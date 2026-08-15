import React, { useState, useMemo } from 'react';
import { Box, Search, Folder, Building2 } from 'lucide-react';
import { Input } from '#/components/ui/input';
import { Button } from '#/components/ui/button';
import { Badge } from '#/components/ui/badge';
import { ServiceCard } from '#/components/projects/service/service-card';
import { useAppStore } from '#/stores/app-store';
import { useOrganizationStore } from '#/stores/organization-store';
import { useProjectListByOrganization } from 'virtual:openoxide-live';

export function OverviewServicesTab() {
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');
	const [selectedProjectId, setSelectedProjectId] = useState<string>('all');

	const { organizations, activeOrg, setActiveOrg } = useOrganizationStore();
	const orgId = activeOrg?.id || 1;

	// Live Projects stream for active organization & Zustand RAM store fallback
	const { data: liveProjects } = useProjectListByOrganization(BigInt(orgId));
	const storeProjects = useAppStore((state) => state.projects);
	const storeApps = useAppStore((state) => state.applications);
	const storeDbs = useAppStore((state) => state.databases);
	const storeComposes = useAppStore((state) => state.composes);

	const projectsToUse = useMemo(() => {
		if (Array.isArray(liveProjects) && liveProjects.length > 0) return liveProjects;
		if (Array.isArray(storeProjects) && storeProjects.length > 0) return storeProjects;
		return [];
	}, [liveProjects, storeProjects]);

	// Extract all services across all projects and environments
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

		// 1. Process Projects and their Environments / Service arrays
		projectsToUse.forEach((proj: any) => {
			const pId = Number(proj.id) || 1;
			const projName = proj.name || `Project #${pId}`;

			// Top-level service arrays on project
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
				...(proj.postgresDbs || proj.postgreses || []).map((db: any) => ({ ...db, kind: 'postgres' })),
				...(proj.mysqlDbs || proj.mysqls || []).map((db: any) => ({ ...db, kind: 'mysql' })),
				...(proj.mariadbDbs || proj.mariadbs || []).map((db: any) => ({ ...db, kind: 'mariadb' })),
				...(proj.mongoDbs || proj.mongos || []).map((db: any) => ({ ...db, kind: 'mongo' })),
				...(proj.redisDbs || proj.redises || []).map((db: any) => ({ ...db, kind: 'redis' })),
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

			// Nested Environments
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

		// 2. Process Zustand RAM Store flat items
		(storeApps || []).forEach((app: any) => {
			const id = Number(app.id);
			if (id && !serviceMap.has(`APP-${id}`)) {
				const pId = Number(app.project_id || 1);
				serviceMap.set(`APP-${id}`, {
					projectId: pId,
					type: 'APP',
					id,
					name: app.name || app.app_name || `App #${id}`,
					subtitle: `Project #${pId} / production`,
					status: app.application_status || app.status || 'idle',
					createdAt: app.created_at || Date.now() / 1000,
				});
			}
		});

		(storeComposes || []).forEach((comp: any) => {
			const id = Number(comp.id);
			if (id && !serviceMap.has(`COMPOSE-${id}`)) {
				const pId = Number(comp.project_id || 1);
				serviceMap.set(`COMPOSE-${id}`, {
					projectId: pId,
					type: 'COMPOSE',
					id,
					name: comp.name || comp.app_name || `Compose #${id}`,
					subtitle: `Project #${pId} / production`,
					status: comp.compose_status || comp.status || 'idle',
					createdAt: comp.created_at || Date.now() / 1000,
				});
			}
		});

		(storeDbs || []).forEach((db: any) => {
			const id = Number(db.id);
			if (id && !serviceMap.has(`DATABASE-${id}`)) {
				const pId = Number(db.project_id || 1);
				serviceMap.set(`DATABASE-${id}`, {
					projectId: pId,
					type: 'DATABASE',
					id,
					name: db.name || db.database_name || `DB #${id}`,
					subtitle: `Project #${pId} / production`,
					status: db.database_status || db.status || 'idle',
					dbKind: (db.db_type || db.type || 'postgres').toLowerCase(),
					createdAt: db.created_at || Date.now() / 1000,
				});
			}
		});

		return Array.from(serviceMap.values());
	}, [projectsToUse, storeApps, storeDbs, storeComposes]);

	// Filtered services by search, type, AND project ID
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

			const matchesProject =
				selectedProjectId === 'all' || String(s.projectId) === String(selectedProjectId);

			return matchesSearch && matchesType && matchesProject;
		});
	}, [allServices, searchQuery, typeFilter, selectedProjectId]);

	return (
		<div className="space-y-4">
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
		</div>
	);
}
