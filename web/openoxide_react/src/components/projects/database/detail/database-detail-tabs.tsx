import {MonitoringTab} from '#/components/projects/common/monitoring-tab';
import {ComposeBackupsTab} from '#/components/projects/compose/detail/compose-backups-tab';
import {DatabaseGeneralTab} from './database-general-tab';
import {DatabaseArchitectureTab} from './database-architecture-tab';
import {DatabaseLogsTab} from './database-logs-tab';
import {DatabaseAdvancedTab} from './database-advanced-tab';
import {DatabaseDeploymentsTab} from './database-deployments-tab';

interface DatabaseDetailTabsProps {
	activeTab: string;
	database: any;
	parsedDbId: number;
	kind: string;
	isRedis: boolean;
	schedules: any;
	backups: any;
	deployments: any;
	actionLoading: any;
	isBuilding: boolean;
	handleAction: (action: any) => any;
	refetchAll: () => void;
	handleUpdateEnv: (body: Record<string, unknown>) => any;
	onOpenArchitecture: () => void;
}

/** Renders the active tab panel for the database detail page. */
export function DatabaseDetailTabs({
	activeTab,
	database,
	parsedDbId,
	kind,
	isRedis,
	schedules,
	backups,
	deployments,
	actionLoading,
	isBuilding,
	handleAction,
	refetchAll,
	handleUpdateEnv,
	onOpenArchitecture,
}: DatabaseDetailTabsProps) {
	return (
		<>
			{activeTab === 'General' && (
				<DatabaseGeneralTab
					database={database}
					actionLoading={actionLoading}
					isBuilding={isBuilding}
					onAction={handleAction}
					handleUpdate={handleUpdateEnv}
					onUpdated={refetchAll}
					databaseId={parsedDbId}
					kind={kind}
					backups={backups}
					onOpenArchitecture={onOpenArchitecture}
				/>
			)}
			{activeTab === 'Architecture' && (
				<DatabaseArchitectureTab
					database={database}
					databaseId={parsedDbId}
					databaseKind={kind}
					schedules={schedules}
					backups={backups}
					onRefresh={refetchAll}
				/>
			)}
			{activeTab === 'Deployments' && (
				<DatabaseDeploymentsTab
					dbId={parsedDbId}
					kind={kind}
					database={database}
					deployments={deployments}
					onRefresh={refetchAll}
					onAction={handleAction}
				/>
			)}
			{activeTab === 'Backups' && !isRedis && (
				<ComposeBackupsTab
					compose={database}
					backups={backups}
					databaseId={parsedDbId}
					databaseKind={kind}
				/>
			)}
			{activeTab === 'Logs' && <DatabaseLogsTab database={database} />}
			{activeTab === 'Monitoring' && (
				<MonitoringTab
					app={database}
					appId={parsedDbId}
					entityType="database"
				/>
			)}
			{activeTab === 'Advanced' && (
				<DatabaseAdvancedTab database={database} onUpdated={refetchAll} />
			)}
		</>
	);
}
