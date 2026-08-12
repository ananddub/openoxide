import {Cpu, HardDrive, Network, Layers, Disc, Database} from 'lucide-react';


interface MonitoringCardsProps {
	metrics: any;
}

export function MonitoringCards({metrics}: MonitoringCardsProps) {
	return (
		<div className="flex flex-col gap-6">
			{/* Core Resource Metrics (CPU, Memory, Docker Disk, Host Disk Space) */}
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
				{/* 1. CPU Usage */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-xs">
					<div className="flex items-center justify-between">
						<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
							<Cpu className="w-3.5 h-3.5 text-primary" /> CPU Usage
						</span>
						<span className="text-xs font-mono font-bold text-primary">{metrics.cpuPercent.toFixed(2)}%</span>
					</div>
					<div className="flex flex-col gap-1">
						<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
							<div
								className="h-full bg-primary transition-all duration-500 rounded-full"
								style={{width: `${Math.min(metrics.cpuPercent, 100)}%`}}
							/>
						</div>
						<span className="text-[10px] text-muted-foreground mt-1">Processor cores utilization</span>
					</div>
				</div>

				{/* 2. Memory Usage */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-xs">
					<div className="flex items-center justify-between">
						<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
							<HardDrive className="w-3.5 h-3.5 text-emerald-500" /> RAM Memory
						</span>
						<span className="text-xs font-mono font-bold text-emerald-500">{metrics.memPercent.toFixed(2)}%</span>
					</div>
					<div className="flex flex-col gap-1">
						<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
							<div
								className="h-full bg-emerald-500 transition-all duration-500 rounded-full"
								style={{width: `${Math.min(metrics.memPercent, 100)}%`}}
							/>
						</div>
						<span className="text-[10px] text-muted-foreground mt-1">
							{metrics.memUsage} / {metrics.memLimit}
						</span>
					</div>
				</div>

				{/* 3. Docker Disk Usage */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-xs">
					<div className="flex items-center justify-between">
						<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
							<Disc className="w-3.5 h-3.5 text-amber-500" /> Docker Disk Usage
						</span>
						<span className="text-xs font-mono font-bold text-amber-500">{metrics.dockerDiskUsage}</span>
					</div>
					<div className="flex flex-col gap-1">
						<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
							<div
								className="h-full bg-amber-500 transition-all duration-500 rounded-full"
								style={{width: `${Math.min(metrics.dockerDiskPercent, 100)}%`}}
							/>
						</div>
						<span className="text-[10px] text-muted-foreground mt-1">Container writable layer size</span>
					</div>
				</div>

				{/* 4. Disk Space */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-3 shadow-xs">
					<div className="flex items-center justify-between">
						<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
							<Database className="w-3.5 h-3.5 text-purple-500" /> Disk Space
						</span>
						<span className="text-xs font-mono font-bold text-purple-500">{metrics.diskSpacePercent.toFixed(1)}%</span>
					</div>
					<div className="flex flex-col gap-1">
						<div className="w-full h-2 bg-secondary rounded-full overflow-hidden">
							<div
								className="h-full bg-purple-500 transition-all duration-500 rounded-full"
								style={{width: `${Math.min(metrics.diskSpacePercent, 100)}%`}}
							/>
						</div>
						<span className="text-[10px] text-muted-foreground mt-1">
							{metrics.diskSpaceUsed} / {metrics.diskSpaceTotal}
						</span>
					</div>
				</div>
			</div>

			{/* Network I/O, Block I/O, and PIDs */}
			<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-5 gap-4">
				{/* Network In RX */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-2 shadow-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
						<Network className="w-3.5 h-3.5 text-blue-500" /> Network In (RX)
					</span>
					<span className="text-lg font-extrabold text-foreground font-mono">{metrics.netRx}</span>
					<span className="text-[10px] text-muted-foreground">Received ingress traffic</span>
				</div>

				{/* Network Out TX */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-2 shadow-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
						<Network className="w-3.5 h-3.5 text-indigo-500" /> Network Out (TX)
					</span>
					<span className="text-lg font-extrabold text-foreground font-mono">{metrics.netTx}</span>
					<span className="text-[10px] text-muted-foreground">Transmitted egress traffic</span>
				</div>

				{/* Block Disk Read */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-2 shadow-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
						<Disc className="w-3.5 h-3.5 text-emerald-500" /> Block Read (I/O)
					</span>
					<span className="text-lg font-extrabold text-foreground font-mono">{metrics.blockRead}</span>
					<span className="text-[10px] text-muted-foreground">Volume read throughput</span>
				</div>

				{/* Block Disk Write */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-2 shadow-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
						<Disc className="w-3.5 h-3.5 text-rose-500" /> Block Write (I/O)
					</span>
					<span className="text-lg font-extrabold text-foreground font-mono">{metrics.blockWrite}</span>
					<span className="text-[10px] text-muted-foreground">Volume write throughput</span>
				</div>

				{/* Process Threads */}
				<div className="bg-card border border-border rounded-xl p-4 flex flex-col gap-2 shadow-xs">
					<span className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
						<Layers className="w-3.5 h-3.5 text-sky-500" /> PIDs / Threads
					</span>
					<span className="text-lg font-extrabold text-primary font-mono">{metrics.pids}</span>
					<span className="text-[10px] text-muted-foreground">Active container processes</span>
				</div>
			</div>
		</div>
	);
}
