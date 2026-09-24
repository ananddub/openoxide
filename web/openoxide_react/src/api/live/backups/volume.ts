import {useConnectMutation, useConnectQuery} from '../query';
import type {BackupVolume} from '#/gen/connect/openoxide/api/v1/backup_pb';
import {
	BACKUP_INVALIDATE,
	backupClient,
	toBigInt,
	toBool,
	toOptString,
} from './shared';
import type {VolumeBackupPayload} from './types';

const boolToBigint = (value: boolean): bigint => (value ? 1n : 0n);

function toVolumeBackupPayload(backup: BackupVolume): VolumeBackupPayload {
	return {
		id: backup.id,
		name: backup.name,
		volume_name: backup.volumeName,
		prefix: backup.prefix,
		service_type: backup.serviceType,
		app_name: backup.appName,
		service_name: backup.serviceName,
		// Legacy surface exposed the booleans as 0/1 bigints.
		turn_off: boolToBigint(backup.turnOff),
		cron_expression: backup.cronExpression,
		keep_latest_count: backup.keepLatestCount,
		enabled: boolToBigint(backup.enabled),
		destination_id: backup.destinationId,
		organization_id: backup.organizationId,
		created_at: backup.createdAt,
		updated_at: backup.updatedAt,
		service_id: backup.serviceId,
		application_id: backup.applicationId,
		compose_id: backup.composeId,
		postgres_id: backup.postgresId,
		mysql_id: backup.mysqlId,
		mariadb_id: backup.mariadbId,
		mongo_id: backup.mongoId,
		redis_id: backup.redisId,
		libsql_id: backup.libsqlId,
	};
}

/** Volume backups (legacy live: useBackupListVolumeBackups). */
export function useBackupListVolumeBackups() {
	return useConnectQuery<VolumeBackupPayload[]>(
		['backups', 'volume-list'],
		async () => {
			const response = await backupClient.listVolumeBackups({});
			return response.backups.map(toVolumeBackupPayload);
		},
	);
}

/** Create a volume backup rule (legacy REST: POST /backups/volume; the loose
 * snake_case body from the dialogs is translated to the typed request). */
export function useVolumeBackupCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			backupClient.createVolumeBackup({
				name: String(input.name ?? ''),
				volumeName: String(input.volume_name ?? ''),
				prefix: String(input.prefix ?? ''),
				serviceType: String(input.service_type ?? ''),
				appName: String(input.app_name ?? ''),
				serviceName: toOptString(input.service_name),
				turnOff: toBool(input.turn_off) ?? false,
				cronExpression: String(input.cron_expression ?? ''),
				keepLatestCount: toBigInt(input.keep_latest_count),
				destinationId: toBigInt(input.destination_id),
				organizationId: toBigInt(input.organization_id),
				serviceId: toBigInt(input.service_id),
				applicationId: toBigInt(input.application_id),
				composeId: toBigInt(input.compose_id),
				databaseId: toBigInt(input.database_id),
				postgresId: toBigInt(input.postgres_id),
				mysqlId: toBigInt(input.mysql_id),
				mariadbId: toBigInt(input.mariadb_id),
				mongoId: toBigInt(input.mongo_id),
				redisId: toBigInt(input.redis_id),
				libsqlId: toBigInt(input.libsql_id),
			}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Update a volume backup rule by id (fields to change in the body; PATCH
 * semantics — absent keys keep their stored values). */
export function useVolumeBackupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return backupClient.updateVolumeBackup({
				id: BigInt(id),
				name: toOptString(body.name),
				volumeName: toOptString(body.volume_name),
				prefix: toOptString(body.prefix),
				serviceType: toOptString(body.service_type),
				appName: toOptString(body.app_name),
				serviceName: toOptString(body.service_name),
				turnOff: toBool(body.turn_off),
				cronExpression: toOptString(body.cron_expression),
				keepLatestCount: toBigInt(body.keep_latest_count),
				destinationId: toBigInt(body.destination_id),
				enabled: toBool(body.enabled),
			});
		},
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Delete a volume backup rule by id (legacy REST: DELETE /backups/volume/{id}). */
export function useVolumeBackupDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.deleteVolumeBackup({id: BigInt(id)}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Run a volume backup now by id (legacy REST: POST /backups/volume/{id}/run). */
export function useVolumeBackupRun() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.runVolumeBackup({id: BigInt(id)}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Restore a volume backup by id (legacy REST: POST /backups/volume/{id}/restore). The backup file is now a real request field. */
export function useVolumeBackupRestore() {
	return useConnectMutation(
		(input: {id: number | bigint; backup_file?: string}) =>
			backupClient.restoreVolume({
				id: BigInt(input.id),
				backupFile: input.backup_file ?? '',
			}),
		{invalidate: BACKUP_INVALIDATE},
	);
}
