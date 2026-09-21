import {WatchedTable} from '#/gen/connect/openoxide/api/v1/realtime_pb';

/**
 * Table-level realtime invalidation map: each watched SQLite table lists the
 * query-key prefixes its changes invalidate. The live hooks then refetch
 * through the normal permission-checked RPCs.
 */

/** Query-key prefixes that a watched table invalidates. */
export const TABLE_QUERY_KEYS: Record<WatchedTable, string[]> = {
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	[WatchedTable.UNSPECIFIED]: [],
	[WatchedTable.AI_GENERATIONS]: [],
	[WatchedTable.AI_SETTINGS]: ['ai_settings'],
	[WatchedTable.ALERT_EVENTS]: [],
	[WatchedTable.ALERT_RULES]: [],
	// Middleware CRUD rides the domain dialogs (the domains table itself is
	// untouched), so the domain lists need the explicit prefix here.
	[WatchedTable.APPLICATION_MIDDLEWARES]: ['domains', 'overview-domains'],
	[WatchedTable.AUDIT_LOGS]: ['audit_logs'],
	[WatchedTable.BACKGROUND_POLICIES]: ['settings'],
	[WatchedTable.BACKUP_EXECUTIONS]: ['backups', 'overview-backups'],
	[WatchedTable.BACKUPS]: ['backups', 'overview-backups'],
	[WatchedTable.CERTIFICATE_RENEWALS]: ['certificates'],
	[WatchedTable.CERTIFICATES]: ['certificates'],
	// Database networks feed the per-database network pickers and the
	// `database-networks` registry list.
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
	// Database mounts and port mappings are keyed under the `databases`
	// prefix (['databases', 'mounts', …] and the detail-tab port lists).
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
	// Cleanup runs only refresh server status timestamps; no query exposes
	// them yet.
	[WatchedTable.SERVER_CLEANUP_EXECUTIONS]: [],
	// Server-management writes (cleanup cron, metrics config) land inside
	// the SERVERS-backed payloads, so the server lists must refetch.
	[WatchedTable.SERVER_MANAGEMENT]: ['servers'],
	[WatchedTable.SERVER_MIGRATIONS]: ['servers'],
	[WatchedTable.SERVER_PRIVATE_NETWORKS]: ['servers'],
	// Stateful per-service migrations surface on the server lists (source +
	// target rows) and drive the migration wizard's live progress view.
	[WatchedTable.APPLICATION_MIGRATIONS]: ['servers', 'migrations'],
	[WatchedTable.MIGRATION_OBJECTS]: ['migrations'],
	[WatchedTable.SERVERS]: ['servers'],
	[WatchedTable.SERVICE_APPLICATION]: [
		'applications',
		'overview-services',
	],
	[WatchedTable.SERVICE_COMPOSE]: ['composes', 'overview-services'],
	[WatchedTable.SERVICE_DATABASE]: ['databases', 'overview-services'],
	// Git-source settings (provider/repo/branch per app or compose) are not
	// mirrored into the SERVICE_* rows, so invalidate the service lists too.
	[WatchedTable.SERVICE_GIT]: ['applications', 'composes'],
	// The services row carries app_status for every service kind, so a
	// status flip (e.g. STARTING → RUNNING after a database deploy) must
	// refetch each kind's list/detail queries, not just the overview.
	[WatchedTable.SERVICES]: [
		'overview-services',
		'applications',
		'composes',
		'databases',
	],
	[WatchedTable.SETTINGS]: ['settings'],
	[WatchedTable.SSH_KEYS]: ['ssh_keys'],
	// Tag renames/recolors surface on project cards (hashtags derive from the
	// project description) and project dialogs, so projects is included.
	[WatchedTable.TAGS]: ['tags', 'projects'],
	[WatchedTable.USER_POLICY]: [],
	[WatchedTable.USERS]: ['auth'],
	[WatchedTable.VAULT_PROVIDERS]: ['vault_providers'],
	[WatchedTable.VOLUME_BACKUPS]: ['backups', 'overview-backups'],
	// Tables above left empty (AI_GENERATIONS, ALERT_*, AUDIT_LOGS,
	// HEALTH_REPORTS, MONITORING_*, PATCHES, REDIRECTS, RESOURCE_ACCESS,
	// ROLLBACKS, SECURITY, USER_POLICY) have no live query keys yet — the
	// mutation files own their refresh until a query adopts these prefixes.
};

/** A resync event lists every watchable table minus the UNSPECIFIED entry. */
export const RESYNC_TABLE_COUNT = Object.keys(TABLE_QUERY_KEYS).length - 1;
