import {createServiceClient, useConnectQuery} from './query';
import {DatabaseService, DatabaseKind, type Database} from '#/gen/connect/openoxide/api/v1/database_pb';

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
