// Pure data mappers behind useProjectDetails: they fold the org-wide live
// service lists (applications / compose / databases / overview fallback)
// into a single normalized service list. Kept free of React so the hook stays
// a thin composition.

import {appendOverviewServices} from './use-project-details.overview';

export function normalizeDatabaseKind(value: unknown): string {
	const kind = String(value || '').toLowerCase();
	if (kind.includes('postgres')) return 'postgres';
	if (kind.includes('maria')) return 'mariadb';
	if (kind.includes('mysql')) return 'mysql';
	if (kind.includes('mongo')) return 'mongo';
	if (kind.includes('redis')) return 'redis';
	if (kind.includes('libsql') || kind.includes('sqlite')) return 'libsql';
	return kind || 'postgres';
}

/** Server-name resolver bound to the current remote-server list. */
export function makeGetServerName(servers: any[]) {
	return (service: any) => {
		if (service?.server_name || service?.serverName)
			return String(service.server_name || service.serverName);
		const serverId = service?.server_id ?? service?.serverId;
		if (serverId == null) return 'Localhost (Main server)';
		const server = (servers || []).find(
			(item: any) => String(item.id) === String(serverId),
		);
		return String(server?.name || `Server ${serverId}`);
	};
}

/** True when a service row belongs to `projectId` and the active environment. */
function matchesScope(
	item: any,
	projectId: number,
	activeEnvId: number | null,
): boolean {
	if (
		String(item.project_id || item.projectId || projectId) !==
		String(projectId)
	)
		return false;
	const envId = item.environment_id ?? item.environmentId;
	return (
		activeEnvId === null ||
		envId === undefined ||
		envId === null ||
		Number(envId) === Number(activeEnvId)
	);
}

// Collect a project's database rows from the legacy `*Dbs` arrays hung off
// the project payload, tagging each with its kind.
const DB_KIND_FIELDS: [string, string][] = [
	['postgresDbs', 'postgres'],
	['mysqlDbs', 'mysql'],
	['mariadbDbs', 'mariadb'],
	['mongoDbs', 'mongo'],
	['redisDbs', 'redis'],
	['libsqlDbs', 'libsql'],
];

function collectProjectDatabases(project: any): any[] {
	const out: any[] = [];
	if (!project) return out;
	for (const [field, kind] of DB_KIND_FIELDS) {
		if (Array.isArray(project[field]))
			out.push(...project[field].map((d: any) => ({...d, kind})));
	}
	return out;
}

export function buildProjectServices(args: {
	applications: any[];
	composes: any[];
	databases: any[];
	overviewServices: any[];
	project: any;
	projectId: number;
	activeEnvId: number | null;
	getServerName: (service: any) => string;
}): any[] {
	const {
		applications,
		composes,
		databases,
		overviewServices,
		project,
		projectId,
		activeEnvId,
		getServerName,
	} = args;
	const result: any[] = [];
	const seenKeys = new Set<string>();

	// 1. Applications from the project query or the org-wide live list.
	[...(applications || []), ...(project?.applications || [])].forEach(
		(app: any) => {
			if (!matchesScope(app, projectId, activeEnvId)) return;
			const key = `APP-${app.id}`;
			if (seenKeys.has(key)) return;
			seenKeys.add(key);
			result.push({
				id: app.id,
				type: 'APP',
				name: app.name || app.app_name,
				subtitle: app.app_name || app.name,
				status: app.app_status || app.status || 'STOPPED',
				createdAt: app.created_at || Date.now(),
				projectId: Number(app.project_id || projectId),
				environmentId: app.environment_id ?? app.environmentId,
				serverName: getServerName(app),
			});
		},
	);

	// 2. Compose stacks.
	[...(composes || []), ...(project?.composes || [])].forEach((c: any) => {
		if (!matchesScope(c, projectId, activeEnvId)) return;
		const key = `COMPOSE-${c.id}`;
		if (seenKeys.has(key)) return;
		seenKeys.add(key);
		result.push({
			id: c.id,
			type: 'COMPOSE',
			name: c.name || c.app_name,
			subtitle: c.app_name || c.name,
			status: c.compose_status || c.status || 'STOPPED',
			createdAt: c.created_at || Date.now(),
			projectId: Number(c.project_id || projectId),
			environmentId: c.environment_id ?? c.environmentId,
			serverName: getServerName(c),
		});
	});

	// 3. Databases (org-wide list plus the project's per-kind rows).
	[...(databases || []), ...collectProjectDatabases(project)].forEach(
		(db: any) => {
			if (!matchesScope(db, projectId, activeEnvId)) return;
			const dbKind = normalizeDatabaseKind(
				db.kind || db.type || db.db_kind || db.dbKind,
			);
			const key = `DATABASE-${dbKind}-${String(db.id)}`;
			if (seenKeys.has(key)) return;
			seenKeys.add(key);
			result.push({
				id: db.id,
				type: 'DATABASE',
				name: db.name || db.app_name,
				subtitle: dbKind.toUpperCase(),
				status: db.app_status || db.status || 'STOPPED',
				createdAt: db.created_at || Date.now(),
				dbKind,
				projectId: Number(db.project_id || projectId),
				environmentId: db.environment_id ?? db.environmentId,
				serverName: getServerName(db),
			});
		},
	);

	// 4. Overview-services fallback for anything not in the direct stores.
	appendOverviewServices(result, seenKeys, {
		overviewServices,
		projectId,
		activeEnvId,
		getServerName,
	});

	return result;
}
