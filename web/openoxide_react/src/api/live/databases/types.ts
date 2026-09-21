import {
	DatabaseKind,
	type Database,
	type SwarmSettings,
	type Mount,
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

// The proto kind is a numeric enum; legacy payloads carried a lowercase
// string ('postgres', 'redis', ...), so reverse-map to keep the wire shape.
export function toDatabasePayload(database: Database): DatabasePayload {
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

export function toSwarmSettingsPayload(
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

export type DatabaseMountPayload = {
	id: number;
	mount_type: string;
	host_path?: string | null;
	volume_name?: string | null;
	file_path?: string | null;
	content?: string | null;
	mount_path: string;
};

export function toMountPayload(mount: Mount): DatabaseMountPayload {
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

export type DatabaseAdvancedConfigPayload = {
	settings: Record<string, string>;
	replication_role: string;
};

export type DatabaseNetworkPayload = {
	id: number | string;
	name: string;
	docker_network_name: string;
	server_id?: number | string | null;
	[key: string]: unknown;
};
