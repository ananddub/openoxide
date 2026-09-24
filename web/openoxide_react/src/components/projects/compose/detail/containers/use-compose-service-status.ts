import {useMemo} from 'react';
import {useDeploymentListContainers} from '#/api/live';
import {
	parseComposeFile,
	deriveServiceStatus,
	type ServiceStatus,
} from '#/components/projects/compose/visualizer/compose-parse';
import {buildServiceStatusMap} from './build-compose-containers';

const POLL_MS = 7_000;

/** Live per-service runtime state for the compose architecture canvas. Containers are not a table-watch stream, so this polls the docker snapshot and matches container names to compose service names. Services whose containers cannot be matched fall back to the stack-level `compose_status`, which the monitoring reconciler keeps live. */
export function useComposeServiceStatus(
	compose: any,
	composeFile?: string | null,
): Record<string, ServiceStatus> | undefined {
	const enabled = !!(compose?.app_name || compose?.name);
	const serverId = compose?.destination_id || compose?.server_id;
	const {data} = useDeploymentListContainers(
		serverId ? Number(serverId) : null,
		{enabled, refetchInterval: POLL_MS},
	);

	return useMemo(() => {
		if (!enabled) return undefined;
		const names = parseComposeFile(composeFile ?? '').map(s => s.name);
		const map: Record<string, ServiceStatus> = data
			? (buildServiceStatusMap(compose, data, names) as Record<
					string,
					ServiceStatus
				>)
			: {};
		const stackStatus = deriveServiceStatus(
			compose?.compose_status || compose?.status,
		);
		if (stackStatus) {
			for (const n of names) {
				if (!(n in map)) map[n] = stackStatus;
			}
		}
		return Object.keys(map).length > 0 ? map : undefined;
	}, [enabled, data, compose, composeFile]);
}
