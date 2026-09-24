export interface LinkEnvTemplate {
	key: string;
	value: string;
	checked: boolean;
}

interface TargetNode {
	name?: string;
	dbType?: string;
	port?: number;
}

/** Resolves the default connection port for a database type. */
export const resolveDbPort = (dbType: string, port?: number): number =>
	port ||
	(dbType === 'redis' ? 6379 : dbType === 'mongodb' ? 27017 : 5432);

/** Proposes template environment variables based on the connected database type (postgres / redis / generic). */
export function buildLinkEnvTemplates(
	targetNode?: TargetNode,
): LinkEnvTemplate[] {
	const dbName = targetNode?.name || 'database';
	const dbType = (targetNode?.dbType || 'postgres').toLowerCase();
	const port = resolveDbPort(dbType, targetNode?.port);
	const prefix = dbName.toUpperCase().replace(/[^A-Z0-9]/g, '_');
	const host = dbName.toLowerCase();
	const common = [
		{key: `${prefix}_HOST`, value: host, checked: true},
		{key: `${prefix}_PORT`, value: String(port), checked: true},
	];

	if (dbType.includes('postgres')) {
		return [
			{
				key: 'DATABASE_URL',
				value: `postgres://postgres:password@${host}:${port}/${host}`,
				checked: true,
			},
			...common,
		];
	}
	if (dbType.includes('redis')) {
		return [
			{key: 'REDIS_URL', value: `redis://${host}:${port}`, checked: true},
			...common,
		];
	}
	return [
		{
			key: `${prefix}_URL`,
			value: `${dbType}://${host}:${port}`,
			checked: true,
		},
		...common,
	];
}
