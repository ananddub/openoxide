import {createServiceClient, useConnectMutation, useConnectQuery, useResourceListQuery, useResourceQuery} from './query';
import {readResource} from '#/api/live/resource';
import {RemoteServerService} from '#/gen/connect/openoxide/api/v1/remote_server_pb';
import {ServerControllerService} from '#/gen/connect/openoxide/api/v1/server_controller_pb';

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
	/** Legacy camelCase alias the zustand RemoteServerItem carried; components read `ip_address || ip`. */
	ip?: string;
};

const remoteServerClient = createServiceClient(RemoteServerService);
const serverControllerClient = createServiceClient(ServerControllerService);

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

/** Migration progress for a dependency migration (backend ServerDependencyMigrationDto JSON). */
export type ServerMigrationPayload = {
	migration_id: string;
	status: string;
	source_server_id: number;
	target_server_id: number;
	applications: number;
	build_assignments: number;
	compose_projects: number;
	databases: number;
	certificates: number;
	schedules: number;
	queued_applications: number;
	queued_compose_projects: number;
	error?: string | null;
	[key: string]: unknown;
};

/** Create a remote server (SSH connection fields in the payload). */
export function useRemoteServerCreate() {
	return useConnectMutation((input: Record<string, unknown>) =>
		remoteServerClient.createRemoteServer({payload: JSON.stringify(input)}),
	);
}

/** Update a remote server by id (fields to change in the payload). */
export function useRemoteServerUpdate() {
	return useConnectMutation((input: {id: number | bigint; [field: string]: unknown}) => {
		const {id, ...body} = input;
		return remoteServerClient.updateRemoteServer({id: BigInt(id), payload: JSON.stringify(body)});
	});
}

/** Delete a remote server by id. */
export function useRemoteServerDelete() {
	return useConnectMutation((id: number | bigint) =>
		remoteServerClient.deleteRemoteServer({id: BigInt(id)}),
	);
}

/** Mark a remote server active by id. */
export function useRemoteServerActivate() {
	return useConnectMutation((id: number | bigint) =>
		remoteServerClient.activate({id: BigInt(id)}),
	);
}

/** Mark a remote server inactive by id. */
export function useRemoteServerDeactivate() {
	return useConnectMutation((id: number | bigint) =>
		remoteServerClient.deactivate({id: BigInt(id)}),
	);
}

/** Refresh the stored last-test timestamp for a remote server by id. */
export function useRemoteServerTestConnection() {
	return useConnectMutation((id: number | bigint) =>
		remoteServerClient.testConnection({id: BigInt(id)}),
	);
}

/**
 * Migrate a remote server's dependencies to another server (the target
 * server id rides in the payload).
 */
export function useRemoteServerMigrateDependencies() {
	return useConnectMutation((input: {id: number | bigint; target_server_id: number | bigint}) =>
		remoteServerClient.migrateDependencies({
			id: BigInt(input.id),
			payload: JSON.stringify({target_server_id: Number(input.target_server_id)}),
		}),
	);
}

/**
 * Watch a dependency migration. The backend takes the migration id as a
 * plain (non-JSON) payload string.
 */
export function useRemoteServerMigration(migrationId: string) {
	return useConnectQuery<ServerMigrationPayload>(
		['servers', 'migration', migrationId],
		async () => readResource<ServerMigrationPayload>(
			await remoteServerClient.getMigration({payload: migrationId}),
		),
		{enabled: migrationId !== ''},
	);
}

/** Roll back a dependency migration (migration id as a plain payload string). */
export function useRemoteServerRollbackMigration() {
	return useConnectMutation((migrationId: string) =>
		remoteServerClient.rollbackMigration({payload: migrationId}),
	);
}

/**
 * SSH/agent audit for a remote server (legacy live: useRemoteServerAudit).
 * ResourceRequest takes the server id; OperationResult carries the audit
 * report JSON in its (optional) payload field.
 */
export type ServerAuditPayload = Record<string, unknown>;

export function useRemoteServerAudit() {
	return useConnectMutation(async (id: number | bigint) => {
		const response = await serverControllerClient.audit({id: BigInt(id)});
		return JSON.parse(response.payload || '{}') as ServerAuditPayload;
	});
}
