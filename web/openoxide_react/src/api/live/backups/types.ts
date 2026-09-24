export type VolumeBackupPayload = {
	id: bigint;
	name: string;
	volume_name: string;
	prefix: string;
	service_type: string;
	app_name: string;
	service_name?: string;
	turn_off: bigint;
	cron_expression: string;
	keep_latest_count?: bigint;
	enabled: bigint;
	destination_id?: bigint;
	organization_id?: bigint;
	created_at: bigint;
	updated_at: bigint;
	service_id?: bigint;
	application_id?: bigint;
	compose_id?: bigint;
	database_id?: bigint;
	postgres_id?: bigint;
	mysql_id?: bigint;
	mariadb_id?: bigint;
	mongo_id?: bigint;
	redis_id?: bigint;
	libsql_id?: bigint;
	[key: string]: unknown;
};

export type DatabaseBackupPayload = {
	id: bigint;
	app_name: string;
	schedule: string;
	enabled: bigint;
	database_name?: string;
	prefix: string;
	service_name?: string;
	keep_latest_count?: bigint;
	backup_type: string;
	database_type?: string;
	metadata?: string;
	service_id?: bigint;
	destination_id?: bigint;
	organization_id?: bigint;
	created_at: bigint;
	updated_at: bigint;
	[key: string]: unknown;
};

/** Legacy backup item shape the former zustand `backups` slice carried (numeric id, `destination` name). */
export type BackupItemPayload = {
	id: number;
	name: string;
	backup_type?: string;
	status?: string;
	destination?: string;
	created_at: number;
	[key: string]: unknown;
};
