import {useState} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {
	usePostgresGet,
	useMysqlGet,
	useMariadbGet,
	useMongoGet,
	useRedisGet,
	useLibsqlGet,
	useScheduleListByDatabase,
	useBackupListVolumeBackups,
} from 'virtual:openoxide-live';

export function useDatabaseDetail(dbId: number, targetKind?: string) {
	const [activeTab, setActiveTab] = useState('General');
	const [actionLoading, setActionLoading] = useState<'deploy' | 'reload' | 'start' | 'stop' | null>(null);

	const activeKind = (targetKind || '').toLowerCase();

	// Target query selection with selective query execution to avoid unnecessary 404 console spam
	const postgresQ = usePostgresGet(BigInt(dbId));
	const mysqlQ = useMysqlGet(BigInt(dbId));
	const mariadbQ = useMariadbGet(BigInt(dbId));
	const mongoQ = useMongoGet(BigInt(dbId));
	const redisQ = useRedisGet(BigInt(dbId));
	const libsqlQ = useLibsqlGet(BigInt(dbId));

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

	// Live hooks auto-update — refetch is a no-op
	const refetch = async () => {};

	const deployMutation = $api.useMutation('post', `${activeEndpoint}/deploy` as any);
	const reloadMutation = $api.useMutation('post', `${activeEndpoint}/reload` as any);
	const startMutation = $api.useMutation('post', `${activeEndpoint}/start` as any);
	const stopMutation = $api.useMutation('post', `${activeEndpoint}/stop` as any);
	const cancelMutation = $api.useMutation('post', `${activeEndpoint}/cancel` as any);
	const patchMutation = $api.useMutation('patch', activeEndpoint as any);
	const deleteMutation = $api.useMutation('delete', activeEndpoint as any);

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop' | 'cancel') => {
		try {
			setActionLoading(action as any);

			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path: {id: dbId}}});
				toast.success('Database deployment triggered');
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path: {id: dbId}}});
				toast.success('Database reload triggered');
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: dbId}}});
				toast.success('Database start triggered');
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path: {id: dbId}}});
				toast.success('Database stopped successfully');
			} else if (action === 'cancel') {
				await cancelMutation.mutateAsync({params: {path: {id: dbId}}});
				toast.success('Database action cancelled');
			}
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
		} catch (err: unknown) {
			toast.error(formatApiError(err));
		}
	};

	// Centralized Schedules Query
	const {data: rawSchedules, loading: isLoadingSchedules} = useScheduleListByDatabase(BigInt(dbId));

	// Centralized Backups Query
	const {data: rawBackups, loading: isLoadingBackups} = useBackupListVolumeBackups();

	const schedules = Array.isArray(rawSchedules) ? rawSchedules : [];
	const backups = (Array.isArray(rawBackups) ? rawBackups : []).filter((b: any) =>
		b.database_id === dbId ||
		b.postgres_id === dbId || b.mysql_id === dbId || b.mariadb_id === dbId ||
		b.mongo_id === dbId || b.redis_id === dbId || b.libsql_id === dbId ||
		b.app_name === (database as any)?.app_name || b.app_name === (database as any)?.name
	);

	// Centralized Container Monitoring Stream
	const monitoring = useContainerMonitoring(dbId, 'database');

	const refetchSchedules = () => {};
	const refetchBackups = () => {};

	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const allQueries = [postgresQ, mysqlQ, mariadbQ, mongoQ, redisQ, libsqlQ];
	const isPendingOrFetching = allQueries.some(q => q.loading);
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
