import {createServiceClient} from '../query';
// The old BackupOperationsService envelope RPCs were merged into the typed
// BackupService (list/create/update/delete/run/restore per kind).
import {BackupService} from '#/gen/connect/openoxide/api/v1/backup_pb';

export type LiveRow = Record<string, any>;

export const backupClient = createServiceClient(BackupService);

/** Every backup mutation refreshes the rules list and the overview rollup. */
export const BACKUP_INVALIDATE = [['backups'], ['overview-backups']];

/** Loose 0/1/boolean/undefined → proto bool (undefined stays unset). */
export function toBool(value: unknown): boolean | undefined {
	if (value === undefined || value === null) return undefined;
	return Boolean(Number(value));
}

/** Loose number/string/bigint → proto int64 (undefined stays unset). */
export function toBigInt(value: unknown): bigint | undefined {
	if (value === undefined || value === null || value === '')
		return undefined;
	return BigInt(value as number | string | bigint);
}

/** Loose optional string → proto `optional string` (undefined stays unset). */
export function toOptString(value: unknown): string | undefined {
	if (value === undefined || value === null) return undefined;
	return String(value);
}
