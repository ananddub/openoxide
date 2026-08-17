import {Cpu, HardDrive, Network, Layers, Disc, Database} from 'lucide-react';


interface MonitoringCardsProps {
	metrics: any;
	history?: Array<{time: string; cpu: number; mem: number}>;
}

function MetricAreaChart({
	title,
	icon: Icon,
	color,
	gradientId,
	data = [],
	dataKey,
	unit = '%',
	maxVal = 100,
}: {
	title: string;
	icon: any;
	color: string;
	gradientId: string;
	data?: Array<{time: string; [key: string]: any}>;
	dataKey: string;
	unit?: string;
	maxVal?: number;
}) {
	const points = data.map(d => Number(d[dataKey]) || 0);
	const latest = points.length > 0 ? points[points.length - 1] : 0;
	
	const width = 500;
	const height = 130;
	const padding = 15;

	const chartWidth = width - padding * 2;
	const chartHeight = height - padding * 2;

	const coords = points.map((val, idx) => {
		const x = padding + (idx / Math.max(points.length - 1, 1)) * chartWidth;
		const normalizedVal = Math.min(Math.max(val, 0), maxVal);
		const y = height - padding - (normalizedVal / maxVal) * chartHeight;
		return {x, y, val};
	});

	const linePath = coords.map((c, i) => `${i === 0 ? 'M' : 'L'} ${c.x} ${c.y}`).join(' ');
	const areaPath = coords.length > 0 
		? `${linePath} L ${coords[coords.length - 1].x} ${height - padding} L ${coords[0].x} ${height - padding} Z`
		: '';

	return (
		<div className="bg-card border border-border rounded-xl p-4 shadow-xs flex flex-col gap-3">
			<div className="flex items-center justify-between">
				<div className="flex items-center gap-2">
					<Icon className={`size-4 ${color}`} />
					<span className="text-xs font-bold text-foreground">{title}</span>
				</div>
				<span className={`text-sm font-mono font-extrabold ${color}`}>
					{latest.toFixed(1)}{unit}
				</span>
			</div>

			<div className="w-full h-32 relative">
				{coords.length < 2 ? (
					<div className="w-full h-full flex items-center justify-center text-xs text-muted-foreground font-mono border border-dashed border-border/60 rounded-lg">
						Collecting telemetry points...
					</div>
				) : (
					<svg viewBox={`0 0 ${width} ${height}`} className="w-full h-full overflow-visible">
						<defs>
							<linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
								<stop offset="0%" stopColor="currentColor" stopOpacity="0.3" />
								<stop offset="100%" stopColor="currentColor" stopOpacity="0.0" />
							</linearGradient>
						</defs>

						{/* Horizontal Gridlines */}
						{[0.25, 0.5, 0.75].map((ratio) => (
							<line
								key={ratio}
								x1={padding}
								y1={height - padding - ratio * chartHeight}
								x2={width - padding}
								y2={height - padding - ratio * chartHeight}
								stroke="currentColor"
								strokeDasharray="3 3"
								className="text-border/40"
							/>
						))}

						{/* Fill Area */}
						<path d={areaPath} fill={`url(#${gradientId})`} className={color} />

						{/* Line */}
						<path d={linePath} fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" className={color} />

						{/* Live Pulse Dot */}
						{coords.length > 0 && (
							<circle
								cx={coords[coords.length - 1].x}
								cy={coords[coords.length - 1].y}
								r="4"
								className={`${color} fill-current animate-ping`}
							/>
						)}
						{coords.length > 0 && (
							<circle
								cx={coords[coords.length - 1].x}
								cy={coords[coords.length - 1].y}
								r="4"
								className={`${color} fill-current`}
							/>
						)}
					</svg>
				)}
			</div>

			{/* Time Range */}
			<div className="flex justify-between text-[10px] text-muted-foreground font-mono px-1">
				<span>{data[0]?.time || 'Start'}</span>
				<span>{data[data.length - 1]?.time || 'Live'}</span>
			</div>
		</div>
	);
}

export function MonitoringCards({metrics, history = []}: MonitoringCardsProps) {
	return (
		<div className="flex flex-col gap-6">
			{/* Real-time Telemetry Graphs (CPU & Memory) */}
			<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
				<MetricAreaChart
					title="CPU Utilization Graph"
					icon={Cpu}
					color="text-primary"
					gradientId="cpu-grad"
					data={history}
					dataKey="cpu"
					unit="%"
				/>
				<MetricAreaChart
					title="RAM Memory Utilization Graph"
					icon={HardDrive}
					color="text-emerald-500"
					gradientId="mem-grad"
					data={history}
					dataKey="mem"
					unit="%"
				/>
			</div>

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
