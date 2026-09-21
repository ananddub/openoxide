import {useMemo} from 'react';
import {
	useDatabaseList,
	useOverviewServices,
	usePostgresGet,
	useMysqlGet,
	useMariadbGet,
	useMongoGet,
	useRedisGet,
	useLibsqlGet,
} from '#/api/live';

/**
 * Resolves the active database row from the org-wide live lists + per-kind
 * queries, deriving the concrete engine kind. 0ms instant reads; the
 * table-watch stream keeps them fresh while mounted.
 */
export function useDatabaseQueries(dbId: number, targetKind?: string) {
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

	const refetchQueries = () => {
		postgresQ.refetch?.();
		mysqlQ.refetch?.();
		mariadbQ.refetch?.();
		mongoQ.refetch?.();
		redisQ.refetch?.();
		libsqlQ.refetch?.();
	};

	return {storeDb, liveDb, currentKind, detectedKind, refetchQueries};
}
