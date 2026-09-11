import {useEffect} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {createServiceClient} from '#/api/connect';
import {useOrganizations} from '#/stores/session-context';
import {
	RealtimeService,
	WatchedTable,
	type WatchTablesEvent,
} from '#/gen/connect/openoxide/api/v1/realtime_pb';

/**
 * Table-level realtime invalidation.
 *
 * The backend streams which SQLite tables changed within the active
 * organization (never row data). Each event maps back to the query-key
 * prefixes used by the live hooks, and those queries are invalidated so
 * TanStack refetches them through the normal permission-checked RPCs.
 *
 * The first event on every stream is a full resync (all tables) covering
 * the gap between initial page load and the stream opening.
 */

const realtimeClient = createServiceClient(RealtimeService);

/** Query-key prefixes that a watched table invalidates. */
const TABLE_QUERY_KEYS: Record<WatchedTable, string[]> = {
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	[WatchedTable.UNSPECIFIED]: [],
	[WatchedTable.AI_GENERATIONS]: [],
	[WatchedTable.AI_SETTINGS]: ['ai_settings'],
	[WatchedTable.ALERT_EVENTS]: [],
	[WatchedTable.ALERT_RULES]: [],
	// Middleware CRUD rides the domain dialogs (the domains table itself is
	// untouched), so the domain lists need the explicit prefix here.
	[WatchedTable.APPLICATION_MIDDLEWARES]: ['domains', 'overview-domains'],
	[WatchedTable.AUDIT_LOGS]: [],
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
	[WatchedTable.SERVERS]: ['servers'],
	[WatchedTable.SERVICE_APPLICATION]: ['applications', 'overview-services'],
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
const RESYNC_TABLE_COUNT = Object.keys(TABLE_QUERY_KEYS).length - 1;

/**
 * Opens the per-organization watch stream over the Connect transport and
 * invalidates queries as table changes arrive. Reconnects with backoff
 * when the stream drops, each reconnect starting with a fresh resync
 * event. The stream is aborted on unmount / organization switch so no
 * HTTP connection outlives the subscription.
 */
export function useTableWatch(): void {
	const queryClient = useQueryClient();
	const activeOrg = useOrganizations().activeOrg;
	const orgId = activeOrg?.id || 1;

	useEffect(() => {
		let closed = false;
		let attempt = 0;
		let timer: ReturnType<typeof setTimeout> | undefined;
		const abort = new AbortController();

		const invalidate = (event: WatchTablesEvent) => {
			if (event.tables.length >= RESYNC_TABLE_COUNT) {
				console.info('[table-watch] resync → full invalidation');
				queryClient.invalidateQueries();
				return;
			}
			const prefixes = new Set<string>();
			for (const table of event.tables) {
				for (const key of TABLE_QUERY_KEYS[table] ?? []) {
					prefixes.add(key);
				}
			}
			if (prefixes.size > 0) {
				console.info('[table-watch] change →', [...prefixes]);
			}
			for (const key of prefixes) {
				queryClient.invalidateQueries({queryKey: [key]});
			}
		};

		const connect = async () => {
			try {
				for await (const event of realtimeClient.watchTables({}, {signal: abort.signal})) {
					if (closed || abort.signal.aborted) break;
					attempt = 0;
					invalidate(event);
				}
			} catch (error) {
				// Stream failed (network drop, panel restart, abort).
				// Reconnect below; the resync event heals any missed
				// changes. Surface the failure — a silently dying stream
				// otherwise looks exactly like "realtime not working".
				if (!closed && !abort.signal.aborted) {
					console.warn('[table-watch] stream dropped, reconnecting', error);
				}
			}
			if (!closed && !abort.signal.aborted) {
				attempt = Math.min(attempt + 1, 6);
				timer = setTimeout(connect, Math.min(1000 * 2 ** attempt, 30_000));
			}
		};

		void connect();
		return () => {
			closed = true;
			abort.abort();
			if (timer) clearTimeout(timer);
		};
	}, [queryClient, orgId]);
}
