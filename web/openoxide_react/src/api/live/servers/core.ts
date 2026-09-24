import {useConnectMutation} from '../query';
import {
	RemoteServerStatus,
	RemoteServerType,
} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {remoteServerClient, serverIdMutation} from './shared';

/** Maps a snake_case enum name (e.g. 'DEPLOY') to the proto enum value. */
function enumFromName<E extends number>(
	labels: Record<string, E>,
	name: string | undefined,
	fallback: E,
): E {
	if (!name) return fallback;
	return labels[name.toUpperCase()] ?? fallback;
}

const serverTypeByName = RemoteServerType as unknown as Record<
	string,
	RemoteServerType
>;
const serverStatusByName = RemoteServerStatus as unknown as Record<
	string,
	RemoteServerStatus
>;

/** Create a remote server (SSH connection fields as typed inputs). */
export function useRemoteServerCreate() {
	return useConnectMutation(
		(input: {
			name: string;
			ip_address: string;
			port?: number;
			username: string;
			server_type?: string;
			ssh_key_id?: number | bigint;
			description?: string;
			build_memory_limit?: string;
			build_cpu_limit?: string;
		}) =>
			remoteServerClient.createRemoteServer({
				name: input.name,
				description: input.description,
				ipAddress: input.ip_address,
				port: BigInt(input.port ?? 0),
				username: input.username,
				serverType: enumFromName(
					serverTypeByName,
					input.server_type,
					RemoteServerType.DEPLOY,
				),
				sshKeyId:
					input.ssh_key_id != null ? BigInt(input.ssh_key_id) : undefined,
				buildMemoryLimit: input.build_memory_limit,
				buildCpuLimit: input.build_cpu_limit,
			}),
		{invalidate: [['servers']]},
	);
}

/** Update a remote server by id (PATCH semantics: absent fields unchanged). */
export function useRemoteServerUpdate() {
	return useConnectMutation(
		(input: {
			id: number | bigint;
			name?: string;
			description?: string;
			ip_address?: string;
			port?: number;
			username?: string;
			server_status?: string;
			server_type?: string;
			enable_docker_cleanup?: boolean;
			log_cleanup_cron?: string;
			command?: string;
			metrics_config?: string;
			ssh_key_id?: number | bigint;
			build_memory_limit?: string;
			build_cpu_limit?: string;
		}) =>
			remoteServerClient.updateRemoteServer({
				id: BigInt(input.id),
				name: input.name,
				description: input.description,
				ipAddress: input.ip_address,
				port: input.port != null ? BigInt(input.port) : undefined,
				username: input.username,
				serverStatus:
					input.server_status !== undefined
						? enumFromName(
								serverStatusByName,
								input.server_status,
								RemoteServerStatus.UNSPECIFIED,
							)
						: undefined,
				serverType:
					input.server_type !== undefined
						? enumFromName(
								serverTypeByName,
								input.server_type,
								RemoteServerType.UNSPECIFIED,
							)
						: undefined,
				enableDockerCleanup: input.enable_docker_cleanup,
				logCleanupCron: input.log_cleanup_cron,
				command: input.command,
				metricsConfig: input.metrics_config,
				sshKeyId:
					input.ssh_key_id != null ? BigInt(input.ssh_key_id) : undefined,
				buildMemoryLimit: input.build_memory_limit,
				buildCpuLimit: input.build_cpu_limit,
			}),
		{invalidate: [['servers']]},
	);
}

/** Delete a remote server by id. */
export const useRemoteServerDelete = serverIdMutation(id =>
	remoteServerClient.deleteRemoteServer({id}),
);

/** Mark a remote server active by id. */
export const useRemoteServerActivate = serverIdMutation(id =>
	remoteServerClient.activate({id}),
);

/** Mark a remote server inactive by id. */
export const useRemoteServerDeactivate = serverIdMutation(id =>
	remoteServerClient.deactivate({id}),
);

/** Refresh the stored last-test timestamp for a remote server by id. */
export const useRemoteServerTestConnection = serverIdMutation(id =>
	remoteServerClient.testConnection({id}),
);
