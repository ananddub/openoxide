import {createServiceClient, useResourceListQuery, useResourceQuery} from './query';
import {RemoteServerService} from '#/gen/connect/openoxide/api/v1/remote_server_pb';

export type RemoteServerPayload = {
	id: number;
	name: string;
	description?: string;
	ip_address: string;
	port: number;
	username: string;
	app_name: string;
	server_status: string;
	server_type: string;
	enable_docker_cleanup: number;
	log_cleanup_cron?: string;
	command: string;
	metrics_config: string;
	ssh_key_id?: number;
	created_at: number;
	updated_at: number;
	build_memory_limit?: string;
	build_cpu_limit?: string;
};

const remoteServerClient = createServiceClient(RemoteServerService);

/** Remote servers (legacy live: useRemoteServerList). */
export function useRemoteServerList() {
	return useResourceListQuery<RemoteServerPayload>(
		['servers', 'list'],
		() => remoteServerClient.listRemoteServers({}),
	);
}

/** Single remote server (legacy live: useRemoteServerGet). */
export function useRemoteServerGet(id: bigint) {
	return useResourceQuery<RemoteServerPayload>(
		['servers', 'get', id.toString()],
		() => remoteServerClient.getRemoteServer({id}),
	);
}
