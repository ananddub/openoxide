import {createServiceClient, useActiveOrgId, useConnectMutation, useConnectQuery, useResourceListQuery} from './query';
import {DatabaseService, DatabaseKind, type Database} from '#/gen/connect/openoxide/api/v1/database_pb';
import {OverviewOperationsService} from '#/gen/connect/openoxide/api/v1/overview_operations_pb';
import type {OverviewServiceItemPayload} from './overview';
import {DatabaseNetworkService} from '#/gen/connect/openoxide/api/v1/database_network_pb';
import type {
	AdvancedConfig,
	Mount,
	SwarmSettings,
} from '#/gen/connect/openoxide/api/v1/database_pb';

/**
 * Org-wide database list item (all-optional fields) derived from the
 * overview services stream — keyed under the `databases` prefix so
 * SERVICE_DATABASE table changes invalidate it.
 */
export type DatabaseItemPayload = {
	id: number;
	name?: string;
	kind?: string;
	project_id?: number;
	environment_id?: number;
	status?: string;
	server_id?: number;
	server_name?: string;
	created_at?: number;
	updated_at?: number;
};

export type DatabasePayload = {
	id: bigint;
	kind: string;
	name: string;
	description?: string;
	docker_image: string;
	database_name?: string;
	database_user?: string;
	database_password?: string;
	external_port?: bigint;
	command?: string;
	args: string[];
	env_var?: string;
	memory_limit?: string;
	cpu_limit?: string;
	replicas: bigint;
	network_ids: string[];
	status: string;
	last_error?: string;
	environment_id: bigint;
	server_id?: bigint;
	created_at: bigint;
	updated_at: bigint;
};

const databaseClient = createServiceClient(DatabaseService);
const overviewClient = createServiceClient(OverviewOperationsService);

// The proto kind is a numeric enum; legacy payloads carried a lowercase
// string ('postgres', 'redis', ...), so reverse-map to keep the wire shape.
function toDatabasePayload(database: Database): DatabasePayload {
	return {
		id: database.id,
		kind: (DatabaseKind[database.kind] ?? '').toLowerCase(),
		name: database.name,
		description: database.description,
		docker_image: database.dockerImage,
		database_name: database.databaseName,
		database_user: database.databaseUser,
		database_password: database.databasePassword,
		external_port: database.externalPort,
		command: database.command,
		args: database.args,
		env_var: database.envVar,
		memory_limit: database.memoryLimit,
		cpu_limit: database.cpuLimit,
		replicas: database.replicas,
		network_ids: database.networkIds,
		status: database.status,
		last_error: database.lastError,
		environment_id: database.environmentId,
		server_id: database.serverId,
		created_at: database.createdAt,
		updated_at: database.updatedAt,
	};
}

/**
 * Organization-wide database list (legacy app-store `databases` slice).
 * There is no org-wide Database RPC, so this derives from the overview
 * services stream — the same source the legacy realtime sync fed the store
 * from — and is keyed under the `databases` prefix so SERVICE_DATABASE
 * table changes invalidate it.
 */
export function useDatabaseList() {
	const orgId = useActiveOrgId();
	return useConnectQuery<DatabaseItemPayload[]>(
		['databases', orgId.toString()],
		async () => {
			const response = await overviewClient.listOrganizationServices({
				organizationId: orgId,
			});
			return response.items
				.map((item) => JSON.parse(item.payload) as OverviewServiceItemPayload)
				.filter((service) => service.service_type === 'DATABASE')
				.map((service) => ({
					id: Number(service.id),
					name: service.name,
					kind: (service.db_kind ?? '').toLowerCase() || undefined,
					project_id: Number(service.project_id),
					environment_id: Number(service.environment_id),
					status: service.status,
					server_id:
						service.server_id === null ? undefined : Number(service.server_id),
					server_name: service.server_name ?? undefined,
					created_at: Number(service.created_at),
					updated_at: Number(service.created_at),
				} satisfies DatabaseItemPayload));
		},
	);
}

/**
 * Single database of any kind (legacy live: per-kind getters).
 * GetDatabaseRequest requires the kind discriminator alongside the id.
 */
export function useDatabaseGet(kind: DatabaseKind, id: bigint) {
	return useConnectQuery<DatabasePayload>(
		['databases', 'get', kind.toString(), id.toString()],
		async () => toDatabasePayload(await databaseClient.getDatabase({kind, id})),
		{enabled: id > 0n},
	);
}

// Per-kind thin wrappers preserving the legacy hook surface; each just
// resolves to useDatabaseGet with its DatabaseKind.
export function usePostgresGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.POSTGRES, id);
}

export function useMysqlGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.MYSQL, id);
}

export function useMariadbGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.MARIADB, id);
}

export function useMongoGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.MONGO, id);
}

