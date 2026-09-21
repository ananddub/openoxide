import {useState, useMemo} from 'react';
import {ComposeVisualizer} from '#/components/projects/compose/visualizer/compose-visualizer';
import {CreateBackupModal} from '#/components/projects/compose/detail/backups/create-backup-modal';
import {TerminalCard} from '#/components/shared/terminal-card';
import {ComposeDirectContainerLogsModal} from '#/components/projects/compose/detail/logs/compose-direct-container-logs-modal';
import {useDatabaseBackupActions} from './use-database-backup-actions';

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
	const dbId =
		databaseId ?? (database?.id ? Number(database.id) : undefined);
	const kind = (
		databaseKind ||
		database?.kind ||
		'postgres'
	).toLowerCase();
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
	const {handleSaveBackup, handleDeleteBackup} = useDatabaseBackupActions({
		database,
		dbId,
		kind,
		editingBackupData,
		onSaved: () => {
			onRefresh?.();
			setActiveModal(null);
			setEditingBackupData(null);
		},
	});

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
				onDeleteBackup={
					supportsVolumeBackup ? handleDeleteBackup : undefined
				}
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

			{/* Terminal Shell — inline card */}
			{activeModal === 'terminal' && (
				<TerminalCard
					app={database}
					enabled={true}
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
