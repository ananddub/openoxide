import {createFileRoute, Link} from '@tanstack/react-router';
import {Box, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {StatusBadge} from '#/components/shared/status-badge';
import {toast} from 'sonner';
import {useAppDetail} from '#/hooks/projects/use-app-detail';
import {ServiceLayout} from '#/components/projects/detail/service-layout';

// Tabs
import {GeneralTab} from '#/components/projects/app/detail/general-tab';
import {AppArchitectureTab} from '#/components/projects/app/detail/app-architecture-tab';
import {EnvironmentTab} from '#/components/projects/common/environment-tab';
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

const TABS = [
	'General',
	'Architecture',
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

	const appExtras = app as {app_name?: string; last_error?: string};
	const name = app.name || appExtras.app_name || `Application #${appId}`;

	return (
		<ServiceLayout
			id={String(id)}
			name={name}
			subtitle={appExtras.app_name || app.name}
			icon={<Box className="size-4 text-primary" strokeWidth={1.7} />}
			actions={
				<>
					<StatusBadge
						status={app.app_status || ''}
						error={appExtras.last_error}
					/>
					<Button
						variant="ghost"
						size="icon"
						title="Refresh status"
						onClick={() => {
							refetchAll();
							toast.success('Application status refreshed');
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
