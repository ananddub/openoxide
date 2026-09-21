import {createServiceClient} from '../query';
// Only BackupOperationsService is served by the backend (BackupService has
// no controller), so both the database- and volume-backup hooks target the
// operations variant.
import {BackupOperationsService} from '#/gen/connect/openoxide/api/v1/backup_operations_pb';

export type LiveRow = Record<string, any>;

export const backupClient = createServiceClient(BackupOperationsService);

/** Every backup mutation refreshes the rules list and the overview rollup. */
export const BACKUP_INVALIDATE = [['backups'], ['overview-backups']];
