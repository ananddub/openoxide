import {useConnectMutation} from '../query';
import {useOverviewBackups} from '../overview';
import {BACKUP_INVALIDATE, backupClient, type LiveRow} from './shared';
import type {BackupItemPayload} from './types';

/** Create a database backup rule (fields in the payload). */
export function useDatabaseBackupCreate() {
	return useConnectMutation(
		(input: Record<string, unknown>) =>
			backupClient.createDatabaseBackup({payload: JSON.stringify(input)}),
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Update a database backup rule by id (fields to change in the payload). */
export function useDatabaseBackupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return backupClient.updateDatabaseBackup({
				id: BigInt(id),
				payload: JSON.stringify(body),
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
		{invalidate: BACKUP_INVALIDATE},
	);
}

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
