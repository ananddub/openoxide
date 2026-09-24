import {parseDatabaseKind} from '#/api/live';

export interface DerivedDbCredentials {
	kind: string;
	dbKind: ReturnType<typeof parseDatabaseKind>;
	isRedis: boolean;
	isLibsql: boolean;
	dbUser: string;
	dbName: string;
	currentPassword: string;
	internalPort: number;
	/** `/<dbName>` path segment (empty for Redis / LibSQL). */
	dbNamePath: string;
	/** `<user>:<password>` (or just `:<password>` for Redis). */
	userAuthPart: string;
}

/** Normalises the many field spellings a database record may arrive with and derives the connection fragments shared by the internal & external credentials cards. */
export function deriveDatabaseCredentials(
	database: any,
): DerivedDbCredentials {
	const kind = (database?.kind || 'postgres').toLowerCase();
	const isRedis = kind.includes('redis');
	const isLibsql = kind.includes('libsql');
	const rawDb = database as unknown as Record<string, unknown>;

	const defaultUser = kind.includes('postgres')
		? 'postgres'
		: kind.includes('mysql') ||
			  kind.includes('maria') ||
			  kind.includes('mongo')
			? 'root'
			: isRedis
				? 'default'
				: 'admin';

	const dbUser = String(
		database?.database_user ||
			rawDb?.database_user ||
			rawDb?.databaseUser ||
			rawDb?.db_user ||
			defaultUser,
	);
	const dbName = String(
		database?.database_name ||
			rawDb?.database_name ||
			rawDb?.databaseName ||
			database?.name ||
			'',
	);
	const currentPassword = String(
		rawDb?.database_password ||
			rawDb?.databasePassword ||
			rawDb?.password ||
			rawDb?.database_root_password ||
			rawDb?.db_password ||
			rawDb?.postgres_password ||
			rawDb?.mysql_password ||
			rawDb?.mongo_password ||
			'',
	);

	const internalPort =
		kind.includes('mysql') || kind.includes('maria')
			? 3306
			: kind.includes('mongo')
				? 27017
				: isRedis
					? 6379
					: isLibsql
						? 8080
						: 5432;

	const dbNamePath = isRedis || isLibsql ? '' : `/${dbName || 'db'}`;
	const userAuthPart = isRedis
		? currentPassword
			? `:${currentPassword}`
			: ''
		: `${dbUser}:${currentPassword}`;

	return {
		kind,
		dbKind: parseDatabaseKind(kind),
		isRedis,
		isLibsql,
		dbUser,
		dbName,
		currentPassword,
		internalPort,
		dbNamePath,
		userAuthPart,
	};
}

/** Assemble a `<scheme>://<auth>@<host>:<port><path>` connection string. */
export const buildConnectionString = (
	kind: string,
	userAuthPart: string,
	host: string,
	port: number,
	dbNamePath: string,
): string => `${kind}://${userAuthPart}@${host}:${port}${dbNamePath}`;