export function useRedisGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.REDIS, id);
}

export function useLibsqlGet(id: bigint) {
	return useDatabaseGet(DatabaseKind.LIBSQL, id);
}

/** Create a database of the given kind (legacy REST: POST /databases/{kind}). */
export function useDatabaseCreate() {
	return useConnectMutation(
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
				serverId: input.server_id !== undefined ? BigInt(input.server_id) : undefined,
				dockerImage: input.docker_image,
				databaseName: input.database_name,
				databaseUser: input.database_user,
				databasePassword: input.database_password,
				externalPort: input.external_port !== undefined ? BigInt(input.external_port) : undefined,
				args: input.args ?? [],
				envVar: input.env_var,
				networkIds: input.network_ids ?? [],
			}),
		{invalidate: [['databases']]},
	);
}

/** Update a database of the given kind by id (legacy REST: PATCH /databases/{kind}/{id}). */
export function useDatabaseUpdate() {
	return useConnectMutation(
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
			replicas?: number | bigint;
			server_id?: number | bigint;
			network_ids?: string[];
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
				externalPort: input.external_port !== undefined ? BigInt(input.external_port) : undefined,
				command: input.command,
				args: input.args ?? [],
				envVar: input.env_var,
				memoryLimit: input.memory_limit,
				cpuLimit: input.cpu_limit,
				replicas: input.replicas !== undefined ? BigInt(input.replicas) : undefined,
				serverId: input.server_id !== undefined ? BigInt(input.server_id) : undefined,
				networkIds: input.network_ids ?? [],
			}),
		{invalidate: [['databases']]},
	);
}

/** Delete a database of the given kind by id (legacy REST: DELETE /databases/{kind}/{id}). */
export function useDatabaseDelete() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.deleteDatabase({kind: input.kind, id: BigInt(input.id)}),
		{invalidate: [['databases']]},
	);
}

/** Deploy a database of the given kind by id (legacy REST: POST /databases/{kind}/{id}/deploy). */
export function useDatabaseDeploy() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.deploy({kind: input.kind, id: BigInt(input.id)}),
		// Deploys also insert a DEPLOYMENTS row, so refresh that list too.
		{invalidate: [['databases'], ['deployments']]},
	);
}

/** Redeploy a database of the given kind by id. */
export function useDatabaseRedeploy() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.redeploy({kind: input.kind, id: BigInt(input.id)}),
		{invalidate: [['databases'], ['deployments']]},
	);
}

/** Reload a database's configuration by id. */
export function useDatabaseReload() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.reload({kind: input.kind, id: BigInt(input.id)}),
		{invalidate: [['databases']]},
	);
}

/** Start a stopped database of the given kind by id. */
export function useDatabaseStart() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.start({kind: input.kind, id: BigInt(input.id)}),
		{invalidate: [['databases']]},
	);
}

/** Stop a running database of the given kind by id. */
export function useDatabaseStop() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.stop({kind: input.kind, id: BigInt(input.id)}),
		{invalidate: [['databases']]},
	);
}

/** Cancel the running deployment of a database of the given kind by id. */
export function useDatabaseCancel() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; id: number | bigint}) =>
			databaseClient.cancel({kind: input.kind, id: BigInt(input.id)}),
		{invalidate: [['databases'], ['deployments']]},
	);
}

/** Map a legacy kind string ('postgres', 'mysql', ...) to the proto enum. */
export function parseDatabaseKind(kind: string): DatabaseKind {
	const value = String(kind || '').toLowerCase();
	if (value.includes('mariadb')) return DatabaseKind.MARIADB;
	if (value.includes('mysql')) return DatabaseKind.MYSQL;
	if (value.includes('mongo')) return DatabaseKind.MONGO;
	if (value.includes('redis')) return DatabaseKind.REDIS;
	if (value.includes('libsql')) return DatabaseKind.LIBSQL;
	return DatabaseKind.POSTGRES;
}

export type DatabaseSwarmSettingsPayload = {
	health_check_swarm?: string | null;
	restart_policy_swarm?: string | null;
	placement_swarm?: string | null;
	update_config_swarm?: string | null;
	rollback_config_swarm?: string | null;
	mode_swarm?: string | null;
	labels_swarm?: string | null;
	network_swarm?: string | null;
	endpoint_spec_swarm?: string | null;
	ulimits_swarm?: string | null;
	stop_grace_period_swarm?: number | null;
};

