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

/** One `application_migrations` row (stateful per-service migration). */
export type StatefulMigrationPayload = {
	id: string;
	organization_id: number;
	service_id: number;
	source_server_id: number;
	target_server_id: number;
	status: string;
	phase: string;
	progress: number;
	cancel_requested: number;
	error?: string | null;
	log_path: string;
	created_at: number;
	updated_at: number;
};

/** One `migration_objects` checkpoint row. */
export type MigrationObjectPayload = {
	migration_id: string;
	obj_key: string;
	kind: string;
	source_ref: string;
	dest_ref: string;
	status: string;
	bytes: number;
	error?: string | null;
	created_at: number;
	updated_at: number;
};

export type ServerAuditPayload = Record<string, unknown>;
