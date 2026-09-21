import {createFileRoute, Link} from '@tanstack/react-router';
import {Layers2, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import {toast} from 'sonner';
import {useComposeDetail} from '#/hooks/projects/use-compose-detail';
import {useComposeUpdate} from '#/api/live';
import {ServiceLayout} from '#/components/projects/detail/service-layout';

// Tabs
import {ComposeGeneralTab} from '#/components/projects/compose/detail/general-tab';
import {ComposeArchitectureTab} from '#/components/projects/compose/detail/compose-architecture-tab';
import {EnvironmentTab} from '#/components/projects/common/environment-tab';
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

const TABS = [
	'General',
	'Architecture',
	'Environment',
	'Containers',
	'Domains',
	'Deployments',
	'Schedules',
	'Backups',
	'Logs',
	'Monitoring',
	'Advanced',
] as const;

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
		last_error?: string;
		compose_type?: string;
	};
	return (
		<ServiceLayout
			id={String(id)}
			name={
				compose.name || composeExtras.app_name || `Compose #${composeId}`
			}
			subtitle={composeExtras.app_name || compose.name}
			icon={<Layers2 className="size-4 text-primary" strokeWidth={1.7} />}
			actions={
				<>
					<StatusBadge
						status={compose.compose_status || ''}
						error={composeExtras.last_error}
					/>
					<span className="inline-flex items-center gap-1.5 rounded-lg bg-muted/40 px-2 py-1 text-[11px] font-medium text-muted-foreground select-none">
						{composeExtras.compose_type === 'STACK'
							? 'Swarm Stack'
							: 'Docker Compose'}
					</span>
					<Button
						variant="ghost"
						size="icon"
						title="Refresh status"
						onClick={() => {
							refetchAll();
							toast.success('Compose stack refreshed');
						}}
						className="size-7 rounded-lg text-muted-foreground hover:text-foreground">
						<RefreshCw className="size-3.5" />
					</Button>
				</>
			}
			tabs={TABS}
			activeTab={activeTab}
			onTabChange={setActiveTab}>
			{activeTab === 'General' && (
				<ComposeGeneralTab
					compose={compose}
					onAction={handleAction}
					onUpdated={refetchAll}
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
			{activeTab === 'Environment' && (
				<EnvironmentTab app={compose} handleUpdate={handleUpdateEnv} />
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
