import {
	deriveServiceStatus,
	type ComposeService,
} from '#/components/projects/compose/visualizer/compose-parse';

export function dbSupportsVolumeBackup(kind: string): boolean {
	return !kind.includes('redis');
}

export function dbInternalPort(kind: string): number {
	return kind.includes('mysql') || kind.includes('maria')
		? 3306
		: kind.includes('mongo')
			? 27017
			: kind.includes('redis')
				? 6379
				: kind.includes('libsql')
					? 8080
					: 5432;
}

/** Single-service topology for a database (shared by tab + preview). */
export function buildDbArchService(
	database: any,
	kind: string,
): ComposeService[] {
	const externalPort = database?.external_port || undefined;
	return [
		{
			name: database?.app_name || database?.name || kind || 'database',
			image: kind || 'postgres',
			dependsOn: [],
			envVars: {},
			volumes: (database as any)?.volume_name
				? [(database as any).volume_name]
				: [`${kind}_data`],
			ports: externalPort
				? [String(externalPort)]
				: [String(dbInternalPort(kind))],
			status: deriveServiceStatus(
				database?.app_status || database?.status || undefined,
			),
		},
	];
}

export function filterDbArchBackups(
	backups: unknown,
	dbId: number | undefined,
	database: any,
): any[] {
	const list = Array.isArray(backups) ? backups : [];
	return list.filter(
		(b: any) =>
			b.postgres_id === dbId ||
			b.mysql_id === dbId ||
			b.mariadb_id === dbId ||
			b.mongo_id === dbId ||
			b.redis_id === dbId ||
			b.libsql_id === dbId ||
			b.app_name === database?.app_name ||
			b.database_id === dbId ||
			b.database_name === database?.app_name ||
			b.app_name === database?.name,
	);
}
