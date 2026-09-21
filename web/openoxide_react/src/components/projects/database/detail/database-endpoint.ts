export type DatabaseEndpoint =
	| '/postgres/{id}'
	| '/mysql/{id}'
	| '/mariadb/{id}'
	| '/mongo/{id}'
	| '/redis/{id}'
	| '/libsql/{id}';

export function databaseKind(database: any): string {
	return String(
		database?.kind || database?.database_kind || 'postgres',
	).toLowerCase();
}

export function databaseEndpoint(database: any): DatabaseEndpoint {
	const kind = databaseKind(database);
	if (kind.includes('mariadb')) return '/mariadb/{id}';
	if (kind.includes('mysql')) return '/mysql/{id}';
	if (kind.includes('mongo')) return '/mongo/{id}';
	if (kind.includes('redis')) return '/redis/{id}';
	if (kind.includes('libsql')) return '/libsql/{id}';
	return '/postgres/{id}';
}
