import {createFileRoute} from '@tanstack/react-router';
import {Activity, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {GlobalMonitoringCards} from '#/components/monitoring/global-monitoring-cards';

export const Route = createFileRoute('/_app/monitoring')({
	component: MonitoringPage,
});

function MonitoringPage() {
	return (
		<div className="flex w-full animate-in flex-col gap-6 pb-10 duration-200 fade-in">
			{/* Page Header */}
			<div className="flex flex-col justify-between gap-4 border-b border-border/40 pb-5 sm:flex-row sm:items-center">
				<div>
					<h1 className="flex items-center gap-2.5 text-2xl font-extrabold tracking-tight text-foreground">
						<Activity className="size-6 text-primary" /> System & Docker
						Monitoring
					</h1>
					<p className="mt-1 text-xs text-muted-foreground">
						Real-time telemetry overview for CPU, Memory, Block I/O,
						and Network I/O.
					</p>
				</div>
				<div className="flex items-center gap-2">
					<Button
						variant="outline"
						size="sm"
						onClick={() => window.location.reload()}
						className="flex h-8 items-center gap-1.5 rounded-lg border-border text-xs font-semibold">
						<RefreshCw className="size-3.5" /> Refresh Telemetry
					</Button>
				</div>
			</div>

			{/* Telemetry Cards Grid */}
			<GlobalMonitoringCards />
		</div>
	);
}
