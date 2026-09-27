import {useState, useEffect, useCallback, useRef} from 'react';
import {createServiceClient} from '#/api/live/query';
import {DeploymentService} from '#/gen/connect/openoxide/api/v1/deployment_pb';
import {
	parseStats,
	type MonitoringEntityType,
} from '#/utils/container-metrics';

export type {
	ContainerMetrics,
	MonitoringEntityType,
} from '#/utils/container-metrics';

const deploymentClient = createServiceClient(DeploymentService);

/** Live docker stats for an application / database / compose project over the panel's ConnectRPC stream. Each event payload is raw `docker stats --format '{{json .}}'` output (one JSON object per line); the first object of a chunk drives the UI. */
export function useContainerMonitoring(
	id: number,
	entityType: MonitoringEntityType = 'application',
	enabled = true,
	live = true,
) {
	const [isLive, setIsLive] = useState(live);
	const [isLoading, setIsLoading] = useState(false);
	const [hasError, setHasError] = useState(false);
	const [rawStats, setRawStats] = useState<unknown>(null);
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
		const key = `${entityType}:${id}`;
		if (lastKeyRef.current !== key) {
			lastKeyRef.current = key;
			setRawStats(null);
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
					const firstLine = event.payload
						.split('\n')
						.map(line => line.trim())
						.find(Boolean);
					if (firstLine) {
						frames += 1;
						// Docker's first stream frame is the average since the
						// container started — near-zero CPU on long-lived
						// containers. Skip it in every mode; the second frame
						// onward are real ~1s deltas.
						if (frames >= 2) setRawStats(firstLine);
					}
					// Live mode keeps consuming; one-shot mode stops after the first usable snapshot.
					if (!isLive && frames >= 2) break;
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
	}, [id, entityType, isLive, refetchTrigger, enabled]);

	return {
		isLive,
		setIsLive,
		isLoading,
		hasError,
		metrics: parseStats(rawStats),
		triggerRefresh,
	};
}
