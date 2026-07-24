import {createFileRoute, Link} from '@tanstack/react-router';
import {FolderOpen, Layers2, ChevronRight, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useComposeDetail} from '#/hooks/projects/use-compose-detail';
import {$api} from '#/api/query';

// Tabs
import {ComposeGeneralTab} from '#/components/projects/compose/detail/general-tab';
import {EnvironmentTab} from '#/components/projects/app/detail/environment-tab';
import {ComposeDomainsTab} from '#/components/projects/compose/detail/compose-domains-tab';
import {ComposeDeploymentsTab} from '#/components/projects/compose/detail/compose-deployments-tab';
import {ComposeLogsTab} from '#/components/projects/compose/detail/compose-logs-tab';
import {MonitoringTab} from '#/components/projects/app/detail/monitoring-tab';
import {ComposeSchedulesTab} from '#/components/projects/compose/detail/compose-schedules-tab';
import {ComposeBackupsTab} from '#/components/projects/compose/detail/compose-backups-tab';
import {ComposeContainersTab} from '#/components/projects/compose/detail/compose-containers-tab';
import {ComposeAdvancedTab} from '#/components/projects/compose/detail/compose-advanced-tab';

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
		'Containers',
		'Domains',
		'Deployments',
		'Schedules',
		'Backups',
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
			{/* Header */}
			<header className="flex flex-col border-b border-border/60 pb-0">
				<div className="flex flex-wrap items-center justify-between gap-4">
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

					<div className="flex items-center gap-2">
						<Button variant="outline" size="icon" onClick={() => refetch()} className="w-8 h-8 border-border rounded-lg">
							<RefreshCw className="w-3.5 h-3.5" />
						</Button>
						<span className="inline-flex items-center gap-1.5 text-xs px-2.5 py-1.5 rounded-lg border border-border text-muted-foreground bg-muted/20 font-semibold select-none">
							<Layers2 className="w-3.5 h-3.5" /> Docker Compose
						</span>
					</div>
				</div>

				{/* Tabs Navigation Bar — Identical to Application detail page! */}
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

			{/* Tab Views */}
			<div className="w-full">
				{activeTab === 'General' && (
					<ComposeGeneralTab compose={compose} onAction={handleAction} onUpdated={refetch} />
				)}
				{activeTab === 'Environment' && (
					<EnvironmentTab app={compose} handleUpdate={handleUpdateEnv} />
				)}
				{activeTab === 'Containers' && (
					<ComposeContainersTab compose={compose} onUpdated={refetch} />
				)}
				{activeTab === 'Domains' && (
					<ComposeDomainsTab composeId={parsedComposeId} />
				)}
				{activeTab === 'Deployments' && (
					<ComposeDeploymentsTab composeId={parsedComposeId} />
				)}
				{activeTab === 'Schedules' && (
					<ComposeSchedulesTab compose={compose} />
				)}
				{activeTab === 'Backups' && (
					<ComposeBackupsTab compose={compose} />
				)}
				{activeTab === 'Logs' && (
					<ComposeLogsTab compose={compose} />
				)}
				{activeTab === 'Monitoring' && (
					<MonitoringTab app={compose} />
				)}
				{activeTab === 'Advanced' && (
					<ComposeAdvancedTab compose={compose} />
				)}
			</div>
		</div>
	);
}
