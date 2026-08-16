import {useState, useMemo} from 'react';
import {$api} from '#/api/query';
import {toast} from 'sonner';
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

import { useAppStore } from '#/stores/app-store';

export function useDatabaseDetail(dbId: number, targetKind?: string) {
	const [activeTab, setActiveTab] = useState('General');
	const [actionLoading, setActionLoading] = useState<'deploy' | 'reload' | 'start' | 'stop' | null>(null);

	const activeKind = (targetKind || '').toLowerCase();

	// 0ms Instant Zustand Store Read with fallback to overviewServices
	const databases = useAppStore((state) => state.databases);
	const overviewServices = useAppStore((state) => state.overviewServices);

	const storeDb = useMemo(() => {
		const targetK = (targetKind || '').toLowerCase();

		// 1. Try finding in databases array matching id AND kind
		const direct = databases.find((d: any) => {
			if (String(d.id) !== String(dbId)) return false;
			if (!targetK) return true;
			const dk = String(d.kind || d.type || '').toLowerCase();
			return dk.includes(targetK) || targetK.includes(dk);
		});
		if (direct) return direct;

		// 2. Try finding in overviewServices matching id AND kind
		const service = overviewServices.find((s: any) => {
			if (String(s.id) !== String(dbId)) return false;
			if (!targetK) return true;
			const sk = String(s.kind || s.db_kind || s.dbKind || s.type || '').toLowerCase();
			return sk.includes(targetK) || targetK.includes(sk);
		});

		if (service) {
			return {
				id: service.id,
				name: service.name,
				database_name: service.name,
				kind: service.kind || targetKind || 'postgres',
				project_id: service.project_id,
				status: service.status,
				created_at: service.created_at,
			} as any;
		}

		// 3. Fallback to any matching ID if no kind match
		return databases.find((d) => String(d.id) === String(dbId)) ||
			overviewServices.find((s) => String(s.id) === String(dbId));
	}, [databases, overviewServices, dbId, targetKind]);

	// Target query selection with selective query execution
	const postgresQ = usePostgresGet(BigInt(dbId));
	const mysqlQ = useMysqlGet(BigInt(dbId));
	const mariadbQ = useMariadbGet(BigInt(dbId));
	const mongoQ = useMongoGet(BigInt(dbId));
	const redisQ = useRedisGet(BigInt(dbId));
	const libsqlQ = useLibsqlGet(BigInt(dbId));

	// Select active query result matching targetKind strictly
	const database = storeDb ||
		(activeKind.includes('redis') ? redisQ.data : null) ||
		(activeKind.includes('postgres') ? postgresQ.data : null) ||
		(activeKind.includes('mysql') ? mysqlQ.data : null) ||
		(activeKind.includes('mariadb') ? mariadbQ.data : null) ||
		(activeKind.includes('mongo') ? mongoQ.data : null) ||
		(activeKind.includes('libsql') ? libsqlQ.data : null);

	const detectedKind = targetKind || (
		redisQ.data ? 'redis'
		: postgresQ.data ? 'postgres'
		: mysqlQ.data ? 'mysql'
		: mariadbQ.data ? 'mariadb'
		: mongoQ.data ? 'mongo'
		: libsqlQ.data ? 'libsql'
		: null
	);

	const currentKind = (database?.kind || detectedKind || targetKind || 'postgres').toLowerCase();

	const statusUpper = (database?.app_status || (database as any)?.status || (database as any)?.application_status || '').toUpperCase();
	const isDeployed = statusUpper === 'RUNNING' || statusUpper === 'HEALTHY' || statusUpper === 'SUCCESS' || statusUpper === 'COMPLETED';

	// Schedules
	const {data: rawSchedules, loading: isLoadingSchedules} = useScheduleListByDatabase(BigInt(dbId));
	const schedules = useMemo(() => (Array.isArray(rawSchedules) ? rawSchedules : []), [rawSchedules]);

	// Backups — filter locally by database_id
	const {data: rawBackupsAll, loading: isLoadingBackups} = useBackupListVolumeBackups();
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackupsAll) ? rawBackupsAll : [];
		return all.filter((b: any) => b.database_id === dbId);
	}, [rawBackupsAll, dbId]);

	// Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(dbId, currentKind || 'postgres');

	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const deployMutation = $api.useMutation('post', `/${currentKind as 'postgres'}/{id}/deploy` as any);
	const reloadMutation = $api.useMutation('post', `/${currentKind as 'postgres'}/{id}/reload` as any);
	const startMutation = $api.useMutation('post', `/${currentKind as 'postgres'}/{id}/start` as any);
	const stopMutation = $api.useMutation('post', `/${currentKind as 'postgres'}/{id}/stop` as any);
	const patchMutation = $api.useMutation('patch', `/${currentKind as 'postgres'}/{id}` as any);

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop') => {
		setActionLoading(action);
		try {
			if (action === 'deploy') {
				await deployMutation.mutateAsync({params: {path: {id: dbId}}} as any);
				toast.success(`${currentKind.toUpperCase()} deployment triggered`);
			} else if (action === 'reload') {
				await reloadMutation.mutateAsync({params: {path: {id: dbId}}} as any);
				toast.success(`${currentKind.toUpperCase()} reloaded`);
			} else if (action === 'start') {
				await startMutation.mutateAsync({params: {path: {id: dbId}}} as any);
				toast.success(`${currentKind.toUpperCase()} started`);
			} else if (action === 'stop') {
				await stopMutation.mutateAsync({params: {path: {id: dbId}}} as any);
				toast.success(`${currentKind.toUpperCase()} stopped`);
			}
			refetchAll();
		} catch (err) {
			toast.error(`Action failed: ${action}`);
		} finally {
			setActionLoading(null);
		}
	};

	const handleUpdateEnv = async (patchData: Record<string, unknown>) => {
		try {
			await patchMutation.mutateAsync({
				params: {path: {id: dbId}},
				body: patchData,
			} as any);
			toast.success('Database settings updated');
			refetchAll();
		} catch (err) {
			toast.error('Failed to update database settings');
		}
	};

	return {
		database,
		currentKind,
		isDeployed,
		schedules,
		backups,
		monitoring,
		isLoading: !database,
		isLoadingSchedules,
		isLoadingBackups,
		actionLoading,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdateEnv,
	};
}
