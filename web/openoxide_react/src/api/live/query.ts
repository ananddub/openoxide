import {useQuery, type UseQueryResult} from '@tanstack/react-query';
import {createServiceClient} from '#/api/connect';
import {readResourceList, readResource} from '#/api/live/resource';

/**
 * Live-data hook helpers over ConnectRPC queries.
 *
 * Replaces the former `virtual:openoxide-live` Socket.IO hooks. Data shape is
 * intentionally preserved (snake_case JSON payloads), so consumer components
 * keep working unchanged; only the transport moved from socket pushes to
 * Connect queries with refetch-based liveness.
 */

// QueryClient defaults: 5 min staleTime, refetchOnWindowFocus off. Live
// queries lean on the table-watch stream (useTableWatch) for freshness:
// data is cached for 1 minute, and any table change in the active
// organization invalidates the affected queries immediately — so a longer
// staleTime no longer means stale screens, only fewer redundant fetches.
export const LIVE_STALE_TIME_MS = 60_000;

export function useConnectQuery<Output>(
	key: unknown[],
	fetcher: () => Promise<Output>,
	options?: {enabled?: boolean},
): UseQueryResult<Output> {
	return useQuery({
		queryKey: key,
		queryFn: fetcher,
		staleTime: LIVE_STALE_TIME_MS,
		refetchOnWindowFocus: true,
		enabled: options?.enabled,
	});
}

/** Wraps a Resource-pattern list response into its parsed payload array. */
export function useResourceListQuery<Payload>(
	key: unknown[],
	fetcher: () => Promise<{items: Array<{payload: string}>}>,
): UseQueryResult<Payload[]> {
	return useConnectQuery(key, async () => {
		const response = await fetcher();
		return readResourceList<Payload>(response);
	});
}

/** Wraps a Resource-pattern single response into its parsed payload. */
export function useResourceQuery<Payload>(
	key: unknown[],
	fetcher: () => Promise<{payload: string}>,
): UseQueryResult<Payload> {
	return useConnectQuery(key, async () => {
		const response = await fetcher();
		return readResource<Payload>(response);
	});
}

export {createServiceClient};
