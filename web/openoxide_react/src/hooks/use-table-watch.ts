import {useEffect} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {createServiceClient} from '#/api/connect';
import {useAppStore} from '#/stores/app-store';
import {useOrganizationStore} from '#/stores/organization-store';
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
	[WatchedTable.APPLICATION_MIDDLEWARES]: [],
	[WatchedTable.AUDIT_LOGS]: [],
	[WatchedTable.BACKGROUND_POLICIES]: ['settings'],
	[WatchedTable.BACKUP_EXECUTIONS]: ['backups', 'overview-backups'],
	[WatchedTable.BACKUPS]: ['backups', 'overview-backups'],
	[WatchedTable.CERTIFICATE_RENEWALS]: ['certificates'],
	[WatchedTable.CERTIFICATES]: ['certificates'],
	[WatchedTable.DATABASE_NETWORKS]: [],
	[WatchedTable.DEPLOYMENTS]: ['deployments'],
	[WatchedTable.DESTINATIONS]: ['destinations'],
	[WatchedTable.DNS_PROVIDERS]: ['dns_providers'],
	[WatchedTable.DOMAINS]: ['domains', 'overview-domains'],
	[WatchedTable.ENVIRONMENTS]: ['environments'],
	[WatchedTable.GIT_PROVIDERS]: [],
	[WatchedTable.GROUP_POLICY]: ['permission-groups'],
	[WatchedTable.GROUPS]: ['permission-groups'],
	[WatchedTable.HEALTH_REPORTS]: [],
	[WatchedTable.MONITORING_AGENTS]: [],
	[WatchedTable.MONITORING_MAINTENANCE_WINDOWS]: [],
	[WatchedTable.MONITORING_POLICY]: [],
	[WatchedTable.MOUNTS]: [],
	[WatchedTable.NOTIFICATION_DELIVERY_ATTEMPTS]: ['notifications'],
	[WatchedTable.NOTIFICATION_RESOURCE_BINDINGS]: ['notifications'],
	[WatchedTable.NOTIFICATIONS]: ['notifications'],
	[WatchedTable.ORGANIZATION]: ['organizations'],
	[WatchedTable.ORGANIZATION_INVITES]: ['permission-invites'],
	[WatchedTable.ORGANIZATION_MEMBERS]: ['permission-members'],
	[WatchedTable.PATCHES]: [],
	[WatchedTable.POLICY]: ['permission-groups'],
	[WatchedTable.PORTS]: [],
	[WatchedTable.PREVIEW_DEPLOYMENTS]: [],
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
	[WatchedTable.SERVER_CLEANUP_EXECUTIONS]: [],
	[WatchedTable.SERVER_MANAGEMENT]: [],
	[WatchedTable.SERVER_MIGRATIONS]: ['servers'],
	[WatchedTable.SERVER_PRIVATE_NETWORKS]: ['servers'],
	[WatchedTable.SERVERS]: ['servers'],
	[WatchedTable.SERVICE_APPLICATION]: ['applications', 'overview-services'],
	[WatchedTable.SERVICE_COMPOSE]: ['composes', 'overview-services'],
	[WatchedTable.SERVICE_DATABASE]: ['databases', 'overview-services'],
	[WatchedTable.SERVICE_GIT]: [],
	[WatchedTable.SERVICES]: ['overview-services'],
	[WatchedTable.SETTINGS]: ['settings'],
	[WatchedTable.SSH_KEYS]: ['ssh_keys'],
	[WatchedTable.TAGS]: ['tags'],
	[WatchedTable.USER_POLICY]: [],
	[WatchedTable.USERS]: ['auth'],
	[WatchedTable.VAULT_PROVIDERS]: ['vault_providers'],
	[WatchedTable.VOLUME_BACKUPS]: ['backups', 'overview-backups'],
};

/** A resync event lists every watchable table minus the UNSPECIFIED entry. */
const RESYNC_TABLE_COUNT = Object.keys(TABLE_QUERY_KEYS).length - 1;

/**
 * Opens the per-organization watch stream over the Connect transport and
 * invalidates queries as table changes arrive. Reconnects with backoff
 * when the stream drops, each reconnect starting with a fresh resync
 * event. The stream is aborted on unmount / organization switch so no
 * HTTP connection outlives the subscription.
 *
 * The stream's liveness also drives the app store's `isWsConnected` flag
 * (true while events are flowing, false on drop) — replacing the legacy
 * socket-era constant.
 */
export function useTableWatch(): void {
	const queryClient = useQueryClient();
	const activeOrg = useOrganizationStore((state) => state.activeOrg);
	const orgId = activeOrg?.id || 1;
	const setWsConnected = useAppStore((state) => state.setWsConnected);

	useEffect(() => {
		let closed = false;
		let attempt = 0;
		let timer: ReturnType<typeof setTimeout> | undefined;
		const abort = new AbortController();

		const invalidate = (event: WatchTablesEvent) => {
			if (event.tables.length >= RESYNC_TABLE_COUNT) {
				queryClient.invalidateQueries();
				return;
			}
			const prefixes = new Set<string>();
			for (const table of event.tables) {
				for (const key of TABLE_QUERY_KEYS[table] ?? []) {
					prefixes.add(key);
				}
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
					setWsConnected(true);
					invalidate(event);
				}
			} catch {
				// Stream failed (network drop, panel restart, abort).
				// Reconnect below; the resync event heals any missed
				// changes.
			}
			setWsConnected(false);
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
	}, [queryClient, orgId, setWsConnected]);
}
