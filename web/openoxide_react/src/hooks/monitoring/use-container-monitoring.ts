import {useState, useEffect, useCallback, useRef} from 'react';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {
	parseStats,
	type MonitoringEntityType,
} from '#/utils/container-metrics';
import {useMonitoringCache} from './monitoring-cache';

export type {
	ContainerMetrics,
	MonitoringEntityType,
} from '#/utils/container-metrics';

const deploymentClient = createServiceClient(DeploymentService);

/** True when a docker-stats line's container name belongs to `service`.
 *  Compose names containers `{project}-{service}-N` (or `_service_N`), so match
 *  the service token on a name boundary rather than a loose substring. */
function lineMatchesService(line: string, service: string): boolean {
	const match = line.match(/"Name"\s*:\s*"([^"]*)"/i);
	const name = (match?.[1] ?? '').toLowerCase();
	if (!name) return false;
	const svc = service.toLowerCase();
	return (
		name === svc ||
		name.includes(`-${svc}-`) ||
		name.includes(`_${svc}_`) ||
		name.endsWith(`-${svc}`) ||
		name.endsWith(`_${svc}`) ||
		name.includes(`-${svc}.`) ||
		name.includes(`_${svc}.`)
	);
}

/** Live docker stats for an application / database / compose project over the panel's ConnectRPC stream. Each event payload is raw `docker stats --format '{{json .}}'` output (one JSON object per line); the first object of a chunk drives the UI. */
export function useContainerMonitoring(
	id: number,
	entityType: MonitoringEntityType = 'application',
	enabled = true,
	live = true,
	serviceName?: string,
) {
	// Cache is scoped to the current detail page (via MonitoringCacheProvider);
	// it is null when used outside one, in which case caching is skipped.
	const statsCache = useMonitoringCache();
	const cacheKey = `${entityType}:${id}:${serviceName ?? ''}`;
	const [isLive, setIsLive] = useState(live);
	const [isLoading, setIsLoading] = useState(false);
	const [hasError, setHasError] = useState(false);
	const [rawStats, setRawStats] = useState<unknown>(
		() => statsCache?.get(cacheKey) ?? null,
	);
	const [refetchTrigger, setRefetchTrigger] = useState(0);
	const lastKeyRef = useRef<string | null>(null);

	const triggerRefresh = useCallback(() => {
		setRefetchTrigger(prev => prev + 1);
	}, []);

	useEffect(() => {
		if (!id || !enabled) {
			setIsLoading(false);
			return;
		}

		let isMounted = true;
		const controller = new AbortController();

		setIsLoading(true);
		setHasError(false);
		// Keep the last snapshot while a refresh is in flight — clearing it
		// here made every consumer blink once per refetch. Only a genuinely
		// different target resets the value.
		const key = cacheKey;
		if (lastKeyRef.current !== key) {
			lastKeyRef.current = key;
			// Show the cached value for this target (if any) rather than
			// blanking; the live stream refreshes it in place.
			setRawStats(statsCache?.get(key) ?? null);
		}

		const streamStats = async () => {
			const request = {id: BigInt(id)};
			try {
				const call =
					entityType === 'database'
						? deploymentClient.streamDatabaseStats(request, {
								signal: controller.signal,
							})
						: entityType === 'compose'
							? deploymentClient.streamComposeStats(request, {
									signal: controller.signal,
								})
							: deploymentClient.streamApplicationStats(request, {
									signal: controller.signal,
								});
				let frames = 0;
				for await (const event of call) {
					if (!isMounted) return;
					setIsLoading(false);
					if (event.error) {
						setHasError(true);
						continue;
					}
					const lines = event.payload
						.split('\n')
						.map(line => line.trim())
						.filter(Boolean);
					// For a specific compose service, keep the line whose
					// container name matches it; the compose stream interleaves
					// every project container. Otherwise take the first line.
					const pick = serviceName
						? lines.find(line => lineMatchesService(line, serviceName))
						: lines[0];
					if (pick) {
						frames += 1;
						// Docker's first stream frame is the since-start average
						// (near-zero CPU). Skip it only on a cold start; if a
						// cached value already exists (tab re-mount), show every
						// frame immediately so the value never blanks.
						const hadCache = statsCache?.has(key) ?? false;
						if (frames >= 2 || hadCache) {
							statsCache?.set(key, pick);
							setRawStats(pick);
						}
					}
					// One-shot mode stops after a usable snapshot (immediately
					// when priming from cache, else after the 2nd real frame).
					if (!isLive && (frames >= 2 || (statsCache?.has(key) ?? false)))
						break;
				}
			} catch (err: unknown) {
				const isAbort = (err as {name?: string})?.name === 'AbortError';
				if (!isAbort && isMounted) {
					setHasError(true);
				}
			} finally {
				if (isMounted) setIsLoading(false);
			}
		};

		void streamStats();

		return () => {
			isMounted = false;
			controller.abort();
		};
	}, [id, entityType, isLive, refetchTrigger, enabled, serviceName, statsCache]);

	return {
		isLive,
		setIsLive,
		isLoading,
		hasError,
		metrics: parseStats(rawStats),
		triggerRefresh,
	};
}
