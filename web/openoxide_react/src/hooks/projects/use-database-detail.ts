import {useState, useMemo, useEffect} from 'react';
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
			const resolvedKind = service.db_kind || service.dbKind || (service.kind && service.kind !== 'database' ? service.kind : undefined) || targetKind;
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

	const activeKind = (
		storeDb?.kind ||
		storeDb?.type ||
		(storeDb as any)?.db_kind ||
		targetKind ||
		''
	).toLowerCase();

	const isRedis = activeKind.includes('redis');
	const isMysql = activeKind.includes('mysql');
	const isMariadb = activeKind.includes('mariadb');
	const isMongo = activeKind.includes('mongo');
	const isLibsql = activeKind.includes('libsql');
	const isPostgres = activeKind.includes('postgres') || (!isRedis && !isMysql && !isMariadb && !isMongo && !isLibsql && activeKind === 'postgres');

	// Target query selection with 0n ID guard for inactive queries to save CPU & network
	const postgresQ = usePostgresGet(isPostgres ? BigInt(dbId) : 0n);
	const mysqlQ = useMysqlGet(isMysql ? BigInt(dbId) : 0n);
	const mariadbQ = useMariadbGet(isMariadb ? BigInt(dbId) : 0n);
	const mongoQ = useMongoGet(isMongo ? BigInt(dbId) : 0n);
	const redisQ = useRedisGet(isRedis ? BigInt(dbId) : 0n);
	const libsqlQ = useLibsqlGet(isLibsql ? BigInt(dbId) : 0n);

	// Select active query result matching targetKind strictly
	const liveDb = (isRedis ? redisQ.data : null) ||
		(isPostgres ? postgresQ.data : null) ||
		(isMysql ? mysqlQ.data : null) ||
		(isMariadb ? mariadbQ.data : null) ||
		(isMongo ? mongoQ.data : null) ||
		(isLibsql ? libsqlQ.data : null);

	const detectedKind = targetKind || (
		redisQ.data ? 'redis'
		: postgresQ.data ? 'postgres'
		: mysqlQ.data ? 'mysql'
		: mariadbQ.data ? 'mariadb'
		: mongoQ.data ? 'mongo'
		: libsqlQ.data ? 'libsql'
		: null
	);

	const currentKind = (storeDb?.kind || detectedKind || targetKind || 'postgres').toLowerCase();

	const [localStatusOverride, setLocalStatusOverride] = useState<string | null>(null);

	// Auto-clear localStatusOverride and sync Zustand store once backend query syncs with backend state
	useEffect(() => {
		if (liveDb) {
			const fetchedStatus = ((liveDb as any).status || (liveDb as any).app_status || '').toUpperCase();
			const dbKind = (detectedKind || currentKind || targetKind || 'postgres').toLowerCase();
			if (fetchedStatus && (storeDb as any)?.status !== fetchedStatus) {
				(useAppStore.getState() as any).updateServiceStatus?.(dbId, fetchedStatus, dbKind);
			}
			if (localStatusOverride) {
				const overrideUpper = (localStatusOverride || '').toUpperCase();
				if (
					fetchedStatus === overrideUpper ||
					(overrideUpper === 'STARTING' && (fetchedStatus === 'RUNNING' || fetchedStatus === 'HEALTHY')) ||
					(overrideUpper === 'STOPPING' && (fetchedStatus === 'STOPPED' || fetchedStatus === 'IDLE')) ||
					(overrideUpper === 'CANCELLING' && (fetchedStatus === 'CANCELLED' || fetchedStatus === 'STOPPED' || fetchedStatus === 'IDLE'))
				) {
					setLocalStatusOverride(null);
				}
			}
		}
	}, [liveDb, localStatusOverride, dbId, detectedKind, currentKind, targetKind, (storeDb as any)?.status]);

	const raw = liveDb || storeDb;
	const database = useMemo(() => {
		if (!raw) return null;
		const effectiveStatus = localStatusOverride || (raw as any).status || (raw as any).app_status || (storeDb as any)?.status || 'STOPPED';
		return {
			...raw,
			status: effectiveStatus,
			app_status: effectiveStatus,
		};
	}, [raw, storeDb, localStatusOverride]);

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
		postgresQ.refetch?.();
		mysqlQ.refetch?.();
		mariadbQ.refetch?.();
		mongoQ.refetch?.();
		redisQ.refetch?.();
		libsqlQ.refetch?.();
		monitoring.triggerRefresh();
	};

	const handleAction = async (action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel') => {
		setActionLoading(action as any);
		const currentSt = (database?.app_status || database?.status || '').toUpperCase();
		const isCurrentlyBuilding = ['QUEUED', 'BUILDING', 'STARTING', 'PREPARING', 'PENDING', 'DEPLOYING'].includes(currentSt);
		const intermediateStatus = (action === 'stop' || action === 'cancel')
			? (isCurrentlyBuilding ? 'CANCELLING' : 'STOPPING')
			: action === 'start' ? 'STARTING'
			: 'DEPLOYING';

		const kind = (currentKind || targetKind || 'postgres').toLowerCase();
		setLocalStatusOverride(intermediateStatus);
		(useAppStore.getState() as any).updateServiceStatus?.(dbId, intermediateStatus, kind);

		try {
			const kind = (currentKind || targetKind || 'postgres').toLowerCase();
			const endpoint = `/${kind}/{id}/${action}` as any;
			const res = await client.POST(endpoint, {
				params: { path: { id: dbId } },
			} as any);

			const resObj = res as Record<string, unknown>;
			if (resObj?.error) {
				toast.error(formatApiError(resObj.error));
				setLocalStatusOverride(null);
				return;
			}

			toast.success(`${kind.toUpperCase()} ${action} triggered`);
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
			setLocalStatusOverride(null);
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
