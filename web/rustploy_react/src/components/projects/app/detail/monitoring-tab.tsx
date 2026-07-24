import {useState} from 'react';
import {Activity, Cpu, HardDrive, Network, RefreshCw, Radio} from 'lucide-react';
import {Button} from '#/components/ui/button';
import {$api} from '#/api/query';

interface MonitoringTabProps {
	app: any;
	appId: number;
}

export function MonitoringTab({app, appId}: MonitoringTabProps) {
	const [isLive, setIsLive] = useState(true);

	// Query stats with stream: false to get real-time frames with Dokploy-matching 3s refetch interval
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
		},
		{
			enabled: !!appId,
			refetchInterval: isLive ? 3000 : false,
		}
	);

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
		if (!bytes || isNaN(bytes)) return '0 B';
		const units = ['B', 'KB', 'MB', 'GB', 'TB'];
		let i = 0;
		let v = bytes;
		while (v >= 1024 && i < units.length - 1) {
			v /= 1024;
			i++;
		}
		return `${v.toFixed(1)} ${units[i]}`;
	};

	const fmtPercent = (v?: number): string => {
		return v != null && !isNaN(v) ? `${v.toFixed(2)}%` : '0.00%';
	};

	const cpuVal = stats?.cpu_percent || 0;
	const memVal = stats?.memory_percent || 0;

	return (
		<div className="flex flex-col gap-6">
			{/* Header */}
			<section className="bg-card border border-border rounded-xl p-5 flex items-center justify-between gap-4 shadow-sm">
				<div>
					<h3 className="text-sm font-bold text-foreground">Container Metrics & Telemetry</h3>
					<p className="text-xs text-muted-foreground mt-1">Real-time resource utilization for container: <span className="font-mono text-foreground font-semibold">{app?.app_name || app?.name || 'app'}</span></p>
				</div>
				<div className="flex items-center gap-2">
					<Button
						variant={isLive ? 'default' : 'outline'}
						size="sm"
						onClick={() => setIsLive(!isLive)}
						className="h-8 text-xs font-semibold flex items-center gap-1.5 rounded-lg"
					>
						<Radio className={`w-3.5 h-3.5 ${isLive ? 'animate-pulse text-emerald-400' : ''}`} />
						{isLive ? 'Live Sync (3s)' : 'Paused'}
					</Button>
					<Button
						variant="outline"
						size="sm"
						onClick={() => refetch()}
						disabled={isLoading}
						className="border-border text-foreground hover:bg-muted font-semibold flex items-center gap-1.5 h-8 text-xs rounded-lg"
					>
						<RefreshCw className={`w-3.5 h-3.5 ${isLoading ? 'animate-spin' : ''}`} /> Refresh
					</Button>
				</div>
			</section>

			{error ? (
				<div className="rounded-xl bg-rose-500/5 border border-rose-500/20 p-6 text-center text-xs text-rose-500 font-semibold">
					Failed to stream container telemetry. The application container may be stopped or initializing.
				</div>
			) : isLoading && !stats ? (
				<div className="flex justify-center py-20 bg-card border border-border rounded-xl shadow-sm">
					<div className="flex items-center gap-2 text-xs font-semibold text-muted-foreground">
						<RefreshCw className="w-4 h-4 animate-spin text-primary" /> Sampling live container metrics...
					</div>
				</div>
			) : !stats ? (
				<div className="flex flex-col items-center justify-center py-16 text-center text-muted-foreground bg-card border border-border rounded-xl shadow-sm">
					<Activity className="w-8 h-8 opacity-30 mb-2" />
					<p className="text-xs font-semibold text-foreground">No container metrics available</p>
					<p className="text-[11px] text-muted-foreground mt-1">Ensure the application container is running to view live CPU and memory telemetry</p>
				</div>
			) : (
				<>
					{/* Core Metrics Grid */}
					<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
						{/* CPU */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<div className="flex items-center justify-between">
								<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
									<Cpu className="w-3.5 h-3.5 text-primary" /> CPU Usage
								</span>
								<span className="text-xs font-mono font-bold text-primary">{fmtPercent(stats.cpu_percent)}</span>
							</div>
							<div className="flex flex-col gap-1">
								<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
									<div
										className="h-full bg-primary transition-all duration-500 rounded-full"
										style={{width: `${Math.min(cpuVal, 100)}%`}}
									/>
								</div>
								<span className="text-[10px] text-muted-foreground mt-1">Processor cores utilization</span>
							</div>
						</div>

						{/* Memory */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<div className="flex items-center justify-between">
								<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
									<HardDrive className="w-3.5 h-3.5 text-emerald-500" /> RAM Memory
								</span>
								<span className="text-xs font-mono font-bold text-emerald-500">{fmtPercent(stats.memory_percent)}</span>
							</div>
							<div className="flex flex-col gap-1">
								<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
									<div
										className="h-full bg-emerald-500 transition-all duration-500 rounded-full"
										style={{width: `${Math.min(memVal, 100)}%`}}
									/>
								</div>
								<span className="text-[10px] text-muted-foreground mt-1">
									{formatBytes(stats.memory_usage)} / {formatBytes(stats.memory_limit)}
								</span>
							</div>
						</div>

						{/* Network RX */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
								<Network className="w-3.5 h-3.5 text-blue-500" /> Network In (RX)
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground">{formatBytes(stats.network_rx)}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Total received ingress traffic</span>
							</div>
						</div>

						{/* Network TX */}
						<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-sm">
							<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
								<Network className="w-3.5 h-3.5 text-indigo-500" /> Network Out (TX)
							</span>
							<div className="flex flex-col">
								<span className="text-xl font-extrabold text-foreground">{formatBytes(stats.network_tx)}</span>
								<span className="text-[10px] text-muted-foreground mt-0.5">Total transmitted egress traffic</span>
							</div>
						</div>
					</div>

					{/* Extended Block I/O metrics */}
					<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between shadow-sm">
							<div>
								<span className="text-xs font-semibold text-foreground block">Block Disk Read</span>
								<span className="text-[10px] text-muted-foreground">Volume I/O data read from host disk</span>
							</div>
							<span className="text-sm font-mono font-bold text-foreground">{formatBytes(stats.block_read)}</span>
						</div>

						<div className="bg-card border border-border rounded-xl p-4 flex items-center justify-between shadow-sm">
							<div>
								<span className="text-xs font-semibold text-foreground block">Block Disk Write</span>
								<span className="text-[10px] text-muted-foreground">Volume I/O data written to host disk</span>
							</div>
							<span className="text-sm font-mono font-bold text-foreground">{formatBytes(stats.block_write)}</span>
						</div>
					</div>
				</>
			)}
		</div>
	);
}
