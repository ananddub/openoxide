import {createFileRoute, Link} from '@tanstack/react-router';
import {Button} from '#/components/ui/button';
import {useAppDetail} from '#/hooks/projects/app/use-app-detail';
import {ServiceLayout} from '#/components/projects/detail/service-layout';

// Tabs
import {GeneralTab} from '#/components/projects/app/detail/general-tab';
import {AppArchitectureTab} from '#/components/projects/app/detail/app-architecture-tab';
import {DomainsTab} from '#/components/projects/app/detail/domains-tab';
import {DeploymentsTab} from '#/components/projects/app/detail/deployments-tab';
import {LogsTab} from '#/components/projects/app/detail/logs-tab';
import {MonitoringTab} from '#/components/projects/common/monitoring-tab';
import {SchedulesTab} from '#/components/projects/app/detail/schedules-tab';
import {VolumeBackupsTab} from '#/components/projects/app/detail/volume-backups-tab';
import {AdvancedTab} from '#/components/projects/app/detail/advanced-tab';

export const Route = createFileRoute('/_app/projects/$id/app/$appId')({
	component: AppDetailPage,
});

function AppDetailPage() {
	const {id, appId} = Route.useParams();
	const parsedAppId = Number(appId);

	const {
		app,
		domains,
		schedules,
		backups,
		deployments,
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

	const appExtras = app as {
		app_name?: string;
	};
	const name = app.name || appExtras.app_name || `Application #${appId}`;

	return (
		<ServiceLayout id={String(id)} name={name}>
			{activeTab === 'General' && (
				<GeneralTab
					app={app}
					handleAction={handleAction}
					handleUpdate={handleUpdate}
					onUpdated={refetchAll}
					domains={domains}
					schedules={schedules}
					backups={backups}
					onOpenArchitecture={() => setActiveTab('Architecture')}
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
				/>
			)}
			{activeTab === 'Advanced' && (
				<AdvancedTab app={app} onUpdated={refetchAll} />
			)}
		</ServiceLayout>
	);
}