function toSwarmSettingsPayload(
	settings: SwarmSettings,
): DatabaseSwarmSettingsPayload {
	return {
		health_check_swarm: settings.healthCheck ?? null,
		restart_policy_swarm: settings.restartPolicy ?? null,
		placement_swarm: settings.placement ?? null,
		update_config_swarm: settings.updateConfig ?? null,
		rollback_config_swarm: settings.rollbackConfig ?? null,
		mode_swarm: settings.mode ?? null,
		labels_swarm: settings.labels ?? null,
		network_swarm: settings.network ?? null,
		endpoint_spec_swarm: settings.endpointSpec ?? null,
		ulimits_swarm: settings.ulimits ?? null,
		stop_grace_period_swarm:
			settings.stopGracePeriod != null && settings.stopGracePeriod !== 0n
				? Number(settings.stopGracePeriod)
				: null,
	};
}

/** Swarm settings for a database (legacy REST: GET /database-management/{kind}/{id}/swarm-settings). */
export function useDatabaseSwarmSettings(kind: DatabaseKind, id: bigint) {
	return useConnectQuery<DatabaseSwarmSettingsPayload>(
		['databases', 'swarm-settings', kind.toString(), id.toString()],
		async () =>
			toSwarmSettingsPayload(await databaseClient.getSwarmSettings({kind, id})),
		{enabled: id > 0n},
	);
}

/** Replace a database's swarm settings (legacy REST: PUT /database-management/{kind}/{id}/swarm-settings). */
export function useDatabaseUpdateSwarmSettings() {
	return useConnectMutation(
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
		{invalidate: [['databases']]},
	);
}

export type DatabaseMountPayload = {
	id: number;
	mount_type: string;
	host_path?: string | null;
	volume_name?: string | null;
	file_path?: string | null;
	content?: string | null;
	mount_path: string;
};

function toMountPayload(mount: Mount): DatabaseMountPayload {
	return {
		id: Number(mount.id),
		mount_type: mount.mountType,
		host_path: mount.hostPath ?? null,
		volume_name: mount.volumeName ?? null,
		file_path: mount.filePath ?? null,
		content: mount.content ?? null,
		mount_path: mount.mountPath,
	};
}

/** Mounts attached to a database (legacy REST: GET /database-management/{kind}/{id}/mounts). */
export function useDatabaseMountList(kind: DatabaseKind, id: bigint) {
	return useConnectQuery<DatabaseMountPayload[]>(
		['databases', 'mounts', kind.toString(), id.toString()],
		async () => {
			const response = await databaseClient.listMounts({kind, id});
			return response.mounts.map(toMountPayload);
		},
		{enabled: id > 0n},
	);
}

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
export function useDatabaseMountCreate() {
	return useConnectMutation(
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
		// Mount rows are keyed under the databases prefix (MOUNTS table is
		// also mapped there in the table-watch stream).
		{invalidate: [['databases']]},
	);
}

/** Update a database mount by id (legacy REST: PUT /database-management/{kind}/{database_id}/mounts/{id}). */
export function useDatabaseMountUpdate() {
	return useConnectMutation(
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
		{invalidate: [['databases']]},
	);
}

/** Remove a database mount by id (legacy REST: DELETE /database-management/{kind}/{database_id}/mounts/{id}). */
export function useDatabaseMountDelete() {
	return useConnectMutation(
		(input: {kind: DatabaseKind; database_id: number | bigint; id: number | bigint}) =>
			databaseClient.deleteMount({
				kind: input.kind,
				databaseId: BigInt(input.database_id),
				id: BigInt(input.id),
			}),
		{invalidate: [['databases']]},
	);
}

export type DatabaseAdvancedConfigPayload = {
	settings: Record<string, string>;
	replication_role: string;
};

/** Postgres advanced configuration (legacy REST: GET /database-management/postgres/{id}/advanced-config). */
export function useDatabasePostgresAdvancedConfig(id: bigint) {
	return useConnectQuery<DatabaseAdvancedConfigPayload>(
		['databases', 'postgres-config', id.toString()],
		async () => {
			const config: AdvancedConfig = await databaseClient.getPostgresAdvancedConfig(
				{kind: DatabaseKind.POSTGRES, id},
			);
			return {
				settings: config.config ? (JSON.parse(config.config) as Record<string, string>) : {},
				replication_role: config.replicationRole || 'STANDALONE',
			};
		},
		{enabled: id > 0n},
	);
}

/** Update postgres advanced configuration (legacy REST: PUT /database-management/postgres/{id}/advanced-config). */
export function useDatabaseUpdatePostgresAdvancedConfig() {
	return useConnectMutation(
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
		{invalidate: [['databases']]},
	);
}

const databaseNetworkClient = createServiceClient(DatabaseNetworkService);

export type DatabaseNetworkPayload = {
	id: number | string;
	name: string;
	docker_network_name: string;
	server_id?: number | string | null;
	[key: string]: unknown;
};

/** Database networks across servers (legacy REST: GET /database-networks). */
export function useDatabaseNetworkList() {
	return useResourceListQuery<DatabaseNetworkPayload>(
		['database-networks', 'list'],
		() => databaseNetworkClient.listNetworks({}),
	);
}
