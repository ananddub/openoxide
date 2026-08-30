import {createServiceClient, useResourceListQuery} from './query';
import {BackupService} from '#/gen/connect/openoxide/api/v1/backup_pb';

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

const backupClient = createServiceClient(BackupService);

/** Volume backups (legacy live: useBackupListVolumeBackups). */
export function useBackupListVolumeBackups() {
	return useResourceListQuery<VolumeBackupPayload>(
		['backups', 'volume-list'],
		() => backupClient.listBackups({}),
	);
}
