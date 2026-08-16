import {useState, useMemo} from 'react';
import {client} from '#/api/client';
import {formatApiError} from '#/api/utils';
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
			const resolvedKind = service.db_kind || service.dbKind || (service.kind && service.kind !== 'database' ? service.kind : undefined) || targetKind || 'postgres';
			return {
				id: service.id,
				name: service.name,
				database_name: service.name,
				kind: resolvedKind,
				project_id: service.project_id,
				environment_id: service.environment_id,
				status: service.status,
				created_at: service.created_at,
			} as any;
		}

		// 3. Do not fall back to an ID match of a different kind if targetKind is specified
		if (targetK) return undefined;

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
	const fetchedDb = (activeKind.includes('redis') ? redisQ.data : null) ||
		(activeKind.includes('postgres') ? postgresQ.data : null) ||
		(activeKind.includes('mysql') ? mysqlQ.data : null) ||
		(activeKind.includes('mariadb') ? mariadbQ.data : null) ||
		(activeKind.includes('mongo') ? mongoQ.data : null) ||
		(activeKind.includes('libsql') ? libsqlQ.data : null) ||
		postgresQ.data || mysqlQ.data || redisQ.data || mongoQ.data || mariadbQ.data || libsqlQ.data;

	const database = useMemo(() => {
		const raw = (fetchedDb && storeDb) ? { ...storeDb, ...fetchedDb } : (fetchedDb || storeDb);
		if (!raw) return null;
		const effectiveStatus = raw.status || raw.app_status || raw.application_status || 'STOPPED';
		return {
			...raw,
			status: effectiveStatus,
			app_status: effectiveStatus,
		};
	}, [fetchedDb, storeDb]);

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

	const statusUpper = (database?.status || database?.app_status || (database as any)?.application_status || '').toUpperCase();
	const isDeployed = ['RUNNING', 'DONE', 'HEALTHY', 'SUCCESS', 'COMPLETED', 'UP', 'ACTIVE', 'OK'].includes(statusUpper);

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

	const allDeployments = useAppStore((state) => state.deployments || []);
	const dbDeployments = useMemo(() => {
		return allDeployments.filter((d: any) => String(d.database_id) === String(dbId));
	}, [allDeployments, dbId]);

	const refetchAll = () => {
		monitoring.triggerRefresh();
	};

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel') => {
		setActionLoading(action as any);
		try {
			const kind = (currentKind || targetKind || 'postgres').toLowerCase();
			const endpoint = `/${kind}/{id}/${action}` as any;
			const res = await client.POST(endpoint, {
				params: { path: { id: dbId } },
			} as any);

			const resObj = res as Record<string, unknown>;
			if (resObj?.error) {
				toast.error(formatApiError(resObj.error));
				return;
			}

			toast.success(`${kind.toUpperCase()} ${action} triggered`);
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
		} finally {
			setActionLoading(null);
		}
	};

	const handleUpdateEnv = async (patchData: Record<string, unknown>) => {
		try {
			const kind = (currentKind || targetKind || 'postgres').toLowerCase();
			const endpoint = `/${kind}/{id}` as any;
			const res = await client.PATCH(endpoint, {
				params: { path: { id: dbId } },
				body: patchData,
			} as any);

			const resObj = res as Record<string, unknown>;
			if (resObj?.error) {
				toast.error(formatApiError(resObj.error));
				return;
			}

			toast.success('Database settings updated');
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return {
		database,
		currentKind,
		isDeployed,
		schedules,
		backups,
		deployments: dbDeployments,
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
