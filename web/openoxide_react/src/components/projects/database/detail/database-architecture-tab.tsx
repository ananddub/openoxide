import {useState, useMemo} from 'react';
import {useQueryClient} from '@tanstack/react-query';
import {toast} from 'sonner';
import {$api} from '#/api/query';
import {formatApiError} from '#/api/utils';
import {ComposeVisualizer} from '#/components/projects/compose/visualizer/compose-visualizer';
import {CreateBackupModal} from '#/components/projects/compose/detail/backups/create-backup-modal';
import {TerminalModal} from '#/components/projects/common/terminal-modal';
import {ComposeDirectContainerLogsModal} from '#/components/projects/compose/detail/logs/compose-direct-container-logs-modal';

interface DatabaseArchitectureTabProps {
	database: any;
	databaseId?: number;
	databaseKind?: string;
	schedules?: any[];
	backups?: any[];
	onRefresh?: () => void;
}

export function DatabaseArchitectureTab({
	database,
	databaseId,
	databaseKind,
	backups: passedBackups,
	onRefresh,
}: DatabaseArchitectureTabProps) {
	const queryClient = useQueryClient();
	const dbId = databaseId ?? (database?.id ? Number(database.id) : undefined);
	const kind = (databaseKind || database?.kind || 'postgres').toLowerCase();
	const supportsVolumeBackup = !kind.includes('redis');
	const internalPort =
		kind.includes('mysql') || kind.includes('maria')
			? 3306
			: kind.includes('mongo')
				? 27017
				: kind.includes('redis')
					? 6379
					: kind.includes('libsql')
						? 8080
						: 5432;
	const externalPort = database?.external_port || undefined;

	const createBackupMutation = $api.useMutation('post', '/backups/volume');
	const patchBackupMutation = $api.useMutation(
		'patch',
		'/backups/volume/{id}',
	);
	const deleteBackupMutation = $api.useMutation(
		'delete',
		'/backups/volume/{id}',
	);

	const [activeModal, setActiveModal] = useState<
		'domain' | 'backup' | 'terminal' | 'logs' | null
	>(null);
	const [editingBackupData, setEditingBackupData] = useState<any | null>(
		null,
	);

	const dbBackups = useMemo(() => {
		const list = Array.isArray(passedBackups) ? passedBackups : [];
		return list.filter(
			(b: any) =>
				b.postgres_id === dbId ||
				b.mysql_id === dbId ||
				b.mariadb_id === dbId ||
				b.mongo_id === dbId ||
				b.redis_id === dbId ||
				b.libsql_id === dbId ||
				b.app_name === database?.app_name ||
				b.database_id === dbId ||
				b.database_name === database?.app_name ||
				b.app_name === database?.name,
		);
	}, [passedBackups, dbId, database]);

	const dbServices = useMemo(
		() => [
			{
				name: database?.app_name || database?.name || kind || 'database',
				image: kind || 'postgres',
				dependsOn: [],
				envVars: {},
				volumes: (database as any)?.volume_name
					? [(database as any).volume_name]
					: [`${kind}_data`],
				ports: externalPort
					? [String(externalPort)]
					: [String(internalPort)],
			},
		],
		[database, kind, externalPort, internalPort],
	);

	const servicesList = useMemo(
		() => [database?.app_name || database?.name || kind || 'database'],
		[database, kind],
	);

	// Handlers
	const handleAddBackup = () => {
		setEditingBackupData(null);
		setActiveModal('backup');
	};

	const handleEditBackup = (backupData: any) => {
		setEditingBackupData(backupData);
		setActiveModal('backup');
	};

	const handleOpenTerminal = () => {
		setActiveModal('terminal');
	};

	const handleViewLogs = () => {
		setActiveModal('logs');
	};

	const handleDeleteBackup = async (backupData: any) => {
		const backupId = backupData?.id;
		if (!backupId) return;
		try {
			await deleteBackupMutation.mutateAsync({
				params: {path: {id: backupId}},
			});
			toast.success('Backup rule removed');
			queryClient.invalidateQueries();
			onRefresh?.();
		} catch (e: any) {
			toast.error(e?.message || 'Failed to delete backup rule');
		}
	};

	const handleSaveBackup = async (data: {
		name: string;
		serviceName: string;
		volumeName: string;
		cronExpr: string;
		prefix: string;
		turnOff: boolean;
		destinationId: number;
	}) => {
		try {
			if (editingBackupData?.id) {
				await patchBackupMutation.mutateAsync({
					params: {path: {id: editingBackupData.id}},
					body: {
						name: data.name,
						volume_name: data.volumeName,
						prefix: data.prefix || '',
						service_name: data.serviceName,
						turn_off: data.turnOff ? 1 : 0,
						cron_expression: data.cronExpr,
					} as any,
				});
				toast.success('Volume backup updated successfully');
			} else {
				await createBackupMutation.mutateAsync({
					body: {
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
						postgres_id: kind.includes('postgres') ? dbId : undefined,
						mysql_id: kind.includes('mysql') ? dbId : undefined,
						mariadb_id: kind.includes('mariadb') ? dbId : undefined,
						mongo_id: kind.includes('mongo') ? dbId : undefined,
						libsql_id: kind.includes('libsql') ? dbId : undefined,
						destination_id: data.destinationId,
					} as any,
				});
				toast.success('Volume backup created successfully');
			}
			queryClient.invalidateQueries();
			onRefresh?.();
			setActiveModal(null);
			setEditingBackupData(null);
		} catch (error: unknown) {
			toast.error(formatApiError(error));
		}
	};

	return (
		<div className="flex w-full animate-in flex-col gap-4 duration-200 fade-in">
			<div>
				<h3 className="text-sm font-bold text-foreground">
					Database Topology & Connections
				</h3>
				<p className="text-xs text-muted-foreground">
					{supportsVolumeBackup
						? 'Interactive real-time map of database service, volume backups, and maintenance schedules.'
						: 'Interactive real-time map of the Redis service and maintenance schedules.'}
				</p>
			</div>

			<ComposeVisualizer
				customServices={dbServices}
				backups={supportsVolumeBackup ? (dbBackups as any) : []}
				onAddBackup={supportsVolumeBackup ? handleAddBackup : undefined}
				onOpenTerminal={handleOpenTerminal}
				onViewLogs={handleViewLogs}
				onEditBackup={supportsVolumeBackup ? handleEditBackup : undefined}
				onDeleteBackup={supportsVolumeBackup ? handleDeleteBackup : undefined}
			/>

			{/* Backup Modal */}
			{supportsVolumeBackup && activeModal === 'backup' && (
				<CreateBackupModal
					isOpen={true}
					onClose={() => {
						setActiveModal(null);
						setEditingBackupData(null);
					}}
					editingBackup={editingBackupData}
					servicesList={servicesList}
					defaultServiceName={database?.name || database?.app_name}
					onCreate={handleSaveBackup}
				/>
			)}

			{/* Terminal Shell Modal */}
			{activeModal === 'terminal' && (
				<TerminalModal
					app={database}
					open={true}
					onClose={() => setActiveModal(null)}
				/>
			)}

			{/* Live Container Logs Modal */}
			{activeModal === 'logs' && (
				<ComposeDirectContainerLogsModal
					isOpen={true}
					onClose={() => setActiveModal(null)}
					compose={database}
					serviceName={database?.app_name || database?.name}
				/>
			)}
		</div>
	);
}
