import {RefreshCw, Radio} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {useContainerMonitoring} from '#/hooks/use-container-monitoring';
import {MonitoringCards} from './monitoring/monitoring-cards';

interface MonitoringTabProps {
	app: any;
	appId?: number;
}

export function MonitoringTab({app, appId}: MonitoringTabProps) {
	const resolvedAppId = appId || app?.id || app?.application_id || app?.compose_id || 0;
	const {
		isLive,
		setIsLive,
		isLoading,
		metrics,
		triggerRefresh,
	} = useContainerMonitoring(resolvedAppId);

	const fallbackMetrics = metrics || {
		cpuPercent: 0,
		memPercent: 0,
		memUsage: '0 MB',
		memLimit: '0 MB',
		dockerDiskUsage: '0 MB',
		dockerDiskPercent: 0,
		diskSpacePercent: 0,
		diskSpaceUsed: '0 GB',
		diskSpaceTotal: '0 GB',
		netRx: '0 B',
		netTx: '0 B',
		blockRead: '0 B',
		blockWrite: '0 B',
		pids: 0,
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between gap-4 shadow-xs">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Metrics & Live Telemetry</h3>
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

			{/* Render Resource Metric Cards */}
			<MonitoringCards metrics={fallbackMetrics} />
		</div>
	);
}
