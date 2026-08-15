import {useState, useEffect, useMemo} from 'react';
import {
	useProjectGet,
	useEnvironmentListByProject,
	useRemoteServerList,
	useApplicationListByEnvironment,
	useComposeListByEnvironment,
	usePostgresListByEnvironment,
	useMysqlListByEnvironment,
	useMariadbListByEnvironment,
	useMongoListByEnvironment,
	useRedisListByEnvironment,
	useLibsqlListByEnvironment,
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

	// 0ms Instant Zustand Store Read
	const storeProject = useAppStore((state) =>
		state.projects.find((p) => String(p.id) === String(projectId))
	);

	// Queries
	const {data: liveProject} = useProjectGet(BigInt(projectId));
	const project = liveProject || storeProject;
	const {data: envs = []} = useEnvironmentListByProject(BigInt(projectId));
	const {data: servers = []} = useRemoteServerList();

	const [userSelectedEnvId, setUserSelectedEnvId] = useState<number | null>(null);

	// Reset user selected environment choice whenever projectId changes
	useEffect(() => {
		setUserSelectedEnvId(null);
	}, [projectId]);

	// Synchronously compute active environment ID during render (0ms delay)
	const activeEnvId = useMemo(() => {
		if (envs.length === 0) return null;
		if (userSelectedEnvId !== null && envs.some(e => Number(e.id) === Number(userSelectedEnvId))) {
			return userSelectedEnvId;
		}
		const def = envs.find(e => e.is_default) || envs[0];
		return def ? Number(def.id) : null;
	}, [envs, userSelectedEnvId]);

	const selectedEnv = useMemo(
		() => (activeEnvId !== null ? envs.find(e => Number(e.id) === Number(activeEnvId)) || null : null),
		[envs, activeEnvId]
	);

	// Fetch environment services with instant activeEnvId
	const envId = activeEnvId || 0;
	const {data: apps, loading: isLoadingApps} = useApplicationListByEnvironment(BigInt(envId));
	const {data: composes, loading: isLoadingComposes} = useComposeListByEnvironment(BigInt(envId));

	const {data: pgDbs} = usePostgresListByEnvironment(BigInt(envId));
	const {data: myDbs} = useMysqlListByEnvironment(BigInt(envId));
	const {data: mariaDbs} = useMariadbListByEnvironment(BigInt(envId));
	const {data: mongoDbs} = useMongoListByEnvironment(BigInt(envId));
	const {data: redisDbs} = useRedisListByEnvironment(BigInt(envId));
	const {data: libsqlDbs} = useLibsqlListByEnvironment(BigInt(envId));

	const databases = useMemo(() => {
		const list: Record<string, unknown>[] = [];
		if (pgDbs) list.push(...pgDbs.map(d => ({...d, kind: 'postgres'})));
		if (myDbs) list.push(...myDbs.map(d => ({...d, kind: 'mysql'})));
		if (mariaDbs) list.push(...mariaDbs.map(d => ({...d, kind: 'mariadb'})));
		if (mongoDbs) list.push(...mongoDbs.map(d => ({...d, kind: 'mongo'})));
		if (redisDbs) list.push(...redisDbs.map(d => ({...d, kind: 'redis'})));
		if (libsqlDbs) list.push(...libsqlDbs.map(d => ({...d, kind: 'libsql'})));
		return list;
	}, [pgDbs, myDbs, mariaDbs, mongoDbs, redisDbs, libsqlDbs]);

	// Live hooks auto-push updates — no manual refetch needed
	const handleRefresh = () => {};

	const isLoading = isLoadingApps || isLoadingComposes;
	const totalServices = (apps ?? []).length + (composes ?? []).length + databases.length;

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
		const sQuery = searchQuery.toLowerCase().trim();

		// Map apps
		const mappedApps = (apps ?? []).map(app => ({
			key: `app-${app.id}`,
			projectId,
			type: 'APP' as const,
			id: app.id,
			name: app.name,
			subtitle: 'Application',
			status: app.app_status || 'idle',
			createdAt: app.created_at,
		}));

		// Map composes
		const mappedComposes = (composes ?? []).map(compose => ({
			key: `compose-${compose.id}`,
			projectId,
			type: 'COMPOSE' as const,
			id: compose.id,
			name: compose.name,
			subtitle: 'Docker Compose',
			status: compose.compose_status || 'idle',
			createdAt: compose.created_at,
		}));

		// Map databases
		const mappedDatabases = databases.map(db => ({
			key: `database-${db.kind}-${db.id}`,
			projectId,
			type: 'DATABASE' as const,
			id: db.id,
			name: db.name,
			subtitle: db.kind,
			status: db.app_status || 'idle',
			createdAt: db.created_at,
			dbKind: db.kind,
		}));

		let list = [...mappedApps, ...mappedComposes, ...mappedDatabases];

		// Apply Search
		if (sQuery) {
			list = list.filter((item: any) => String(item.name || '').toLowerCase().includes(sQuery));
		}

		// Apply Type Filter
		if (typeFilter !== 'all') {
			list = list.filter((item: any) => {
				if (typeFilter === 'app') return item.type === 'APP';
				if (typeFilter === 'compose') return item.type === 'COMPOSE';
				if (typeFilter === 'database') return item.type === 'DATABASE';
				return true;
			});
		}

		// Apply Status Filter
		if (statusFilter !== 'all') {
			list = list.filter((item: any) => {
				const st = String(item.status || '').toLowerCase();
				const isRunning =
					st.includes('running') ||
					st.includes('active') ||
					st.includes('healthy') ||
					st.includes('up');

				return statusFilter === 'running' ? isRunning : !isRunning;
			});
		}

		return list;
	}, [apps, composes, databases, searchQuery, typeFilter, statusFilter, projectId]);

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
