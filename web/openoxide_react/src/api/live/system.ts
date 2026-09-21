import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {SystemService} from '#/gen/connect/openoxide/api/v1/system_pb';
import {GlobalOperationsApiService} from '#/gen/connect/openoxide/api/v1/global_pb';

/**
 * Panel instance operations over ConnectRPC: runtime health (database +
 * docker), host Docker disk usage, and Docker prune.
 */

const systemClient = createServiceClient(SystemService);
const dockerClient = createServiceClient(GlobalOperationsApiService);

/** `SystemService.Health` result (backend gate: org:read). */
export type SystemHealthPayload = {
	success: boolean;
	operation: string;
	message: string | null;
};

export function useSystemHealth(options?: {enabled?: boolean}) {
	return useConnectQuery<SystemHealthPayload>(
		['system', 'health'],
		async () => {
			const response = await systemClient.health({});
			return {
				success: response.success,
				operation: response.operation,
				message: response.message || null,
			};
		},
		options,
	);
}

/** One `docker system df` row; values are docker CLI display strings. */
export type DockerDiskUsageItem = {
	Type: string;
	TotalCount: string;
	Active: string;
	Size: string;
	Reclaimable: string;
};

/** Host Docker disk usage (backend gate: server:read). */
export function useDockerDiskUsage(options?: {enabled?: boolean}) {
	return useConnectQuery<DockerDiskUsageItem[]>(
		['system', 'disk-usage'],
		async () => {
			const response = await dockerClient.diskUsage({});
			const parsed = JSON.parse(response.payload || '{}') as {
				items?: DockerDiskUsageItem[];
			};
			return parsed.items ?? [];
		},
		options,
	);
}

/** Prune selection mirrors `DockerPruneRequestDto`. */
export type DockerPrunePayload = {
	containers: boolean;
	images: boolean;
	networks: boolean;
	volumes: boolean;
};

/** Run docker prune (backend gate: server:create). */
export function useDockerPrune() {
	return useConnectMutation(
		(input: DockerPrunePayload) =>
			dockerClient.prune({payload: JSON.stringify(input)}),
		{invalidate: [['system', 'disk-usage']]},
	);
}
