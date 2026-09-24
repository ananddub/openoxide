import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {SystemService} from '#/gen/connect/openoxide/api/v1/system_pb';
import {GlobalService} from '#/gen/connect/openoxide/api/v1/global_pb';

/** Panel instance operations over ConnectRPC: runtime health (database + docker), host Docker disk usage, and Docker prune. */

const systemClient = createServiceClient(SystemService);
const dockerClient = createServiceClient(GlobalService);

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
				operation: 'health',
				message: response.message ?? null,
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
			const parsed = JSON.parse(response.dataJson || '{}') as {
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

/** Prune result reduced to the legacy operation-result shape. */
export type DockerPruneResult = {
	success: boolean;
	operation: string;
	message: string | null;
};

/** Run docker prune (backend gate: server:create). */
export function useDockerPrune() {
	return useConnectMutation(
		async (input: DockerPrunePayload) => {
			const response = await dockerClient.prune({
				containers: input.containers,
				images: input.images,
				networks: input.networks,
				volumes: input.volumes,
			});
			return {
				success: true,
				operation: 'prune',
				message:
					response.operations
						.map(operation => operation.stdout.trim())
						.filter(Boolean)
						.join('\n') || null,
			} satisfies DockerPruneResult;
		},
		{invalidate: [['system', 'disk-usage']]},
	);
}
