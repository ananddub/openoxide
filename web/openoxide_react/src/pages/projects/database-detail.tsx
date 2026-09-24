import {createFileRoute, Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {useDatabaseDetail} from '#/hooks/projects/database/use-database-detail';
import {useDatabaseUpdate, parseDatabaseKind} from '#/api/live';

import {ServiceLayout} from '#/components/projects/detail/service-layout';
import {DatabaseDetailTabs} from '#/components/projects/database/detail/database-detail-tabs';

export const Route = createFileRoute('/_app/projects/$id/database/$dbId')({
	validateSearch: (search: Record<string, unknown>) => ({
		kind: (search.kind as string) || undefined,
		// The active service tab rides the URL so the sidebar's project nav and deep links can drive it.
		tab: (search.tab as string) || undefined,
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

	const kind = detectedKind || targetKind || 'postgres';

	// Legacy fields ride the wire payload but sit outside the union type.
	const dbExtras = database as {
		app_name?: string;
	};
	return (
		<ServiceLayout
			id={String(id)}
			name={database.name || dbExtras.app_name}>
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
				onOpenArchitecture={() => setActiveTab('Architecture')}
			/>
		</ServiceLayout>
	);
}
