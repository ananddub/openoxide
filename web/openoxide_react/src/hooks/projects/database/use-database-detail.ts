import {useMemo} from 'react';
import {useUrlTab} from '#/hooks/ui/use-url-tab';
import {useContainerMonitoring} from '#/hooks/monitoring/use-container-monitoring';
import {
	useScheduleListByDatabase,
	useBackupListVolumeBackups,
	useDeploymentItems,
} from '#/api/live';
import {useDatabaseQueries} from './use-database-detail.data';
import {useDatabaseStatus} from './use-database-detail.status';
import {useDatabaseActions} from './use-database-detail.actions';

export function useDatabaseDetail(dbId: number, targetKind?: string) {
	const [activeTab, setActiveTab] = useUrlTab('General');

	const {storeDb, liveDb, currentKind, detectedKind, refetchQueries} =
		useDatabaseQueries(dbId, targetKind);
	const {database, isDeployed, isBuilding, setLocalStatusOverride} =
		useDatabaseStatus(liveDb, storeDb);

	// Schedules
	const {data: rawSchedules, isLoading: isLoadingSchedules} =
		useScheduleListByDatabase(BigInt(dbId));
	const schedules = useMemo(
		() => (Array.isArray(rawSchedules) ? rawSchedules : []),
		[rawSchedules],
	);

	// Volume backups store a typed resource foreign key, not a generic database_id.
	const {data: rawBackupsAll, isLoading: isLoadingBackups} =
		useBackupListVolumeBackups();
	const backups = useMemo(() => {
		const all = Array.isArray(rawBackupsAll) ? rawBackupsAll : [];
		return all.filter((backup: any) =>
			[
				backup.application_id,
				backup.postgres_id,
				backup.mysql_id,
				backup.mariadb_id,
				backup.mongo_id,
				backup.redis_id,
				backup.libsql_id,
			].some(value => Number(value) === Number(dbId)),
		);
	}, [rawBackupsAll, dbId]);

	// Live Container Monitoring Stream
	const monitoring = useContainerMonitoring(
		dbId,
		currentKind || 'postgres',
		activeTab === 'Monitoring',
	);

	// Live deployments query filtered to this database; the table-watch
	// stream (DEPLOYMENTS table) invalidates ['deployments'] while mounted.
	const {data: allDeployments = []} = useDeploymentItems();
	const dbDeployments = useMemo(() => {
		return allDeployments.filter(
			(d: any) => String(d.database_id) === String(dbId),
		);
	}, [allDeployments, dbId]);

	const refetchAll = () => {
		refetchQueries();
		monitoring.triggerRefresh();
	};

	const {actionLoading, handleAction, handleUpdateEnv} =
		useDatabaseActions({
			dbId,
			currentKind,
			targetKind,
			database,
			setLocalStatusOverride,
			refetchAll,
		});

	return {
		database,
		currentKind,
		detectedKind,
		isDeployed,
		isBuilding,
		schedules,
		backups,
		deployments: dbDeployments,
		monitoring,
		isLoading: !database,
		isLoadingSchedules,
		isLoadingBackups,
		actionLoading,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdateEnv,
	};
}
