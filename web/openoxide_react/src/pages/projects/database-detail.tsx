import {createFileRoute, Link} from '@tanstack/react-router';
import {RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import {toast} from 'sonner';
import {useDatabaseDetail} from '#/hooks/projects/use-database-detail';
import {useDatabaseUpdate, parseDatabaseKind} from '#/api/live';
import type {DeploymentItemPayload} from '#/api/live';

import {ServiceLayout} from '#/components/projects/detail/service-layout';
import {kindIcon} from '#/components/projects/database/detail/database-kind-icon';
import {DatabaseDetailTabs} from '#/components/projects/database/detail/database-detail-tabs';

export const Route = createFileRoute('/_app/projects/$id/database/$dbId')({
	validateSearch: (search: Record<string, unknown>) => ({
		kind: (search.kind as string) || undefined,
	}),
	component: DatabaseDetailPage,
});

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
			kind: parseDatabaseKind(
				targetKind || (database?.kind as string) || '',
			),
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
	).find(d =>
		['ERROR', 'CANCELLED'].includes((d.status || '').toUpperCase()),
	);
	const latestError = failedDeployment?.error_message ?? null;
	const kind = detectedKind || targetKind || 'postgres';

	// Legacy fields ride the wire payload but sit outside the union type.
	const dbExtras = database as {
		app_name?: string;
		last_error?: string;
		app_status?: string;
	};
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
			<DatabaseDetailTabs
				activeTab={activeTab}
				database={database}
				parsedDbId={parsedDbId}
				kind={kind}
				isRedis={isRedis}
				schedules={schedules}
				backups={backups}
				deployments={deployments}
				actionLoading={actionLoading}
				isBuilding={isBuilding}
				handleAction={handleAction}
				refetchAll={refetchAll}
				handleUpdateEnv={handleUpdateEnv}
			/>
		</ServiceLayout>
	);
}
