import {createFileRoute, Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {useAppDetail} from '#/hooks/projects/use-app-detail';

import {AppHeader} from '#/components/projects/app/detail/app-header';

// Tabs
import {GeneralTab} from '#/components/projects/app/detail/general-tab';
import {AppArchitectureTab} from '#/components/projects/app/detail/app-architecture-tab';
import {EnvironmentTab} from '#/components/projects/common/environment-tab';
import {DomainsTab} from '#/components/projects/app/detail/domains-tab';
import {DeploymentsTab} from '#/components/projects/app/detail/deployments-tab';
import {PreviewDeploymentsTab} from '#/components/projects/app/detail/preview-deployments-tab';
import {LogsTab} from '#/components/projects/app/detail/logs-tab';
import {MonitoringTab} from '#/components/projects/common/monitoring-tab';
import {SchedulesTab} from '#/components/projects/app/detail/schedules-tab';
import {VolumeBackupsTab} from '#/components/projects/app/detail/volume-backups-tab';
import {AdvancedTab} from '#/components/projects/app/detail/advanced-tab';

export const Route = createFileRoute('/_app/projects/$id/app/$appId')({
	component: AppDetailPage,
});

const TABS = [
	'General',
	'Environment',
	'Domains',
	'Deployments',
	'Volume Backups',
	'Monitoring',
	'Logs',
	'Schedules',
	'Advanced',
] as const;

function AppDetailPage() {
	const {id, appId} = Route.useParams();
	const parsedAppId = Number(appId);

	const {
		app,
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
		handleUpdate,
	} = useAppDetail(parsedAppId);

	if (isLoading) {
		return (
			<div className="flex min-h-[400px] w-full items-center justify-center text-xs font-semibold text-muted-foreground">
				Loading Application details...
			</div>
		);
	}

	if (!app) {
		return (
			<div className="flex min-h-[400px] w-full flex-col items-center justify-center gap-3 text-center">
				<p className="text-sm font-bold text-foreground">
					Application not found
				</p>
				<p className="text-xs text-muted-foreground">
					The requested application could not be loaded or was removed.
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
			<AppHeader
				id={id}
				app={app}
				activeTab={activeTab}
				setActiveTab={setActiveTab}
				refetch={refetchAll}
				tabs={TABS}
			/>

			{/* Tab Views */}
			<div className="w-full">
				{activeTab === 'General' && (
					<GeneralTab
						app={app}
						handleAction={handleAction}
						onUpdated={refetchAll}
					/>
				)}
				{activeTab === 'Architecture' && (
					<AppArchitectureTab
						app={app}
						domains={domains}
						schedules={schedules}
						backups={backups}
						onRefresh={refetchAll}
					/>
				)}
				{activeTab === 'Environment' && (
					<EnvironmentTab app={app} handleUpdate={handleUpdate} />
				)}
				{activeTab === 'Domains' && (
					<DomainsTab app={app} domains={domains} onRefresh={refetchAll} />
				)}
				{activeTab === 'Deployments' && (
					<DeploymentsTab
						appId={parsedAppId}
						deployments={deployments}
						onRefresh={refetchAll}
					/>
				)}
				{activeTab === 'Preview Deployments' && (
					<PreviewDeploymentsTab app={app} />
				)}
				{activeTab === 'Schedules' && (
					<SchedulesTab
						app={app}
						schedules={schedules}
						onRefresh={refetchAll}
					/>
				)}
				{activeTab === 'Volume Backups' && (
					<VolumeBackupsTab
						app={app}
						backups={backups}
						onRefresh={refetchAll}
					/>
				)}
				{activeTab === 'Logs' && <LogsTab app={app} />}
				{activeTab === 'Monitoring' && (
					<MonitoringTab
						app={app}
						appId={parsedAppId}
						entityType="application"
						monitoring={monitoring}
					/>
				)}
				{activeTab === 'Advanced' && (
					<AdvancedTab app={app} onUpdated={refetchAll} />
				)}
			</div>
		</div>
	);
}
