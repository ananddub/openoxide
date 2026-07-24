import {Activity, Cpu, HardDrive, Network, RefreshCw} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';

interface MonitoringTabProps {
	appId: number;
}

export function MonitoringTab({appId}: MonitoringTabProps) {
	// Query stats with stream: false to get a one-off frame
	const {data, isLoading, error, refetch} = $api.useQuery(
		'get',
		'/deployments/application/{id}/stats',
		{
			params: {
				path: {id: appId},
				query: {
					query: {
						stream: false,
					},
				} as any,
			},
		}
	);

	// Cast the response data to Docker stats structure
	const stats = data as {
		cpu_percent?: number;
		memory_usage?: number;
		memory_limit?: number;
		memory_percent?: number;
		network_rx?: number;
		network_tx?: number;
		block_read?: number;
		block_write?: number;
	} | null;

	const formatBytes = (bytes?: number): string => {
		if (!bytes) return '0 B';
		const units = ['B', 'KB', 'MB', 'GB'];
		let i = 0;
		let v = bytes;
		while (v >= 1024 && i < units.length - 1) {
			v /= 1024;
			i++;
		}
		return `${v.toFixed(1)} ${units[i]}`;
	};

	const fmtPercent = (v?: number): string => {
		return v != null ? `${v.toFixed(2)}%` : '0.00%';
	};

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between gap-4">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Metrics</h3>
					<p className="text-xs text-muted-foreground mt-1">Real-time resource utilization and telemetry streaming</p>
				</div>
				<Button variant="outline" size="sm" onClick={() => refetch()} disabled={isLoading} className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs">
					<RefreshCw className={`w-3.5 h-3.5 ${isLoading ? 'animate-spin' : ''}`} /> Refresh
				</Button>
			</section>

			{error ? (
				<div className="rounded-xl bg-rose-500/5 border border-rose-500/10 p-4 text-xs text-rose-500">
					Failed to stream container telemetry. The container may be stopped or initializing.
				</div>
			) : isLoading ? (
				<div className="flex justify-center py-20 bg-card border border-border rounded-xl">
					<div className="w-6 h-6 border-2 border-primary/30 border-t-primary rounded-full animate-spin" />
				</div>
			) : !stats ? (
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground bg-card border border-border rounded-xl">
					<Activity className="w-8 h-8 opacity-30 mb-2" />
					<p className="text-xs font-semibold">No monitoring statistics available</p>
				</div>
			) : (
				<>
					{/* Core resource metrics */}
					<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
						{/* CPU */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1">
								<Cpu className="w-3.5 h-3.5 text-primary" /> CPU
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground">{fmtPercent(stats.cpu_percent)}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Utilization percent</span>
							</div>
						</div>

						{/* Memory */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1">
								<HardDrive className="w-3.5 h-3.5 text-emerald-500" /> Memory
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground">{fmtPercent(stats.memory_percent)}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">
									{formatBytes(stats.memory_usage)} / {formatBytes(stats.memory_limit)}
								</span>
							</div>
						</div>

						{/* Network RX */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1">
								<Network className="w-3.5 h-3.5 text-blue-500" /> Network In
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground">{formatBytes(stats.network_rx)}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Received traffic</span>
							</div>
						</div>

						{/* Network TX */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1">
								<Network className="w-3.5 h-3.5 text-indigo-500" /> Network Out
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground">{formatBytes(stats.network_tx)}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Transmitted traffic</span>
							</div>
						</div>
					</div>

					{/* Extended Block I/O metrics */}
					<div className="grid grid-cols-2 gap-4">
						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between">
							<span className="text-xs font-semibold text-muted-foreground">Block Disk Read</span>
							<span className="text-sm font-bold text-foreground">{formatBytes(stats.block_read)}</span>
						</div>
						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between">
							<span className="text-xs font-semibold text-muted-foreground">Block Disk Write</span>
							<span className="text-sm font-bold text-foreground">{formatBytes(stats.block_write)}</span>
						</div>
					</div>
				</>
			)}
		</div>
	);
}
