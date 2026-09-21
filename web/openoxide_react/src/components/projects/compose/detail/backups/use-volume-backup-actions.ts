import {toast} from 'sonner';
import {
	useVolumeBackupCreate,
	useVolumeBackupDelete,
	useVolumeBackupRestore,
	useVolumeBackupRun,
} from '#/api/live';
import {formatApiError} from '#/api/utils';
import {buildDatabaseResource} from './backup-mappers';

export interface CreateBackupData {
	name: string;
	serviceName: string;
	volumeName: string;
	cronExpr: string;
	prefix: string;
	turnOff: boolean;
	destinationId: number;
}

interface BackupActionOptions {
	compose: any;
	isDatabase: boolean;
	normalizedDatabaseKind: string;
	databaseId?: number;
}

/** Encapsulates the four volume-backup mutations + their toast handling. */
export function useVolumeBackupActions(opts: BackupActionOptions) {
	const {compose, isDatabase, normalizedDatabaseKind, databaseId} = opts;
	const createMutation = useVolumeBackupCreate();
	const runMutation = useVolumeBackupRun();
	const restoreMutation = useVolumeBackupRestore();
	const deleteMutation = useVolumeBackupDelete();

	const run = async (fn: () => Promise<unknown>, success: string) => {
		try {
			await fn();
			toast.success(success);
		} catch (err: any) {
			toast.error(formatApiError(err));
		}
	};

	const handleCreate = (data: CreateBackupData) => {
		const databaseResource = isDatabase
			? buildDatabaseResource(normalizedDatabaseKind, databaseId)
			: {compose_id: compose?.id};
		return run(
			() =>
				createMutation.mutateAsync({
					name: data.name,
					...databaseResource,
					app_name: compose?.app_name,
					service_name: data.serviceName,
					volume_name: data.volumeName,
					cron_expression: data.cronExpr,
					prefix: data.prefix,
					turn_off: data.turnOff ? 1 : 0,
					destination_id: data.destinationId,
					organization_id: compose?.organization_id,
					service_type: isDatabase
						? normalizedDatabaseKind.toUpperCase()
						: 'COMPOSE',
				}),
			'Volume backup rule created successfully',
		);
	};

	return {
		handleCreate,
		handleRun: (id: number) =>
			run(
				() => runMutation.mutateAsync(id),
				'Volume snapshot triggered successfully',
			),
		handleRestore: (id: number) =>
			run(
				() => restoreMutation.mutateAsync({id, backup_file: ''}),
				'Volume snapshot restore initiated',
			),
		handleDelete: (id: number) =>
			run(
				() => deleteMutation.mutateAsync(id),
				'Volume backup rule deleted',
			),
	};
}
