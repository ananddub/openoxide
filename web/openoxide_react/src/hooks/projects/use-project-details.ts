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

	// 0ms Instant Zustand Store Reads (ZERO Extra WebSockets or HTTP Requests!)
	const storeProject = useAppStore((state) =>
		state.projects.find((p) => String(p.id) === String(projectId))
	);
	const servers = useAppStore((state) => state.servers);
	const overviewServices = useAppStore((state) => state.overviewServices);

	// Queries
	const {data: liveProject} = useProjectGet(BigInt(projectId));
	const project = liveProject || storeProject;
	const {data: envs = []} = useEnvironmentListByProject(BigInt(projectId));

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

	// Instant Filtered Services directly from Realtime Zustand Store
	const projectServices = useMemo(() => {
		return (overviewServices || []).filter((s: any) => {
			if (!s || String(s.project_id) !== String(projectId)) return false;
			if (activeEnvId !== null && s.environment_id !== undefined && s.environment_id !== null) {
				return Number(s.environment_id) === Number(activeEnvId);
			}
			return true;
		});
	}, [overviewServices, projectId, activeEnvId]);

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
			const rawType = String(s?.service_type || s?.serviceType || s?.type || s?.kind || 'APP').toUpperCase();
			const dbKind = s?.db_kind || s?.dbKind || s?.kind;
			return {
				key: `${rawType.toLowerCase()}-${dbKind || 'svc'}-${s.id}`,
				projectId,
				type: rawType as 'APP' | 'COMPOSE' | 'DATABASE',
				id: s.id,
				name: String(s?.name || s?.app_name || s?.appName || ''),
				subtitle: rawType === 'APP' ? 'Application' : rawType === 'COMPOSE' ? 'Docker Compose' : (dbKind ? String(dbKind).toLowerCase() : 'Database'),
				status: String(s?.status || s?.app_status || 'idle'),
				createdAt: s?.createdAt || s?.created_at,
				dbKind: dbKind ? String(dbKind).toLowerCase() : undefined,
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
