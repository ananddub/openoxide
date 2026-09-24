// Folds the org-wide live service lists into one normalized list, then filters it.

/** Engine kinds a database row can carry. */
const DATABASE_KINDS = [
	'postgres',
	'mysql',
	'mariadb',
	'mongo',
	'redis',
	'libsql',
];

// Project payload hangs its database rows on one array per kind.
const DB_KIND_FIELDS: [string, string][] = DATABASE_KINDS.map(kind => [
	`${kind}Dbs`,
	kind,
]);

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

// Database rows live on the legacy per-kind `*Dbs` arrays of the project payload.
function collectProjectDatabases(project: any): any[] {
	const out: any[] = [];
	if (!project) return out;
	for (const [field, kind] of DB_KIND_FIELDS) {
		if (Array.isArray(project[field]))
			out.push(...project[field].map((d: any) => ({...d, kind})));
	}
	return out;
}

type ScopeArgs = {
	projectId: number;
	activeEnvId: number | null;
	getServerName: (service: any) => string;
};

/** Overview-services fallback: fills anything not in the direct live lists. */
function appendOverviewServices(
	result: any[],
	seenKeys: Set<string>,
	args: {overviewServices: any[]} & ScopeArgs,
): void {
	const {overviewServices, projectId, activeEnvId, getServerName} = args;
	overviewServices.forEach((s: any) => {
		if (!s || String(s.project_id || s.projectId) !== String(projectId))
			return;
		const envId = s.environment_id ?? s.environmentId;
		if (
			activeEnvId !== null &&
			envId !== undefined &&
			envId !== null &&
			Number(envId) !== Number(activeEnvId)
		)
			return;
		const sType = String(s.service_type || s.type || '').toUpperCase();
		const isDb =
			sType === 'DATABASE' ||
			sType.includes('DB') ||
			!!s.db_kind ||
			!!s.dbKind;
		const isCompose = sType === 'COMPOSE' || sType.includes('COMPOSE');
		const normalizedType = isDb
			? 'DATABASE'
			: isCompose
				? 'COMPOSE'
				: 'APP';
		const dbKind = isDb
			? normalizeDatabaseKind(s.db_kind || s.dbKind || s.kind)
			: undefined;
		const key = isDb
			? `DATABASE-${dbKind}-${s.id}`
			: `${normalizedType}-${s.id}`;
		if (seenKeys.has(key)) return;
		seenKeys.add(key);
		result.push({
			id: s.id,
			type: normalizedType,
			name: s.name,
			subtitle: isDb
				? dbKind
					? dbKind.toUpperCase()
					: 'DATABASE'
				: s.name || s.app_name,
			status: s.status || s.app_status || 'STOPPED',
			createdAt: s.created_at || Date.now(),
			...(dbKind ? {dbKind} : {}),
			projectId: Number(s.project_id || projectId),
			environmentId: envId,
			serverName: getServerName(s),
		});
	});
}

export function buildProjectServices(
	args: {
		applications: any[];
		composes: any[];
		databases: any[];
		overviewServices: any[];
		project: any;
	} & ScopeArgs,
): any[] {
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

/** Search / type / status filtering over the normalized service list. */
export function buildFilteredServices(args: {
	projectServices: any[];
	searchQuery: string;
	typeFilter: string;
	statusFilter: string;
	projectId: number;
}): any[] {
	const {
		projectServices,
		searchQuery,
		typeFilter,
		statusFilter,
		projectId,
	} = args;
	const sQuery = String(searchQuery || '')
		.toLowerCase()
		.trim();

	let list = projectServices.map((s: any) => {
		const rawKind = String(
			s?.kind || s?.db_kind || s?.dbKind || '',
		).toLowerCase();
		const isDb =
			s?.type === 'database' ||
			s?.type === 'DATABASE' ||
			DATABASE_KINDS.includes(rawKind);
		const rawType = isDb
			? 'DATABASE'
			: String(
					s?.service_type || s?.serviceType || s?.type || 'APP',
				).toUpperCase();
		const dbKind = rawKind || (isDb ? 'postgres' : undefined);
		return {
			key: `${rawType.toLowerCase()}-${dbKind || 'svc'}-${s.id}`,
			projectId,
			type: rawType as 'APP' | 'COMPOSE' | 'DATABASE',
			id: s.id,
			name: String(s?.name || s?.app_name || s?.appName || ''),
			subtitle:
				rawType === 'APP'
					? 'Application'
					: rawType === 'COMPOSE'
						? 'Docker Compose'
						: dbKind
							? dbKind.toUpperCase()
							: 'Database',
			status: String(s?.status || s?.app_status || 'idle'),
			createdAt: s?.createdAt || s?.created_at,
			dbKind: dbKind,
			serverName: s?.serverName || 'Localhost (Main server)',
		};
	});

	if (sQuery) {
		list = list.filter((item: any) =>
			String(item?.name || '')
				.toLowerCase()
				.includes(sQuery),
		);
	}

	if (typeFilter !== 'all') {
		list = list.filter((item: any) => {
			const itemType = String(item?.type || '').toLowerCase();
			if (typeFilter === 'app') return itemType === 'app';
			if (typeFilter === 'compose') return itemType === 'compose';
			if (typeFilter === 'database') return itemType === 'database';
			return true;
		});
	}

	if (statusFilter !== 'all') {
		list = list.filter((item: any) => {
			const st = String(item?.status || '').toLowerCase();
			const isRunning =
				st.includes('running') ||
				st.includes('active') ||
				st.includes('healthy') ||
				st.includes('up');
			return statusFilter === 'running' ? isRunning : !isRunning;
		});
	}

	return list;
}
