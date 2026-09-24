import {createFileRoute, Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {useComposeDetail} from '#/hooks/projects/compose/use-compose-detail';
import {useComposeUpdate} from '#/api/live';
import {ServiceLayout} from '#/components/projects/detail/service-layout';

// Tabs
import {ComposeGeneralTab} from '#/components/projects/compose/detail/general-tab';
import {ComposeArchitectureTab} from '#/components/projects/compose/detail/compose-architecture-tab';
import {ComposeDomainsTab} from '#/components/projects/compose/detail/compose-domains-tab';
import {ComposeDeploymentsTab} from '#/components/projects/compose/detail/compose-deployments-tab';
import {ComposeLogsTab} from '#/components/projects/compose/detail/compose-logs-tab';
import {MonitoringTab} from '#/components/projects/common/monitoring-tab';
import {ComposeSchedulesTab} from '#/components/projects/compose/detail/compose-schedules-tab';
import {ComposeBackupsTab} from '#/components/projects/compose/detail/compose-backups-tab';
import {ComposeContainersTab} from '#/components/projects/compose/detail/compose-containers-tab';
import {ComposeAdvancedTab} from '#/components/projects/compose/detail/compose-advanced-tab';

export const Route = createFileRoute(
	'/_app/projects/$id/compose/$composeId',
)({
	component: ComposeDetailPage,
});

function ComposeDetailPage() {
	const {id, composeId} = Route.useParams();
	const parsedComposeId = Number(composeId);

	const {
		compose,
		domains,
		schedules,
		backups,
		deployments,
		isLoading,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
	} = useComposeDetail(parsedComposeId);

	const patchCompose = useComposeUpdate();

	const handleUpdateEnv = async (body: Record<string, unknown>) => {
		await patchCompose.mutateAsync({id: parsedComposeId, ...body});
		refetchAll();
	};

	if (isLoading) {
		return (
			<div className="flex min-h-[400px] w-full items-center justify-center text-xs font-semibold text-muted-foreground">
				Loading Compose stack details...
			</div>
		);
	}

	if (!compose) {
		return (
			<div className="flex min-h-[400px] w-full flex-col items-center justify-center gap-3 text-center">
				<p className="text-sm font-bold text-foreground">
					Compose stack not found
				</p>
				<p className="text-xs text-muted-foreground">
					The requested compose stack could not be loaded or was removed.
				</p>
				<Link to="/projects/$id" params={{id: String(id)}}>
					<Button variant="outline" className="h-8 text-xs">
						Return to Project
					</Button>
				</Link>
			</div>
		);
	}

	// Legacy fields ride the wire payload but sit outside the union type.
	const composeExtras = compose as {
		app_name?: string;
	};
	return (
		<ServiceLayout
			id={String(id)}
			name={
				compose.name || composeExtras.app_name || `Compose #${composeId}`
			}>
			{activeTab === 'General' && (
				<ComposeGeneralTab
					compose={compose}
					onAction={handleAction}
					handleUpdate={handleUpdateEnv}
					onUpdated={refetchAll}
					domains={domains}
					schedules={schedules}
					backups={backups}
					onOpenArchitecture={() => setActiveTab('Architecture')}
				/>
			)}
			{activeTab === 'Architecture' && (
				<ComposeArchitectureTab
					compose={compose}
					domains={domains}
					schedules={schedules}
					backups={backups}
					onNavigateTab={setActiveTab}
					onRefresh={refetchAll}
				/>
			)}
			{activeTab === 'Containers' && (
				<ComposeContainersTab compose={compose} onUpdated={refetchAll} />
			)}
			{activeTab === 'Domains' && (
				<ComposeDomainsTab
					composeId={parsedComposeId}
					compose={compose}
					domains={domains}
				/>
			)}
			{activeTab === 'Deployments' && (
				<ComposeDeploymentsTab
					composeId={parsedComposeId}
					deployments={deployments}
				/>
			)}
			{activeTab === 'Schedules' && (
				<ComposeSchedulesTab compose={compose} schedules={schedules} />
			)}
			{activeTab === 'Backups' && (
				<ComposeBackupsTab compose={compose} backups={backups} />
			)}
			{activeTab === 'Logs' && <ComposeLogsTab compose={compose} />}
			{activeTab === 'Monitoring' && (
				<MonitoringTab
					app={compose}
					appId={parsedComposeId}
					entityType="compose"
				/>
			)}
			{activeTab === 'Advanced' && (
				<ComposeAdvancedTab
					compose={compose}
					projectId={Number(id)}
					onUpdated={refetchAll}
				/>
			)}
		</ServiceLayout>
	);
}
