import React, { useState, useMemo } from 'react';
import { Box, Search, Folder, Building2 } from 'lucide-react';
import { Input } from '#/components/ui/input';
import { Button } from '#/components/ui/button';
import { ServiceCard } from '#/components/projects/service/service-card';
import { useOrganizationStore } from '#/stores/organization-store';
import { $api } from '#/api/query';
import { useProjectListByOrganization } from 'virtual:openoxide-live';

export function OverviewServicesTab() {
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');
	const [selectedProjectId, setSelectedProjectId] = useState<string>('all');

	const { organizations, activeOrg, setActiveOrg } = useOrganizationStore();
	const orgId = activeOrg?.id || 1;

	// Live Projects for filter dropdown
	const { data: projectsToUse = [] } = useProjectListByOrganization(BigInt(orgId));

	// Single backend endpoint query for ALL services in the organization
	const { data: rawServices, isLoading } = $api.useQuery('get', '/overview/services/organization/{organization_id}', {
		params: {
			path: {
				organization_id: orgId,
			},
		},
	});

	// Transform API services list for rendering
	const allServices = useMemo(() => {
		if (!rawServices || !Array.isArray(rawServices)) return [];

		return rawServices.map((svc: any) => ({
			key: `${svc.service_type}-${svc.id}`,
			projectId: Number(svc.project_id),
			type: svc.service_type as 'APP' | 'COMPOSE' | 'DATABASE',
			id: Number(svc.id),
			name: svc.name,
			subtitle: `${svc.project_name} / ${svc.environment_name}`,
			status: svc.status || 'idle',
			createdAt: Number(svc.created_at),
			dbKind: svc.db_kind || undefined,
		}));
	}, [rawServices]);

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
			{isLoading ? (
				<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
					{[1, 2, 3].map((i) => (
						<div key={i} className="h-36 animate-pulse rounded-xl bg-card border border-border/40" />
					))}
				</div>
			) : filteredServices.length === 0 ? (
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
