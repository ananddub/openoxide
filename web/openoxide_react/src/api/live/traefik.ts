import {
	createServiceClient,
	useConnectMutation,
	useConnectQuery,
} from './query';
import {
	TraefikService,
	TraefikFileNodeType,
	type TraefikFileTreeNode,
	type TraefikLogEntry,
} from '#/gen/connect/openoxide/api/v1/traefik_pb';

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

/** Paged request-log payload (legacy live: `{items, total_count, page, page_size}`). */
export type TraefikRequestLogsPayload = {
	items: Record<string, unknown>[];
	total_count: number;
	page: number;
	page_size: number;
};

const traefikClient = createServiceClient(TraefikService);

/** Enum value → member name lower-cased ('file', 'directory'); the legacy
 * payload compared with lower-case strings. */
function nodeTypeName(value: TraefikFileNodeType): string {
	return (TraefikFileNodeType[value] ?? '').toLowerCase();
}

function toFileTreePayload(
	node: TraefikFileTreeNode,
): TraefikFileTreeNodePayload {
	return {
		name: node.name,
		relative_path: node.relativePath,
		node_type: nodeTypeName(node.nodeType),
		size: Number(node.size),
		is_readonly: node.isReadonly,
		modified_at: Number(node.modifiedAt),
		children: node.children.map(toFileTreePayload),
	};
}

function toLogEntryPayload(
	entry: TraefikLogEntry,
): Record<string, unknown> {
	return {
		timestamp: entry.timestamp,
		client_ip: entry.clientIp,
		method: entry.method,
		path: entry.path,
		status: entry.status,
		duration_ms: entry.durationMs,
		service_name: entry.serviceName,
		router_name: entry.routerName,
	};
}

/** Traefik dynamic-config file tree for a server (legacy REST: GET /traefik/files/tree). 0n targets the panel's local Traefik (proto `optional int64` stays unset). */
export function useTraefikFileTree(serverId: bigint = 0n) {
	return useConnectQuery<TraefikFileTreeNodePayload[]>(
		['traefik', 'file-tree', serverId.toString()],
		async () => {
			const response = await traefikClient.getFileTree({
				serverId: serverId > 0n ? serverId : undefined,
			});
			return response.nodes.map(toFileTreePayload);
		},
	);
}

/** Content of a single Traefik config file (legacy REST: GET /traefik/files/content). Empty path (no file selected) disables the query. */
export function useTraefikFileContent(serverId: bigint, path: string) {
	return useConnectQuery<TraefikFileContentPayload>(
		['traefik', 'file-content', serverId.toString(), path],
		async () => {
			const response = await traefikClient.getFileContent({
				serverId: serverId > 0n ? serverId : undefined,
				path,
			});
			const file = response.file;
			return {
				path: file?.path ?? path,
				content: file?.content ?? '',
				is_readonly: file?.isReadonly ?? false,
			};
		},
		{enabled: path !== ''},
	);
}

/** Overwrite a Traefik config file (legacy REST: PUT /traefik/files/content). A null server_id targets the panel's local Traefik. */
export function useTraefikFileContentUpdate() {
	return useConnectMutation(
		(input: {
			server_id?: number | bigint | null;
			path: string;
			content: string;
		}) =>
			traefikClient.updateFileContent({
				serverId:
					input.server_id === null || input.server_id === undefined
						? undefined
						: BigInt(input.server_id),
				path: input.path,
				content: input.content,
			}),
		// Traefik file writes are filesystem-side (no CDC table), so the tree and content queries must be invalidated directly.
		{invalidate: [['traefik']]},
	);
}

/** Traefik health snapshot (legacy REST: GET /traefik/health). Returns the HealthDto shape (is_healthy, rawdata_status, configuration_errors). */
export function useTraefikHealth(options?: {enabled?: boolean}) {
	return useConnectQuery<{
		is_healthy: boolean;
		rawdata_status: string;
		configuration_errors: string[];
		[key: string]: unknown;
	}>(
		['traefik', 'health'],
		async () => {
			const response = await traefikClient.health({});
			const health = response.health;
			return {
				is_healthy: health?.isHealthy ?? false,
				rawdata_status: health?.rawdataStatus ?? '',
				configuration_errors: health?.configurationErrors ?? [],
			};
		},
		options,
	);
}

/** Traefik request-logging status (legacy REST: GET /traefik/requests/status). The typed response carries `is_active`, `log_path` and `cron_expression` (consumers read the snake_case keys). */
export function useTraefikRequestsStatus() {
	return useConnectQuery<{
		is_active: boolean;
		log_path: string;
		cron_expression: string;
		[key: string]: unknown;
	}>(['traefik', 'requests-status'], async () => {
		const response = await traefikClient.getRequestStatus({});
		const status = response.status;
		return {
			is_active: status?.isActive ?? false,
			log_path: status?.logPath ?? '',
			cron_expression: status?.cronExpression ?? '',
		};
	});
}

/** Toggle Traefik request logging on/off (legacy REST: POST /traefik/requests/toggle). */
export function useTraefikRequestsToggle() {
	return useConnectMutation(
		(input: {enabled: boolean}) =>
			traefikClient.toggleRequests({enable: input.enabled}),
		{invalidate: [['traefik']]},
	);
}

/** Traefik request logs (legacy REST: GET /traefik/requests/logs). */
export function useTraefikRequestLogs(serverId: bigint = 0n) {
	return useConnectQuery<TraefikRequestLogsPayload>(
		['traefik', 'request-logs', serverId.toString()],
		async () => {
			const response = await traefikClient.getRequestLogs({
				serverId: serverId > 0n ? serverId : undefined,
			});
			return {
				items: response.entries.map(toLogEntryPayload),
				total_count: Number(response.totalCount),
				page: Number(response.page),
				page_size: Number(response.pageSize),
			};
		},
	);
}
