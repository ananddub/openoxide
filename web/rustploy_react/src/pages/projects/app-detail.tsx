import {createFileRoute, Link} from '@tanstack/react-router';
import {FolderOpen, Box, ChevronRight, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useAppDetail} from '#/hooks/projects/use-app-detail';

// Tabs
import {GeneralTab} from '#/components/projects/app/detail/general-tab';
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

function AppDetailPage() {
	const {id, appId} = Route.useParams();
	const parsedAppId = Number(appId);

	const {
		app,
		isLoading,
		refetch,
		activeTab,
		setActiveTab,
		handleAction,
		handleUpdate,
	} = useAppDetail(parsedAppId);

	const TABS = [
		'General',
		'Environment',
		'Domains',
		'Deployments',
		'Preview Deployments',
		'Schedules',
		'Volume Backups',
		'Logs',
		'Monitoring',
		'Advanced',
	] as const;

	return (
		<div className="flex flex-col gap-6 w-full pb-10 animate-in fade-in duration-200">
			{/* Header & Breadcrumbs */}
			{/* Header & Breadcrumbs */}
			<header className="border-b border-border/40">
				{/* Breadcrumb */}
				<div className="flex items-center gap-1.5 text-xs text-muted-foreground mb-3.5 font-semibold">
					<Link to="/projects" className="hover:text-foreground flex items-center gap-1 transition-colors">
						<FolderOpen className="w-3.5 h-3.5" /> Projects
					</Link>
					<ChevronRight className="w-3 h-3 opacity-40" />
					<Link to={`/projects/${id}` as any} className="hover:text-foreground transition-colors">
						Project Details
					</Link>
					<ChevronRight className="w-3 h-3 opacity-40" />
					<span className="text-foreground font-bold">{app?.name || 'Loading...'}</span>
				</div>

				{/* Title row */}
				<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
					<div>
						<h1 className="text-2xl font-extrabold tracking-tight text-foreground bg-gradient-to-r from-foreground to-foreground/80 bg-clip-text">
							{app?.name || 'Loading...'}
						</h1>
						<p className="text-xs text-muted-foreground font-mono mt-1">{app?.app_name}</p>
					</div>

					<div className="flex items-center gap-2">
						<Button variant="outline" size="icon" onClick={() => refetch()} className="w-8 h-8 border-border rounded-lg">
							<RefreshCw className="w-3.5 h-3.5" />
						</Button>
						{(() => {
							const st = (app?.app_status || '').toUpperCase();
							if (['QUEUED', 'STARTING', 'BUILDING', 'DEPLOYING', 'REBUILDING', 'REDEPLOYING'].includes(st)) {
								return (
									<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-amber-500/10 text-amber-500 border-amber-500/30">
										<span className="w-2 h-2 rounded-full bg-amber-500 animate-pulse" />
										{st === 'QUEUED' ? 'QUEUED' : 'STARTING...'}
									</span>
								);
							}
							if (['STOPPING', 'CANCELLING'].includes(st)) {
								return (
									<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-rose-500/10 text-rose-500 border-rose-500/30">
										<span className="w-2 h-2 rounded-full bg-rose-500 animate-pulse" />
										STOPPING...
									</span>
								);
							}
							if (['RUNNING', 'DONE', 'SUCCESS', 'ACTIVE', 'OK'].includes(st)) {
								return (
									<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-emerald-500/10 text-emerald-500 border-emerald-500/30">
										<span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
										RUNNING
									</span>
								);
							}
							if (st === 'ERROR') {
								return (
									<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-rose-500/10 text-rose-500 border-rose-500/30">
										<span className="w-2 h-2 rounded-full bg-rose-500" />
										ERROR
									</span>
								);
							}
							return (
								<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1 rounded-lg border font-bold select-none bg-rose-500/10 text-rose-500 border-rose-500/30">
									<span className="w-2 h-2 rounded-full bg-rose-500" />
									STOPPED
								</span>
							);
						})()}
						<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1.5 rounded-lg border border-border text-muted-foreground bg-muted/20 font-semibold select-none">
							<Box className="w-3.5 h-3.5" /> Rustploy App
						</span>
					</div>
				</div>

				{/* Tabs Navigation Bar */}
				<div className="flex overflow-x-auto mt-6 scrollbar-none gap-2 border-b border-border/40 w-full -mb-[1px]">
					{TABS.map(tab => {
						const isActive = activeTab === tab;
						return (
							<button
								key={tab}
								onClick={() => setActiveTab(tab)}
								className={`px-4 pb-2.5 pt-2 text-xs font-bold whitespace-nowrap border-b-2 transition-all duration-150 -mb-[1px] cursor-pointer ${
									isActive
										? 'border-foreground text-foreground font-extrabold'
										: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border/40'
								}`}
							>
								{tab}
							</button>
						);
					})}
				</div>
			</header>

			{/* Active Tab Panel Content */}
			{isLoading ? (
				<div className="flex justify-center py-20">
					<div className="w-6 h-6 border-2 border-primary/30 border-t-primary rounded-full animate-spin" />
				</div>
			) : app ? (
				<main className="flex-1 mt-1">
					{activeTab === 'General' && <GeneralTab app={app} onUpdated={refetch} handleAction={handleAction} />}
					{activeTab === 'Environment' && <EnvironmentTab app={app} handleUpdate={handleUpdate} />}
					{activeTab === 'Domains' && <DomainsTab app={app} />}
					{activeTab === 'Deployments' && <DeploymentsTab appId={parsedAppId} />}
					{activeTab === 'Preview Deployments' && <PreviewDeploymentsTab app={app} />}
					{activeTab === 'Logs' && <LogsTab app={app} />}
					{activeTab === 'Monitoring' && <MonitoringTab app={app} appId={parsedAppId} />}
					{activeTab === 'Schedules' && <SchedulesTab app={app} />}
					{activeTab === 'Volume Backups' && <VolumeBackupsTab app={app} />}
					{activeTab === 'Advanced' && <AdvancedTab app={app} onUpdated={refetch} />}
				</main>
			) : (
				<div className="text-center py-20 border border-dashed border-border rounded-xl">
					<p className="text-sm font-semibold text-muted-foreground">Failed to load application details.</p>
				</div>
			)}
		</div>
	);
}
