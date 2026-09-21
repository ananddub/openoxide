import {useState, useEffect, useMemo} from 'react';
import {
	useProjectGet,
	useEnvironmentListByProject,
	useRemoteServerList,
	useApplicationList,
	useDatabaseList,
	useComposeList,
	useOverviewServices,
} from '#/api/live';
import {
	buildProjectServices,
	makeGetServerName,
} from './use-project-details.mappers';
import {buildFilteredServices} from './use-project-details.filters';

export function useProjectDetails(projectId: number) {
	// Modals State
	const [showCreateEnv, setShowCreateEnv] = useState(false);
	const [showProjectEnv, setShowProjectEnv] = useState(false);
	const [showEnvVars, setShowEnvVars] = useState(false);
	const [showCreateApp, setShowCreateApp] = useState(false);
	const [showCreateCompose, setShowCreateCompose] = useState(false);
	const [showCreateDatabase, setShowCreateDatabase] = useState(false);

	// Org-wide service lists as live queries (table-watch keeps them
	// fresh); shapes mirror the legacy store slices.
	const serversQuery = useRemoteServerList();
	const servers = serversQuery.data ?? [];
	const applications = useApplicationList().data ?? [];
	const databases = useDatabaseList().data ?? [];
	const composes = useComposeList().data ?? [];
	const overviewServices = useOverviewServices().data ?? [];
	const getServerName = useMemo(
		() => makeGetServerName(servers),
		[servers],
	);

	// Queries — live project query keyed ['projects','get',id]; the
	// table-watch stream invalidates it on project changes while mounted.
	const {data: liveProject} = useProjectGet(BigInt(projectId));
	const project = liveProject || null;
	const {data: envs = []} = useEnvironmentListByProject(BigInt(projectId));

	// Read URL search params for persistent environment selection across navigations
	const urlEnvId =
		typeof window !== 'undefined'
			? (() => {
					const val = new URLSearchParams(window.location.search).get(
						'env',
					);
					return val && !isNaN(Number(val)) ? Number(val) : null;
				})()
			: null;

	const [userSelectedEnvId, setUserSelectedEnvIdState] = useState<
		number | null
	>(urlEnvId);

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
		if (
			userSelectedEnvId !== null &&
			envs.some(e => Number(e.id) === Number(userSelectedEnvId))
		) {
			return userSelectedEnvId;
		}
		if (
			urlEnvId !== null &&
			envs.some(e => Number(e.id) === Number(urlEnvId))
		) {
			return urlEnvId;
		}
		const def = envs.find(e => e.is_default) || envs[0];
		return def ? Number(def.id) : null;
	}, [envs, userSelectedEnvId, urlEnvId]);

	const selectedEnv = useMemo(
		() =>
			activeEnvId !== null
				? envs.find(e => Number(e.id) === Number(activeEnvId)) || null
				: null,
		[envs, activeEnvId],
	);

	// Instant filtered services folded from the realtime live lists.
	const projectServices = useMemo(
		() =>
			buildProjectServices({
				applications,
				composes,
				databases,
				overviewServices,
				project,
				projectId,
				activeEnvId,
				getServerName,
			}),
		[
			applications,
			composes,
			databases,
			overviewServices,
			project,
			projectId,
			activeEnvId,
			getServerName,
		],
	);

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

	const filteredServices = useMemo(
		() =>
			buildFilteredServices({
				projectServices,
				searchQuery,
				typeFilter,
				statusFilter,
				projectId,
			}),
		[projectServices, searchQuery, typeFilter, statusFilter, projectId],
	);

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
