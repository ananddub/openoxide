import {WatchedTable} from '#/gen/connect/openoxide/api/v1/realtime_pb';

// Watched SQLite table -> query-key prefixes its changes invalidate.
export const TABLE_QUERY_KEYS: Record<WatchedTable, string[]> = {
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	[WatchedTable.UNSPECIFIED]: [],
	[WatchedTable.AI_GENERATIONS]: [],
	[WatchedTable.AI_SETTINGS]: ['ai_settings'],
	[WatchedTable.ALERT_EVENTS]: [],
	[WatchedTable.ALERT_RULES]: [],
	// Middleware CRUD rides the domain dialogs, so the domain lists need the prefix.
	[WatchedTable.APPLICATION_MIDDLEWARES]: ['domains', 'overview-domains'],
	[WatchedTable.AUDIT_LOGS]: ['audit_logs'],
	[WatchedTable.BACKGROUND_POLICIES]: ['settings'],
	[WatchedTable.BACKUP_EXECUTIONS]: ['backups', 'overview-backups'],
	[WatchedTable.BACKUPS]: ['backups', 'overview-backups'],
	[WatchedTable.CERTIFICATE_RENEWALS]: ['certificates'],
	[WatchedTable.CERTIFICATES]: ['certificates'],
	// Networks feed the per-database pickers and the `database-networks` list.
	[WatchedTable.DATABASE_NETWORKS]: ['databases', 'database-networks'],
	[WatchedTable.DEPLOYMENTS]: ['deployments'],
	[WatchedTable.DESTINATIONS]: ['destinations'],
	[WatchedTable.DNS_PROVIDERS]: ['dns_providers'],
	[WatchedTable.DOMAINS]: ['domains', 'overview-domains'],
	[WatchedTable.ENVIRONMENTS]: ['environments'],
	[WatchedTable.GIT_PROVIDERS]: ['git-providers'],
	[WatchedTable.GROUP_POLICY]: ['permission-groups'],
	[WatchedTable.GROUPS]: ['permission-groups'],
	[WatchedTable.HEALTH_REPORTS]: [],
	[WatchedTable.MONITORING_AGENTS]: [],
	[WatchedTable.MONITORING_MAINTENANCE_WINDOWS]: [],
	[WatchedTable.MONITORING_POLICY]: [],
	// Mounts and port mappings are keyed under `databases`.
	[WatchedTable.MOUNTS]: ['databases'],
	[WatchedTable.NOTIFICATION_DELIVERY_ATTEMPTS]: ['notifications'],
	[WatchedTable.NOTIFICATION_RESOURCE_BINDINGS]: ['notifications'],
	[WatchedTable.NOTIFICATIONS]: ['notifications'],
	[WatchedTable.ORGANIZATION]: ['organizations'],
	[WatchedTable.ORGANIZATION_INVITES]: ['permission-invites'],
	[WatchedTable.ORGANIZATION_MEMBERS]: ['permission-members'],
	[WatchedTable.PATCHES]: [],
	[WatchedTable.POLICY]: ['permission-groups'],
	[WatchedTable.PORTS]: ['databases'],
	[WatchedTable.PREVIEW_DEPLOYMENTS]: ['deployments'],
	[WatchedTable.PROJECTS]: ['projects'],
	[WatchedTable.PROJECT_TAGS]: ['projects', 'tags'],
	[WatchedTable.REDIRECTS]: [],
	[WatchedTable.REGISTRIES]: ['registries'],
	[WatchedTable.RESOURCE_ACCESS]: [],
	[WatchedTable.ROLLBACKS]: [],
	[WatchedTable.SCHEDULE_EXECUTIONS]: ['schedules'],
	[WatchedTable.SCHEDULE_RUNTIME_POLICIES]: ['schedules'],
	[WatchedTable.SCHEDULES]: ['schedules'],
	[WatchedTable.SECURITY]: [],
	// Cleanup only refreshes server status timestamps; no query exposes them yet.
	[WatchedTable.SERVER_CLEANUP_EXECUTIONS]: [],
	// Server-management writes land inside the SERVERS-backed payloads.
	[WatchedTable.SERVER_MANAGEMENT]: ['servers'],
	[WatchedTable.SERVER_MIGRATIONS]: ['servers'],
	[WatchedTable.SERVER_PRIVATE_NETWORKS]: ['servers'],
	// Source + target rows both live on the server lists.
	[WatchedTable.APPLICATION_MIGRATIONS]: ['servers', 'migrations'],
	[WatchedTable.MIGRATION_OBJECTS]: ['migrations'],
	[WatchedTable.SERVERS]: ['servers'],
	[WatchedTable.SERVICE_APPLICATION]: [
		'applications',
		'overview-services',
	],
	[WatchedTable.SERVICE_COMPOSE]: ['composes', 'overview-services'],
	[WatchedTable.SERVICE_DATABASE]: ['databases', 'overview-services'],
	// Git-source settings aren't mirrored into the SERVICE_* rows.
	[WatchedTable.SERVICE_GIT]: ['applications', 'composes'],
	// SERVICES carries app_status for every kind, so a flip refetches all three.
	[WatchedTable.SERVICES]: [
		'overview-services',
		'applications',
		'composes',
		'databases',
	],
	[WatchedTable.SETTINGS]: ['settings'],
	[WatchedTable.SSH_KEYS]: ['ssh_keys'],
	// Project cards derive hashtags from the description, so projects too.
	[WatchedTable.TAGS]: ['tags', 'projects'],
	[WatchedTable.USER_POLICY]: [],
	[WatchedTable.USERS]: ['auth'],
	[WatchedTable.VAULT_PROVIDERS]: ['vault_providers'],
	[WatchedTable.VOLUME_BACKUPS]: ['backups', 'overview-backups'],
	// Empty entries have no live query keys yet; their mutation files own refresh.
};

/** A resync event lists every watchable table minus the UNSPECIFIED entry. */
export const RESYNC_TABLE_COUNT = Object.keys(TABLE_QUERY_KEYS).length - 1;
