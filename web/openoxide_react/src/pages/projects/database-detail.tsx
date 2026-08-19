import {createFileRoute, Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {useDatabaseDetail} from '#/hooks/projects/use-database-detail';
import {$api} from '#/api/query';
import {useState} from 'react';

import {DatabaseHeader} from '#/components/projects/database/detail/database-header';

// Tabs
import {DatabaseGeneralTab} from '#/components/projects/database/detail/database-general-tab';
import {DatabaseArchitectureTab} from '#/components/projects/database/detail/database-architecture-tab';
import {EnvironmentTab} from '#/components/projects/common/environment-tab';
import {DatabaseLogsTab} from '#/components/projects/database/detail/database-logs-tab';
import {MonitoringTab} from '#/components/projects/common/monitoring-tab';
import {ComposeBackupsTab} from '#/components/projects/compose/detail/compose-backups-tab';
import {DatabaseAdvancedTab} from '#/components/projects/database/detail/database-advanced-tab';
import {DatabaseDeploymentsTab} from '#/components/projects/database/detail/database-deployments-tab';
import {DeleteDatabaseDialog} from '#/components/projects/database/delete-database-dialog';

export const Route = createFileRoute('/_app/projects/$id/database/$dbId')({
	validateSearch: (search: Record<string, unknown>) => ({
		kind: (search.kind as string) || undefined,
	}),
	component: DatabaseDetailPage,
});

const TABS = [
	'General',
	'Architecture',
	'Environment',
	'Deployments',
	'Backups',
	'Logs',
	'Monitoring',
	'Advanced',
] as const;

function DatabaseDetailPage() {
	const {id, dbId} = Route.useParams();
	const search: {tab?: string; kind?: string} = Route.useSearch();
	const windowKind =
		typeof window !== 'undefined'
			? new URLSearchParams(window.location.search).get('kind')
			: null;
	const targetKind = search?.kind || windowKind || undefined;
	const parsedDbId = Number(dbId);
	const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

	const {
		database,
		schedules,
		backups,
		deployments,
		monitoring,
		isLoading,
		actionLoading,
		isBuilding,
		detectedKind,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
	} = useDatabaseDetail(parsedDbId, targetKind);

	const patchDatabase = $api.useMutation('patch', '/postgres/{id}');

	const handleUpdateEnv = async (body: Record<string, unknown>) => {
		await patchDatabase.mutateAsync({
			params: {path: {id: parsedDbId}},
			body,
		});
		refetchAll();
	};

	if (isLoading) {
		return (
			<div className="flex min-h-[400px] w-full items-center justify-center text-xs font-semibold text-muted-foreground">
				Loading Database details...
			</div>
		);
	}

	if (!database) {
		return (
			<div className="flex min-h-[400px] w-full flex-col items-center justify-center gap-3 text-center">
				<p className="text-sm font-bold text-foreground">
					Database not found
				</p>
				<p className="text-xs text-muted-foreground">
					The requested database could not be loaded or was removed.
				</p>
				<Link to="/projects/$id" params={{id: String(id)}}>
					<Button variant="outline" className="h-8 text-xs">
						Return to Project
					</Button>
				</Link>
			</div>
		);
	}

	return (
		<div className="flex w-full animate-in flex-col gap-6 pb-10 duration-200 fade-in">
			<DatabaseHeader
				id={id}
				database={database as any}
				detectedKind={detectedKind || 'postgres'}
				actionLoading={actionLoading}
				isBuilding={isBuilding}
				activeTab={activeTab}
				setActiveTab={setActiveTab}
				refetch={refetchAll}
				onOpenDeleteDialog={() => setIsDeleteDialogOpen(true)}
				onAction={handleAction}
				tabs={TABS}
			/>

			{/* Tab Views */}
			<div className="w-full">
				{activeTab === 'General' && (
					<DatabaseGeneralTab
						database={database as any}
						actionLoading={actionLoading}
						isBuilding={isBuilding}
						onAction={handleAction}
						onUpdated={refetchAll}
					/>
				)}
				{activeTab === 'Architecture' && (
					<DatabaseArchitectureTab
						database={database}
						schedules={schedules}
						backups={backups}
						onRefresh={refetchAll}
					/>
				)}
				{activeTab === 'Environment' && (
					<EnvironmentTab app={database} handleUpdate={handleUpdateEnv} />
				)}
				{activeTab === 'Deployments' && (
					<DatabaseDeploymentsTab
						dbId={parsedDbId}
						kind={detectedKind || 'postgres'}
						database={database}
						deployments={deployments}
						onRefresh={refetchAll}
						onAction={handleAction}
					/>
				)}
				{activeTab === 'Backups' && (
					<ComposeBackupsTab compose={database} />
				)}
				{activeTab === 'Logs' && <DatabaseLogsTab database={database} />}
				{activeTab === 'Monitoring' && (
					<MonitoringTab
						app={database}
						appId={parsedDbId}
						entityType="database"
						monitoring={monitoring}
					/>
				)}
				{activeTab === 'Advanced' && (
					<DatabaseAdvancedTab
						database={database}
						onUpdated={refetchAll}
					/>
				)}
			</div>

			<DeleteDatabaseDialog
				isOpen={isDeleteDialogOpen}
				onClose={() => setIsDeleteDialogOpen(false)}
				database={database}
			/>
		</div>
	);
}
