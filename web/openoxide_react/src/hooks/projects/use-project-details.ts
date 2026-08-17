import {useState, useEffect, useMemo} from 'react';
import {
	useProjectGet,
	useEnvironmentListByProject,
} from 'virtual:openoxide-live';

import { useAppStore } from '#/stores/app-store';

export function useProjectDetails(projectId: number) {
	// Modals State
	const [showCreateEnv, setShowCreateEnv] = useState(false);
	const [showProjectEnv, setShowProjectEnv] = useState(false);
	const [showEnvVars, setShowEnvVars] = useState(false);
	const [showCreateApp, setShowCreateApp] = useState(false);
	const [showCreateCompose, setShowCreateCompose] = useState(false);
	const [showCreateDatabase, setShowCreateDatabase] = useState(false);

	// 0ms Instant Zustand Store Reads for Applications, Databases, Compose, and Overview Services
	const storeProject = useAppStore((state) =>
		state.projects.find((p) => String(p.id) === String(projectId))
	);
	const servers = useAppStore((state) => state.servers);
	const applications = useAppStore((state) => state.applications || []);
	const databases = useAppStore((state) => state.databases || []);
	const composes = useAppStore((state) => state.composes || []);
	const overviewServices = useAppStore((state) => state.overviewServices || []);

	// Queries
	const {data: liveProject} = useProjectGet(BigInt(projectId));
	const project = liveProject || storeProject;
	const {data: envs = []} = useEnvironmentListByProject(BigInt(projectId));

	// Read URL search params for persistent environment selection across navigations
	const urlEnvId = typeof window !== 'undefined' ? (() => {
		const val = new URLSearchParams(window.location.search).get('env');
		return val && !isNaN(Number(val)) ? Number(val) : null;
	})() : null;

	const [userSelectedEnvId, setUserSelectedEnvIdState] = useState<number | null>(urlEnvId);

	const setUserSelectedEnvId = (envId: number | null) => {
		setUserSelectedEnvIdState(envId);
		if (typeof window !== 'undefined' && envId) {
			const url = new URL(window.location.href);
			url.searchParams.set('env', String(envId));
			window.history.replaceState({}, '', url.toString());
		}
	};

	// Reset user selected environment choice whenever projectId changes
	useEffect(() => {
		setUserSelectedEnvIdState(null);
	}, [projectId]);

	// Synchronously compute active environment ID during render (0ms delay)
	const activeEnvId = useMemo(() => {
		if (envs.length === 0) return null;
		if (userSelectedEnvId !== null && envs.some(e => Number(e.id) === Number(userSelectedEnvId))) {
			return userSelectedEnvId;
		}
		if (urlEnvId !== null && envs.some(e => Number(e.id) === Number(urlEnvId))) {
			return urlEnvId;
		}
		const def = envs.find(e => e.is_default) || envs[0];
		return def ? Number(def.id) : null;
	}, [envs, userSelectedEnvId, urlEnvId]);

	const selectedEnv = useMemo(
		() => (activeEnvId !== null ? envs.find(e => Number(e.id) === Number(activeEnvId)) || null : null),
		[envs, activeEnvId]
	);

	// Instant Filtered Services directly from Realtime Zustand Store (Applications + Compose + Databases + Overview)
	const projectServices = useMemo(() => {
		const result: any[] = [];
		const seenKeys = new Set<string>();

		// 1. Applications from Zustand store
		// 1. Applications from project query or Zustand store
		const allApps = [
			...(applications || []),
			...((project as any)?.applications || []),
		];
		allApps.forEach((app: any) => {
			if (String(app.project_id || app.projectId || projectId) === String(projectId)) {
				const envId = app.environment_id ?? app.environmentId;
				if (activeEnvId === null || envId === undefined || envId === null || Number(envId) === Number(activeEnvId)) {
					const key = `APP-${app.id}`;
					if (!seenKeys.has(key)) {
						seenKeys.add(key);
						const status = app.app_status || app.status || 'STOPPED';
						result.push({
							id: app.id,
							type: 'APP',
							name: app.name || app.app_name,
							subtitle: app.app_name || app.name,
							status,
							createdAt: app.created_at || Date.now(),
							projectId: Number(app.project_id || projectId),
							environmentId: envId,
						});
					}
				}
			}
		});

		// 2. Compose stacks from project query or Zustand store
		const allComposes = [
			...(composes || []),
			...((project as any)?.composes || []),
		];
		allComposes.forEach((c: any) => {
			if (String(c.project_id || c.projectId || projectId) === String(projectId)) {
				const envId = c.environment_id ?? c.environmentId;
				if (activeEnvId === null || envId === undefined || envId === null || Number(envId) === Number(activeEnvId)) {
					const key = `COMPOSE-${c.id}`;
					if (!seenKeys.has(key)) {
						seenKeys.add(key);
						const status = c.compose_status || c.status || 'STOPPED';
						result.push({
							id: c.id,
							type: 'COMPOSE',
							name: c.name || c.app_name,
							subtitle: c.app_name || c.name,
							status,
							createdAt: c.created_at || Date.now(),
							projectId: Number(c.project_id || projectId),
							environmentId: envId,
						});
					}
				}
			}
		});

		// 3. Databases from project query or Zustand store
		const projectDbs: any[] = [];
		if (project) {
			if (Array.isArray((project as any).postgresDbs)) projectDbs.push(...(project as any).postgresDbs.map((d: any) => ({ ...d, kind: 'postgres' })));
			if (Array.isArray((project as any).mysqlDbs)) projectDbs.push(...(project as any).mysqlDbs.map((d: any) => ({ ...d, kind: 'mysql' })));
			if (Array.isArray((project as any).mariadbDbs)) projectDbs.push(...(project as any).mariadbDbs.map((d: any) => ({ ...d, kind: 'mariadb' })));
			if (Array.isArray((project as any).mongoDbs)) projectDbs.push(...(project as any).mongoDbs.map((d: any) => ({ ...d, kind: 'mongo' })));
			if (Array.isArray((project as any).redisDbs)) projectDbs.push(...(project as any).redisDbs.map((d: any) => ({ ...d, kind: 'redis' })));
			if (Array.isArray((project as any).libsqlDbs)) projectDbs.push(...(project as any).libsqlDbs.map((d: any) => ({ ...d, kind: 'libsql' })));
		}
		const allDbs = [...(databases || []), ...projectDbs];
		allDbs.forEach((db: any) => {
			if (String(db.project_id || db.projectId || projectId) === String(projectId)) {
				const envId = db.environment_id ?? db.environmentId;
				if (activeEnvId === null || envId === undefined || envId === null || Number(envId) === Number(activeEnvId)) {
					const dbKind = db.kind || db.type || db.db_kind || db.dbKind || 'postgres';
					const key = `DATABASE-${dbKind}-${db.id}`;
					if (!seenKeys.has(key)) {
						seenKeys.add(key);
						const status = db.app_status || db.status || 'STOPPED';
						result.push({
							id: db.id,
							type: 'DATABASE',
							name: db.name || db.app_name,
							subtitle: dbKind.toUpperCase(),
							status,
							createdAt: db.created_at || Date.now(),
							dbKind,
							projectId: Number(db.project_id || projectId),
							environmentId: envId,
						});
					}
				}
			}
		});

		// 4. Fallback to Overview Services for any items not in direct stores
		overviewServices.forEach((s: any) => {
			if (!s || String(s.project_id || s.projectId) !== String(projectId)) return;
			const envId = s.environment_id ?? s.environmentId;
			if (activeEnvId !== null && envId !== undefined && envId !== null && Number(envId) !== Number(activeEnvId)) return;

			const sType = String(s.service_type || s.type || '').toUpperCase();
			const isDb = sType === 'DATABASE' || sType.includes('DB') || !!s.db_kind || !!s.dbKind;
			const isCompose = sType === 'COMPOSE' || sType.includes('COMPOSE');
			const normalizedType = isDb ? 'DATABASE' : isCompose ? 'COMPOSE' : 'APP';
			const dbKind = isDb ? (s.db_kind || s.dbKind || 'postgres') : undefined;
			const key = isDb ? `DATABASE-${dbKind}-${s.id}` : `${normalizedType}-${s.id}`;

			if (!seenKeys.has(key)) {
				seenKeys.add(key);
				result.push({
					id: s.id,
					type: normalizedType,
					name: s.name,
					subtitle: isDb ? (dbKind ? dbKind.toUpperCase() : 'DATABASE') : (s.name || s.app_name),
					status: s.status || s.app_status || 'STOPPED',
					createdAt: s.created_at || Date.now(),
					...(dbKind ? { dbKind } : {}),
					projectId: Number(s.project_id || projectId),
					environmentId: envId,
				});
			}
		});

		return result;
	}, [applications, databases, composes, overviewServices, project, projectId, activeEnvId]);

	// Live hooks auto-push updates — no manual refetch needed
	const handleRefresh = () => {};

	// 0ms instant loading state — never block UI
	const isLoading = false;
	const totalServices = projectServices.length;

	// Filter State
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');
	const [statusFilter, setStatusFilter] = useState('all');

	// Reset filters when environment changes
	useEffect(() => {
		setSearchQuery('');
		setTypeFilter('all');
		setStatusFilter('all');
	}, [activeEnvId]);

	// Filter Services Logic
	const filteredServices = useMemo(() => {
		const sQuery = String(searchQuery || '').toLowerCase().trim();

		let list = projectServices.map((s: any) => {
			const rawKind = String(s?.kind || s?.db_kind || s?.dbKind || '').toLowerCase();
			const isDb = s?.type === 'database' || s?.type === 'DATABASE' || ['postgres', 'mysql', 'mariadb', 'mongo', 'redis', 'libsql'].includes(rawKind);
			const rawType = isDb ? 'DATABASE' : String(s?.service_type || s?.serviceType || s?.type || 'APP').toUpperCase();
			const dbKind = rawKind || (isDb ? 'postgres' : undefined);
			return {
				key: `${rawType.toLowerCase()}-${dbKind || 'svc'}-${s.id}`,
				projectId,
				type: rawType as 'APP' | 'COMPOSE' | 'DATABASE',
				id: s.id,
				name: String(s?.name || s?.app_name || s?.appName || ''),
				subtitle: rawType === 'APP' ? 'Application' : rawType === 'COMPOSE' ? 'Docker Compose' : (dbKind ? dbKind.toUpperCase() : 'Database'),
				status: String(s?.status || s?.app_status || 'idle'),
				createdAt: s?.createdAt || s?.created_at,
				dbKind: dbKind,
			};
		});

		// Apply Search
		if (sQuery) {
			list = list.filter((item: any) => String(item?.name || '').toLowerCase().includes(sQuery));
		}

		// Apply Type Filter
		if (typeFilter !== 'all') {
			list = list.filter((item: any) => {
				const itemType = String(item?.type || '').toLowerCase();
				if (typeFilter === 'app') return itemType === 'app';
				if (typeFilter === 'compose') return itemType === 'compose';
				if (typeFilter === 'database') return itemType === 'database';
				return true;
			});
		}

		// Apply Status Filter
		if (statusFilter !== 'all') {
			list = list.filter((item: any) => {
				const st = String(item?.status || '').toLowerCase();
				const isRunning =
					st.includes('running') ||
					st.includes('active') ||
					st.includes('healthy') ||
					st.includes('up');

				return statusFilter === 'running' ? isRunning : !isRunning;
			});
		}

		return list;
	}, [projectServices, searchQuery, typeFilter, statusFilter, projectId]);

	return {
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
		selectedEnvId: activeEnvId,
		setSelectedEnvId: setUserSelectedEnvId,
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
		refetchEnvs: () => {},
	};
}
