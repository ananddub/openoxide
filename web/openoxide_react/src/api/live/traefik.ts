import {createServiceClient, useConnectMutation, useResourceListQuery, useResourceQuery} from './query';
// Only TraefikOperationsService is served by the backend (TraefikService has
// no controller), so the live layer targets the operations variant.
import {TraefikOperationsService} from '#/gen/connect/openoxide/api/v1/traefik_operations_pb';

/** Traefik file-tree node (backend TraefikFileTreeNodeDto JSON). */
export type TraefikFileTreeNodePayload = {
	name: string;
	relative_path: string;
	node_type: string;
	size: number;
	is_readonly: boolean;
	modified_at: number;
	children: TraefikFileTreeNodePayload[];
	[key: string]: unknown;
};

/** Traefik file content (backend TraefikFileContentDto JSON). */
export type TraefikFileContentPayload = {
	path: string;
	content: string;
	is_readonly: boolean;
	[key: string]: unknown;
};

const traefikClient = createServiceClient(TraefikOperationsService);

/**
 * Traefik dynamic-config file tree for a server (legacy REST:
 * GET /traefik/files/tree). The backend carries the server id in the
 * query's `organization_id` slot; 0n targets the panel's local Traefik.
 */
export function useTraefikFileTree(serverId: bigint = 0n) {
	return useResourceListQuery<TraefikFileTreeNodePayload>(
		['traefik', 'file-tree', serverId.toString()],
		() => traefikClient.tree({organizationId: serverId}),
	);
}

/**
 * Content of a single Traefik config file (legacy REST:
 * GET /traefik/files/content). The path rides in the request payload and the
 * server id in the `organization_id` slot.
 */
export function useTraefikFileContent(serverId: bigint, path: string) {
	return useResourceQuery<TraefikFileContentPayload>(
		['traefik', 'file-content', serverId.toString(), path],
		() => traefikClient.getContent({organizationId: serverId, payload: JSON.stringify({path})}),
	);
}

/**
 * Overwrite a Traefik config file (legacy REST: PUT /traefik/files/content).
 * The backend takes `{server_id?, path, content}` in the payload; a null
 * server_id targets the panel's local Traefik.
 */
export function useTraefikFileContentUpdate() {
	return useConnectMutation(
		(input: {server_id?: number | bigint | null; path: string; content: string}) =>
			traefikClient.updateContent({
				payload: JSON.stringify({
					server_id: input.server_id === null || input.server_id === undefined ? null : Number(input.server_id),
					path: input.path,
					content: input.content,
				}),
			}),
	);
}

/**
 * Traefik health snapshot for a server (legacy REST: GET /traefik/health).
 * Returns the backend's HealthDto JSON (is_healthy, rawdata_status,
 * configuration_errors).
 */
export function useTraefikHealth() {
	return useResourceQuery<{
		is_healthy: boolean;
		rawdata_status: string;
		configuration_errors: string[];
		[key: string]: unknown;
	}>(['traefik', 'health'], async () => {
		const response = await traefikClient.health({});
		return JSON.parse(response.payload || '{}');
	});
}

/**
 * Traefik request-logging status (legacy REST: GET /traefik/requests/status).
 * Returns whether request logging is currently enabled.
 */
export function useTraefikRequestsStatus() {
	return useResourceQuery<{
		enabled: boolean;
		[key: string]: unknown;
	}>(['traefik', 'requests-status'], async () => {
		const response = await traefikClient.requestStatus({});
		return JSON.parse(response.payload || '{}');
	});
}

/** Toggle Traefik request logging on/off (legacy REST: POST /traefik/requests/toggle). */
export function useTraefikRequestsToggle() {
	return useConnectMutation((input: {enabled: boolean}) =>
		traefikClient.toggleRequests({
			payload: JSON.stringify({enabled: input.enabled}),
		}),
	);
}

/** Traefik request logs (legacy REST: GET /traefik/requests/logs). */
export function useTraefikRequestLogs(serverId: bigint = 0n) {
	return useResourceListQuery<Record<string, unknown>>(
		['traefik', 'request-logs', serverId.toString()],
		() => traefikClient.logs({organizationId: serverId}),
	);
}
