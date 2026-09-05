import {useState, useMemo, useEffect} from 'react';
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
	useDatabaseDeploy,
	useDatabaseRedeploy,
	useDatabaseReload,
	useDatabaseStart,
	useDatabaseStop,
	useDatabaseUpdate,
	useDeploymentItems,
	parseDatabaseKind,
	useDatabaseList,
	useOverviewServices,
} from '#/api/live';

export function useDatabaseDetail(dbId: number, targetKind?: string) {
	const [activeTab, setActiveTab] = useState('General');
	const [actionLoading, setActionLoading] = useState<
		'deploy' | 'reload' | 'start' | 'stop' | null
	>(null);

	// Connect mutations used by handleAction / handleUpdateEnv below; the
	// kind is resolved per call from the current database row.
	const deployMutation = useDatabaseDeploy();
	const redeployMutation = useDatabaseRedeploy();
	const reloadMutation = useDatabaseReload();
	const startMutation = useDatabaseStart();
	const stopMutation = useDatabaseStop();
	const updateMutation = useDatabaseUpdate();
	const databaseActionMutations = {
		deploy: deployMutation.mutateAsync,
		redeploy: redeployMutation.mutateAsync,
		reload: reloadMutation.mutateAsync,
		start: startMutation.mutateAsync,
		stop: stopMutation.mutateAsync,
	};

	// 0ms Instant live-query reads: org-wide database list keyed
	// ['databases'] plus the overview services stream keyed
	// ['overview-services'] — the table-watch stream refetches both while
	// this page is mounted.
	const databases = useDatabaseList().data ?? [];
	const overviewServices = useOverviewServices().data ?? [];

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
			const sk = String(
				s.kind || s.db_kind || s.dbKind || s.type || '',
			).toLowerCase();
			return sk.includes(targetK) || targetK.includes(sk);
		});

		if (service) {
			const resolvedKind = service.db_kind || targetKind;
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

		return (
			databases.find(d => String(d.id) === String(dbId)) ||
			overviewServices.find(s => String(s.id) === String(dbId))
		);
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
	const isPostgres =
		activeKind.includes('postgres') ||
		(!isRedis &&
			!isMysql &&
			!isMariadb &&
			!isMongo &&
			!isLibsql &&
			activeKind === 'postgres');

	// Target query selection with 0n ID guard for inactive queries to save CPU & network
	const postgresQ = usePostgresGet(isPostgres ? BigInt(dbId) : 0n);
	const mysqlQ = useMysqlGet(isMysql ? BigInt(dbId) : 0n);
	const mariadbQ = useMariadbGet(isMariadb ? BigInt(dbId) : 0n);
	const mongoQ = useMongoGet(isMongo ? BigInt(dbId) : 0n);
	const redisQ = useRedisGet(isRedis ? BigInt(dbId) : 0n);
	const libsqlQ = useLibsqlGet(isLibsql ? BigInt(dbId) : 0n);

	// Select active query result matching targetKind strictly
	const liveDb =
		(isRedis ? redisQ.data : null) ||
		(isPostgres ? postgresQ.data : null) ||
		(isMysql ? mysqlQ.data : null) ||
		(isMariadb ? mariadbQ.data : null) ||
		(isMongo ? mongoQ.data : null) ||
		(isLibsql ? libsqlQ.data : null);

	const detectedKind =
		targetKind ||
		(redisQ.data
			? 'redis'
			: postgresQ.data
				? 'postgres'
				: mysqlQ.data
					? 'mysql'
					: mariadbQ.data
						? 'mariadb'
						: mongoQ.data
							? 'mongo'
							: libsqlQ.data
								? 'libsql'
								: null);

	const currentKind = (
		storeDb?.kind ||
		detectedKind ||
		targetKind ||
		'postgres'
	).toLowerCase();

	const [localStatusOverride, setLocalStatusOverride] = useState<
		string | null
	>(null);

	// Auto-clear localStatusOverride when live/store status updates or safety timeout occurs
	useEffect(() => {
		const fetchedStatus = (
			(liveDb as any)?.status ||
			(liveDb as any)?.app_status ||
			(storeDb as any)?.status ||
			(storeDb as any)?.app_status ||
			''
		).toUpperCase();

		if (localStatusOverride && fetchedStatus) {
			const overrideUpper = localStatusOverride.toUpperCase();
			if (
				fetchedStatus === overrideUpper ||
				(overrideUpper === 'DEPLOYING' &&
					[
						'BUILDING',
						'QUEUED',
						'PREPARING',
						'DEPLOYING',
						'RUNNING',
						'HEALTHY',
						'SUCCESS',
						'DONE',
					].includes(fetchedStatus)) ||
				(overrideUpper === 'STARTING' &&
					['RUNNING', 'HEALTHY', 'STARTING'].includes(fetchedStatus)) ||
				(overrideUpper === 'STOPPING' &&
					['STOPPED', 'IDLE', 'STOPPING'].includes(fetchedStatus)) ||
				(overrideUpper === 'CANCELLING' &&
					['CANCELLED', 'STOPPED', 'IDLE'].includes(fetchedStatus))
			) {
				setLocalStatusOverride(null);
			}
		}
	}, [liveDb, storeDb, localStatusOverride]);

	useEffect(() => {
		if (localStatusOverride) {
			const timer = setTimeout(() => setLocalStatusOverride(null), 4000);
			return () => clearTimeout(timer);
		}
	}, [localStatusOverride]);

	const raw = liveDb || storeDb;
	const database = useMemo(() => {
		if (!raw) return null;
		const liveStatus =
			(liveDb as any)?.status || (liveDb as any)?.app_status;
		const storeStatus =
			(storeDb as any)?.status || (storeDb as any)?.app_status;
		const rawStatus = (raw as any).status || (raw as any).app_status;
		const effectiveStatus =
			localStatusOverride ||
			liveStatus ||
			storeStatus ||
			rawStatus ||
			'STOPPED';
		return {
			...raw,
			status: effectiveStatus,
			app_status: effectiveStatus,
		};
	}, [raw, liveDb, storeDb, localStatusOverride]);

	const statusUpper = (
		database?.status ||
		database?.app_status ||
		(database as any)?.application_status ||
		''
	).toUpperCase();
	const isDeployed = [
		'RUNNING',
		'DONE',
		'HEALTHY',
		'SUCCESS',
		'COMPLETED',
		'UP',
		'ACTIVE',
		'OK',
	].includes(statusUpper);
	const isBuilding = [
		'QUEUED',
		'BUILDING',
		'STARTING',
		'PREPARING',
		'PENDING',
		'DEPLOYING',
	].includes(statusUpper);

	// Schedules
	const {data: rawSchedules, isLoading: isLoadingSchedules} =
		useScheduleListByDatabase(BigInt(dbId));
	const schedules = useMemo(
		() => (Array.isArray(rawSchedules) ? rawSchedules : []),
		[rawSchedules],
	);

	// Volume backups store a typed resource foreign key, not a generic database_id.
	const {data: rawBackupsAll, isLoading: isLoadingBackups} =
		useBackupListVolumeBackups();
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackupsAll) ? rawBackupsAll : [];
		return all.filter((backup: any) =>
			[
				backup.application_id,
				backup.postgres_id,
				backup.mysql_id,
				backup.mariadb_id,
				backup.mongo_id,
				backup.redis_id,
				backup.libsql_id,
			].some(value => Number(value) === Number(dbId)),
		);
	}, [rawBackupsAll, dbId]);

	// Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(
		dbId,
		currentKind || 'postgres',
		activeTab === 'Monitoring',
	);

	// Live deployments query filtered to this database; the table-watch
	// stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: allDeployments = []} = useDeploymentItems();
	const dbDeployments = useMemo(() => {
		return allDeployments.filter(
			(d: any) => String(d.database_id) === String(dbId),
		);
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

	const handleAction = async (
		action: 'deploy' | 'reload' | 'start' | 'stop' | 'redeploy' | 'cancel',
	) => {
		setActionLoading(action as any);
		const currentSt = (
			database?.app_status ||
			database?.status ||
			''
		).toUpperCase();
		const isCurrentlyBuilding = [
			'QUEUED',
			'BUILDING',
			'STARTING',
			'PREPARING',
			'PENDING',
			'DEPLOYING',
		].includes(currentSt);
		const intermediateStatus =
			action === 'stop' || action === 'cancel'
				? isCurrentlyBuilding
					? 'CANCELLING'
					: 'STOPPING'
				: action === 'start'
					? 'STARTING'
					: 'DEPLOYING';

		setLocalStatusOverride(intermediateStatus);
		// Legacy updateServiceStatus store write removed — table-watch
		// invalidates ['databases'] when the action lands, refetching
		// fresh status through the live query.

		try {
			const kind = parseDatabaseKind(
				currentKind || targetKind || 'postgres',
			);
			const actionInput = {kind, id: dbId};
			if (action === 'deploy') {
				await databaseActionMutations.deploy(actionInput);
			} else if (action === 'redeploy') {
				await databaseActionMutations.redeploy(actionInput);
			} else if (action === 'reload') {
				await databaseActionMutations.reload(actionInput);
			} else if (action === 'start') {
				await databaseActionMutations.start(actionInput);
			} else if (action === 'stop') {
				await databaseActionMutations.stop(actionInput);
			}

			toast.success(
				`${(currentKind || targetKind || 'postgres').toUpperCase()} ${action} triggered`,
			);
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
			const kind = parseDatabaseKind(
				currentKind || targetKind || 'postgres',
			);
			await updateMutation.mutateAsync({
				kind,
				id: dbId,
				...patchData,
			} as Parameters<typeof updateMutation.mutateAsync>[0]);

			toast.success('Database settings updated');
			refetchAll();
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	return {
		database,
		currentKind,
		detectedKind,
		isDeployed,
		isBuilding,
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
