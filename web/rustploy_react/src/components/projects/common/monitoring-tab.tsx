import {RefreshCw, Radio, AlertCircle, Loader2} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useContainerMonitoring, type MonitoringEntityType} from '#/hooks/use-container-monitoring';
import {MonitoringCards} from './monitoring/monitoring-cards';

interface MonitoringTabProps {
	app: any;
	appId?: number;
	entityType?: MonitoringEntityType;
}

export function MonitoringTab({app, appId, entityType = 'application'}: MonitoringTabProps) {
	const resolvedAppId = appId || app?.id || app?.application_id || app?.compose_id || 0;
	const {
		isLive,
		setIsLive,
		isLoading,
		metrics,
		hasError,
		triggerRefresh,
	} = useContainerMonitoring(resolvedAppId, entityType);

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between gap-4 shadow-xs">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Metrics &amp; Live Telemetry</h3>
					<p className="text-xs text-muted-foreground mt-1">
						Real-time resource utilization for container:{' '}
						<span className="font-mono text-foreground font-semibold">
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
						className="h-8 text-xs font-semibold gap-1.5 border-border cursor-pointer"
					>
						<RefreshCw className={`w-3.5 h-3.5 ${isLoading ? 'animate-spin' : ''}`} />
						Refresh
					</Button>
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className={`h-8 text-xs font-semibold gap-1.5 cursor-pointer ${
							isLive ? 'bg-emerald-500 hover:bg-emerald-600 text-white border-transparent' : 'border-border'
						}`}
					>
						<Radio className={`w-3.5 h-3.5 ${isLive ? 'animate-pulse' : ''}`} />
						{isLive ? 'Live Stream' : 'Stream Off'}
					</Button>
				</div>
			</section>

			{/* States */}
			{isLoading && !metrics && (
				<div className="flex items-center justify-center gap-3 py-16 text-muted-foreground">
					<Loader2 className="w-5 h-5 animate-spin text-primary" />
					<span className="text-sm font-medium">Connecting to container stream…</span>
				</div>
			)}

			{hasError && !metrics && (
				<div className="flex flex-col items-center justify-center gap-3 py-16 text-center">
					<AlertCircle className="w-8 h-8 text-destructive/70" />
					<p className="text-sm font-semibold text-foreground">Could not connect to container</p>
					<p className="text-xs text-muted-foreground max-w-xs">
						The container may be stopped or unreachable. Start it first, then click Refresh.
					</p>
					<Button variant="outline" size="sm" onClick={triggerRefresh} className="h-8 text-xs mt-1 cursor-pointer">
						<RefreshCw className="w-3.5 h-3.5 mr-1.5" /> Try Again
					</Button>
				</div>
			)}

			{/* Render Resource Metric Cards only when we have real data */}
			{metrics && <MonitoringCards metrics={metrics} />}
		</div>
	);
}
