import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {formatApiError} from '#/api/utils';
import {
	useVolumeBackupCreate,
	useVolumeBackupDelete,
	useVolumeBackupUpdate,
} from '#/api/live';
import {buildDatabaseResource} from '#/components/projects/compose/detail/backups/backup-mappers';
import type {CreateBackupData} from '#/components/projects/compose/detail/backups/use-volume-backup-actions';

interface DatabaseBackupOptions {
	database: any;
	dbId?: number;
	kind: string;
	editingBackupData?: any | null;
	onSaved?: () => void;
}

/** Create/update/delete volume-backup rules scoped to a database resource. */
export function useDatabaseBackupActions(opts: DatabaseBackupOptions) {
	const {database, dbId, kind, editingBackupData, onSaved} = opts;
	const queryClient = useQueryClient();
	const createBackupMutation = useVolumeBackupCreate();
	const patchBackupMutation = useVolumeBackupUpdate();
	const deleteBackupMutation = useVolumeBackupDelete();

	const refresh = () => {
		queryClient.invalidateQueries();
		onSaved?.();
	};

	const handleSaveBackup = async (data: CreateBackupData) => {
		try {
			if (editingBackupData?.id) {
				await patchBackupMutation.mutateAsync({
					id: editingBackupData.id,
					name: data.name,
					volume_name: data.volumeName,
					prefix: data.prefix || '',
					service_name: data.serviceName,
					turn_off: data.turnOff ? 1 : 0,
					cron_expression: data.cronExpr,
				});
				toast.success('Volume backup updated successfully');
			} else {
				await createBackupMutation.mutateAsync({
					name: data.name,
					volume_name: data.volumeName,
					prefix: data.prefix || '',
					service_type: 'database',
					app_name:
						database?.app_name ||
						database?.name ||
						data.serviceName ||
						'database',
					service_name: data.serviceName,
					turn_off: data.turnOff ? 1 : 0,
					cron_expression: data.cronExpr,
					...buildDatabaseResource(kind, dbId),
					destination_id: data.destinationId,
				});
				toast.success('Volume backup created successfully');
			}
			refresh();
		} catch (error: unknown) {
			toast.error(formatApiError(error));
		}
	};

	const handleDeleteBackup = async (backupData: any) => {
		const backupId = backupData?.id;
		if (!backupId) return;
		try {
			await deleteBackupMutation.mutateAsync(backupId);
			toast.success('Backup rule removed');
			refresh();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete backup rule');
		}
	};

	return {handleSaveBackup, handleDeleteBackup};
}
