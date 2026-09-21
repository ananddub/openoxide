import {useState} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {
	useVolumeBackupCreate,
	useVolumeBackupDelete,
	useVolumeBackupUpdate,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import type {ArchitectureOwner} from './types';

export interface BackupFormData {
	name: string;
	serviceName: string;
	volumeName: string;
	cronExpr: string;
	prefix: string;
	turnOff: boolean;
	destinationId: number;
}

export function useBackupActions(
	owner: ArchitectureOwner,
	onRefresh?: () => void,
) {
	const queryClient = useQueryClient();
	const create = useVolumeBackupCreate();
	const patch = useVolumeBackupUpdate();
	const removeMutation = useVolumeBackupDelete();
	const [editing, setEditing] = useState<any | null>(null);

	const isCompose = owner.kind === 'compose';
	const scopeField = isCompose ? 'compose_id' : 'application_id';

	/** Returns true when the backup rule was saved successfully. */
	const save = async (data: BackupFormData): Promise<boolean> => {
		try {
			if (editing?.id) {
				await patch.mutateAsync({
					id: editing.id,
					name: data.name,
					volume_name: data.volumeName,
					prefix: data.prefix || '',
					service_name: data.serviceName,
					turn_off: data.turnOff ? 1 : 0,
					cron_expression: data.cronExpr,
				});
				toast.success('Volume backup updated successfully');
			} else {
				await create.mutateAsync({
					name: data.name,
					volume_name: data.volumeName,
					prefix: data.prefix || '',
					service_type: isCompose ? 'compose' : 'standalone',
					app_name:
						owner.appName || owner.name || data.serviceName || 'app',
					service_name: data.serviceName,
					turn_off: data.turnOff ? 1 : 0,
					cron_expression: data.cronExpr,
					[scopeField]: owner.id,
					destination_id: data.destinationId,
				});
				toast.success(
					isCompose
						? `Backup rule attached to ${data.serviceName}`
						: 'Volume backup created successfully',
				);
			}
			queryClient.invalidateQueries();
			setEditing(null);
			return true;
		} catch (error: unknown) {
			toast.error(formatApiError(error));
			return false;
		}
	};

	const remove = async (backupData: any) => {
		if (!backupData?.id) return;
		try {
			await removeMutation.mutateAsync(backupData.id);
			toast.success('Backup rule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete backup rule');
		}
	};

	return {editing, setEditing, save, remove};
}
