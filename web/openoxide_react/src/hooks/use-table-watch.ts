import {useEffect} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {Code, ConnectError} from '@connectrpc/connect';
import {createServiceClient} from '#/api/connect';
import {refreshAccessToken} from '#/api/client';
import {useOrganizations} from '#/stores/session-context';
import {
	RealtimeService,
	type WatchTablesEvent,
} from '#/gen/connect/openoxide/api/v1/realtime_pb';
import {
	RESYNC_TABLE_COUNT,
	TABLE_QUERY_KEYS,
} from './use-table-watch.keys';

/**
 * Table-level realtime invalidation.
 *
 * The backend streams which SQLite tables changed within the active
 * organization (never row data). Each event maps back to the query-key
 * prefixes used by the live hooks (see ./use-table-watch.keys), and those
 * queries are invalidated so TanStack refetches them through the normal
 * permission-checked RPCs.
 *
 * The first event on every stream is a full resync (all tables) covering
 * the gap between initial page load and the stream opening.
 */

const realtimeClient = createServiceClient(RealtimeService);

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
			// A server-stream's `unauthenticated` arrives as an
			// end-of-stream frame, never through the unary
			// interceptor's refresh-and-replay — so an expired access
			// token would otherwise re-fail every retry forever (the
			// backoff pins at its 30s cap and the tab goes permanently
			// stale). Detect it and heal via an explicit refresh,
			// reconnecting immediately instead of backing off.
			let authHealed = false;
			try {
				for await (const event of realtimeClient.watchTables(
					{},
					{signal: abort.signal},
				)) {
					if (closed || abort.signal.aborted) break;
					attempt = 0;
					invalidate(event);
				}
			} catch (error) {
				// Stream failed (network drop, panel restart, abort).
				// Reconnect below; the resync event heals any missed
				// changes. Surface the failure — a silently dying stream
				// otherwise looks exactly like "realtime not working".
				if (
					error instanceof ConnectError &&
					error.code === Code.Unauthenticated
				) {
					authHealed =
						(await refreshAccessToken().catch(() => null)) !== null;
				}
				if (!closed && !abort.signal.aborted) {
					console.warn(
						'[table-watch] stream dropped, reconnecting',
						error,
					);
				}
			}
			if (!closed && !abort.signal.aborted) {
				attempt = authHealed ? 0 : Math.min(attempt + 1, 6);
				timer = setTimeout(
					connect,
					authHealed ? 0 : Math.min(1000 * 2 ** attempt, 30_000),
				);
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
