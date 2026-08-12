import {createFileRoute} from '@tanstack/react-router';
import {Activity, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {GlobalMonitoringCards} from '#/components/monitoring/global-monitoring-cards';

export const Route = createFileRoute('/_app/monitoring')({
	component: MonitoringPage,
});

function MonitoringPage() {
	return (
		<div className="flex flex-col gap-6 w-full pb-10 animate-in fade-in duration-200">
			{/* Page Header */}
			<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-5">
				<div>
					<h1 className="text-2xl font-extrabold tracking-tight text-foreground flex items-center gap-2.5">
						<Activity className="size-6 text-primary" /> System & Docker Monitoring
					</h1>
					<p className="text-xs text-muted-foreground mt-1">
						Real-time telemetry overview for CPU, Memory, Disk Space, Docker Disk Usage, Block I/O, and Network I/O.
					</p>
				</div>
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={() => window.location.reload()}
						className="h-8 text-xs font-semibold flex items-center gap-1.5 rounded-lg border-border">
						<RefreshCw className="size-3.5" /> Refresh Telemetry
					</Button>
				</div>
			</div>

			{/* Telemetry Cards Grid */}
			<GlobalMonitoringCards />
		</div>
	);
}
