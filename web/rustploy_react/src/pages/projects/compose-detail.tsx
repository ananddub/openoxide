import {createFileRoute, Link} from '@tanstack/react-router';
import {FolderOpen, Layers2, ChevronRight} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useComposeDetail} from '#/hooks/projects/use-compose-detail';
import {$api} from '#/api/query';

// Tabs
import {ComposeGeneralTab} from '#/components/projects/compose/detail/general-tab';
import {EnvironmentTab} from '#/components/projects/app/detail/environment-tab';
import {DomainsTab} from '#/components/projects/app/detail/domains-tab';
import {DeploymentsTab} from '#/components/projects/app/detail/deployments-tab';
import {LogsTab} from '#/components/projects/app/detail/logs-tab';
import {MonitoringTab} from '#/components/projects/app/detail/monitoring-tab';
import {SchedulesTab} from '#/components/projects/app/detail/schedules-tab';
import {AdvancedTab} from '#/components/projects/app/detail/advanced-tab';

export const Route = createFileRoute('/_app/projects/$id/compose/$composeId')({
	component: ComposeDetailPage,
});

function ComposeDetailPage() {
	const {id, composeId} = Route.useParams();
	const parsedComposeId = Number(composeId);

	const {
		compose,
		isLoading,
		refetch,
		activeTab,
		setActiveTab,
		handleAction,
	} = useComposeDetail(parsedComposeId);

	const patchCompose = $api.useMutation('patch', '/compose/{id}');

	const handleUpdateEnv = async (body: any) => {
		await patchCompose.mutateAsync({
			params: {path: {id: parsedComposeId}},
			body,
		});
		refetch();
	};

	const TABS = [
		'General',
		'Environment',
		'Domains',
		'Deployments',
		'Schedules',
		'Logs',
		'Monitoring',
		'Advanced',
	] as const;

	if (isLoading) {
		return (
			<div className="flex items-center justify-center min-h-[400px] w-full text-muted-foreground text-xs font-semibold">
				Loading Compose stack details...
			</div>
		);
	}

	if (!compose) {
		return (
			<div className="flex flex-col items-center justify-center min-h-[400px] w-full gap-3 text-center">
				<p className="text-sm font-bold text-foreground">Compose stack not found</p>
				<p className="text-xs text-muted-foreground">The requested compose stack could not be loaded or was removed.</p>
				<Link to={`/projects/${id}` as any}>
					<Button variant="outline" className="text-xs h-8">Return to Project</Button>
				</Link>
			</div>
		);
	}

	return (
		<div className="flex flex-col gap-6 w-full pb-10 animate-in fade-in duration-200">
			{/* Top Bar Header */}
			<div className="flex flex-wrap items-center justify-between gap-4 border-b border-border/40 pb-5">
				<div className="flex flex-col gap-1">
					<div className="flex items-center gap-2 text-xs font-medium text-muted-foreground">
						<Link to={`/projects/${id}` as any} className="hover:text-foreground transition-colors flex items-center gap-1">
							<FolderOpen className="w-3.5 h-3.5" /> Project
						</Link>
						<ChevronRight className="w-3 h-3 text-muted-foreground/40" />
						<span className="text-foreground font-semibold flex items-center gap-1">
							<Layers2 className="w-3.5 h-3.5 text-secondary-foreground" /> {compose.name}
						</span>
					</div>
					<div className="flex items-center gap-3 mt-1">
						<h1 className="text-xl font-bold tracking-tight text-foreground">{compose.name}</h1>
						<span className="px-2.5 py-0.5 rounded-full bg-secondary/80 border border-border/60 text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
							{compose.compose_type || 'DOCKER-COMPOSE'}
						</span>
					</div>
				</div>
			</div>

			{/* Navigation Tabs */}
			<div className="flex items-center gap-1 border-b border-border/60 overflow-x-auto pb-0">
				{TABS.map(tab => (
					<button
						key={tab}
						onClick={() => setActiveTab(tab)}
						className={`px-4 py-2 text-xs font-bold rounded-t-lg transition-all whitespace-nowrap border-b-2 -mb-px ${
							activeTab === tab
								? 'border-primary text-primary bg-primary/5'
								: 'border-transparent text-muted-foreground hover:text-foreground hover:bg-muted/30'
						}`}>
						{tab}
					</button>
				))}
			</div>

			{/* Tab Views */}
			<div className="w-full">
				{activeTab === 'General' && (
					<ComposeGeneralTab compose={compose} onAction={handleAction} onUpdated={refetch} />
				)}
				{activeTab === 'Environment' && (
					<EnvironmentTab app={compose} handleUpdate={handleUpdateEnv} />
				)}
				{activeTab === 'Domains' && (
					<DomainsTab targetId={parsedComposeId} targetType="compose" />
				)}
				{activeTab === 'Deployments' && (
					<DeploymentsTab targetId={parsedComposeId} targetType="compose" />
				)}
				{activeTab === 'Schedules' && (
					<SchedulesTab targetId={parsedComposeId} targetType="compose" />
				)}
				{activeTab === 'Logs' && (
					<LogsTab targetId={compose.name} targetType="compose" />
				)}
				{activeTab === 'Monitoring' && (
					<MonitoringTab app={compose} />
				)}
				{activeTab === 'Advanced' && (
					<AdvancedTab app={compose} onUpdated={refetch} />
				)}
			</div>
		</div>
	);
}
