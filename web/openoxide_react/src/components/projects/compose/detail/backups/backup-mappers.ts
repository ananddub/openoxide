/**
 * Maps a normalized database kind + id to the sparse resource-id payload the
 * volume-backup create RPC expects (only the matching engine id is set).
 */
export function buildDatabaseResource(
	normalizedKind: string,
	databaseId?: number,
): Record<string, number | undefined> {
	return {
		postgres_id: normalizedKind.includes('postgres')
			? databaseId
			: undefined,
		mysql_id: normalizedKind.includes('mysql') ? databaseId : undefined,
		mariadb_id: normalizedKind.includes('mariadb')
			? databaseId
			: undefined,
		mongo_id: normalizedKind.includes('mongo') ? databaseId : undefined,
		libsql_id: normalizedKind.includes('libsql') ? databaseId : undefined,
	};
}

const DB_ID_FIELDS = [
	'postgres_id',
	'mysql_id',
	'mariadb_id',
	'mongo_id',
	'libsql_id',
] as const;

/** Filters org-wide backups down to those for this compose stack / database. */
export function filterResourceBackups(
	storeBackups: any[],
	opts: {isDatabase: boolean; composeId?: number; databaseId?: number},
): any[] {
	return storeBackups.filter((backup: any) => {
		if (!opts.isDatabase)
			return Number(backup.compose_id) === Number(opts.composeId);
		return DB_ID_FIELDS.some(
			f => Number(backup[f]) === Number(opts.databaseId),
		);
	});
}
