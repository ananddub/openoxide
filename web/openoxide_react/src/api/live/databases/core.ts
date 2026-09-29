import {DatabaseKind} from '#/gen/connect/openoxide/api/v1/database_pb';
import {
	databaseClient,
	dbInputMutation,
	kindIdMutation,
	big,
} from './shared';
import type {DatabaseSwarmSettingsPayload} from './types';

/** Create a database of the given kind (legacy REST: POST /databases/{kind}). */
export const useDatabaseCreate = dbInputMutation(
	(input: {
		kind: DatabaseKind;
		name: string;
		description?: string;
		environment_id: number | bigint;
		server_id?: number | bigint;
		docker_image?: string;
		database_name?: string;
		database_user?: string;
		database_password?: string;
		external_port?: number | bigint;
		args?: string[];
		env_var?: string;
		network_ids?: string[];
	}) =>
		databaseClient.createDatabase({
			kind: input.kind,
			name: input.name,
			description: input.description,
			environmentId: BigInt(input.environment_id),
			serverId: big(input.server_id),
			dockerImage: input.docker_image,
			databaseName: input.database_name,
			databaseUser: input.database_user,
			databasePassword: input.database_password,
			externalPort: big(input.external_port),
			args: input.args ?? [],
			envVar: input.env_var,
			networkIds: input.network_ids ?? [],
		}),
);

/** Update a database of the given kind by id (legacy REST: PATCH /databases/{kind}/{id}). */
export const useDatabaseUpdate = dbInputMutation(
	(input: {
		kind: DatabaseKind;
		id: number | bigint;
		name?: string;
		description?: string;
		docker_image?: string;
		database_name?: string;
		database_user?: string;
		database_password?: string;
		external_port?: number | bigint;
		command?: string;
		args?: string[];
		env_var?: string;
		memory_limit?: string;
		cpu_limit?: string;
		memory_reservation?: string;
		cpu_reservation?: string;
		replicas?: number | bigint;
		server_id?: number | bigint;
		network_ids?: string[];
		detach_rustploy_network?: number | bigint;
	}) =>
		databaseClient.updateDatabase({
			kind: input.kind,
			id: BigInt(input.id),
			name: input.name,
			description: input.description,
			dockerImage: input.docker_image,
			databaseName: input.database_name,
			databaseUser: input.database_user,
			databasePassword: input.database_password,
			externalPort: big(input.external_port),
			command: input.command,
			args: input.args ?? [],
			envVar: input.env_var,
			memoryLimit: input.memory_limit,
			cpuLimit: input.cpu_limit,
			memoryReservation: input.memory_reservation,
			cpuReservation: input.cpu_reservation,
			replicas: big(input.replicas),
			serverId: big(input.server_id),
			networkIds: input.network_ids ?? [],
			detachRustployNetwork: big(input.detach_rustploy_network),
		}),
);

/** Delete a database of the given kind by id (legacy REST: DELETE /databases/{kind}/{id}). */
export const useDatabaseDelete = kindIdMutation((kind, id) =>
	databaseClient.deleteDatabase({kind, id}),
);

/** Replace a database's swarm settings (legacy REST: PUT /database-management/{kind}/{id}/swarm-settings). */
export const useDatabaseUpdateSwarmSettings = dbInputMutation(
	(input: {
		kind: DatabaseKind;
		id: number | bigint;
		values: DatabaseSwarmSettingsPayload;
	}) =>
		databaseClient.updateSwarmSettings({
			kind: input.kind,
			id: BigInt(input.id),
			settings: {
				healthCheck: input.values.health_check_swarm ?? undefined,
				restartPolicy: input.values.restart_policy_swarm ?? undefined,
				placement: input.values.placement_swarm ?? undefined,
				updateConfig: input.values.update_config_swarm ?? undefined,
				rollbackConfig: input.values.rollback_config_swarm ?? undefined,
				mode: input.values.mode_swarm ?? undefined,
				labels: input.values.labels_swarm ?? undefined,
				network: input.values.network_swarm ?? undefined,
				endpointSpec: input.values.endpoint_spec_swarm ?? undefined,
				ulimits: input.values.ulimits_swarm ?? undefined,
				stopGracePeriod:
					input.values.stop_grace_period_swarm != null
						? BigInt(input.values.stop_grace_period_swarm)
						: undefined,
			},
		}),
);

type DatabaseMountInput = {
	kind: DatabaseKind;
	database_id: number | bigint;
	mount_type: string;
	host_path?: string;
	volume_name?: string;
	file_path?: string;
	content?: string;
	mount_path: string;
};

/** Attach a mount to a database (legacy REST: POST /database-management/{kind}/{id}/mounts). */
export const useDatabaseMountCreate = dbInputMutation(
	(input: DatabaseMountInput) =>
		databaseClient.createMount({
			kind: input.kind,
			databaseId: BigInt(input.database_id),
			mountType: input.mount_type,
			hostPath: input.host_path,
			volumeName: input.volume_name,
			filePath: input.file_path,
			content: input.content,
			mountPath: input.mount_path,
		}),
);

/** Update a database mount by id (legacy REST: PUT /database-management/{kind}/{database_id}/mounts/{id}). */
export const useDatabaseMountUpdate = dbInputMutation(
	(input: DatabaseMountInput & {id: number | bigint}) =>
		databaseClient.updateMount({
			kind: input.kind,
			databaseId: BigInt(input.database_id),
			id: BigInt(input.id),
			mountType: input.mount_type,
			hostPath: input.host_path,
			volumeName: input.volume_name,
			filePath: input.file_path,
			content: input.content,
			mountPath: input.mount_path,
		}),
);

/** Remove a database mount by id (legacy REST: DELETE /database-management/{kind}/{database_id}/mounts/{id}). */
export const useDatabaseMountDelete = dbInputMutation(
	(input: {
		kind: DatabaseKind;
		database_id: number | bigint;
		id: number | bigint;
	}) =>
		databaseClient.deleteMount({
			kind: input.kind,
			databaseId: BigInt(input.database_id),
			id: BigInt(input.id),
		}),
);

/** Update postgres advanced configuration (legacy REST: PUT /database-management/postgres/{id}/advanced-config). */
export const useDatabaseUpdatePostgresAdvancedConfig = dbInputMutation(
	(input: {
		id: number | bigint;
		replication_role: string;
		settings: Record<string, string>;
	}) =>
		databaseClient.updatePostgresAdvancedConfig({
			kind: DatabaseKind.POSTGRES,
			id: BigInt(input.id),
			config: {
				replicationRole: input.replication_role,
				config: JSON.stringify(input.settings),
			},
		}),
);
