import {createFileRoute, Link} from '@tanstack/react-router';
import {Database, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import {toast} from 'sonner';
import {useDatabaseDetail} from '#/hooks/projects/use-database-detail';
import {useDatabaseUpdate, parseDatabaseKind} from '#/api/live';
import type {DeploymentItemPayload} from '#/api/live';

import {ServiceLayout} from '#/components/projects/detail/service-layout';
import {
	PostgresqlIcon,
	MysqlIcon,
	MariadbIcon,
	MongodbIcon,
	RedisIcon,
	LibsqlIcon,
} from '#/components/icons/db-icons';

// Tabs
import {DatabaseGeneralTab} from '#/components/projects/database/detail/database-general-tab';
import {DatabaseArchitectureTab} from '#/components/projects/database/detail/database-architecture-tab';
import {EnvironmentTab} from '#/components/projects/common/environment-tab';
import {DatabaseLogsTab} from '#/components/projects/database/detail/database-logs-tab';
import {MonitoringTab} from '#/components/projects/common/monitoring-tab';
import {ComposeBackupsTab} from '#/components/projects/compose/detail/compose-backups-tab';
import {DatabaseAdvancedTab} from '#/components/projects/database/detail/database-advanced-tab';
import {DatabaseDeploymentsTab} from '#/components/projects/database/detail/database-deployments-tab';

export const Route = createFileRoute('/_app/projects/$id/database/$dbId')({
	validateSearch: (search: Record<string, unknown>) => ({
		kind: (search.kind as string) || undefined,
	}),
	component: DatabaseDetailPage,
});

/** Provider mark for the rail's identity tile, by detected db kind. */
function kindIcon(kind: string) {
	const k = kind.toLowerCase();
	if (k.includes('postgres')) return <PostgresqlIcon className="size-5" />;
	if (k.includes('mysql')) return <MysqlIcon className="size-5" />;
	if (k.includes('mariadb')) return <MariadbIcon className="size-5" />;
	if (k.includes('mongo')) return <MongodbIcon className="size-5" />;
	if (k.includes('redis')) return <RedisIcon className="size-5" />;
	if (k.includes('libsql')) return <LibsqlIcon className="size-5" />;
	return <Database className="size-4 text-muted-foreground" />;
}

function DatabaseDetailPage() {
	const {id, dbId} = Route.useParams();
	const search: {tab?: string; kind?: string} = Route.useSearch();
	const windowKind =
		typeof window !== 'undefined'
			? new URLSearchParams(window.location.search).get('kind')
			: null;
	const targetKind = search?.kind || windowKind || undefined;
	const parsedDbId = Number(dbId);

	const {
		database,
		schedules,
		backups,
		deployments,
		isLoading,
		actionLoading,
		isBuilding,
		detectedKind,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
	} = useDatabaseDetail(parsedDbId, targetKind);
	const isRedis = (detectedKind || targetKind)?.toLowerCase() === 'redis';
	const tabs: readonly string[] = isRedis
		? [
				'General',
				'Architecture',
				'Environment',
				'Deployments',
				'Logs',
				'Monitoring',
				'Advanced',
			]
		: [
				'General',
				'Architecture',
				'Environment',
				'Deployments',
				'Backups',
				'Logs',
				'Monitoring',
				'Advanced',
			];

	const patchDatabase = useDatabaseUpdate();

	const handleUpdateEnv = async (body: Record<string, unknown>) => {
		await patchDatabase.mutateAsync({
			kind: parseDatabaseKind(targetKind || (database?.kind as string) || ''),
			id: parsedDbId,
			...body,
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

	// Newest errored/cancelled deployment carries the visible failure reason.
	const failedDeployment: DeploymentItemPayload | undefined = (
		deployments ?? []
	).find(d => ['ERROR', 'CANCELLED'].includes((d.status || '').toUpperCase()));
	const latestError = failedDeployment?.error_message ?? null;
	const kind = detectedKind || targetKind || 'postgres';

	// Legacy fields ride the wire payload but sit outside the union type.
	const dbExtras = database as {app_name?: string; last_error?: string; app_status?: string};
	return (
		<ServiceLayout
			id={String(id)}
			name={database.name || dbExtras.app_name}
			subtitle={dbExtras.app_name || database.name}
			icon={kindIcon(kind)}
			actions={
				<>
					<StatusBadge
						status={database.status || dbExtras.app_status || 'STOPPED'}
						error={dbExtras.last_error || latestError}
						isBuilding={isBuilding}
						actionLoading={actionLoading}
					/>
					<span className="inline-flex items-center gap-1.5 rounded-lg bg-muted/40 px-2 py-1 text-[11px] font-medium text-muted-foreground capitalize select-none">
						{kind} Database
					</span>
					<Button
						variant="ghost"
						size="icon"
						title="Refresh status"
						onClick={() => {
							refetchAll();
							toast.success('Database status refreshed');
						}}
						className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
						<RefreshCw className="size-3.5" />
					</Button>
				</>
			}
			tabs={tabs}
			activeTab={activeTab}
			onTabChange={setActiveTab}>
			{activeTab === 'General' && (
				<DatabaseGeneralTab
					database={database}
					actionLoading={actionLoading}
					isBuilding={isBuilding}
					onAction={handleAction}
					onUpdated={refetchAll}
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
			{activeTab === 'Environment' && (
				<EnvironmentTab app={database} handleUpdate={handleUpdateEnv} />
			)}
			{activeTab === 'Deployments' && (
				<DatabaseDeploymentsTab
					dbId={parsedDbId}
					kind={kind}
					database={database}
					deployments={deployments}
					actionLoading={actionLoading}
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
		</ServiceLayout>
	);
}
