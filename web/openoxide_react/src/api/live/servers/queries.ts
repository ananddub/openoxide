import {useConnectMutation, useConnectQuery} from '../query';
import {
	RemoteServerStatus,
	RemoteServerType,
	type RemoteServer,
} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {remoteServerClient, serverControllerClient} from './shared';
import type {RemoteServerPayload, ServerAuditPayload} from './types';

/** Maps the typed `RemoteServer` message to the snake_case payload shape the server UI components read (`ip_address`/`server_status`/…); enums become their name strings ('ACTIVE', 'DEPLOY', …) and int64s become numbers. */
export function toRemoteServerPayload(
	server: RemoteServer,
): RemoteServerPayload {
	return {
		id: Number(server.id),
		name: server.name,
		description: server.description,
		ip_address: server.ipAddress,
		port: Number(server.port),
		username: server.username,
		app_name: server.appName,
		server_status: RemoteServerStatus[server.serverStatus],
		server_type: RemoteServerType[server.serverType],
		enable_docker_cleanup: server.enableDockerCleanup ? 1 : 0,
		log_cleanup_cron: server.logCleanupCron,
		command: server.command,
		metrics_config: server.metricsConfig,
		ssh_key_id:
			server.sshKeyId !== undefined ? Number(server.sshKeyId) : undefined,
		created_at: Number(server.createdAt),
		updated_at: Number(server.updatedAt),
		build_memory_limit: server.buildMemoryLimit,
		build_cpu_limit: server.buildCpuLimit,
		ip: server.ipAddress,
	};
}

/** Remote servers (legacy live: useRemoteServerList). */
export function useRemoteServerList() {
	return useConnectQuery<RemoteServerPayload[]>(
		['servers', 'list'],
		async () => {
			const response = await remoteServerClient.listRemoteServers({});
			return response.servers.map(toRemoteServerPayload);
		},
	);
}

/** Single remote server (legacy live: useRemoteServerGet). */
export function useRemoteServerGet(id: bigint) {
	return useConnectQuery<RemoteServerPayload>(
		['servers', 'get', id.toString()],
		async () => {
			const response = await remoteServerClient.getRemoteServer({id});
			if (!response.server) throw new Error('remote server not found');
			return toRemoteServerPayload(response.server);
		},
		{enabled: id > 0n},
	);
}

/** SSH/agent audit for a remote server (legacy live: useRemoteServerAudit). The typed `ServerAudit` message is surfaced as-is (camelCase fields). */
export function useRemoteServerAudit() {
	return useConnectMutation(async (id: number | bigint) => {
		const response = await serverControllerClient.audit({id: BigInt(id)});
		return (response.audit ?? {}) as unknown as ServerAuditPayload;
	});
}
