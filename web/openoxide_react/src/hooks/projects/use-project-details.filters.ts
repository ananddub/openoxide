// Search / type / status filtering over the normalized project service list.
const DB_KINDS = [
	'postgres',
	'mysql',
	'mariadb',
	'mongo',
	'redis',
	'libsql',
];

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
			DB_KINDS.includes(rawKind);
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
