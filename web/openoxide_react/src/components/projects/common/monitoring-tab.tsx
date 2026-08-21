import {RefreshCw, Radio, AlertCircle, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {
	useContainerMonitoring,
	type MonitoringEntityType,
} from '#/hooks/use-container-monitoring';
import {MonitoringCards} from './monitoring/monitoring-cards';

interface MonitoringTabProps {
	app: any;
	appId?: number;
	entityType?: MonitoringEntityType;
}

export function MonitoringTab({
	app,
	appId,
	entityType = 'application',
}: MonitoringTabProps) {
	const resolvedAppId =
		appId || app?.id || app?.application_id || app?.compose_id || 0;
	const activeMonitoring = useContainerMonitoring(resolvedAppId, entityType);

	const {isLive, setIsLive, isLoading, metrics, hasError, triggerRefresh} =
		activeMonitoring;

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="flex items-center justify-between gap-4 rounded-xl border border-border bg-card p-5 shadow-xs">
				<div>
					<h3 className="text-sm font-bold text-foreground">
						Container Metrics &amp; Live Telemetry
					</h3>
					<p className="mt-1 text-xs text-muted-foreground">
						Real-time resource utilization for container:{' '}
						<span className="font-mono font-semibold text-foreground">
							{app?.app_name || app?.name || 'app'}
						</span>
					</p>
				</div>
				<div className="flex items-center gap-3">
					<Button
						variant="outline"
						size="sm"
						onClick={triggerRefresh}
						disabled={isLoading}
						className="h-8 cursor-pointer gap-1.5 border-border text-xs font-semibold">
						<RefreshCw
							className={`h-3.5 w-3.5 ${isLoading ? 'animate-spin' : ''}`}
						/>
						Refresh
					</Button>
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className={`h-8 cursor-pointer gap-1.5 text-xs font-semibold ${
							isLive
								? 'border-transparent bg-emerald-500 text-white hover:bg-emerald-600'
								: 'border-border'
						}`}>
						<Radio
							className={`h-3.5 w-3.5 ${isLive ? 'animate-pulse' : ''}`}
						/>
						{isLive ? 'Live Stream' : 'Stream Off'}
					</Button>
				</div>
			</section>

			{/* States */}
			{isLoading && !metrics && (
				<div className="flex items-center justify-center gap-3 py-16 text-muted-foreground">
					<Loader2 className="h-5 w-5 animate-spin text-primary" />
					<span className="text-sm font-medium">
						Connecting to container stream…
					</span>
				</div>
			)}

			{hasError && !metrics && (
				<div className="flex flex-col items-center justify-center gap-3 py-16 text-center">
					<AlertCircle className="h-8 w-8 text-destructive/70" />
					<p className="text-sm font-semibold text-foreground">
						Could not connect to container
					</p>
					<p className="max-w-xs text-xs text-muted-foreground">
						The container may be stopped or unreachable. Start it first,
						then click Refresh.
					</p>
					<Button
						variant="outline"
						size="sm"
						onClick={triggerRefresh}
						className="mt-1 h-8 cursor-pointer text-xs">
						<RefreshCw className="mr-1.5 h-3.5 w-3.5" /> Try Again
					</Button>
				</div>
			)}

			{/* Render Resource Metric Cards and Live SVG Graphs when we have real data */}
			{metrics && (
				<MonitoringCards
					metrics={metrics}
					history={activeMonitoring.history}
				/>
			)}
		</div>
	);
}
