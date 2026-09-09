import {createServiceClient, useConnectMutation, useResourceListQuery} from './query';
import {useOverviewBackups} from './overview';
// Only BackupOperationsService is served by the backend (BackupService has
// no controller), so both the database- and volume-backup hooks target the
// operations variant.
import {BackupOperationsService} from '#/gen/connect/openoxide/api/v1/backup_operations_pb';

type LiveRow = Record<string, any>;

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

const backupClient = createServiceClient(BackupOperationsService);

/** Volume backups (legacy live: useBackupListVolumeBackups). */
export function useBackupListVolumeBackups() {
	return useResourceListQuery<VolumeBackupPayload>(
		['backups', 'volume-list'],
		() => backupClient.listVolumeBackups({}),
	);
}

/** Create a volume backup rule (legacy REST: POST /backups/volume; fields in the payload). */
export function useVolumeBackupCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			backupClient.createVolumeBackup({payload: JSON.stringify(input)}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Update a volume backup rule by id (fields to change in the payload). */
export function useVolumeBackupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return backupClient.updateVolumeBackup({id: BigInt(id), payload: JSON.stringify(body)});
		},
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Delete a volume backup rule by id (legacy REST: DELETE /backups/volume/{id}). */
export function useVolumeBackupDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.deleteVolumeBackup({id: BigInt(id)}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Run a volume backup now by id (legacy REST: POST /backups/volume/{id}/run). */
export function useVolumeBackupRun() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.runVolumeBackup({id: BigInt(id)}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/**
 * Restore a volume backup by id (legacy REST: POST /backups/volume/{id}/restore).
 * The backup file rides in the payload (`{backup_file}`).
 */
export function useVolumeBackupRestore() {
	return useConnectMutation(
		(input: {id: number | bigint; backup_file?: string}) =>
			backupClient.restoreVolume({
				id: BigInt(input.id),
				payload: JSON.stringify({backup_file: input.backup_file ?? ''}),
			}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Create a database backup rule (fields in the payload). */
export function useDatabaseBackupCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			backupClient.createDatabaseBackup({payload: JSON.stringify(input)}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Update a database backup rule by id (fields to change in the payload). */
export function useDatabaseBackupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return backupClient.updateDatabaseBackup({id: BigInt(id), payload: JSON.stringify(body)});
		},
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Delete a database backup rule by id. */
export function useDatabaseBackupDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.deleteDatabaseBackup({id: BigInt(id)}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Run a database backup now by id. */
export function useDatabaseBackupRun() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.runDatabaseBackup({id: BigInt(id)}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/** Restore a database backup by id. The backup file rides in the payload
 * (`{backup_file}`).
 */
export function useDatabaseBackupRestore() {
	return useConnectMutation(
		(input: {id: number | bigint; backup_file?: string}) =>
			backupClient.restoreDatabase({
				id: BigInt(input.id),
				payload: JSON.stringify({backup_file: input.backup_file ?? ''}),
			}),
		{invalidate: [['backups'], ['overview-backups']]},
	);
}

/**
 * Legacy backup item shape the former zustand `backups` slice carried
 * (numeric id, `destination` name).
 */
export type BackupItemPayload = {
	id: number;
	name: string;
	backup_type?: string;
	status?: string;
	destination?: string;
	created_at: number;
	[key: string]: unknown;
};

/**
 * Organization-wide backups in the legacy item shape (the former zustand
 * `backups` slice data). Built over useOverviewBackups, whose key prefix
 * `['overview-backups']` is invalidated by the table-watch BACKUPS/
 * BACKUP_EXECUTIONS/VOLUME_BACKUPS entries, so rule changes and run
 * results refetch automatically.
 */
export function useBackupItems(organizationId?: bigint) {
	void organizationId;
	const query = useOverviewBackups();
	return {
		...query,
		data: query.data?.map(
			(b): BackupItemPayload => ({
				...(b as unknown as LiveRow),
				id: Number(b.id),
				name: b.name,
				backup_type: b.backup_type,
				status: b.status,
				destination: b.destination,
				created_at: Number(b.created_at || 0),
			}),
		),
	};
}
