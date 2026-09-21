import type {OverviewServiceItem} from './overview-services-table';

function normalizeDbKind(raw: string): string {
	const dbKind = raw.toUpperCase();
	if (dbKind.includes('POSTGRES')) return 'POSTGRES';
	if (dbKind.includes('MARIADB')) return 'MARIADB';
	if (dbKind.includes('MYSQL')) return 'MYSQL';
	if (dbKind.includes('MONGO')) return 'MONGO';
	if (dbKind.includes('REDIS')) return 'REDIS';
	if (dbKind.includes('LIBSQL') || dbKind.includes('SQLITE'))
		return 'LIBSQL';
	return dbKind;
}

function resolveServerName(svc: any, serverDirectory: any[]): string {
	const server = serverDirectory.find(
		(item: any) => Number(item.id) === Number(svc.server_id),
	);
	if (svc.server_name) return svc.server_name;
	if (server)
		return server.name || server.hostname || `Server ${server.id}`;
	if (svc.server_id == null) return 'Localhost (Main server)';
	return `Server ${svc.server_id}`;
}

/** Dedupe raw overview rows and map them into table view-models. */
export function toServiceItems(
	rawServices: any[] | undefined,
	serverDirectory: any[],
): OverviewServiceItem[] {
	if (!rawServices || !Array.isArray(rawServices)) return [];

	const uniqueServices = new Map<string, any>();
	for (const svc of rawServices) {
		const type = String(svc.service_type || 'APP').toUpperCase();
		uniqueServices.set(
			`${type}:${normalizeDbKind(svc.db_kind || '')}:${String(svc.id)}`,
			svc,
		);
	}

	return Array.from(uniqueServices.values()).map((svc: any) => ({
		key: `${svc.service_type}-${svc.db_kind || ''}-${svc.id}`,
		id: Number(svc.id),
		name: svc.name,
		type: (svc.service_type || 'APP') as 'APP' | 'COMPOSE' | 'DATABASE',
		status: svc.status || 'done',
		createdAt: Number(svc.created_at),
		projectId: Number(svc.project_id),
		projectName: svc.project_name || 'Project',
		environmentId: Number(svc.environment_id),
		environmentName: svc.environment_name || 'production',
		dbKind: svc.db_kind || undefined,
		serverName: resolveServerName(svc, serverDirectory),
	}));
}
