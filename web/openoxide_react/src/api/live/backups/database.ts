import {useConnectMutation} from '../query';
import {
	BACKUP_INVALIDATE,
	backupClient,
	toBigInt,
	toBool,
	toOptString,
} from './shared';
import {useOverviewBackups} from '../overview';
import type {LiveRow} from './shared';
import type {BackupItemPayload} from './types';

/** Create a database backup rule (typed fields; the loose snake_case dialog
 * body is translated here). */
export function useDatabaseBackupCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			backupClient.createDatabaseBackup({
				appName: String(input.app_name ?? ''),
				schedule: String(input.schedule ?? ''),
				databaseName: String(input.database_name ?? ''),
				prefix: String(input.prefix ?? ''),
				serviceName: toOptString(input.service_name),
				keepLatestCount: toBigInt(input.keep_latest_count),
				backupType: String(input.backup_type ?? ''),
				databaseType: String(input.database_type ?? ''),
				metadata: toOptString(input.metadata),
				serviceId: toBigInt(input.service_id),
				destinationId: toBigInt(input.destination_id) ?? 0n,
				organizationId: toBigInt(input.organization_id) ?? 0n,
			}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Update a database backup rule by id (fields to change in the body; PATCH
 * semantics — absent keys keep their stored values). */
export function useDatabaseBackupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return backupClient.updateDatabaseBackup({
				id: BigInt(id),
				appName: toOptString(body.app_name),
				schedule: toOptString(body.schedule),
				databaseName: toOptString(body.database_name),
				prefix: toOptString(body.prefix),
				serviceName: toOptString(body.service_name),
				keepLatestCount: toBigInt(body.keep_latest_count),
				backupType: toOptString(body.backup_type),
				databaseType: toOptString(body.database_type),
				metadata: toOptString(body.metadata),
				destinationId: toBigInt(body.destination_id),
				enabled: toBool(body.enabled),
			});
		},
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Delete a database backup rule by id. */
export function useDatabaseBackupDelete() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.deleteDatabaseBackup({id: BigInt(id)}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Run a database backup now by id. */
export function useDatabaseBackupRun() {
	return useConnectMutation(
		(id: number | bigint) =>
			backupClient.runDatabaseBackup({id: BigInt(id)}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Restore a database backup by id; the backup file is a real field now. */
export function useDatabaseBackupRestore() {
	return useConnectMutation(
		(input: {id: number | bigint; backup_file?: string}) =>
			backupClient.restoreDatabase({
				id: BigInt(input.id),
				backupFile: input.backup_file ?? '',
			}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Organization-wide backups in the legacy item shape (the former zustand `backups` slice data). Built over useOverviewBackups, whose key prefix `['overview-backups']` is invalidated by the table-watch BACKUPS/ BACKUP_EXECUTIONS/VOLUME_BACKUPS entries, so rule changes and run results refetch automatically. */
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
