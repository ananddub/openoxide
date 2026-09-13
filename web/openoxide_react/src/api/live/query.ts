import {useMutation, useQuery, useQueryClient, type UseMutationResult, type UseQueryResult} from '@tanstack/react-query';
import {createServiceClient} from '#/api/connect';
import {useOrganizations} from '#/stores/session-context';
import {readResourceList, readResource} from '#/api/live/resource';

/**
 * Live-data hook helpers over ConnectRPC queries.
 *
 * Payloads keep the backend DTO's snake_case JSON fields (Rust serde wire
 * format); liveness comes from refetches driven by the realtime table-watch
 * stream.
 */

// QueryClient defaults: 5 min staleTime, refetchOnWindowFocus off. Live
// queries lean on the table-watch stream (useTableWatch) for freshness:
// data is cached for 1 minute, and any table change in the active
// organization invalidates the affected queries immediately — so a longer
// staleTime no longer means stale screens, only fewer redundant fetches.
export const LIVE_STALE_TIME_MS = 60_000;

/**
 * Active organization id for org-scoped list queries (falls back to the
 * legacy default org 1, mirroring useTableWatch's org resolution).
 */
export function useActiveOrgId(): bigint {
	const activeOrg = useOrganizations().activeOrg;
	return BigInt(activeOrg?.id || 1);
}

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
	options?: {enabled?: boolean},
): UseQueryResult<Payload> {
	return useConnectQuery(key, async () => {
		const response = await fetcher();
		return readResource<Payload>(response);
	}, options);
}

/**
 * Mutation over a ConnectRPC client method. The table-watch stream
 * invalidates affected queries on success, so no manual invalidation list
 * is needed; pass `invalidate` only when a mutation must refresh something
 * the stream cannot observe (e.g. agent-side state).
 */
export function useConnectMutation<Input, Output>(
	mutate: (input: Input) => Promise<Output>,
	options?: {
		invalidate?: Array<unknown[]>;
		onSuccess?: (data: Output, input: Input) => void | Promise<void>;
		onError?: (error: Error, input: Input) => void | Promise<void>;
	},
): UseMutationResult<Output, Error, Input> {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: mutate,
		onSuccess: async (data, input) => {
			for (const key of options?.invalidate ?? []) {
				await queryClient.invalidateQueries({queryKey: key});
			}
			await options?.onSuccess?.(data, input);
		},
		onError: async (error, input) => {
			await options?.onError?.(error, input);
		},
	});
}

export {createServiceClient};
