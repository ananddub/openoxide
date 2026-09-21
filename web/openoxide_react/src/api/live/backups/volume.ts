import {useConnectMutation, useResourceListQuery} from '../query';
import {BACKUP_INVALIDATE, backupClient} from './shared';
import type {VolumeBackupPayload} from './types';

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
		{invalidate: BACKUP_INVALIDATE},
	);
}

/** Update a volume backup rule by id (fields to change in the payload). */
export function useVolumeBackupUpdate() {
	return useConnectMutation(
		(input: {id: number | bigint; [field: string]: unknown}) => {
			const {id, ...body} = input;
			return backupClient.updateVolumeBackup({
				id: BigInt(id),
				payload: JSON.stringify(body),
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
		{invalidate: BACKUP_INVALIDATE},
	);
}
