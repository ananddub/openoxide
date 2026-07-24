import {useEffect, useRef, useCallback} from 'react';
import type {paths} from '#/types/api.d.ts';

// Extract only SSE paths (those that have text/event-stream response)
type SsePaths = {
	[P in keyof paths]: paths[P] extends {
		get: {
			responses: {200: {content: {'text/event-stream': unknown}}};
		};
	}
		? P
		: never;
}[keyof paths];

// Extract the SSE event type for a given path
type SseEventType<P extends SsePaths> = paths[P] extends {
	get: {responses: {200: {content: {'text/event-stream': infer T}}}};
}
	? T
	: never;

// Extract query params for a given path
type SseQueryParams<P extends SsePaths> = paths[P] extends {
	get: {parameters: {query?: infer Q}};
}
	? Q
	: never;

// Extract path params for a given path
type SsePathParams<P extends SsePaths> = paths[P] extends {
	get: {parameters: {path?: infer PathP}};
}
	? PathP
	: never;

type UseSSEOptions<P extends SsePaths> = {
	/** Called for every parsed SSE event */
	onMessage: (event: SseEventType<P>) => void;
	/** Called on connection error */
	onError?: (event: Event) => void;
	/** Set to false to skip connecting */
	enabled?: boolean;
} & (SsePathParams<P> extends Record<string, unknown>
	? {path: SsePathParams<P>}
	: {path?: never}) &
	(SseQueryParams<P> extends Record<string, unknown>
		? {query?: SseQueryParams<P>}
		: {query?: never});

/**
 * Type-safe hook for SSE endpoints defined in the OpenAPI spec.
 *
 * @example
 * useSSE('/deployments/{id}/logs', {
 *   path: { id: deploymentId },
 *   onMessage: (event) => console.log(event.line),
 * });
 *
 * @example
 * useSSE('/api/monitoring/stream/containers', {
 *   onMessage: (event) => console.log(event.cpu_percent),
 * });
 */
export function useSSE<P extends SsePaths>(
	path: P,
	options: UseSSEOptions<P>,
) {
	const {onMessage, onError, enabled = true} = options;
	const pathParams =
		(options as {path?: Record<string, unknown>}).path ?? {};
	const queryParams =
		(options as {query?: Record<string, unknown>}).query ?? {};

	// Stable refs — prevent effect from restarting on every render
	const onMessageRef = useRef(onMessage);
	const onErrorRef = useRef(onError);
	onMessageRef.current = onMessage;
	onErrorRef.current = onError;

	const buildUrl = useCallback(
		() => {
			// Replace path params: /deployments/{id}/logs → /deployments/42/logs
			let resolvedPath: string = path;
			for (const [key, value] of Object.entries(pathParams)) {
				resolvedPath = resolvedPath.replace(`{${key}}`, String(value));
			}

			const url = new URL(resolvedPath, window.location.origin);
			for (const [key, value] of Object.entries(queryParams)) {
				if (value != null) {
					url.searchParams.set(key, String(value));
				}
			}

			return url.toString();
		}, // eslint-disable-next-line react-hooks/exhaustive-deps
		[path, JSON.stringify(pathParams), JSON.stringify(queryParams)],
	);

	useEffect(() => {
		if (!enabled) return;

		const url = buildUrl();
		const es = new EventSource(url, {withCredentials: true});

		es.onmessage = (e: MessageEvent) => {
			try {
				const data = JSON.parse(e.data as string) as SseEventType<P>;
				onMessageRef.current(data);
			} catch {
				// Non-JSON frame — skip
			}
		};

		es.onerror = e => {
			onErrorRef.current?.(e);
		};

		return () => {
			es.close();
		};
	}, [enabled, buildUrl]);
}
