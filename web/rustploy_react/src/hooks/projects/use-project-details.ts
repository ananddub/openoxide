import {useState, useEffect, useMemo} from 'react';
import {$api} from '#/api/query';

export function useProjectDetails(projectId: number) {
	// Modals State
	const [showCreateEnv, setShowCreateEnv] = useState(false);
	const [showProjectEnv, setShowProjectEnv] = useState(false);
	const [showEnvVars, setShowEnvVars] = useState(false);
	const [showCreateApp, setShowCreateApp] = useState(false);
	const [showCreateCompose, setShowCreateCompose] = useState(false);
	const [showCreateDatabase, setShowCreateDatabase] = useState(false);

	// Queries
	const {data: project, refetch: refetchProject} = $api.useQuery('get', '/projects/{id}', {params: {path: {id: projectId}}});
	const {data: envs = [], refetch: refetchEnvs} = $api.useQuery('get', '/environments/project/{project_id}', {params: {path: {project_id: projectId}}});
	const {data: servers = []} = $api.useQuery('get', '/remote-servers', {});

	const [selectedEnvId, setSelectedEnvId] = useState<number | null>(null);

	// Auto-select default/first environment
	useEffect(() => {
		if (envs.length > 0 && !selectedEnvId) {
			const def = envs.find(e => e.is_default) || envs[0];
			setSelectedEnvId(def.id);
		}
	}, [envs, selectedEnvId]);

	const selectedEnv = useMemo(() => envs.find(e => e.id === selectedEnvId) || null, [envs, selectedEnvId]);

	// Fetch environment services
	const envId = selectedEnvId || 0;
	const {data: apps = [], isLoading: isLoadingApps, refetch: refetchApps} = $api.useQuery('get', '/applications/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});
	const {data: composes = [], isLoading: isLoadingComposes, refetch: refetchComposes} = $api.useQuery('get', '/compose/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});

	const {data: pgDbs = [], refetch: refetchPg} = $api.useQuery('get', '/postgres/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});
	const {data: myDbs = [], refetch: refetchMy} = $api.useQuery('get', '/mysql/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});
	const {data: mariaDbs = [], refetch: refetchMaria} = $api.useQuery('get', '/mariadb/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});
	const {data: mongoDbs = [], refetch: refetchMongo} = $api.useQuery('get', '/mongo/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});
	const {data: redisDbs = [], refetch: refetchRedis} = $api.useQuery('get', '/redis/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});
	const {data: libsqlDbs = [], refetch: refetchLibsql} = $api.useQuery('get', '/libsql/environment/{environment_id}', {params: {path: {environment_id: envId}}}, {enabled: !!envId});

	const databases = useMemo(() => {
		const list: any[] = [];
		if (pgDbs) list.push(...pgDbs.map(d => ({...d, kind: 'postgres'})));
		if (myDbs) list.push(...myDbs.map(d => ({...d, kind: 'mysql'})));
		if (mariaDbs) list.push(...mariaDbs.map(d => ({...d, kind: 'mariadb'})));
		if (mongoDbs) list.push(...mongoDbs.map(d => ({...d, kind: 'mongo'})));
		if (redisDbs) list.push(...redisDbs.map(d => ({...d, kind: 'redis'})));
		if (libsqlDbs) list.push(...libsqlDbs.map(d => ({...d, kind: 'libsql'})));
		return list;
	}, [pgDbs, myDbs, mariaDbs, mongoDbs, redisDbs, libsqlDbs]);

	const handleRefresh = () => {
		refetchProject();
		refetchEnvs();
		if (envId) {
			refetchApps();
			refetchComposes();
			refetchPg();
			refetchMy();
			refetchMaria();
			refetchMongo();
			refetchRedis();
			refetchLibsql();
		}
	};

	const isLoading = isLoadingApps || isLoadingComposes;
	const totalServices = apps.length + composes.length + databases.length;

	// Filter State
	const [searchQuery, setSearchQuery] = useState('');
	const [typeFilter, setTypeFilter] = useState('all');
	const [statusFilter, setStatusFilter] = useState('all');

	// Reset filters when environment changes
	useEffect(() => {
		setSearchQuery('');
		setTypeFilter('all');
		setStatusFilter('all');
	}, [selectedEnvId]);

	// Filter Services Logic
	const filteredServices = useMemo(() => {
		const sQuery = searchQuery.toLowerCase().trim();

		// Map apps
		const mappedApps = apps.map(app => ({
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
		const mappedComposes = composes.map(compose => ({
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
			list = list.filter(item => item.name.toLowerCase().includes(sQuery));
		}

		// Apply Type Filter
		if (typeFilter !== 'all') {
			list = list.filter(item => {
				if (typeFilter === 'app') return item.type === 'APP';
				if (typeFilter === 'compose') return item.type === 'COMPOSE';
				if (typeFilter === 'database') return item.type === 'DATABASE';
				return true;
			});
		}

		// Apply Status Filter
		if (statusFilter !== 'all') {
			list = list.filter(item => {
				const isRunning =
					item.status.toLowerCase().includes('running') ||
					item.status.toLowerCase().includes('active') ||
					item.status.toLowerCase().includes('healthy') ||
					item.status.toLowerCase().includes('up');

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
	};
}
