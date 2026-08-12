import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';

export function useDatabaseDetail(dbId: number, targetKind?: string) {
	const [activeTab, setActiveTab] = useState('General');
	const [actionLoading, setActionLoading] = useState<'deploy' | 'reload' | 'start' | 'stop' | null>(null);

	const activeKind = (targetKind || '').toLowerCase();
	const pollInterval = actionLoading ? false : 2000;

	// Target query selection with selective query execution to avoid unnecessary 404 console spam
	const postgresQ = $api.useQuery('get', '/postgres/{id}', {params: {path: {id: dbId}}}, {
		retry: false,
		refetchInterval: !activeKind || activeKind.includes('postgres') ? pollInterval : false,
		enabled: activeKind ? activeKind.includes('postgres') : true,
	});
	const mysqlQ = $api.useQuery('get', '/mysql/{id}', {params: {path: {id: dbId}}}, {
		retry: false,
		refetchInterval: !activeKind || activeKind.includes('mysql') ? pollInterval : false,
		enabled: activeKind ? activeKind.includes('mysql') : true,
	});
	const mariadbQ = $api.useQuery('get', '/mariadb/{id}', {params: {path: {id: dbId}}}, {
		retry: false,
		refetchInterval: !activeKind || activeKind.includes('mariadb') ? pollInterval : false,
		enabled: activeKind ? activeKind.includes('mariadb') : true,
	});
	const mongoQ = $api.useQuery('get', '/mongo/{id}', {params: {path: {id: dbId}}}, {
		retry: false,
		refetchInterval: !activeKind || activeKind.includes('mongo') ? pollInterval : false,
		enabled: activeKind ? activeKind.includes('mongo') : true,
	});
	const redisQ = $api.useQuery('get', '/redis/{id}', {params: {path: {id: dbId}}}, {
		retry: false,
		refetchInterval: !activeKind || activeKind.includes('redis') ? pollInterval : false,
		enabled: activeKind ? activeKind.includes('redis') : true,
	});
	const libsqlQ = $api.useQuery('get', '/libsql/{id}', {params: {path: {id: dbId}}}, {
		retry: false,
		refetchInterval: !activeKind || activeKind.includes('libsql') ? pollInterval : false,
		enabled: activeKind ? activeKind.includes('libsql') : true,
	});

	// Select active query result
	const database = (activeKind.includes('redis') ? redisQ.data : null) ||
		(activeKind.includes('postgres') ? postgresQ.data : null) ||
		(activeKind.includes('mysql') ? mysqlQ.data : null) ||
		(activeKind.includes('mariadb') ? mariadbQ.data : null) ||
		(activeKind.includes('mongo') ? mongoQ.data : null) ||
		(activeKind.includes('libsql') ? libsqlQ.data : null) ||
		postgresQ.data || mysqlQ.data || mariadbQ.data || mongoQ.data || redisQ.data || libsqlQ.data;

	const detectedKind = targetKind || (
		postgresQ.data ? 'postgres'
		: mysqlQ.data ? 'mysql'
		: mariadbQ.data ? 'mariadb'
		: mongoQ.data ? 'mongo'
		: redisQ.data ? 'redis'
		: libsqlQ.data ? 'libsql'
		: null
	);

	const currentKind = (database?.kind || detectedKind || 'postgres').toLowerCase();

	const statusUpper = (database?.app_status || (database as any)?.status || (database as any)?.application_status || '').toUpperCase();
	const isBuilding = ['STARTING', 'BUILDING', 'QUEUED', 'PREPARING'].includes(statusUpper) || actionLoading === 'deploy' || actionLoading === 'start';



	let activeEndpoint: '/postgres/{id}' | '/mysql/{id}' | '/mariadb/{id}' | '/mongo/{id}' | '/redis/{id}' | '/libsql/{id}' = '/postgres/{id}';
	if (currentKind.includes('mysql')) activeEndpoint = '/mysql/{id}';
	else if (currentKind.includes('mariadb')) activeEndpoint = '/mariadb/{id}';
	else if (currentKind.includes('mongo')) activeEndpoint = '/mongo/{id}';
	else if (currentKind.includes('redis')) activeEndpoint = '/redis/{id}';
	else if (currentKind.includes('libsql')) activeEndpoint = '/libsql/{id}';

	const refetch = async () => {
		if (currentKind.includes('postgres')) await postgresQ.refetch();
		else if (currentKind.includes('mysql')) await mysqlQ.refetch();
		else if (currentKind.includes('mariadb')) await mariadbQ.refetch();
		else if (currentKind.includes('mongo')) await mongoQ.refetch();
		else if (currentKind.includes('redis')) await redisQ.refetch();
		else if (currentKind.includes('libsql')) await libsqlQ.refetch();
		else {
			await Promise.all([
				postgresQ.refetch(),
				mysqlQ.refetch(),
				mariadbQ.refetch(),
				mongoQ.refetch(),
				redisQ.refetch(),
				libsqlQ.refetch(),
			]);
		}
	};

	const deployMutation = $api.useMutation('post', `${activeEndpoint}/deploy` as any);
	const reloadMutation = $api.useMutation('post', `${activeEndpoint}/reload` as any);
	const startMutation = $api.useMutation('post', `${activeEndpoint}/start` as any);
	const stopMutation = $api.useMutation('post', `${activeEndpoint}/stop` as any);
	const cancelMutation = $api.useMutation('post', `${activeEndpoint}/cancel` as any);
	const patchMutation = $api.useMutation('patch', activeEndpoint as any);
	const deleteMutation = $api.useMutation('delete', activeEndpoint as any);

	const queryClient = useQueryClient();

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop' | 'cancel') => {
		try {
			setActionLoading(action as any);

			await queryClient.cancelQueries();

			let res: Record<string, unknown> | undefined;
			if (action === 'deploy') {
				res = (await deployMutation.mutateAsync({params: {path: {id: dbId}}})) as unknown as Record<string, unknown>;
				toast.success('Database deployment triggered');
			} else if (action === 'reload') {
				res = (await reloadMutation.mutateAsync({params: {path: {id: dbId}}})) as unknown as Record<string, unknown>;
				toast.success('Database reload triggered');
			} else if (action === 'start') {
				res = (await startMutation.mutateAsync({params: {path: {id: dbId}}})) as unknown as Record<string, unknown>;
				toast.success('Database start triggered');
			} else if (action === 'stop') {
				res = (await stopMutation.mutateAsync({params: {path: {id: dbId}}})) as unknown as Record<string, unknown>;
				toast.success('Database stopped successfully');
			} else if (action === 'cancel') {
				res = (await cancelMutation.mutateAsync({params: {path: {id: dbId}}})) as unknown as Record<string, unknown>;
				toast.success('Database action cancelled');
			}

			const targetStatus = action === 'stop' ? 'STOPPED' : action === 'start' ? 'RUNNING' : 'STARTING';
			const updatedDb = (res?.data as Record<string, unknown>)?.database || res?.database || {id: dbId, app_status: targetStatus};

			queryClient.setQueriesData({exact: false}, (oldData: unknown) => {
				const obj = oldData as Record<string, unknown> | undefined;
				if (obj && typeof obj === 'object' && (obj.id === dbId || String(obj.id) === String(dbId))) {
					return {...obj, ...updatedDb, app_status: targetStatus};
				}
				return oldData;
			});

			await refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		} finally {
			setActionLoading(null);
		}
	};

	const handleUpdate = async (body: Record<string, unknown>) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: dbId}},
				body,
			});
			toast.success('Database updated successfully');
			refetch();
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	// Centralized Schedules Query
	const {data: rawSchedules = [], isLoading: isLoadingSchedules, refetch: refetchSchedules} = $api.useQuery(
		'get',
		'/schedules/database/{database_id}' as any,
		{
			params: {path: {database_id: dbId}},
			enabled: !!dbId,
		} as any
	);

	// Centralized Backups Query
	const {data: rawBackups = [], isLoading: isLoadingBackups, refetch: refetchBackups} = $api.useQuery(
		'get',
		'/backups/volume',
		{
			params: {query: {database_id: dbId}},
			enabled: !!dbId,
		} as any
	);

	const schedules = Array.isArray(rawSchedules) ? rawSchedules : [];
	const backups = (Array.isArray(rawBackups) ? rawBackups : []).filter((b: any) =>
		b.database_id === dbId ||
		b.postgres_id === dbId || b.mysql_id === dbId || b.mariadb_id === dbId ||
		b.mongo_id === dbId || b.redis_id === dbId || b.libsql_id === dbId ||
		b.app_name === (database as any)?.app_name || b.app_name === (database as any)?.name
	);

	// Centralized Container Monitoring Stream
	const monitoring = useContainerMonitoring(dbId, 'database');

	const refetchAll = () => {
		refetch();
		refetchSchedules();
		refetchBackups();
		monitoring.triggerRefresh();
	};

	const allQueries = [postgresQ, mysqlQ, mariadbQ, mongoQ, redisQ, libsqlQ];
	const isPendingOrFetching = allQueries.some(q => q.status === 'pending' || q.isFetching || q.isLoading);
	const isLoading = !database && isPendingOrFetching;

	return {
		database,
		schedules,
		backups,
		monitoring,
		isLoading,
		isLoadingSchedules,
		isLoadingBackups,
		actionLoading,
		isBuilding,
		detectedKind: currentKind,
		refetch,
		refetchSchedules,
		refetchBackups,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
		deleteMutation,
	};
}
