import {useState, useMemo} from 'react';
import {ComposeVisualizer} from '#/components/projects/compose/visualizer/compose-visualizer';
import {
	buildDbArchService,
	dbSupportsVolumeBackup,
	filterDbArchBackups,
} from './arch-visual-data';
import {CreateBackupModal} from '#/components/projects/compose/detail/backups/create-backup-modal';
import {TerminalCard} from '#/components/shared/terminal-card';
import {ComposeDirectContainerLogsModal} from '#/components/projects/compose/detail/logs/compose-direct-container-logs-modal';
import {useDatabaseBackupActions} from './use-database-backup-actions';
import {SectionHeader} from '#/components/settings/list-card';

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
	const supportsVolumeBackup = dbSupportsVolumeBackup(kind);

	const [activeModal, setActiveModal] = useState<
		'domain' | 'backup' | 'terminal' | 'logs' | null
	>(null);
	const [editingBackupData, setEditingBackupData] = useState<any | null>(
		null,
	);

	const dbBackups = useMemo(
		() => filterDbArchBackups(passedBackups, dbId, database),
		[passedBackups, dbId, database],
	);

	const dbServices = useMemo(
		() => buildDbArchService(database, kind),
		[database, kind],
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
		<div className="flex w-full animate-in flex-col gap-5 duration-200 fade-in">
			<SectionHeader
				title="Architecture"
				description={
					supportsVolumeBackup
						? 'Live map of this database, its volume backups and maintenance schedules. Click any node for context actions.'
						: 'Live map of the Redis service and its maintenance schedules.'
				}
			/>

			<ComposeVisualizer
				customServices={dbServices}
				monitorId={dbId}
				monitorType="database"
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
