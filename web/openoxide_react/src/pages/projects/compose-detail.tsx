import {createFileRoute, Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {useComposeDetail} from '#/hooks/projects/use-compose-detail';
import {$api} from '#/api/query';

import {ComposeHeader} from '#/components/projects/compose/detail/compose-header';

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
		monitoring,
		isLoading,
		refetchAll,
		activeTab,
		setActiveTab,
		handleAction,
	} = useComposeDetail(parsedComposeId);

	const patchCompose = $api.useMutation('patch', '/compose/{id}');

	const handleUpdateEnv = async (body: Record<string, unknown>) => {
		await patchCompose.mutateAsync({
			params: {path: {id: parsedComposeId}},
			body,
		});
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

	return (
		<div className="flex w-full animate-in flex-col gap-6 pb-10 duration-200 fade-in">
			<ComposeHeader
				id={id}
				compose={compose}
				activeTab={activeTab}
				setActiveTab={setActiveTab}
				refetch={refetchAll}
				tabs={TABS}
			/>

			{/* Tab Views */}
			<div className="w-full">
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
						monitoring={monitoring}
					/>
				)}
				{activeTab === 'Advanced' && (
					<ComposeAdvancedTab compose={compose} />
				)}
			</div>
		</div>
	);
}
