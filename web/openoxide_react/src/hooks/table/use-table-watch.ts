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
} from '#/utils/table-query-keys';

// Realtime table invalidation: streamed table names → prefixes in #/utils/table-query-keys → refetch.

const realtimeClient = createServiceClient(RealtimeService);

// Reconnects with backoff; each stream is aborted on unmount / org switch.
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
				// Reconnect below; the resync heals missed changes, and the warn keeps a dead stream from looking like broken realtime. unauthenticated arrives as an end-of-stream frame, bypassing the unary refresh-and-replay interceptor.
				if (
					error instanceof ConnectError &&
					error.code === Code.Unauthenticated
				) {
					const refresh = await refreshAccessToken().catch(() => null);
					// Session is dead: the app shell drops the user and routes to
					// sign-in — reconnecting would just loop against a 401.
					if (refresh?.status === 'expired') return;
					authHealed = refresh?.status === 'ok';
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
